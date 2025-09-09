import {default as CardComponent} from "./Card.tsx";
import type {SelectedCardsProps} from "../states/SelectedCards.ts";

export default function SelectedCards(
  {
    props,
    count,
  }: {
    props: SelectedCardsProps,
    count: number,
  }
) {
  const result = []
  for (let i = 0; i < count; i++) {
    result.push(
      <CardComponent
        key={i}
        card={props.state.cards[i]}
        selected={props.state.selectedSlot === i}
        onClick={() => props.selectSlot(i)}
      />
    )
  }
  return result
}