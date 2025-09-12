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
    use std::fmt::{Debug, Display, Formatter};
    use wasm_bindgen::prelude::*;
    use wasm_bindgen_futures::JsFuture;
    use web_sys::{window, Response};
    use anyhow::{Context, Result};
    use futures::AsyncReadExt;
    use wasm_streams::ReadableStream;

    #[wasm_bindgen]
    extern "C" {
        #[wasm_bindgen(js_namespace = console)]
        pub fn log(s: &str);
    }

    #[derive(Debug)]
    pub struct JsErr(String);

    impl std::error::Error for JsErr {}

    impl Display for JsErr {
        fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
            f.write_str(&self.0)
        }
    }

    impl<T: JsCast> From<T> for JsErr {
        fn from(_v: T) -> Self {
            JsErr("JS cast error".to_owned()) // might be useful extract some information here before throwing away JsCast
        }
    }

    pub async fn read_file(path: &str) -> Result<Box<[u8]>> {
        let window = window().context("no window")?;
        let resp_value = JsFuture::from(window.fetch_with_str(&path)).await.map_err(JsErr::from)?;

        let resp: Response = resp_value.dyn_into().map_err(JsErr::from)?;

        let raw_body = resp.body().context("no body")?;
        let body = ReadableStream::from_raw(raw_body.dyn_into().map_err(JsErr::from)?);
        let mut stream = body.into_async_read();

        let mut buf: Vec<u8> = Default::default();
        stream.read_to_end(&mut buf).await?;

        Ok(buf.into())
    }

    pub static ROOT_PATH: &'static str = "";
}

#[cfg(not(target_arch = "wasm32"))]
mod platform {
    use async_std::io::ReadExt;
    use anyhow::Result;

    pub fn log(s: &str) {
        println!("{s}");
    }

    pub async fn read_file(path: &str) -> Result<Box<[u8]>> {
        let mut buf = Default::default();
        let mut file = async_std::fs::File::open(path).await?;
        file.read_to_end(&mut buf).await?;
        Ok(buf.into())
    }

    pub static ROOT_PATH: &'static str = "../";
}

pub use platform::{log, read_file, ROOT_PATH};

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
    pub win_count: u64,
    pub lose_count: u64,
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
