import type {Solution} from "../types/Solution"
import {type Card, type Rank, ranks, suits} from "../types/Card.ts";

export const mock_solution: Solution = {
  hands: [
    {
      hand: hand("7s2c"),
      beatsMeCount: 0,
      isBeatenCount: 5,
    },
    {
      hand: hand("JhTh"),
      beatsMeCount: 4,
      isBeatenCount: 1
    },
    {
      hand: hand("AhAd"),
      beatsMeCount: 4,
      isBeatenCount: 0
    },

  ],
  boardPossibilities: 5,
}

type Hand = Card[]

function hand(s: string): Hand {
  if (s.length !== 4) {
    throw new Error(`expected hand string "${s}" to have 4 characters`)
  }
  const s0 = s[0] === "T" ? "10" : s[0] as Rank
  const s2 = s[2] === "T" ? "10" : s[2] as Rank
  if (!ranks.includes(s0)) {
    throw new Error(`hand's first card's rank "${s[0]}" is invalid`)
  }
  if (!suits.includes(s[1])) {
    throw new Error(`hand's first card's suit "${s[1]}" is invalid`)
  }
  if (!ranks.includes(s2)) {
    throw new Error(`hand's second card's rank "${s[2]}" is invalid`)
  }
  if (!suits.includes(s[3])) {
    throw new Error(`hand's second card's suit "${s[3]}" is invalid`)
  }
  return [
    {
      rank: s[0] as Rank,
      suit: s[1],
    },
    {
      rank: s[2] as Rank,
      suit: s[3],
    }
  ]
}