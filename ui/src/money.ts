import { html, watch } from "@arrow-js/core";
import { reactive } from "./hack";

let count = 0;
interface Diff {
  id: number;
  amt: number;
}
export const moneyState = reactive({
  money: 0,
  diffs: [] as Diff[],
  moneyAnimated: 0,
  moneyWas: 0,
});

watch(() => {
  if (moneyState.money !== moneyState.moneyWas) {
    const diff = moneyState.money - moneyState.moneyWas;
    moneyState.moneyWas = moneyState.money;
    moneyState.diffs.push({ id: count++, amt: diff });
    setTimeout(() => {
      moneyState.diffs.shift();
    }, 3000);
  }
});

let timeout: number | null = null;
watch(() => {
  moneyState.money;
  if (typeof timeout === "number") return;
  const animate = () => {
    if (moneyState.money === moneyState.moneyAnimated) {
      timeout = null;
      return;
    }
    const actualDiff = Math.abs(moneyState.money - moneyState.moneyAnimated);
    const diff = Math.max(Math.floor(actualDiff / 30), 1);
    if (actualDiff <= 1) {
      moneyState.moneyAnimated = moneyState.money;
      timeout = null;
      return;
    }
    if (moneyState.money > moneyState.moneyAnimated) {
      moneyState.moneyAnimated += diff;
    } else {
      moneyState.moneyAnimated -= diff;
    }

    timeout = setTimeout(animate, 10);
  };
  timeout = setTimeout(animate, 10);
});

export function money() {
  return html`<div id="money" class="no-mouse">
    ${() => html`<div>$${() => moneyState.moneyAnimated}</div>`.key("money")}
    ${() =>
      moneyState.diffs.map(({ amt, id }) =>
        html`<div
          id="${`diff-${id}`}"
          class="diff ${amt < 0 ? "negative" : ""}"
        >
          ${amt < 0 ? "-" : "+"}$${Math.abs(amt)}
        </div>`.key(`diff-${id}`),
      )}
  </div>`;
}
