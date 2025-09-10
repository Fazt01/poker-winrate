import styles from './Card.module.css'
import {type Card, cardToText} from '../types/Card.ts'

export default function Card(
  {card, selected, onClick, onRightClick}: {
    card: Card | null,
    selected: boolean,
    onClick?: () => void,
    onRightClick?: () => void
  },
) {
  return (
    <div
      className={`${styles["card-outer"]} ${selected ? ` ${styles.selected}` : ""}`}
      onClick={onClick}
      onContextMenu={(e) => {
        e.preventDefault()
        onRightClick?.()
      }}
    >
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