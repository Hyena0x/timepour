use ratatui::{
    style::{Color, Style},
    text::{Line, Span},
};

/// Represents the visual state of the layout relative to the active timer.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum VisualState {
    /// The timer is actively ticking downwards.
    Running,
    /// The timer has been paused by the user.
    Paused,
    /// The timer has reached zero.
    Completed,
}

/// Stores information about the actively falling piece to be rendered.
pub struct ActivePiece {
    /// List of absolute cell coordinates `(row, col)` currently occupied by this piece.
    pub cells: Vec<(u16, u16)>,
    /// The color assigned to the active piece.
    pub color: Color,
    /// Absolute target column where it will land.
    pub spawn_col: u16,
    /// Absolute target row where it will land.
    pub landing_row: u16,
    /// The normalized final rotated shape array.
    pub final_shape: Vec<(u16, u16)>,
}

/// A core rendering engine that translates a linear progress (0.0 to 1.0)
/// into a fully deterministic Tetris block packing game matrix.
///
/// The renderer calculates falling blocks, ghost projections, and settled stacks.
#[derive(Debug, Clone, Copy, Default)]
pub struct BlockStackRenderer;

impl BlockStackRenderer {
    /// Constructs a new `BlockStackRenderer`.
    pub fn new() -> Self {
        Self
    }

    /// Primary rendering loop. Computes the current matrix of blocks based on the terminal
    /// size and the progression of time. Outputs a matrix of ratatui `Line` vectors ready for rendering.
    pub fn build_frame<'a>(
        &self,
        cols: u16,
        rows: u16,
        progress: f32,
        _tick: u64,
        visual_state: VisualState,
    ) -> Vec<Line<'a>> {
        if cols == 0 || rows == 0 {
            return Vec::new();
        }

        let settled = self.settled_grid(cols, rows, progress);
        let active_info = self.active_piece_info(cols, rows, progress, visual_state);

        let mut ghost_cells = Vec::new();
        if let Some(active) = &active_info {
            for (dr, dc) in &active.final_shape {
                let r = active.landing_row.saturating_add(*dr);
                let c = active.spawn_col.saturating_add(*dc);
                if r < rows && c < cols {
                    ghost_cells.push((r, c));
                }
            }
        }

        let mut lines = Vec::with_capacity(rows as usize);
        for row in 0..rows {
            let mut spans = Vec::with_capacity(cols as usize);
            for col in 0..cols {
                let mut cell_filled = None;
                if let Some(active) = &active_info
                    && active.cells.iter().any(|(r, c)| *r == row && *c == col)
                {
                    cell_filled = Some((active.color, "██"));
                }

                let is_ghost = ghost_cells.iter().any(|(r, c)| *r == row && *c == col);

                if let Some((color, text)) = cell_filled {
                    spans.push(Span::styled(text.to_string(), Style::default().fg(color)));
                } else if let Some(color) = settled[row as usize][col as usize] {
                    let text = if matches!(visual_state, VisualState::Paused) {
                        "░░"
                    } else {
                        "██"
                    };
                    let style =
                        if matches!(visual_state, VisualState::Paused | VisualState::Completed) {
                            Style::default().fg(Color::DarkGray)
                        } else {
                            Style::default().fg(color)
                        };
                    spans.push(Span::styled(text.to_string(), style));
                } else if is_ghost && matches!(visual_state, VisualState::Running) {
                    // ghost is wireframe
                    spans.push(Span::styled(
                        "[]".to_string(),
                        Style::default().fg(Color::DarkGray),
                    ));
                } else {
                    spans.push(Span::raw("  "));
                }
            }
            lines.push(Line::from(spans));
        }

        lines
    }

    /// Renders the specialized 4x4 matrix representing the NEXT up piece waiting to fall.
    pub fn build_next_piece_preview<'a>(
        &self,
        cols: u16,
        rows: u16,
        progress: f32,
    ) -> Vec<Line<'a>> {
        let capacity = self.piece_capacity(cols, rows);
        let settled_count = self.settled_piece_count(cols, rows, progress);
        if settled_count >= capacity {
            return vec![Line::from("        "); 4];
        }

        let next_piece = settled_count + 1;
        let shape = self.shape_for(next_piece);
        let color = self.color_for_piece(next_piece);

        let row_offset = if next_piece.is_multiple_of(7) { 0 } else { 1 };

        let mut lines = Vec::new();
        for r in 0..4_u16 {
            let mut spans = Vec::new();
            for c in 0..4_u16 {
                if r >= row_offset
                    && shape
                        .iter()
                        .any(|(dr, dc)| *dr == (r - row_offset) && *dc == c)
                {
                    spans.push(Span::styled("██", Style::default().fg(color)));
                } else {
                    spans.push(Span::raw("  "));
                }
            }
            lines.push(Line::from(spans));
        }
        lines
    }

    fn active_piece_info(
        &self,
        cols: u16,
        rows: u16,
        progress: f32,
        visual_state: VisualState,
    ) -> Option<ActivePiece> {
        if !matches!(visual_state, VisualState::Running) {
            return None;
        }

        let settled = self.settled_grid(cols, rows, progress);
        let settled_count = self.settled_piece_count(cols, rows, progress);
        let capacity = self.piece_capacity(cols, rows);

        if settled_count >= capacity {
            return None;
        }

        let base_shape = self.shape_for(settled_count);
        let (spawn_col, rot) = self.choose_placement(&settled, &base_shape, settled_count);
        let final_shape = Self::rotate_shape(&base_shape, rot);

        if let Some(landing_row) = self.find_landing_row(&settled, &final_shape, spawn_col) {
            let fractional = self.progress_fraction(cols, rows, progress);
            let animated_row = (fractional * (landing_row as f32 + 1.0)).floor() as u16;
            let animated_row = animated_row.min(landing_row);

            let width = self.shape_width(&final_shape);
            let start_col = cols.saturating_sub(width) / 2;
            let col_diff = spawn_col as f32 - start_col as f32;

            // Animation glides piece horizontally in the upper half of its stroke
            let slide_fraction = (fractional * 2.0).clamp(0.0, 1.0);
            let animated_col = (start_col as f32 + col_diff * slide_fraction).round() as i16;
            let animated_col = animated_col.max(0) as u16;

            // Animate the rotation so it visibly spins into place as it falls!
            let animated_rot = ((slide_fraction * 4.0).floor() as u8).min(rot);
            let animated_shape = Self::rotate_shape(&base_shape, animated_rot);

            let color = self.color_for_piece(settled_count);

            let active_cells = animated_shape
                .iter()
                .filter_map(|(dr, dc)| {
                    let row = animated_row.saturating_add(*dr);
                    let col = animated_col.saturating_add(*dc);
                    (row < rows && col < cols).then_some((row, col))
                })
                .collect();

            Some(ActivePiece {
                cells: active_cells,
                color,
                spawn_col,
                landing_row,
                final_shape,
            })
        } else {
            None
        }
    }

    fn settled_grid(&self, cols: u16, rows: u16, progress: f32) -> Vec<Vec<Option<Color>>> {
        let mut grid = vec![vec![None; cols as usize]; rows as usize];
        let settled_count = self.settled_piece_count(cols, rows, progress);

        for piece_index in 0..settled_count {
            let base_shape = self.shape_for(piece_index);
            let (col, rot) = self.choose_placement(&grid, &base_shape, piece_index);
            let final_shape = Self::rotate_shape(&base_shape, rot);
            let color = self.color_for_piece(piece_index);
            self.place_piece(&mut grid, &final_shape, col, color);
        }

        grid
    }

    fn piece_capacity(&self, cols: u16, rows: u16) -> u16 {
        if cols == 0 || rows == 0 {
            return 0;
        }

        let mut grid = vec![vec![None; cols as usize]; rows as usize];
        let mut count = 0;

        loop {
            let base_shape = self.shape_for(count);
            let (col, rot) = self.choose_placement(&grid, &base_shape, count);
            let final_shape = Self::rotate_shape(&base_shape, rot);

            if let Some(row) = self.find_landing_row(&grid, &final_shape, col) {
                for (dr, dc) in &final_shape {
                    let r = row.saturating_add(*dr) as usize;
                    let c = col.saturating_add(*dc) as usize;
                    if r < rows as usize && c < cols as usize {
                        grid[r][c] = Some(Color::Reset);
                    }
                }
                count += 1;
                if row == 0 {
                    break;
                }
            } else {
                break;
            }
            if count >= 10000 {
                break;
            }
        }

        count
    }

    fn settled_piece_count(&self, cols: u16, rows: u16, progress: f32) -> u16 {
        let capacity = self.piece_capacity(cols, rows);
        (progress.clamp(0.0, 1.0) * capacity as f32).floor() as u16
    }

    fn progress_fraction(&self, cols: u16, rows: u16, progress: f32) -> f32 {
        let capacity = self.piece_capacity(cols, rows) as f32;
        let scaled = progress.clamp(0.0, 1.0) * capacity;
        scaled.fract()
    }

    fn rotate_shape(shape: &[(u16, u16)], times: u8) -> Vec<(u16, u16)> {
        let mut res = shape.to_vec();
        for _ in 0..(times % 4) {
            let mut next = Vec::new();
            for &(r, c) in &res {
                // 90 deg clockwise: (r, c) -> (c, -r)
                next.push((c, 100 - r));
            }
            let min_r = next.iter().map(|(r, _)| *r).min().unwrap_or(0);
            let min_c = next.iter().map(|(_, c)| *c).min().unwrap_or(0);
            for p in &mut next {
                p.0 -= min_r;
                p.1 -= min_c;
            }
            res = next;
        }
        res
    }

    fn shape_for(&self, piece_index: u16) -> Vec<(u16, u16)> {
        match piece_index % 7 {
            0 => vec![(0, 0), (0, 1), (0, 2), (0, 3)], // I
            1 => vec![(0, 0), (1, 0), (1, 1), (1, 2)], // L
            2 => vec![(0, 2), (1, 0), (1, 1), (1, 2)], // J
            3 => vec![(0, 0), (0, 1), (1, 0), (1, 1)], // O
            4 => vec![(0, 1), (0, 2), (1, 0), (1, 1)], // S
            5 => vec![(0, 1), (1, 0), (1, 1), (1, 2)], // T
            _ => vec![(0, 0), (0, 1), (1, 1), (1, 2)], // Z
        }
    }

    fn color_for_piece(&self, piece_index: u16) -> Color {
        match piece_index % 7 {
            0 => Color::Cyan,
            1 => Color::Rgb(255, 165, 0),
            2 => Color::Blue,
            3 => Color::Yellow,
            4 => Color::Green,
            5 => Color::Magenta,
            _ => Color::Red,
        }
    }

    fn shape_width(&self, shape: &[(u16, u16)]) -> u16 {
        shape.iter().map(|(_, c)| c + 1).max().unwrap_or(1)
    }

    fn shape_height(&self, shape: &[(u16, u16)]) -> u16 {
        shape.iter().map(|(r, _)| r + 1).max().unwrap_or(1)
    }

    fn choose_placement(
        &self,
        grid: &[Vec<Option<Color>>],
        base_shape: &[(u16, u16)],
        piece_index: u16,
    ) -> (u16, u8) {
        let cols = grid.first().map_or(0, |r| r.len()) as u16;
        let mut best_col = 0;
        let mut best_rot = 0;
        let mut deepest_row = None;
        let mut max_contacts = 0;

        let mut seed = piece_index as u32;
        seed ^= seed << 13;
        seed ^= seed >> 17;
        seed ^= seed << 5;
        let hash = seed.wrapping_mul(2654435761);

        for rot in 0..4 {
            let shape = Self::rotate_shape(base_shape, rot);
            let width = self.shape_width(&shape);
            let span = cols.saturating_sub(width);

            let offset = (hash % (span as u32 + 1)) as u16;

            for i in 0..=span {
                let c = (i + offset) % (span + 1);
                if let Some(row) = self.find_landing_row(grid, &shape, c) {
                    let contacts = self.count_contacts(grid, &shape, row, c);
                    let replace = match deepest_row {
                        None => true,
                        Some(dr) if row > dr => true,
                        Some(dr) if row == dr && contacts > max_contacts => true,
                        _ => false,
                    };

                    if replace {
                        deepest_row = Some(row);
                        best_col = c;
                        best_rot = rot;
                        max_contacts = contacts;
                    }
                }
            }
        }

        (best_col, best_rot)
    }

    fn count_contacts(
        &self,
        grid: &[Vec<Option<Color>>],
        shape: &[(u16, u16)],
        row: u16,
        col: u16,
    ) -> u16 {
        let mut score = 0;
        let rows = grid.len() as u16;
        let cols = grid.first().map_or(0, |l| l.len()) as u16;

        for &(dr, dc) in shape {
            let r = row.saturating_add(dr);
            let c = col.saturating_add(dc);

            if r + 1 == rows
                || grid
                    .get(r as usize + 1)
                    .and_then(|l| l.get(c as usize))
                    .is_some_and(|opt| opt.is_some())
            {
                score += 1;
            }
            if c == 0
                || grid
                    .get(r as usize)
                    .and_then(|l| l.get(c as usize - 1))
                    .is_some_and(|opt| opt.is_some())
            {
                score += 1;
            }
            if c + 1 == cols
                || grid
                    .get(r as usize)
                    .and_then(|l| l.get(c as usize + 1))
                    .is_some_and(|opt| opt.is_some())
            {
                score += 1;
            }
        }
        score
    }

    fn find_landing_row(
        &self,
        grid: &[Vec<Option<Color>>],
        shape: &[(u16, u16)],
        col: u16,
    ) -> Option<u16> {
        let rows = grid.len() as u16;
        let piece_height = self.shape_height(shape);
        let mut row = 0_u16;

        if self.collides(grid, shape, row, col) {
            return None;
        }

        while row + piece_height < rows && !self.collides(grid, shape, row + 1, col) {
            row += 1;
        }

        Some(row)
    }

    fn collides(
        &self,
        grid: &[Vec<Option<Color>>],
        shape: &[(u16, u16)],
        row: u16,
        col: u16,
    ) -> bool {
        let rows = grid.len() as u16;
        let cols = grid.first().map(|r| r.len()).unwrap_or(0) as u16;

        shape.iter().any(|(dr, dc)| {
            let r = row.saturating_add(*dr);
            let c = col.saturating_add(*dc);
            r >= rows || c >= cols || grid[r as usize][c as usize].is_some()
        })
    }

    fn place_piece(
        &self,
        grid: &mut [Vec<Option<Color>>],
        shape: &[(u16, u16)],
        col: u16,
        color: Color,
    ) {
        if let Some(row) = self.find_landing_row(grid, shape, col) {
            for (dr, dc) in shape {
                let r = row.saturating_add(*dr) as usize;
                let c = col.saturating_add(*dc) as usize;
                if let Some(cell) = grid.get_mut(r).and_then(|line| line.get_mut(c)) {
                    *cell = Some(color);
                }
            }
        }
    }
}
