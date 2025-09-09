pub mod types;
pub mod solve;
mod wasm_types;

use anyhow::Error;
use solve as solve_inner;
use wasm_bindgen::prelude::*;
use crate::wasm_types::{from_wasm_table, to_wasm_solution};

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

#[wasm_bindgen(getter_with_clone)]
#[derive(Clone, Debug)]
pub struct Card {
    pub rank: String,
    pub suit: String,
}

#[wasm_bindgen]
impl Card {
    #[wasm_bindgen(constructor)]
    pub fn new(rank: String, suit: String) -> Card {
        Card {
            rank,
            suit,
        }
    }
}

#[wasm_bindgen]
#[derive(Clone, Debug)]
pub struct MaybeCard(Option<Card>);

#[wasm_bindgen]
impl MaybeCard {
    #[wasm_bindgen(constructor)]
    pub fn new(card: Option<Card>) -> MaybeCard {
        MaybeCard(card)
    }
}

#[wasm_bindgen(getter_with_clone)]
#[derive(Clone, Debug)]
pub struct Table {
    pub hand: Box<[MaybeCard]>,
    pub board: Box<[MaybeCard]>,
}

#[wasm_bindgen]
impl Table {
    #[wasm_bindgen(constructor)]
    pub fn new(hand: Box<[MaybeCard]>, board: Box<[MaybeCard]>) -> Table {
        Table {
            hand,
            board,
        }
    }
}

#[wasm_bindgen(getter_with_clone)]
#[derive(Clone, Debug)]
pub struct Solution {
    pub hands: Box<[HandSolution]>,
    pub board_possibilities: u64,
}

#[wasm_bindgen(getter_with_clone)]
#[derive(Clone, Debug)]
pub struct HandSolution {
    pub hand: Box<[Card]>,
    pub beats_me_count: u64,
    pub is_beaten_count: u64,
}

#[wasm_bindgen]
pub fn solve(t: &Table) -> Result<Solution, String> {
    let table = to_str_err(from_wasm_table(t))?;
    let solution = to_str_err(solve_inner::solve(&table))?;
    Ok(to_wasm_solution(&solution))
}

fn to_str_err<T>(v: Result<T, Error>) -> Result<T, String> {
    v.map_err(|e| format!("{:#}", e))
}