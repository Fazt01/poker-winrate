import styles from './Card.module.css'
import type {Card, Suit} from '../types/Card.ts'

export default function Card(
  {card, selected, onClick}: {
    card: Card | null,
    selected: boolean,
    onClick?: () => void,
  },
) {
  return (
    <div className={`${styles["card-outer"]} ${selected ? ` ${styles.selected}` : ""}`} onClick={onClick}>
      <div className={`${coloringStyle(card)} ${styles.card}`}>
        {card && cardToText(card)}
      </div>
    </div>
  )
}

function coloringStyle(card: Card | null) {
  if (!card) {
    return styles.blank
  }
  if (card.suit === 'h' || card.suit === 'd') {
    return styles.red
  }
  return styles.black
}

const suitsMap: Record<Suit, string> = {
  h: "♥",
  d: "♦",
  s: "♠",
  c: "♣",
}

function cardToText(card: Card): string {
  let rank: string = card.rank
  if (rank === "10") {
    rank = "T"
  }
  return rank + suitsMap[card.suit]
}