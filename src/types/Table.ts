import type {Card} from "./Card.ts";

export interface Table {
  hand: (Card | null)[]
  board: (Card | null)[]
}