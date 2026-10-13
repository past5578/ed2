#[derive(Default)]
pub(super) struct CursorPos {
    pub x: usize,
    pub y: usize,
}

pub(super) enum Direction {
    Left,
    Right,
    Up,
    Down,
}

impl CursorPos {
    pub fn move_in(&mut self, direction: Direction, max_x: usize, max_y: usize) {
        match direction {
            Direction::Up => self.y = self.y.saturating_sub(1),
            Direction::Down => self.y = (self.y + 1).min(max_y),
            Direction::Left => self.x = self.x.saturating_sub(1),
            Direction::Right => self.x = (self.x + 1).min(max_x),
        }
    }
}
