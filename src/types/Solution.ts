import type {Card} from "./Card.ts";

export interface Solution {
  hands: HandSolution[]
  boardPossibilities: number
}

export interface HandSolution {
  hand: Card[]
  beatsMeCount: number
  isBeatenCount: number
}