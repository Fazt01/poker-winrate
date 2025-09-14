import type {Solution} from "../types/Solution.ts";
import styles from "./Stats.module.css";

export default function ({solution}: { solution: Solution | null }) {
  let winCount: number | null = null
  let loseCount: number | null = null
  let possibilities: number | null = null
  if (solution !== null) {
    winCount = 0
    loseCount = 0
    possibilities = solution.boardPossibilities * solution.hands.length
    for (const hand of solution.hands) {
      winCount += hand.isBeatenCount
      loseCount += hand.beatsMeCount
    }
  }
  return <div className={styles.stats}>
    <div className={styles.row}></div>
    <div className={styles.row}>
      <div className={styles.label}>win:</div>
      <div className={styles.value}>{solution ? format(100 * winCount! / possibilities!) : "N/A"}%</div>
    </div>
    <div className={styles.row}>
      <div className={styles.label}>win or draw:</div>
      <div
        className={styles.value}>{solution ? format(100 * (possibilities! - loseCount!) / possibilities!) : "N/A"}%
      </div>
    </div>
    <div className={styles.row}>
      <div className={styles.label}>better than other hands:</div>
      <div className={styles.value}>{solution ? format(100 * solution.winCount / solution.hands.length) : "N/A"}%</div>
    </div>
    <div className={styles.row}>
      <div className={styles.label}>better than or equal to other hands:</div>
      <div className={styles.value}>{solution
        ? format(100 * (solution.hands.length - solution.loseCount) / solution.hands.length)
        : "N/A"}%
      </div>
    </div>
  </div>
}

function format(n: number): string {
  return n.toFixed(2)
}