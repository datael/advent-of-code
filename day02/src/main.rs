use lib::{self, read_all_lines_from_stdin};
use std::ops::Add;

#[derive(PartialEq, Eq, Copy, Clone, Debug)]
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

impl Add for Score {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Score(self.0 + rhs.0)
    }
}

impl From<Shape> for Score {
    fn from(value: Shape) -> Self {
        match value {
            Shape::Rock => Score(1),
            Shape::Paper => Score(2),
            Shape::Scissors => Score(3),
        }
    }
}

impl Shape {
    fn beats(&self, other: &Self) -> bool {
        match self {
            Shape::Rock => *other == Shape::Scissors,
            Shape::Paper => *other == Shape::Rock,
            Shape::Scissors => *other == Shape::Paper,
        }
    }
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

#[derive(Debug)]
struct Round {
    their_shape: Shape,
    our_shape: Shape,
}

impl Round {
    fn calculate_outcome(&self) -> Outcome {
        if self.our_shape == self.their_shape {
            Outcome::Draw
        } else if self.our_shape.beats(&self.their_shape) {
            Outcome::Win
        } else {
            Outcome::Lose
        }
    }

    fn calculate_our_score(&self) -> Score {
        let outcome_score: Score = self.calculate_outcome().into();
        let shape_score: Score = self.our_shape.into();

        outcome_score + shape_score
    }
}

impl From<String> for Round {
    fn from(value: String) -> Self {
        let mut iter = value.split(" ").into_iter();

        Round {
            their_shape: iter.next().unwrap().to_string().into(),
            our_shape: iter.next().unwrap().to_string().into(),
        }
    }
}

struct Tournament {
    rounds: Vec<Round>,
}

impl Tournament {
    fn calculate_our_score(&self) -> Score {
        self.rounds
            .iter()
            .map(|round| round.calculate_our_score())
            .fold(Score(0), Score::add)
    }
}

fn main() {
    let tournament = Tournament {
        rounds: read_all_lines_from_stdin()
            .into_iter()
            .map(Into::into)
            .collect(),
    };

    println!("My score was: {}", tournament.calculate_our_score().0);
}
