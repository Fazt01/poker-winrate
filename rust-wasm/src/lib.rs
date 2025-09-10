pub mod signal;
pub mod solve;
pub mod types;
mod wasm_types;

use crate::wasm_types::{from_wasm_table, to_wasm_solution};
use anyhow::Error;
use solve as solve_inner;
use wasm_bindgen::prelude::*;

#[cfg(target_arch = "wasm32")]
mod platform {
    use wasm_bindgen::prelude::*;

    #[wasm_bindgen]
    extern "C" {
        #[wasm_bindgen(js_namespace = console, js_name = log)]
        fn console_log(s: &str);
    }

    pub fn log(s: &str) {
        console_log(s);
    }
}

#[cfg(not(target_arch = "wasm32"))]
mod platform {
    pub fn log(s: &str) {
        println!("{s}");
    }
}

pub use platform::log;

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
        Card { rank, suit }
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
        Table { hand, board }
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
pub async fn solve(
    cancellation_token: &signal::AbortSignal,
    t: &Table,
) -> Result<Solution, String> {
    let mut drop_detector = signal::DropDetector {
        s: "parsing",
        f: |s| log(format!("dropped {}", s).as_str()),
    };
    let table = to_str_err(from_wasm_table(t))?;
    drop_detector.s = "pending";
    let solution_result = solve_inner::solve(cancellation_token.clone(), &table).await;
    drop_detector.s = "error result";
    let solution = to_str_err(solution_result)?;
    let result = Ok(to_wasm_solution(&solution));
    drop_detector.s = "success result";
    result
}

fn to_str_err<T>(v: Result<T, Error>) -> Result<T, String> {
    v.map_err(|e| format!("{:#}", e))
}
