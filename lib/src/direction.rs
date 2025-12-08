use crate::Offset;

type Offset2D = Offset<isize, 2>;

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
    pub fn into_offset(self) -> Offset2D {
        match self {
            Direction::Up => Offset::<isize, 2>([0, -1]),
            Direction::Right => Offset::<isize, 2>([1, 0]),
            Direction::Down => Offset::<isize, 2>([0, 1]),
            Direction::Left => Offset::<isize, 2>([-1, 0]),
        }
    }
}
