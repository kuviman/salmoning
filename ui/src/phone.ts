import { ArrowTemplate, html } from "@arrow-js/core";
import { reactive } from "./hack";
import { bridge_reply } from "./salmoning";

function lol<T>(value: any): T {
  return value;
}
export const phoneState = reactive({
  tasks: [] as TaskType[],
  questText: "spaghetti",
  inviter: "kuviman",
  teamLeader: lol<string | null>(null),
});

type TaskType = "job" | "invite" | "change_name" | "base" | "settings";
interface Task {
  priority: number;
  template: () => ArrowTemplate;
}

const tasks: Record<TaskType, Task> = {
  base: { priority: 0, template: base_phone },
  settings: { priority: 1, template: settings },
  change_name: { priority: 2, template: change_name },
  job: { priority: 5, template: new_job },
  invite: { priority: 10, template: team_invite },
};

export function phone_swap_task(task: TaskType): void {
  phoneState.tasks.shift();
  phone_add_task(task);
}

export function phone_remove_task(task: TaskType): void {
  phoneState.tasks = phoneState.tasks.filter((t) => t !== task);
}

export function phone_interact_key(): void {
  if (phoneState.tasks.length === 0) {
    phone_add_task("base");
    return;
  }
  if (phoneState.tasks[0] === "job") {
    phone_remove_task("job");
    return;
  }
  if (phoneState.tasks[0] === "settings") {
    phone_remove_task("settings");
    return;
  }
  if (phoneState.tasks[0] === "base") {
    phone_remove_task("base");
    return;
  }
}

export function phone_add_task(task: TaskType): void {
  if (!phoneState.tasks.includes(task)) {
    phoneState.tasks.push(task);
  }
  phoneState.tasks.sort((a, b) => {
    return tasks[b].priority - tasks[a].priority;
  });
}

export function phone() {
  return html`
    <div
      id="phone"
      class="${() => (phoneState.tasks.length ? "" : "phone_down")}"
    >
      ${() =>
        phoneState.tasks.length
          ? tasks[phoneState.tasks[0]].template()
          : undefined}
    </div>
  `;
}

function new_job() {
  function dismiss() {
    phone_remove_task("job");
  }
  return html`
    <div class="screen">
      <p>Someone is summoning you!</p>
      <p>"${() => phoneState.questText}"</p>
      <div class="flex-row">
        <div class="button accept" @click="${dismiss}">Nice</div>
        <div class="button decline" @click="${dismiss}">OK</div>
      </div>
    </div>
  `;
}

function team_invite() {
  function dismiss(answer: "accept_invite" | "decline_invite") {
    phone_remove_task("invite");
    bridge_reply({ type: answer });
  }
  return html`
    <div class="screen" id="invite">
      <p>New Message</p>
      <p>"yo, wanna join my team?"</p>
      <p id="inviter">- ${() => phoneState.inviter}</p>
      <div class="flex-row">
        <div class="button accept" @click="${() => dismiss("accept_invite")}">
          (Y)es
        </div>
        <div class="button decline" @click="${() => dismiss("decline_invite")}">
          (N)o
        </div>
      </div>
    </div>
  `;
}

function base_phone() {
  function leave_team() {
    bridge_reply({ type: "leave_team" });
    phone_remove_task("base");
  }
  let iconPath = "./icons";
  if (!import.meta.env.DEV) {
    iconPath = "assets/icons";
  }
  return html`
    <div class="screen">
      ${() =>
        phoneState.teamLeader &&
        html`Leader: ${() => phoneState.teamLeader}
          <div class="button decline" @click="${leave_team}">Leave Team</div>`}
      <div
        class="button secondary"
        @click="${() => phone_swap_task("settings")}"
      >
        Settings
      </div>
      <div class="phone-grid">
        <img src="${iconPath}/msg.png" />
        <img src="${iconPath}/salmazon.png" />
        <img src="${iconPath}/firefish.png" />
        <img src="${iconPath}/fishmail.png" />
        <img src="${iconPath}/phone.png" />
        <img src="${iconPath}/fishmaps.png" />
        <img src="${iconPath}/fishbook.png" />
        <img src="${iconPath}/fishdonalds.png" />
        <img src="${iconPath}/fishcord.png" />
        <img src="${iconPath}/samneats.png" />
        <img src="${iconPath}/fwich.png" />
      </div>
    </div>
  `;
}

function settings() {
  return html`
    <div class="screen">
      Settings
      <div
        class="button secondary"
        @click="${() => phone_swap_task("change_name")}"
      >
        Change Name
      </div>
    </div>
  `;
}
function change_name() {
  return html`
      <div class="screen" id="choose_name">
        Enter your name:
        <input type="text" autocomplete=off id="name_input" @keydown="${(
          e: any,
        ) => {
          if (e.key === "Enter") {
            phone_remove_task("change_name");
            bridge_reply({ type: "change_name", name: e.target.value });
          }
        }}" placeholder="sam"></input>
      </div>
      `;
}
