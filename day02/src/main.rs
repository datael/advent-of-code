use lib::{self, read_all_lines_from_stdin};
use std::ops::Add;

struct Score(i32);

impl Add for Score {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Score(self.0 + rhs.0)
    }
}

// The Elves begin to set up camp on the beach. To decide whose tent gets to
// be closest to the snack storage, a giant Rock Paper Scissors tournament is
// already in progress.

#[derive(PartialEq, Eq, Copy, Clone, Debug)]
enum Shape {
    Rock,
    Paper,
    Scissors,
}

// Rock Paper Scissors is a game between two players. Each game contains many
// rounds; in each round, the players each simultaneously choose one of Rock,
// Paper, or Scissors using a hand shape.

#[derive(Debug)]
struct Round {
    their_shape: Shape,
    our_shape: Shape,
}

// Then, a winner for that round is
// selected: Rock defeats Scissors, Scissors defeats Paper, and Paper defeats
// Rock. If both players choose the same shape, the round instead ends in a
// draw.

enum Outcome {
    Lose,
    Draw,
    Win,
}

impl Shape {
    fn defeats(&self, other: &Self) -> bool {
        match self {
            Shape::Rock => *other == Shape::Scissors,
            Shape::Paper => *other == Shape::Rock,
            Shape::Scissors => *other == Shape::Paper,
        }
    }
}

impl Round {
    fn calculate_outcome(&self) -> Outcome {
        if self.our_shape == self.their_shape {
            Outcome::Draw
        } else if self.our_shape.defeats(&self.their_shape) {
            Outcome::Win
        } else {
            Outcome::Lose
        }
    }

    // The winner of the whole tournament is the player with the highest score.
    // Your total score is the sum of your scores for each round. The score for a
    // single round is the score for the shape you selected (1 for Rock, 2 for
    // Paper, and 3 for Scissors) plus the score for the outcome of the round (0
    // if you lost, 3 if the round was a draw, and 6 if you won).

    fn calculate_our_score(&self) -> Score {
        let outcome_score: Score = self.calculate_outcome().into();
        let shape_score: Score = self.our_shape.into();

        outcome_score + shape_score
    }
}

// The score for a
// single round is the score for the shape you selected (1 for Rock, 2 for
// Paper, and 3 for Scissors) plus the score for the outcome of the round (0
// if you lost, 3 if the round was a draw, and 6 if you won).

impl From<Shape> for Score {
    fn from(value: Shape) -> Self {
        match value {
            Shape::Rock => Score(1),
            Shape::Paper => Score(2),
            Shape::Scissors => Score(3),
        }
    }
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

// Appreciative of your help yesterday, one Elf gives you an encrypted
// strategy guide (your puzzle input) that they say will be sure to help you
// win. "The first column is what your opponent is going to play: A for Rock,
// B for Paper, and C for Scissors. The second column--" Suddenly, the Elf is
// called away to help with someone's tent.

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

// The second column, you reason, must be what you should play in response: X
// for Rock, Y for Paper, and Z for Scissors. Winning every time would be
// suspicious, so the responses must have been carefully chosen.

trait RoundStrategy {
    fn interpret(input: &String) -> Round;
}

struct Part1Strategy;

impl RoundStrategy for Part1Strategy {
    fn interpret(input: &String) -> Round {
        let mut iter = input.split(" ").into_iter();

        Round {
            their_shape: iter.next().unwrap().to_string().into(),
            our_shape: iter.next().unwrap().to_string().into(),
        }
    }
}

// The Elf finishes helping with the tent and sneaks back over to you.
// "Anyway, the second column says how the round needs to end: X means you
// need to lose, Y means you need to end the round in a draw, and Z means you
// need to win. Good luck!"

struct Part2Strategy;

impl RoundStrategy for Part2Strategy {
    fn interpret(input: &String) -> Round {
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

impl Shape {
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

// Since you can't be sure if the Elf is trying to help you or trick you, you
// should calculate the score you would get if you were to follow the strategy
// guide.

struct TournamentGames {
    rounds: Vec<Round>,
}

impl TournamentGames {
    fn calculate_our_score(&self) -> Score {
        self.rounds
            .iter()
            .map(|round| round.calculate_our_score())
            .fold(Score(0), Score::add)
    }
}

fn main() {
    let input: Vec<_> = read_all_lines_from_stdin().into_iter().collect();

    // What would your total score be if everything goes exactly according to your
    // strategy guide?

    let part1_tournament = TournamentGames {
        rounds: input.iter().map(Part1Strategy::interpret).collect(),
    };

    println!("My score was: {}", part1_tournament.calculate_our_score().0);

    // Following the Elf's instructions for the second column, what would your
    // total score be if everything goes exactly according to your strategy guide?

    let part2_tournament = TournamentGames {
        rounds: input.iter().map(Part2Strategy::interpret).collect(),
    };

    println!("My score was: {}", part2_tournament.calculate_our_score().0);
}
