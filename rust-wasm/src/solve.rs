use crate::types::{
    Card, Combination, HandSolution, Rank, ReducedCard, Solution, Suit, Table, RANK_COUNT,
    SUIT_COUNT,
};
use crate::{log, signal};
use anyhow::{bail, Result};
use async_std::task;
use futures::future;
use futures::future::Either;
use itertools::Itertools;
use std::cmp::{Ordering, Reverse};
use std::collections::{HashMap, HashSet};
use std::time::Duration;
use strum::IntoEnumIterator;
use web_time::Instant;

pub async fn solve(cancellation_token: signal::AbortSignal, table: &Table) -> Result<Solution> {
    let deck = full_deck();
    let fut = solve_with_deck(table, &deck);
    futures::pin_mut!(fut);

    match future::select(fut, cancellation_token).await {
        Either::Left((result, _)) => result,
        Either::Right(_) => {
            bail!("solve operation cancelled")
        }
    }
}

fn full_deck() -> Box<[Card]> {
    Suit::iter()
        .cartesian_product(Rank::iter())
        .map(|(suit, rank)| Card { rank, suit })
        .collect_vec()
        .into_boxed_slice()
}

async fn solve_with_deck(table: &Table, deck: &[Card]) -> Result<Solution> {
    let mut yield_timer = YieldTimer::new(Duration::from_millis(50));
    let mut used_cards = table.hand.to_vec();
    used_cards.extend(table.board.iter().flatten());
    let remaining_deck: Vec<_> = deck
        .iter()
        .cloned()
        .filter(|card| !used_cards.contains(card))
        .collect();
    if deck.len() != remaining_deck.len() + used_cards.len() {
        let mut used_cards_set: HashSet<_> = Default::default();
        for &card in &used_cards {
            if !used_cards_set.insert(card) {
                bail!("card \"{card:#?}\" is used multiple times")
            }
        }
        bail!(
            "used cards \"{:#?}\" not from deck",
            used_cards
                .iter()
                .filter(|c| !deck.contains(c))
                .collect_vec()
        );
    }
    let mut candidate_hands = vec![];
    for (i, &card1) in remaining_deck[..remaining_deck.len() - 1]
        .iter()
        .enumerate()
    {
        for &card2 in &remaining_deck[i + 1..] {
            candidate_hands.push(vec![card1, card2].into_boxed_slice())
        }
    }
    candidate_hands.push(table.hand.clone());

    let choose = table.board.iter().filter(|x| x.is_none()).count();
    let choose_from = remaining_deck.len() - 2;

    let mut cache = Default::default();

    let mut hands = Vec::with_capacity(candidate_hands.len());

    let mut last_time = yield_timer.last;
    for (i, candidate_hand) in candidate_hands.iter().enumerate() {
        let current_time = yield_timer.yield_check().await;
        if current_time.duration_since(last_time) >= Duration::from_millis(1000) {
            log(format!("{}/{} hands evaluated", i, candidate_hands.len()).as_str());
            last_time = current_time;
        }
        hands.push(
            hand_solution(
                &candidate_hand,
                table,
                &remaining_deck,
                &mut cache,
                &mut yield_timer,
            )
            .await,
        )
    }

    hands.sort_by_key(|hand| {
        (
            hand.beats_me_count as i64 - hand.is_beaten_count as i64,
            // on same of the win/loss difference, fewer ties is better
            // (just so the result doesn't look so fragmented - expected value is the same though)
            hand.is_beaten_count,
            hand.hand.clone(),
        )
    });
    Ok(Solution {
        hands: hands.into(),
        board_possibilities: n_choose_m(choose_from, choose),
    })
}

fn n_choose_m(n: usize, m: usize) -> u64 {
    ((n as u64 - m as u64 + 1u64)..=(n as u64)).product::<u64>()
        / (1u64..=(m as u64)).product::<u64>()
}

async fn hand_solution(
    candidate_hand: &[Card],
    table: &Table,
    remaining_deck: &[Card],
    cache: &mut HashMap<Box<[ReducedCard]>, u64>,
    yield_timer: &mut YieldTimer,
) -> HandSolution {
    let remaining_deck: Vec<_> = remaining_deck
        .into_iter()
        .cloned()
        .filter(|card| !candidate_hand.contains(card))
        .collect();
    let fixed_board_cards = table.board.iter().cloned().flatten().collect_vec();
    let fixed_cards_count = fixed_board_cards.len() + table.hand.len();
    let mut beats_me_count = 0;
    let mut is_beaten_count = 0;
    let cards_to_fill = table.board.iter().filter(|x| x.is_none()).count();
    let mut fill_cards_map_i = Vec::from_iter(0..cards_to_fill);

    let mut my_final_cards: Vec<_> = fixed_board_cards
        .iter()
        .cloned()
        .chain(table.hand.iter().cloned())
        .chain(fill_cards_map_i.iter().map(|&i| remaining_deck[i]))
        .collect();
    let mut candidate_final_cards: Vec<_> = fixed_board_cards
        .iter()
        .cloned()
        .chain(candidate_hand.iter().cloned())
        .chain(fill_cards_map_i.iter().map(|&i| remaining_deck[i]))
        .collect();

    let mut i: u64 = 1;
    'outer: loop {
        const YIELD_EACH_N: u64 = 2000;
        if i % YIELD_EACH_N == 0 {
            yield_timer.yield_check().await;
        }
        i += 1;

        let my_combination = best_combination(&my_final_cards, cache);
        let candidate = best_combination(&candidate_final_cards, cache);

        match my_combination.cmp(&candidate) {
            Ordering::Less => {
                beats_me_count += 1;
            }
            Ordering::Equal => {}
            Ordering::Greater => {
                is_beaten_count += 1;
            }
        }

        if cards_to_fill == 0 {
            break
        }
        let mut fill_card_i = cards_to_fill - 1;

        while fill_cards_map_i[fill_card_i] + cards_to_fill >= remaining_deck.len() + fill_card_i {
            if fill_card_i == 0 {
                break 'outer;
            }
            fill_card_i -= 1;
        }

        fill_cards_map_i[fill_card_i] += 1;
        my_final_cards[fixed_cards_count + fill_card_i] =
            remaining_deck[fill_cards_map_i[fill_card_i]];
        candidate_final_cards[fixed_cards_count + fill_card_i] =
            remaining_deck[fill_cards_map_i[fill_card_i]];
        fill_card_i += 1;

        while fill_card_i < cards_to_fill {
            fill_cards_map_i[fill_card_i] = fill_cards_map_i[fill_card_i - 1] + 1;
            my_final_cards[fixed_cards_count + fill_card_i] =
                remaining_deck[fill_cards_map_i[fill_card_i]];
            candidate_final_cards[fixed_cards_count + fill_card_i] =
                remaining_deck[fill_cards_map_i[fill_card_i]];
            fill_card_i += 1;
        }
    }

    HandSolution {
        hand: candidate_hand.into(),
        beats_me_count,
        is_beaten_count,
    }
}

fn best_combination(cards: &[Card], cache: &mut HashMap<Box<[ReducedCard]>, u64>) -> u64 {
    let reduced = reduce_card_set(cards);
    *cache
        .entry(reduced)
        .or_insert_with_key(|reduced| best_combination_from_sorted(&reduced).score())
}

fn reduce_card_set(cards: &[Card]) -> Box<[ReducedCard]> {
    let mut suits_counts = [0; SUIT_COUNT];
    for card in cards {
        suits_counts[card.suit as usize] += 1
    }
    let flush_suit = Suit::iter().find(|&suit| suits_counts[suit as usize] >= 5);
    let mut reduced_set = Vec::with_capacity(cards.len());
    for card in cards {
        reduced_set.push(ReducedCard {
            is_flush: flush_suit
                .map(|flush_suit| card.suit == flush_suit)
                .unwrap_or(false),
            rank: card.rank,
        })
    }
    reduced_set.sort_by(|lhs, rhs| lhs.cmp(rhs).reverse());
    reduced_set.into_boxed_slice()
}

fn best_combination_from_sorted(cards_descending: &[ReducedCard]) -> Combination {
    let mut flush_end_i = 0;
    while flush_end_i < cards_descending.len() && cards_descending[flush_end_i].is_flush {
        flush_end_i += 1
    }
    let suited_cards = &cards_descending[0..flush_end_i];
    if let Some(rank) = find_straight_highest_rank(suited_cards) {
        return Combination::StraightFlush(rank);
    }
    let mut ranks_counts = [0; RANK_COUNT];
    for card in cards_descending {
        ranks_counts[card.rank as usize] += 1
    }
    let mut same_of_a_kind: Vec<_> = Rank::iter()
        .map(|rank| (ranks_counts[rank as usize], rank))
        .collect();
    same_of_a_kind.sort_by_key(|&x| Reverse(x));
    if same_of_a_kind[0].0 == 4 {
        return Combination::FourOfAKind([same_of_a_kind[0].1, same_of_a_kind[1].1]);
    }
    if same_of_a_kind[0].0 == 3 && same_of_a_kind[1].0 == 2 {
        return Combination::FullHouse([same_of_a_kind[0].1, same_of_a_kind[1].1]);
    }
    if suited_cards.len() > 0 {
        return Combination::Flush([
            suited_cards[0].rank,
            suited_cards[1].rank,
            suited_cards[2].rank,
            suited_cards[3].rank,
            suited_cards[4].rank,
        ]);
    }
    if let Some(rank) = find_straight_highest_rank(cards_descending) {
        return Combination::Straight(rank);
    }
    if same_of_a_kind[0].0 == 3 {
        return Combination::ThreeOfAKind([
            same_of_a_kind[0].1,
            same_of_a_kind[1].1,
            same_of_a_kind[2].1,
        ]);
    }
    if same_of_a_kind[0].0 == 2 && same_of_a_kind[1].0 == 2 {
        return Combination::TwoPairs([
            same_of_a_kind[0].1,
            same_of_a_kind[1].1,
            same_of_a_kind[2].1,
        ]);
    }
    if same_of_a_kind[0].0 == 2 {
        return Combination::Pair([
            same_of_a_kind[0].1,
            same_of_a_kind[1].1,
            same_of_a_kind[2].1,
            same_of_a_kind[3].1,
        ]);
    }
    Combination::HighCard([
        cards_descending[0].rank,
        cards_descending[1].rank,
        cards_descending[2].rank,
        cards_descending[3].rank,
        cards_descending[4].rank,
    ])
}

fn find_straight_highest_rank(cards_descending: &[ReducedCard]) -> Option<Rank> {
    if cards_descending.len() < 5 {
        return None;
    }
    let mut last_rank = cards_descending[cards_descending.len() - 1].rank;
    let mut run_count = 1;
    let mut straight_highest = None;
    for card in cards_descending[0..cards_descending.len() - 1].iter().rev() {
        if last_rank == card.rank {
            continue;
        }
        if last_rank as u32 + 1 == card.rank as u32 {
            run_count += 1;
            if run_count >= 5 {
                straight_highest = Some(card.rank)
            }
        } else {
            run_count = 1;
        }
        last_rank = card.rank;
    }
    straight_highest
}

pub struct YieldTimer {
    last: Instant,
    interval: Duration,
}

impl YieldTimer {
    pub fn new(interval: Duration) -> Self {
        Self {
            last: Instant::now(),
            interval,
        }
    }

    pub async fn yield_check(&mut self) -> Instant {
        let now = Instant::now();
        if now.duration_since(self.last) >= self.interval {
            // use sleep(0) instead of async_std::task::yield_now, as that still causes frozen UI in javascript
            task::sleep(Duration::from_millis(0)).await;
            self.last = now;
        }
        now
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use futures::executor::block_on;
    use rstest::rstest;
    use std::cmp::Ordering;

    #[rstest]
    #[case(vec![
        Card{rank: Rank::N2,suit: Suit::Hearts},
        Card{rank: Rank::N4,suit: Suit::Hearts},
        Card{rank: Rank::N6,suit: Suit::Hearts},
        Card{rank: Rank::N8,suit: Suit::Hearts},
        Card{rank: Rank::N10,suit: Suit::Diamonds},
        Card{rank: Rank::Q,suit: Suit::Diamonds},
        Card{rank: Rank::N3,suit: Suit::Diamonds},
    ], Combination::HighCard([
        Rank::Q,
        Rank::N10,
        Rank::N8,
        Rank::N6,
        Rank::N4,
    ]))]
    #[case(vec![
        Card{rank: Rank::N2,suit: Suit::Hearts},
        Card{rank: Rank::N4,suit: Suit::Hearts},
        Card{rank: Rank::N6,suit: Suit::Hearts},
        Card{rank: Rank::N8,suit: Suit::Hearts},
        Card{rank: Rank::N2,suit: Suit::Diamonds},
        Card{rank: Rank::Q,suit: Suit::Diamonds},
        Card{rank: Rank::N3,suit: Suit::Diamonds},
    ], Combination::Pair([
        Rank::N2,
        Rank::Q,
        Rank::N8,
        Rank::N6,
    ]))]
    #[case(vec![
        Card{rank: Rank::N2,suit: Suit::Hearts},
        Card{rank: Rank::N4,suit: Suit::Hearts},
        Card{rank: Rank::N6,suit: Suit::Hearts},
        Card{rank: Rank::N8,suit: Suit::Hearts},
        Card{rank: Rank::N2,suit: Suit::Diamonds},
        Card{rank: Rank::Q,suit: Suit::Diamonds},
        Card{rank: Rank::N4,suit: Suit::Diamonds},
    ], Combination::TwoPairs([
        Rank::N4,
        Rank::N2,
        Rank::Q,
    ]))]
    #[case(vec![
        Card{rank: Rank::N2,suit: Suit::Hearts},
        Card{rank: Rank::N4,suit: Suit::Hearts},
        Card{rank: Rank::N6,suit: Suit::Hearts},
        Card{rank: Rank::N8,suit: Suit::Hearts},
        Card{rank: Rank::N4,suit: Suit::Spades},
        Card{rank: Rank::Q,suit: Suit::Diamonds},
        Card{rank: Rank::N4,suit: Suit::Diamonds},
    ], Combination::ThreeOfAKind([
        Rank::N4,
        Rank::Q,
        Rank::N8,
    ]))]
    #[case(vec![
        Card{rank: Rank::N2,suit: Suit::Hearts},
        Card{rank: Rank::N4,suit: Suit::Hearts},
        Card{rank: Rank::N3,suit: Suit::Hearts},
        Card{rank: Rank::N5,suit: Suit::Hearts},
        Card{rank: Rank::N8,suit: Suit::Spades},
        Card{rank: Rank::Q,suit: Suit::Diamonds},
        Card{rank: Rank::N6,suit: Suit::Diamonds},
    ], Combination::Straight(
        Rank::N6
    ))]
    #[case(vec![
        Card{rank: Rank::N2,suit: Suit::Hearts},
        Card{rank: Rank::N4,suit: Suit::Hearts},
        Card{rank: Rank::N3,suit: Suit::Hearts},
        Card{rank: Rank::N5,suit: Suit::Hearts},
        Card{rank: Rank::N8,suit: Suit::Spades},
        Card{rank: Rank::Q,suit: Suit::Hearts},
        Card{rank: Rank::N7,suit: Suit::Diamonds},
    ], Combination::Flush([
        Rank::Q,
        Rank::N5,
        Rank::N4,
        Rank::N3,
        Rank::N2,
    ]))]
    #[case(vec![
        Card{rank: Rank::N2,suit: Suit::Hearts},
        Card{rank: Rank::N4,suit: Suit::Hearts},
        Card{rank: Rank::N3,suit: Suit::Hearts},
        Card{rank: Rank::Q,suit: Suit::Diamonds},
        Card{rank: Rank::N2,suit: Suit::Spades},
        Card{rank: Rank::Q,suit: Suit::Hearts},
        Card{rank: Rank::N2,suit: Suit::Diamonds},
    ], Combination::FullHouse([
        Rank::N2,
        Rank::Q,
    ]))]
    #[case(vec![
        Card{rank: Rank::N2,suit: Suit::Hearts},
        Card{rank: Rank::N4,suit: Suit::Hearts},
        Card{rank: Rank::N2,suit: Suit::Clubs},
        Card{rank: Rank::Q,suit: Suit::Diamonds},
        Card{rank: Rank::N2,suit: Suit::Spades},
        Card{rank: Rank::Q,suit: Suit::Hearts},
        Card{rank: Rank::N2,suit: Suit::Diamonds},
    ], Combination::FourOfAKind([
        Rank::N2,
        Rank::Q,
    ]))]
    #[case(vec![
        Card{rank: Rank::N8,suit: Suit::Hearts},
        Card{rank: Rank::N7,suit: Suit::Hearts},
        Card{rank: Rank::N6,suit: Suit::Clubs},
        Card{rank: Rank::N5,suit: Suit::Clubs},
        Card{rank: Rank::N4,suit: Suit::Clubs},
        Card{rank: Rank::N3,suit: Suit::Clubs},
        Card{rank: Rank::N2,suit: Suit::Clubs},
    ], Combination::StraightFlush(
        Rank::N6,
    ))]
    fn best_combination_matches(#[case] cards: Vec<Card>, #[case] expected: Combination) {
        let mut cache = Default::default();
        let result = best_combination(&cards, &mut cache);
        assert_eq!(result, expected.score());
    }

    #[rstest]
    #[case(Combination::HighCard([
        Rank::J,
        Rank::N10,
        Rank::N8,
        Rank::N6,
        Rank::N4,
    ]), Combination::HighCard([
        Rank::Q,
        Rank::N10,
        Rank::N8,
        Rank::N6,
        Rank::N4,
    ]),
        Ordering::Less
    )]
    #[case(Combination::HighCard([
        Rank::Q,
        Rank::N10,
        Rank::N8,
        Rank::N6,
        Rank::N3,
    ]), Combination::HighCard([
        Rank::Q,
        Rank::N10,
        Rank::N8,
        Rank::N6,
        Rank::N4,
    ]),
        Ordering::Less
    )]
    #[case(Combination::HighCard([
        Rank::Q,
        Rank::N10,
        Rank::N8,
        Rank::N6,
        Rank::N3,
    ]), Combination::HighCard([
        Rank::Q,
        Rank::N10,
        Rank::N8,
        Rank::N6,
        Rank::N3,
    ]),
        Ordering::Equal
    )]
    #[case(Combination::HighCard([
        Rank::Q,
        Rank::N10,
        Rank::N8,
        Rank::N6,
        Rank::N3,
    ]), Combination::Pair([
        Rank::N2,
        Rank::N10,
        Rank::N8,
        Rank::N6,
    ]),
        Ordering::Less
    )]
    fn default_cmp_matches(
        #[case] lhs: Combination,
        #[case] rhs: Combination,
        #[case] expected: Ordering,
    ) {
        let result = lhs.cmp(&rhs);
        assert_eq!(result, expected)
    }

    #[rstest]
    fn solve_with_reduced_deck() {
        let deck = vec![
            Card {
                rank: Rank::N2,
                suit: Suit::Hearts,
            },
            Card {
                rank: Rank::N3,
                suit: Suit::Hearts,
            },
            Card {
                rank: Rank::N4,
                suit: Suit::Hearts,
            },
            Card {
                rank: Rank::N2,
                suit: Suit::Spades,
            },
            Card {
                rank: Rank::N3,
                suit: Suit::Spades,
            },
            Card {
                rank: Rank::N4,
                suit: Suit::Spades,
            },
            Card {
                rank: Rank::N2,
                suit: Suit::Diamonds,
            },
            Card {
                rank: Rank::N3,
                suit: Suit::Diamonds,
            },
            Card {
                rank: Rank::N4,
                suit: Suit::Diamonds,
            },
            Card {
                rank: Rank::N5,
                suit: Suit::Clubs,
            },
            Card {
                rank: Rank::N6,
                suit: Suit::Clubs,
            },
        ];
        let table = Table {
            hand: vec![deck[0], deck[1]].into_boxed_slice(),
            board: vec![Some(deck[3]), Some(deck[4]), Some(deck[5]), None, None].into_boxed_slice(),
        };
        let result = block_on(solve_with_deck(&table, &deck)).unwrap();
        // From 11 deck cards, 2 are mine, 2 are opponents, 3 are on board -> 4 remains to be chosen from into 2 empty board slots.
        // 4 choose 2 = 4! / (2! + (4-2)! = 24 / 4 = 6
        assert_eq!(result.board_possibilities, 6);
        // opponent has 6 cards to choose from = 15
        // +1 for my hand that is also included
        assert_eq!(result.hands.len(), 16);
    }

    #[rstest]
    fn solve_flop_royal_straight() {
        let deck = full_deck();
        let len = deck.len();
        let table = Table {
            hand: vec![deck[len - 1], deck[len - 2]].into_boxed_slice(),
            board: vec![
                Some(deck[len - 3]),
                Some(deck[len - 4]),
                Some(deck[len - 5]),
                None,
                None,
            ]
            .into_boxed_slice(),
        };

        let result = block_on(solve_with_deck(&table, &deck)).unwrap();

        // From 52 deck cards, 2 are mine, 2 are opponents, 3 are on board -> 45 remains to be chosen from into 2 empty board slots.
        // 45 choose 2 = 45! / (2! + (45-2)! = 24 / 4 = 6
        assert_eq!(result.board_possibilities, 990);
        // opponent has 47 cards to choose from = 1081
        // +1 for my hand that is also included
        assert_eq!(result.hands.len(), 1082);

        assert_royal_straigth_always_wins(table, &result);
    }

    #[rstest]
    fn solve_turn_royal_straight() {
        let deck = full_deck();
        let len = deck.len();
        let table = Table {
            hand: vec![deck[len - 1], deck[len - 2]].into_boxed_slice(),
            board: vec![
                Some(deck[len - 3]),
                Some(deck[len - 4]),
                Some(deck[len - 5]),
                Some(deck[0]),
                Some(deck[1]),
            ]
                .into_boxed_slice(),
        };

        let result = block_on(solve_with_deck(&table, &deck)).unwrap();

        assert_royal_straigth_always_wins(table, &result);
    }

    fn assert_royal_straigth_always_wins(table: Table, result: &Solution) {
        // for all but last hand (because last should be equal to mine, and that should tie all the time)
        for hand in &result.hands[0..result.hands.len() - 1] {
            // nothing beats royal straight
            assert_eq!(
                hand.beats_me_count, 0,
                "{hand:?} beats {:?} but it shouldn't",
                table.hand
            );
            // royal straight beats every hand in every possible board state
            assert_eq!(
                hand.is_beaten_count, 990,
                "{hand:?} is not beaten by {:?} but it should",
                table.hand
            );
        }
    }
}
