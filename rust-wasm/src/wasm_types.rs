use crate::types::{Rank, Suit};
use crate::{types, Card, HandSolution, MaybeCard, Solution, Table};
use anyhow::{bail, Context, Ok, Result};

pub fn from_wasm_table(table: &Table) -> Result<types::Table> {
    Ok(types::Table {
        hand: table
            .hand
            .iter()
            .map(|c| Ok(from_wasm_maybe_card(c)?.context("hand cards must all be set, got blank")?))
            .collect::<Result<Vec<_>>>()?
            .into(),
        board: table
            .board
            .iter()
            .map(|c| Ok(from_wasm_maybe_card(c)?))
            .collect::<Result<Vec<_>>>()?
            .into(),
    })
}

fn from_wasm_maybe_card(card: &MaybeCard) -> Result<Option<types::Card>> {
    card.0
        .as_ref()
        .map(|c| {
            Ok(types::Card {
                rank: match c.rank.as_str() {
                    "2" => Rank::N2,
                    "3" => Rank::N3,
                    "4" => Rank::N4,
                    "5" => Rank::N5,
                    "6" => Rank::N6,
                    "7" => Rank::N7,
                    "8" => Rank::N8,
                    "9" => Rank::N9,
                    "10" => Rank::N10,
                    "J" => Rank::J,
                    "Q" => Rank::Q,
                    "K" => Rank::K,
                    "A" => Rank::A,
                    s => bail!("unrecognized rank \"{s}\""),
                },
                suit: match c.suit.as_str() {
                    "h" => Suit::Hearts,
                    "d" => Suit::Diamonds,
                    "s" => Suit::Spades,
                    "c" => Suit::Clubs,
                    s => bail!("unrecognized suit \"{s}\""),
                },
            })
        })
        .map_or(Ok(None), |v| v.map(Some))
}

pub fn to_wasm_solution(solution: &types::Solution) -> Solution {
    Solution {
        hands: solution
            .hands
            .iter()
            .map(|h| HandSolution {
                hand: h
                    .hand
                    .iter()
                    .map(|c| to_wasm_card(c))
                    .collect::<Vec<_>>()
                    .into(),
                beats_me_count: h.beats_me_count,
                is_beaten_count: h.is_beaten_count,
            })
            .collect(),
        board_possibilities: solution.board_possibilities,
        win_count: solution.win_count,
        lose_count: solution.lose_count,
    }
}

pub fn to_wasm_card(card: &types::Card) -> Card {
    Card {
        rank: match card.rank {
            Rank::N2 => "2".to_owned(),
            Rank::N3 => "3".to_owned(),
            Rank::N4 => "4".to_owned(),
            Rank::N5 => "5".to_owned(),
            Rank::N6 => "6".to_owned(),
            Rank::N7 => "7".to_owned(),
            Rank::N8 => "8".to_owned(),
            Rank::N9 => "9".to_owned(),
            Rank::N10 => "10".to_owned(),
            Rank::J => "J".to_owned(),
            Rank::Q => "Q".to_owned(),
            Rank::K => "K".to_owned(),
            Rank::A => "A".to_owned(),
        },
        suit: match card.suit {
            Suit::Hearts => "h".to_owned(),
            Suit::Diamonds => "d".to_owned(),
            Suit::Spades => "s".to_owned(),
            Suit::Clubs => "c".to_owned(),
        },
    }
}
