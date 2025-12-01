use crate::Offset;

#[derive(Debug, Hash, PartialEq, Eq, Clone, Copy)]
pub enum Direction {
    Up,
    Right,
    Down,
    Left,
}

impl Direction {
    pub fn iter() -> impl Iterator<Item = Direction> {
        [
            Direction::Up,
            Direction::Right,
            Direction::Down,
            Direction::Left,
        ]
        .iter()
        .copied()
    }

    #[must_use]
    pub fn into_offset(self) -> Offset {
        match self {
            Direction::Up => Offset { x: 0, y: -1 },
            Direction::Right => Offset { x: 1, y: 0 },
            Direction::Down => Offset { x: 0, y: 1 },
            Direction::Left => Offset { x: -1, y: 0 },
        }
    }
}
