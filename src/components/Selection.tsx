import CardComponent from "./Card.tsx";
import {type Card, ranks, suits} from "../types/Card.ts";

export default function Selection(
  {
    onCardSelected,
  }: {
    onCardSelected: (card: Card) => void;
  }
) {
  let rows = []
  for (const [i, suit] of suits.entries()) {
    let cards = []
    for (const [j, rank] of ranks.entries()) {
      let card = {
        rank,
        suit,
      }
      cards.push(<CardComponent
        key={`${i}-${j}`}
        card={card}
        selected={false}
        onClick={() => onCardSelected(card)}
      ></CardComponent>)
    }
    rows.push(cards)
    rows.push(<br key={`${i}br`}/>)
  }
  return rows
}