use std::{
    io::{self, Stdout},
    time::{Duration, Instant},
};

use crossterm::{
    event::{self, Event, KeyCode},
    execute,
    terminal::{EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode},
};
use ratatui::{
    Terminal,
    backend::CrosstermBackend,
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, BorderType, Borders, Clear, Paragraph},
};

use crate::{
    render::blockstack::{BlockStackRenderer, VisualState},
    timer::{ActiveTimer, SessionKind},
    ui::{centered_rect, hint_text_for, render_medium_pixel_digits, render_pixel_digits},
};

/// A context structure designed to cleanly bundle the many rendering arguments
/// required by the primary view drawing function, fulfilling clean code standards.
struct RenderContext<'a> {
    kind: SessionKind,
    stack: Vec<Line<'a>>,
    next_preview: Vec<Line<'a>>,
    countdown: String,
    visual_state: VisualState,
    playfield_area: Rect,
    side_area: Rect,
    outer_block: Block<'a>,
    area: Rect,
}

/// The main entry point for the Timepour countdown UI experience.
/// Initializes the terminal, manages the run loop, handles user input,
/// and delegates frame computation to the rendering engine.
pub fn run(kind: SessionKind, duration: Option<Duration>) -> color_eyre::Result<()> {
    let started_at = Instant::now();
    let mut timer = ActiveTimer::new(kind, duration, started_at);
    let mut terminal = setup_terminal()?;
    let renderer = BlockStackRenderer::new();
    let mut frozen_tick = 0_u64;

    loop {
        let now = Instant::now();
        let remaining = timer.remaining_at(now);
        let progress = timer.progress_at(now);
        let running_tick = started_at.elapsed().as_millis() as u64 / 120;

        let visual_state = if remaining.is_zero() {
            VisualState::Completed
        } else if timer.is_paused() {
            VisualState::Paused
        } else {
            VisualState::Running
        };

        let tick = if timer.is_paused() {
            frozen_tick
        } else {
            frozen_tick = running_tick;
            running_tick
        };

        terminal.draw(|frame| {
            let area = frame.area();

            let main_color = match kind {
                SessionKind::Focus => Color::Red,
                SessionKind::Break => Color::Cyan, // Or Green
            };

            let show_outer_borders = area.height >= 12 && area.width >= 35;
            let outer_block = if show_outer_borders {
                Block::default()
                    .borders(Borders::ALL)
                    .border_type(BorderType::Thick)
                    .border_style(Style::default().fg(main_color))
                    .title(Line::from(" 🎮 timepour ").alignment(Alignment::Center))
            } else {
                Block::default().padding(ratatui::widgets::Padding::horizontal(1))
            };

            let inner_area = outer_block.inner(area);

            let layout = Layout::default()
                .direction(Direction::Horizontal)
                .constraints([Constraint::Min(0), Constraint::Length(26)])
                .split(inner_area);

            let playfield_area = layout[0];
            let side_area = layout[1];

            let cols = playfield_area.width / 2;
            let rows = playfield_area.height;

            let ctx = RenderContext {
                kind,
                stack: renderer.build_frame(cols, rows, progress, tick, visual_state),
                next_preview: renderer.build_next_piece_preview(cols, rows, progress),
                countdown: format_remaining(remaining),
                visual_state,
                playfield_area,
                side_area,
                outer_block,
                area,
            };

            draw_view(frame, ctx);
        })?;

        if event::poll(Duration::from_millis(100))?
            && let Event::Key(key) = event::read()?
        {
            match key.code {
                KeyCode::Char('q') | KeyCode::Enter | KeyCode::Esc => break,
                KeyCode::Char('p') => {
                    timer.toggle_pause(Instant::now());
                    if timer.is_paused() {
                        frozen_tick = running_tick;
                    }
                }
                _ => {}
            }
        }
    }

    restore_terminal(terminal)?;
    println!("\x07timepour complete");
    Ok(())
}

fn setup_terminal() -> color_eyre::Result<Terminal<CrosstermBackend<Stdout>>> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    Ok(Terminal::new(backend)?)
}

fn restore_terminal(mut terminal: Terminal<CrosstermBackend<Stdout>>) -> color_eyre::Result<()> {
    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    terminal.show_cursor()?;
    Ok(())
}

/// Renders individual widgets given the calculated context map and layouts.
fn draw_view<'a>(frame: &mut ratatui::Frame<'_>, ctx: RenderContext<'a>) {
    frame.render_widget(ctx.outer_block, ctx.area);

    let screen_widget = Paragraph::new(ctx.stack);
    frame.render_widget(screen_widget, ctx.playfield_area);

    if matches!(ctx.visual_state, VisualState::Completed) {
        let game_over_text = " GAME OVER ";
        let overlay_area = centered_rect(ctx.playfield_area, game_over_text.len() as u16 + 4, 3);
        let game_over_widget = Paragraph::new(game_over_text)
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .border_type(BorderType::Thick),
            )
            .style(
                Style::default()
                    .fg(Color::Red)
                    .bg(Color::Black)
                    .add_modifier(Modifier::BOLD),
            )
            .alignment(Alignment::Center);
        frame.render_widget(Clear, overlay_area);
        frame.render_widget(game_over_widget, overlay_area);
    }

    let side_area_h = ctx.side_area.height;
    let side_area_w = ctx.side_area.width;

    // Hide 'TIME' borders to reclaim 2 precious vertical rows if height is extremely tight
    let show_time_borders = side_area_h >= 10;
    let b_size = if show_time_borders { 2 } else { 0 };

    enum ClockSize {
        Giant,
        Medium,
        Small,
    }

    // Size required for text without borders: Giant(5), Medium(3), Small(1)
    let giant_h = 5 + b_size;
    let medium_h = 3 + b_size;

    let clock_size = if side_area_w < 22 {
        ClockSize::Small
    } else if side_area_h >= giant_h {
        ClockSize::Giant
    } else if side_area_h >= medium_h {
        ClockSize::Medium
    } else {
        ClockSize::Small
    };

    let show_next = side_area_h >= 13;
    let show_controls = side_area_h >= 21;
    let show_status = side_area_h >= 17;

    let clock_height = match clock_size {
        ClockSize::Giant => giant_h,
        ClockSize::Medium => medium_h,
        ClockSize::Small => 1 + b_size,
    };

    let mut side_constraints = Vec::new();

    if show_status {
        side_constraints.push(Constraint::Length(4)); // Status
    }

    side_constraints.push(Constraint::Length(clock_height)); // Time

    if show_next {
        side_constraints.push(Constraint::Length(6)); // Next Piece
    }

    if show_controls {
        side_constraints.push(Constraint::Min(0)); // Spacer
        side_constraints.push(Constraint::Length(4)); // Hints
    } else {
        side_constraints.push(Constraint::Min(0)); // Spacer to fill the rest so clock stays top-aligned
    }

    let side_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints(side_constraints)
        .split(ctx.side_area);

    let mode_text = match ctx.kind {
        SessionKind::Focus => "FOCUS MODE",
        SessionKind::Break => "BREAK MODE",
    };

    let status_text = match ctx.visual_state {
        VisualState::Running => "PLAYING",
        VisualState::Paused => "PAUSED",
        VisualState::Completed => "DONE",
    };

    let theme_color = match ctx.kind {
        SessionKind::Focus => Color::Red,
        SessionKind::Break => Color::Cyan, // Or Green
    };

    let border_color = Style::default().fg(theme_color);

    let mut layout_idx = 0;

    if show_status {
        let status_widget = Paragraph::new(vec![
            Line::from(Span::styled(
                mode_text,
                Style::default().add_modifier(Modifier::BOLD),
            )),
            Line::from(Span::styled(status_text, Style::default().fg(theme_color))),
        ])
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_type(BorderType::Thick)
                .title(" STATUS ")
                .border_style(border_color),
        )
        .alignment(Alignment::Center);
        frame.render_widget(status_widget, side_layout[layout_idx]);
        layout_idx += 1;
    }

    let time_block = if show_time_borders {
        Block::default()
            .borders(Borders::ALL)
            .border_type(BorderType::Thick)
            .title(" TIME ")
            .border_style(border_color)
    } else {
        Block::default().padding(ratatui::widgets::Padding::horizontal(1))
    };

    let clock_widget = match clock_size {
        ClockSize::Giant => {
            let pixel_lines = render_pixel_digits(&ctx.countdown, theme_color);
            Paragraph::new(pixel_lines)
                .block(time_block.clone())
                .alignment(Alignment::Center)
        }
        ClockSize::Medium => {
            let pixel_lines = render_medium_pixel_digits(&ctx.countdown, theme_color);
            Paragraph::new(pixel_lines)
                .block(time_block.clone())
                .alignment(Alignment::Center)
        }
        ClockSize::Small => Paragraph::new(Line::from(Span::styled(
            ctx.countdown.to_string(),
            Style::default()
                .fg(theme_color)
                .add_modifier(Modifier::BOLD),
        )))
        .block(time_block.clone())
        .alignment(Alignment::Center),
    };

    frame.render_widget(clock_widget, side_layout[layout_idx]);
    layout_idx += 1;

    if show_next {
        let next_widget = Paragraph::new(ctx.next_preview)
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .border_type(BorderType::Thick)
                    .title(" NEXT ")
                    .border_style(border_color),
            )
            .alignment(Alignment::Center);
        frame.render_widget(next_widget, side_layout[layout_idx]);
        layout_idx += 1;
    }

    layout_idx += 1; // Skip spacer

    if show_controls {
        let hint_widget = Paragraph::new(hint_text_for(ctx.visual_state))
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .border_type(BorderType::Thick)
                    .title(" CONTROLS ")
                    .border_style(border_color),
            )
            .style(Style::default().fg(Color::DarkGray))
            .alignment(Alignment::Center);
        frame.render_widget(hint_widget, side_layout[layout_idx]);
    }
}

/// Helper function defining formatted countdown string strings.
fn format_remaining(duration: Duration) -> String {
    let total = duration.as_secs();
    let minutes = total / 60;
    let seconds = total % 60;
    format!("{minutes:02}:{seconds:02}")
}
