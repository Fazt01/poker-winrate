import type {Card} from "./Card.ts";

export interface Solution {
  hands: HandSolution[]
  boardPossibilities: number
  winCount: number
  loseCount: number
}

export interface HandSolution {
  hand: Card[]
  beatsMeCount: number
  isBeatenCount: number
}