import {useState} from "react";
import type {Card} from "../types/Card.ts";

export function useSelectedCards(count: number, initialSelectedSlot?: number) {
  const [selectedSlot, setSelectedSlot] = useState<number | null>(initialSelectedSlot === undefined ? null : initialSelectedSlot)
  const [cards, setCards] = useState<(Card | null)[]>(Array(count).fill(null))

  const setCardAt = (i: number | null, card: Card | null) => {
    setCards(cards.map((c, j) => {
      if (i === j) {
        return card
      }
      return c
    }))
  }

  // returns whether after selecting a card the selection is cleared (as last card was selected)
  const setSelectedCard = (card: Card | null): boolean => {
    if (selectedSlot === null) {
      return false
    }
    setCardAt(selectedSlot, card)
    const newSlot = selectedSlot + 1
    if (newSlot >= count) {
      setSelectedSlot(null)
      return true
    } else {
      setSelectedSlot((newSlot))
      return false
    }
  }

  const selectSlot = (i: number | null) => {
    setSelectedSlot(i)
    setCardAt(i, null)
  }

  return {
    state: {
      cards,
      selectedSlot
    },
    selectSlot,
    setSelectedSlot,
    setSelectedCard,
  }
}

export type UseSelectedCards = ReturnType<typeof useSelectedCards>
export type SelectedCardsProps = {
  state: UseSelectedCards["state"],
  selectSlot: UseSelectedCards["selectSlot"],
}