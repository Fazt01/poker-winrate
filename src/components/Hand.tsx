import type {SelectedCardsProps} from "../states/SelectedCards.ts";
import SelectedCards from "./SelectedCards.tsx";

export default function Hand(props: SelectedCardsProps) {
  return <SelectedCards count={2} props={props}/>
}