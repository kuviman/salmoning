import { ArrowTemplate, html } from "@arrow-js/core";
import { reactive } from "./hack";
import { bridge_reply } from "./salmoning";
import _ from "lodash";

export type ControlList = { [key: string]: number | string[] | ControlList };

function lol<T>(value: any): T {
  return value;
}

export const phoneState = reactive({
  tasks: [] as TaskType[],
  questText: "spaghetti",
  inviter: "kuviman",
  teamLeader: lol<string | null>(null),
  isSelfLeader: false,
  alertText: "",
  readyCount: 0,
  totalCount: 0,
  controls: lol<ControlList | null>(null),
  mapping: lol<{ key: string; path: string[] } | null>(null),
});

type TaskType =
  | "race_circle"
  | "job"
  | "invite"
  | "change_name"
  | "base"
  | "settings"
  | "races"
  | "alert"
  | "race_list"
  | "race_editor"
  | "controls";
interface Task {
  priority: number;
  template: () => ArrowTemplate;
  closeOnInteract?: boolean;
}

const tasks: Record<TaskType, Task> = {
  base: { priority: 0, template: base_phone, closeOnInteract: true },
  settings: { priority: 1, template: settings, closeOnInteract: true },
  races: { priority: 1, template: races_menu, closeOnInteract: true },
  race_list: { priority: 2, template: race_list, closeOnInteract: true },
  change_name: { priority: 2, template: change_name },
  controls: { priority: 3, template: controls },
  job: { priority: 5, template: new_job, closeOnInteract: true },
  invite: { priority: 10, template: team_invite },
  race_circle: { priority: 15, template: race_circle },
  race_editor: { priority: 18, template: race_editor },
  alert: { priority: 20, template: alert_box, closeOnInteract: true },
};

export function phone_show_alert(text: string): void {
  phoneState.alertText = text;
  phone_add_task("alert");
}
export function phone_swap_task(task: TaskType): void {
  phoneState.tasks.shift();
  phone_add_task(task);
}

export function phone_remove_task(task: TaskType): void {
  phoneState.tasks = phoneState.tasks.filter((t) => t !== task);
}

export function phone_interact_key(mouse: boolean): void {
  if (phoneState.tasks.length === 0) {
    if (!mouse) {
      phone_add_task("base");
    }
    return;
  }
  if (tasks[phoneState.tasks[0]].closeOnInteract) {
    phoneState.tasks.shift();
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

function race_circle() {
  return html`
    <div class="screen">
      <p>
        ${() =>
          phoneState.isSelfLeader
            ? "Ring your bell to start the race."
            : "Wait for the leader's bell to start!"}
      </p>
      <p>
        ${() =>
          phoneState.isSelfLeader
            ? `Ready: ${phoneState.readyCount} / ${phoneState.totalCount}`
            : ""}
      </p>
    </div>
  `;
}

function alert_box() {
  function dismiss() {
    phone_remove_task("alert");
  }
  return html`
    <div class="screen">
      <p>${() => phoneState.alertText}</p>
      <div class="flex-row">
        <div class="button accept" @click="${dismiss}">Ok</div>
      </div>
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
      <p>"yo, wanna join my racing crew?"</p>
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
        html`${() =>
            phoneState.isSelfLeader
              ? "You are a leader"
              : `Leader: ${phoneState.teamLeader}`}
          <div class="button decline" @click="${leave_team}">Leave Team</div>`}
      <div class="flex-row">
        <div
          class="button secondary"
          @click="${() => phone_swap_task("settings")}"
        >
          Settings
        </div>
        <div class="button" @click="${() => phone_swap_task("races")}">
          Races
        </div>
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

function race_list() {
  let races: any = JSON.parse(
    localStorage.getItem("./races") || '{ "races": {} }',
  );
  if (import.meta.env.DEV) {
    races = {
      races: {
        asdf5: {
          track: [
            [5.362746, 12.342802],
            [18.372677, 13.413456],
            [31.47087, 7.8790812],
          ],
        },
        asdf4: {
          track: [
            [5.362746, 12.342802],
            [18.372677, 13.413456],
            [31.47087, 7.8790812],
          ],
        },
        asdf3: {
          track: [
            [5.362746, 12.342802],
            [18.372677, 13.413456],
            [31.47087, 7.8790812],
          ],
        },
        asdf2: {
          track: [
            [5.362746, 12.342802],
            [18.372677, 13.413456],
            [31.47087, 7.8790812],
          ],
        },
        asdf: {
          track: [
            [5.362746, 12.342802],
            [18.372677, 13.413456],
            [31.47087, 7.8790812],
          ],
        },
      },
    };
  }
  function start_race(name: string) {
    return function () {
      bridge_reply({ type: "race_start", name });
      phone_remove_task("race_list");
    };
  }
  return html`<div class="screen">
    Pick a Race:
    <div class="race-list">
      ${Object.entries(races.races).map(([name, track]) => {
        return html`<div class="race-option" @click="${start_race(name)}">
          ${name}<br />
          ${(track as any).track.length} checkpoints
        </div>`;
      })}
    </div>
  </div>`;
}

function race_editor() {
  function cancel() {
    bridge_reply({ type: "race_edit_cancel" });
    phone_remove_task("race_editor");
  }
  function submit() {
    const name = (document.getElementById("race-name") as any).value;
    bridge_reply({ type: "race_edit_submit", name });
    phone_remove_task("race_editor");
  }
  return html`
    <div class="screen">
      Race Editor
      <input type="text" placeholder="name" id="race-name" />
      <div class="button decline" @click="${cancel}">Cancel</div>
      <div class="button accept" @click="${submit}">Save</div>
    </div>
  `;
}
function races_menu() {
  function start_race() {
    if (!phoneState.isSelfLeader && phoneState.teamLeader) {
      phone_show_alert("You must be the leader of the race crew.");
      return;
    }
    phone_swap_task("race_list");
  }
  function new_race() {
    phone_swap_task("race_editor");
    bridge_reply({ type: "race_create" });
  }
  return html`
    <div class="screen">
      Racing
      <div class="button accept" @click="${start_race}">Start Race</div>
      <div class="button secondary" @click="${new_race}">Create Track</div>
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
      <div
        class="button secondary"
        @click="${() => phone_swap_task("controls")}"
      >
        Controls
      </div>
    </div>
  `;
}

function customizer(objValue: any, srcValue: any) {
  if (_.isArray(objValue)) {
    return srcValue;
  }
}

function clear_control(key: string, path: string[]) {
  const patch = {};
  let ptr: any = patch;
  while (path.length) {
    const next = path.shift()!;
    ptr[next] = {};
    ptr = ptr[next];
  }
  ptr[key] = [];
  console.log(patch);

  _.mergeWith(phoneState.controls, patch, customizer);
  phoneState.controls = {
    ...phoneState.controls,
  };
}

const conflictFilter = (key: string) =>
  function <T>(o: T): T {
    if (Array.isArray(o)) {
      return o.filter((k) => k !== key) as T;
    }
    if (typeof o === "object") {
      return _.mapValues(o, conflictFilter(key)) as T;
    }
    return o;
  };

function control_mapper(e: KeyboardEvent) {
  const key = e.code.replaceAll("Key", "");
  if (key !== "Escape") {
    phoneState.controls = _.mapValues(phoneState.controls, conflictFilter(key));
    const patch = {};
    let ptr: any = patch;
    while (phoneState.mapping!.path.length) {
      const next = phoneState.mapping!.path.shift()!;
      ptr[next] = {};
      ptr = ptr[next];
    }
    ptr[phoneState.mapping!.key] = [key];

    _.mergeWith(phoneState.controls, patch, customizer);

    phoneState.controls = {
      ...phoneState.controls,
    };
  }
  phoneState.mapping = null;
  e.preventDefault();
  e.stopPropagation();
}

(phoneState as any).$on("mapping", (value: (typeof phoneState)["mapping"]) => {
  if (value) {
    document.addEventListener("keydown", control_mapper, true);
  } else {
    document.removeEventListener("keydown", control_mapper, true);
  }
});

const controlList = (path: string[]) =>
  function ([key, val]: [
    string,
    ControlList[keyof ControlList],
  ]): ArrowTemplate {
    const isMapping =
      phoneState.mapping?.key === key &&
      _.isEqual(path, phoneState.mapping?.path);
    if (Array.isArray(val)) {
      return html`<div
        class="race-option ${isMapping ? "mapping" : ""}"
        @contextmenu="${(e: any) => {
          e.preventDefault();
          e.stopPropagation();
          clear_control(key, path);
        }}"
        @click="${() => {
          phoneState.mapping = { key, path };
        }}"
      >
        ${key}<br />
        ${isMapping ? "&lt;press a key&gt;" : val[0] || "&lt;unset&gt;"}
      </div>`;
    }
    if (typeof val === "object") {
      return html`<h2>${key.toUpperCase()}</h2>
        ${() => {
          return Object.entries(val)
            .map(controlList([...path, key]))
            .filter(Boolean);
        }}`;
    }
    return html``;
  };

function controls() {
  return html`
    <div class="screen">
      <div class="race-list">
        <h2>GENERAL</h2>
        ${() =>
          Object.entries(phoneState.controls!)
            .map(controlList([]))
            .filter(Boolean)}
      </div>
      <div class="flex-row">
        <div
          class="button accept"
          @click="${() => {
            localStorage.setItem(
              "./controls",
              JSON.stringify(phoneState.controls),
            );
            phoneState.mapping = null;
            phone_remove_task("controls");
            bridge_reply({ type: "new_controls" });
          }}"
        >
          Save
        </div>
        <div
          class="button decline"
          @click="${() => {
            phone_remove_task("controls");
            const c: ControlList = JSON.parse(
              localStorage.getItem("./controls")!,
            );
            phoneState.controls = c;
          }}"
        >
          Cancel
        </div>
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
          if (e.key === "Enter" && e.target.value) {
            phone_remove_task("change_name");
            bridge_reply({ type: "change_name", name: e.target.value });
          }
        }}" placeholder="sam"></input>
      </div>
      `;
}
