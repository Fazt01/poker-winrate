import type {Table} from "../types/Table.ts"
import type {Solution} from "../types/Solution.ts";
import * as wasm from "../../rust-wasm/pkg";
import type {Card, Rank, Suit} from "../types/Card.ts";

export default async function solve(cancellationToken: wasm.AbortSignal, table: Table): Promise<Solution> {
  return fromWasmSolution(await wasm.solve(cancellationToken, toWasmTable(table)))
}

function toMaybeCard(c: Card | null): wasm.MaybeCard {
  if (c === null) {
    return new wasm.MaybeCard(null)
  }
  return new wasm.MaybeCard(new wasm.Card(c.rank, c.suit))
}

function toWasmTable(t: Table): wasm.Table {
  return new wasm.Table(
    t.hand.map(toMaybeCard),
    t.board.map(toMaybeCard),
  )
}

function fromWasmSolution(s: wasm.Solution): Solution {
  return {
    hands: s.hands.map(h => {
      return {
        hand: h.hand.map(c => ({rank: c.rank as Rank, suit: c.suit as Suit})),
        beatsMeCount: Number(h.beats_me_count),
        isBeatenCount: Number(h.is_beaten_count),
      }
    }),
    boardPossibilities: Number(s.board_possibilities),
  }
}
