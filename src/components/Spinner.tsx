import styles from './Spinner.module.css'

export default function Spinner() {
  return (<svg
    width="120"
    height="120"
    viewBox="-60 -60 120 120"
  >
    <circle className={styles.spinner} cx="0" cy="0" r="50"/>
  </svg>)
}