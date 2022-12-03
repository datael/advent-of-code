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

    fn losing_play(&self) -> Self {
        match self {
            Shape::Rock => Shape::Scissors,
            Shape::Paper => Shape::Rock,
            Shape::Scissors => Shape::Paper,
        }
    }

    fn drawing_play(&self) -> Self {
        *self
    }

    fn winning_play(&self) -> Self {
        match self {
            Shape::Rock => Shape::Paper,
            Shape::Paper => Shape::Scissors,
            Shape::Scissors => Shape::Rock,
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

impl From<String> for Outcome {
    fn from(value: String) -> Self {
        match value.as_str() {
            "X" => Self::Lose,
            "Y" => Self::Draw,
            "Z" => Self::Win,
            _ => panic!("Invalid input"),
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

trait RoundStrategy {
    fn interpret(input: String) -> Round;
}

struct Part1Strategy;

impl RoundStrategy for Part1Strategy {
    fn interpret(input: String) -> Round {
        let mut iter = input.split(" ").into_iter();

        Round {
            their_shape: iter.next().unwrap().to_string().into(),
            our_shape: iter.next().unwrap().to_string().into(),
        }
    }
}

struct Part2Strategy;

impl RoundStrategy for Part2Strategy {
    fn interpret(input: String) -> Round {
        let mut iter = input.split(" ").into_iter();
        let their_shape: Shape = iter.next().unwrap().to_string().into();
        let our_shape = match iter.next().unwrap().to_string().into() {
            Outcome::Lose => their_shape.losing_play(),
            Outcome::Draw => their_shape.drawing_play(),
            Outcome::Win => their_shape.winning_play(),
        };

        Round {
            their_shape,
            our_shape,
        }
    }
}

fn main() {
    let tournament = Tournament {
        rounds: read_all_lines_from_stdin()
            .into_iter()
            .map(Part2Strategy::interpret)
            .collect(),
    };

    println!("My score was: {}", tournament.calculate_our_score().0);
}
