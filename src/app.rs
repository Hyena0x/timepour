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
    ui::{centered_rect, hint_text_for, render_pixel_digits},
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
pub fn run(
    kind: SessionKind,
    minutes: Option<u64>,
    seconds: Option<u64>,
) -> color_eyre::Result<()> {
    let started_at = Instant::now();
    let mut timer = ActiveTimer::new(kind, minutes, seconds, started_at);
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

            let outer_block = Block::default()
                .borders(Borders::ALL)
                .border_type(BorderType::Thick)
                .border_style(Style::default().fg(main_color))
                .title(Line::from(" 🎮 timepour ").alignment(Alignment::Center));

            let inner_area = outer_block.inner(area);

            let layout = Layout::default()
                .direction(Direction::Horizontal)
                .constraints([Constraint::Percentage(70), Constraint::Percentage(30)])
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

    let side_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(4), // Status
            Constraint::Length(7), // Time (Requires 5 for font + 2 limits)
            Constraint::Length(6), // Next Piece
            Constraint::Min(0),    // Spacer
            Constraint::Length(4), // Hints
        ])
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
    frame.render_widget(status_widget, side_layout[0]);

    let pixel_lines = render_pixel_digits(&ctx.countdown, theme_color);
    let clock_widget = Paragraph::new(pixel_lines)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_type(BorderType::Thick)
                .title(" TIME ")
                .border_style(border_color),
        )
        .alignment(Alignment::Center);

    frame.render_widget(clock_widget, side_layout[1]);

    let next_widget = Paragraph::new(ctx.next_preview)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_type(BorderType::Thick)
                .title(" NEXT ")
                .border_style(border_color),
        )
        .alignment(Alignment::Center);
    frame.render_widget(next_widget, side_layout[2]);

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
    frame.render_widget(hint_widget, side_layout[4]);
}

/// Helper function defining formatted countdown string strings.
fn format_remaining(duration: Duration) -> String {
    let total = duration.as_secs();
    let minutes = total / 60;
    let seconds = total % 60;
    format!("{minutes:02}:{seconds:02}")
}
