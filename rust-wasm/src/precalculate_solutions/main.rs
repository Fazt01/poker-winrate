use std::fs::File;
use strum::IntoEnumIterator;
use rust_wasm::types::{PrecalculatedSolution, Card, Rank, Suit, Table};
use anyhow::Result;
use async_std::task::block_on;
use serde::{Serialize};
use rust_wasm::solve::{full_deck, solve_with_deck};

fn main() -> Result<()> {
    let deck = full_deck();

    let mut hand_representatives = vec![];

    for rank1 in Rank::iter() {
        for rank2 in Rank::iter() {
            if rank2 > rank1 {
                continue;
            }
            if rank1 != rank2 {
                hand_representatives.push(
                    vec![
                        Card {
                            rank: rank1,
                            suit: Suit::Hearts,
                        },
                        Card {
                            rank: rank2,
                            suit: Suit::Hearts,
                        },
                    ]
                    .into_boxed_slice(),
                )
            }
            hand_representatives.push(
                vec![
                    Card {
                        rank: rank1,
                        suit: Suit::Hearts,
                    },
                    Card {
                        rank: rank2,
                        suit: Suit::Diamonds,
                    },
                ]
                .into_boxed_slice(),
            )
        }
    }

    let mut cache = Default::default();

    let len = hand_representatives.len();
    let mut precalculated_solutions = Vec::with_capacity(len);
    for (i, hand_representative) in hand_representatives.into_iter().enumerate() {
        println!("starting hand {}/{}", i+1, len);
        let table = Table {
            hand: hand_representative.clone(),
            board: vec![None, None, None, None, None].into_boxed_slice(),
        };
        let hand_solution = block_on(solve_with_deck(&table, &deck, &mut cache))?;

        precalculated_solutions.push(PrecalculatedSolution {
            my_hand: hand_representative,
            solution: hand_solution,
        });
    }

    let file = File::create("../precalculated/preflop_solutions.json")?;
    let mut serializer = serde_json::Serializer::new(&file);
    precalculated_solutions.serialize(&mut serializer)?;

    Ok(())
}
