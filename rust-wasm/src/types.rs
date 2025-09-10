use strum_macros::EnumIter;

pub const RANK_COUNT: usize = 13;
pub const SUIT_COUNT: usize = 4;
pub const COMBINATION_SIZE: usize = 5;

#[derive(EnumIter, Default, Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub enum Rank {
    #[default]
    N2,
    N3,
    N4,
    N5,
    N6,
    N7,
    N8,
    N9,
    N10,
    J,
    Q,
    K,
    A,
}

#[derive(EnumIter, Default, Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub enum Suit {
    #[default]
    Hearts,
    Diamonds,
    Spades,
    Clubs,
}

#[derive(Default, Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct Card {
    pub rank: Rank,
    pub suit: Suit,
}

#[derive(Debug)]
pub struct Table {
    pub hand: Box<[Card]>,
    pub board: Box<[Option<Card>]>,
}

#[derive(Debug)]
pub struct Solution {
    pub hands: Box<[HandSolution]>,
    pub board_possibilities: u64,
}

#[derive(Debug)]
pub struct HandSolution {
    pub hand: Box<[Card]>,
    pub beats_me_count: u64,
    pub is_beaten_count: u64,
}

#[derive(Copy, Clone, Debug, Ord, PartialOrd, Eq, PartialEq)]
pub enum Combination {
    HighCard([Rank; 5]), // all cards, from highest
    Pair([Rank; 4]), // rank of the pair, ranks of remaining 3 non-pair cards from highest
    TwoPairs([Rank; 3]), // slice of 2 pair ranks (highest first), and the rank of one remaining card
    ThreeOfAKind([Rank; 3]), // rank of three of a kind, and rank of 2 remaining cards from highest
    Straight(Rank), // rank of highest card in the straight
    Flush([Rank; 5]), // all cards, from highest
    FullHouse([Rank; 2]), // rank of three of a kind, rank of two of a kind
    FourOfAKind([Rank; 2]), // rank of four of a kind, rank of remaining card
    StraightFlush(Rank) // rank of highest card
}

impl Combination {
    pub fn score(&self) -> u64 {
        const WEIGHT_MULTIPLIER: u64 = RANK_COUNT as u64;
        const WEIGHTS: [u64; COMBINATION_SIZE] = [
            WEIGHT_MULTIPLIER.pow(5),
            WEIGHT_MULTIPLIER.pow(4),
            WEIGHT_MULTIPLIER.pow(3),
            WEIGHT_MULTIPLIER.pow(2),
            WEIGHT_MULTIPLIER,
        ];
        const COMBINATION_TYPE_WEIGHT: u64 = WEIGHT_MULTIPLIER.pow(6);
        match self {
            Combination::HighCard(ranks) => {
                ranks.iter().zip(WEIGHTS).map(|(&r, w)| w * r as u64).sum::<u64>()
            }
            Combination::Pair(ranks) => {
                1 * COMBINATION_TYPE_WEIGHT +
                    ranks.iter().zip(WEIGHTS).map(|(&r, w)| w * r as u64).sum::<u64>()
            }
            Combination::TwoPairs(ranks) => {
                2 * COMBINATION_TYPE_WEIGHT +
                    ranks.iter().zip(WEIGHTS).map(|(&r, w)| w * r as u64).sum::<u64>()
            }
            Combination::ThreeOfAKind(ranks) => {
                3 * COMBINATION_TYPE_WEIGHT +
                    ranks.iter().zip(WEIGHTS).map(|(&r, w)| w * r as u64).sum::<u64>()
            }
            Combination::Straight(rank) => {
                4 * COMBINATION_TYPE_WEIGHT + (*rank) as u64 * WEIGHTS[0]
            }
            Combination::Flush(ranks) => {
                5 * COMBINATION_TYPE_WEIGHT +
                    ranks.iter().zip(WEIGHTS).map(|(&r, w)| w * r as u64).sum::<u64>()
            }
            Combination::FullHouse(ranks) => {
                6 * COMBINATION_TYPE_WEIGHT +
                    ranks.iter().zip(WEIGHTS).map(|(&r, w)| w * r as u64).sum::<u64>()
            }
            Combination::FourOfAKind(ranks) => {
                7 * COMBINATION_TYPE_WEIGHT +
                    ranks.iter().zip(WEIGHTS).map(|(&r, w)| w * r as u64).sum::<u64>()
            }
            Combination::StraightFlush(rank) => {
                8 * COMBINATION_TYPE_WEIGHT + (*rank) as u64 * WEIGHTS[0]
            }
        }
    }
}

#[derive(Default, Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct ReducedCard {
    pub is_flush: bool,
    pub rank: Rank,
}
