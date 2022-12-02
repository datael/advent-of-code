enum Shape {
    Rock,
    Paper,
    Scissors,
}

impl From<String> for Shape {
    fn from(value: String) -> Self {
        match value.as_str() {
            "A" | "X" => Self::Rock,
            "B" | "Y" => Self::Paper,
            "C" | "Z" => Self::Scissors,
            _ => panic!("Invalid input"),
        }
    }
}

struct Score(i32);

impl From<Shape> for Score {
    fn from(value: Shape) -> Self {
        match value {
            Shape::Rock => Score(1),
            Shape::Paper => Score(2),
            Shape::Scissors => Score(3),
        }
    }
}

struct Round {
    their_shape: Shape,
    our_shape: Shape,
}

enum Outcome {
    Lose,
    Draw,
    Win,
}

impl From<Outcome> for Score {
    fn from(value: Outcome) -> Self {
        match value {
            Outcome::Lose => Score(0),
            Outcome::Draw => Score(3),
            Outcome::Win => Score(6),
        }
    }
}

fn main() {}
