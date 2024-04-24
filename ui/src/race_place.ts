import { html, watch } from "@arrow-js/core";
import { reactive } from "./hack";

export const racePlaceState = reactive({
  place: 0,
  racers: 0,
  active: false,
  color: "",
});

function suffixize(i: number): string {
  let suffix = "th";
  if (i % 10 === 1) {
    suffix = "st";
  }
  if (i % 10 === 2) {
    suffix = "nd";
  }
  if (i % 10 === 3) {
    suffix = "rd";
  }
  return `${i}${suffix}`;
}

watch(() => {
  racePlaceState.color = colorize(racePlaceState.place);
});

function colorize(i: number): string {
  if (i === 0) {
    return "place-1";
  }
  if (i === 1) {
    return "place-2";
  }
  if (i === 2) {
    return "place-3";
  }
  return "";
}

export function racePlace() {
  return html`${() =>
    racePlaceState.active && racePlaceState.racers > 1
      ? html`<div id="racePlace" class="no-mouse">
          <span class="${() => racePlaceState.color}">
            ${() => suffixize(racePlaceState.place + 1)} /
            ${() => suffixize(racePlaceState.racers)}
          </span>
        </div>`
      : ""}`;
}
