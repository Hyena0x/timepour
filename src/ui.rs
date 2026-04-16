use ratatui::{
    layout::Rect,
    style::{Color, Style},
    text::{Line, Span},
};

use crate::{render::blockstack::VisualState, timer::SessionKind};

/// Maps session and visual states to appropriate header text strings.
pub fn title_text_for(kind: SessionKind, visual_state: VisualState) -> &'static str {
    match (kind, visual_state) {
        (SessionKind::Focus, VisualState::Paused) => "focus paused",
        (SessionKind::Break, VisualState::Paused) => "break paused",
        (SessionKind::Focus, VisualState::Completed) => "focus complete",
        (SessionKind::Break, VisualState::Completed) => "break complete",
        (SessionKind::Focus, VisualState::Running) => "focus",
        (SessionKind::Break, VisualState::Running) => "break",
    }
}

/// Provides localized control hint descriptions for the UI footer.
pub fn hint_text_for(visual_state: VisualState) -> &'static str {
    match visual_state {
        VisualState::Paused => "p resume   q quit",
        VisualState::Completed => "done, returning...",
        VisualState::Running => "p pause/resume   q quit",
    }
}

/// Utility for calculating a centered bounding rectangle within a parent area,
/// used primarily for overlaying the Game Over notification.
pub fn centered_rect(area: Rect, max_width: u16, max_height: u16) -> Rect {
    if area.width == 0 || area.height == 0 {
        return area;
    }

    let width = area.width.min(max_width);
    let height = area.height.min(max_height);
    let x = area.x + area.width.saturating_sub(width) / 2;
    let y = area.y + area.height.saturating_sub(height) / 2;

    Rect {
        x,
        y,
        width,
        height,
    }
}

/// A 5x5 Dot Matrix font mapping for digits 0-9 and the separator ':'.
/// These characters are drawn using Unicode block characters to simulate pixel art.
const DIGIT_FONT: [&[&str]; 11] = [
    &["███", "█ █", "█ █", "█ █", "███"], // 0
    &[" █ ", "██ ", " █ ", " █ ", "███"], // 1
    &["███", "  █", "███", "█  ", "███"], // 2
    &["███", "  █", "███", "  █", "███"], // 3
    &["█ █", "█ █", "███", "  █", "  █"], // 4
    &["███", "█  ", "███", "  █", "███"], // 5
    &["███", "█  ", "███", "█ █", "███"], // 6
    &["███", "  █", "  █", "  █", "  █"], // 7
    &["███", "█ █", "███", "█ █", "███"], // 8
    &["███", "█ █", "███", "  █", "███"], // 9
    &["   ", " █ ", "   ", " █ ", "   "], // :
];

/// Translates a raw time string (e.g., "12:59") into a vector of 5 styled `Line`s,
/// representing a massive pixel-art countdown clock.
pub fn render_pixel_digits<'a>(text: &str, color: Color) -> Vec<Line<'a>> {
    let mut lines = vec![String::new(); 5];
    for c in text.chars() {
        let idx = match c {
            '0'..='9' => (c as u32 - '0' as u32) as usize,
            ':' => 10,
            _ => continue,
        };
        for i in 0..5 {
            lines[i].push_str(DIGIT_FONT[idx][i]);
            lines[i].push(' ');
        }
    }

    lines
        .into_iter()
        .map(|s| Line::from(Span::styled(s, Style::default().fg(color))))
        .collect()
}
