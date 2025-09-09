#[macro_use]
extern crate bencher;

use bencher::Bencher;
use rust_wasm::solve::{solve};
use rust_wasm::types::*;

fn a(bench: &mut Bencher) {
    let table = Table {
        hand: Box::new([
            Card {
                rank: Rank::N2,
                suit: Suit::Hearts,
            },
            Card {
                rank: Rank::N3,
                suit: Suit::Hearts,
            },
        ]),
        board: Box::new([
            Some(Card {
                rank: Rank::N2,
                suit: Suit::Spades,
            }),
            Some(Card {
                rank: Rank::N3,
                suit: Suit::Spades,
            }),
            Some(Card {
                rank: Rank::N4,
                suit: Suit::Spades,
            }),
            Some(Card {
                rank: Rank::N5,
                suit: Suit::Spades,
            }),
            None
        ]),
    };

    bench.iter(|| {
        solve(&table)
    })
}

fn b(bench: &mut Bencher) {
    let table = Table {
        hand: Box::new([
            Card {
                rank: Rank::N2,
                suit: Suit::Hearts,
            },
            Card {
                rank: Rank::N3,
                suit: Suit::Hearts,
            },
        ]),
        board: Box::new([
            Some(Card {
                rank: Rank::N2,
                suit: Suit::Spades,
            }),
            Some(Card {
                rank: Rank::N3,
                suit: Suit::Spades,
            }),
            Some(Card {
                rank: Rank::N4,
                suit: Suit::Spades,
            }),
            None,
            None
        ]),
    };

    bench.iter(|| {
        solve(&table)
    })
}

benchmark_group!(benches, a, b);
benchmark_main!(benches);