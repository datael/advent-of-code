const INPUT_TEST: &str = include_str!("../input_test.txt");
const INPUT: &str = include_str!("../input.txt");

fn main() {
    let test_result = solve::<Part1Strategy>(INPUT_TEST);
    println!("Test Part 1: {}", test_result);
    assert!(test_result == 6440);

    let part_1_result = solve::<Part1Strategy>(INPUT);
    println!("Part 1: {}", part_1_result);

    let test_result = solve::<Part2Strategy>(INPUT_TEST);
    println!("Test Part 2: {}", test_result);
    assert!(test_result == 5905);

    let part_2_result = solve::<Part2Strategy>(INPUT);
    println!("Part 2: {}", part_2_result);
}

const MAX_CARDS: usize = 13;

fn solve<S: Strategy>(input: &str) -> u32 {
    let mut hands: Vec<Hand<S>> = input.lines().map(Hand::new).collect();
    hands.sort();

    hands
        .iter()
        .enumerate()
        .map(|(i, hand)| (i + 1) as u32 * hand.bid)
        .sum()
}

trait Strategy {
    fn strength(card: &Card) -> u32;
    fn adjust_counts(counts: [u32; MAX_CARDS]) -> [u32; MAX_CARDS] {
        counts
    }
}

struct Part1Strategy;

impl Strategy for Part1Strategy {
    fn strength(card: &Card) -> u32 {
        match *card {
            Card::Two => 0,
            Card::Three => 1,
            Card::Four => 2,
            Card::Five => 3,
            Card::Six => 4,
            Card::Seven => 5,
            Card::Eight => 6,
            Card::Nine => 7,
            Card::Ten => 8,
            Card::Jack => 9,
            Card::Queen => 10,
            Card::King => 11,
            Card::Ace => 12,
        }
    }
}

#[derive(Debug)]
struct Hand<S: Strategy> {
    cards: Vec<Card>,
    bid: u32,
    category: Category,

    _phantom: std::marker::PhantomData<S>,
}

impl<S: Strategy> Hand<S> {
    fn category(cards: &Vec<Card>) -> Category {
        let counts =
            cards
                .iter()
                .map(|card| S::strength(card))
                .fold([0; MAX_CARDS], |mut acc, slot| {
                    acc[slot as usize] += 1;
                    acc
                });

        let counts = S::adjust_counts(counts);

        match *counts.iter().max().unwrap() {
            5 => Category::FiveOfAKind,
            4 => Category::FourOfAKind,
            3 => {
                if counts.iter().any(|c| *c == 2) {
                    Category::FullHouse
                } else {
                    Category::ThreeOfAKind
                }
            }
            2 => {
                if counts.iter().filter(|c| **c == 2).count() == 2 {
                    Category::TwoPair
                } else {
                    Category::OnePair
                }
            }
            1 => Category::HighCard,
            _ => unreachable!(),
        }
    }

    fn new(input: &str) -> Self {
        let (hand, bid) = input.split_once(" ").unwrap();

        let cards: Vec<Card> = hand.chars().map(|c: char| c.into()).collect();
        let bid = bid.trim().parse().unwrap();
        let category = Self::category(&cards);

        Self {
            cards,
            bid,
            category,
            _phantom: std::marker::PhantomData,
        }
    }
}

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
enum Card {
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Jack,
    Queen,
    King,
    Ace,
}

impl From<char> for Card {
    fn from(value: char) -> Self {
        match value {
            '2' => Card::Two,
            '3' => Card::Three,
            '4' => Card::Four,
            '5' => Card::Five,
            '6' => Card::Six,
            '7' => Card::Seven,
            '8' => Card::Eight,
            '9' => Card::Nine,
            'T' => Card::Ten,
            'J' => Card::Jack,
            'Q' => Card::Queen,
            'K' => Card::King,
            'A' => Card::Ace,
            _ => unreachable!(), // Assuming valid input
        }
    }
}

#[derive(Debug, PartialEq, PartialOrd, Ord, Eq, Copy, Clone)]
enum Category {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

impl<S: Strategy> PartialEq for Hand<S> {
    fn eq(&self, other: &Self) -> bool {
        self.cards == other.cards && self.category == other.category
    }
}

impl<S: Strategy> Eq for Hand<S> {}

impl<S: Strategy> PartialOrd for Hand<S> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl<S: Strategy> Ord for Hand<S> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match self.category.cmp(&other.category) {
            std::cmp::Ordering::Equal => self.cards_ord(&other),
            other => other,
        }
    }
}

impl<S: Strategy> Hand<S> {
    fn cards_ord(&self, other: &Self) -> std::cmp::Ordering {
        for (our_card, their_card) in self.cards.iter().zip(other.cards.iter()) {
            match S::strength(our_card).cmp(&S::strength(their_card)) {
                std::cmp::Ordering::Equal => continue,
                other => return other,
            }
        }

        std::cmp::Ordering::Equal
    }
}

struct Part2Strategy;

impl Strategy for Part2Strategy {
    fn strength(card: &Card) -> u32 {
        match *card {
            Card::Jack => 0,
            Card::Two => 1,
            Card::Three => 2,
            Card::Four => 3,
            Card::Five => 4,
            Card::Six => 5,
            Card::Seven => 6,
            Card::Eight => 7,
            Card::Nine => 8,
            Card::Ten => 9,
            Card::Queen => 10,
            Card::King => 11,
            Card::Ace => 12,
        }
    }

    fn adjust_counts(counts: [u32; MAX_CARDS]) -> [u32; MAX_CARDS] {
        let mut counts = counts;

        //　Find the most common non-J card
        if let Some(max) = counts
            .iter()
            .enumerate()
            .skip(1)
            .max_by(|a, b| a.1.cmp(b.1))
        {
            // If there is one, add the Js to that one
            counts[max.0] += counts[0];
            counts[0] = 0;
        }

        counts
    }
}
