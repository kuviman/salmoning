import { html } from "@arrow-js/core";
import { reactive } from "./hack";
import { RaceStatistic } from "./salmoning";

export const state = reactive({
  showing: false,
  stats: [] as RaceStatistic[],
});

export function handleStatistic(stat: RaceStatistic) {
  state.stats.push(stat);
  state.stats.sort((a, b) => a.place - b.place);
}

export function raceSummary() {
  return html`${() =>
    state.showing
      ? html`<div id="summary">
          <div class="flex-row space-between">
            <h1>Race Summary</h1>
            <div>
              <div
                class="button padded"
                @click="${() => (state.showing = false)}"
              >
                Close
              </div>
            </div>
          </div>
          ${() =>
            state.stats.map(
              (stat) =>
                html`<div class="statistic">
                  ${stat.place + 1}.
                  ${stat.who.replaceAll("<", "&lt").replaceAll(">", "&gt") ||
                  "&lt;salmoner&gt;"}
                  <div>${stat.duration.toFixed(3)}s</div>
                </div>`,
            )}
        </div>`
      : ""}`;
}
