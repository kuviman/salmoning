import "./reset.css";
import "./style.css";
import { html } from "@arrow-js/core";

import type { OutboundUiMessage } from "./salmoning";
import { bridge_reply } from "./salmoning";
import {
  phone_add_task,
  phone,
  phone_remove_task,
  phoneState,
  phone_interact_key,
} from "./phone";
import { reactive } from "./hack";
import { money, moneyState } from "./money";
import {
  handleStatistic,
  raceSummary,
  state as raceSummaryState,
} from "./race_summary";

function assertUnreachable(_: never): never {
  return _;
}

const state = reactive({
  shopVisible: false,
});

interface Unlocks {
  hats: number[];
  bikes: number[];
}

interface Customizable {
  items: Array<{
    name: string;
    cost: number;
  } | null>;
  index: number;
  equipped: number;
}

interface Customizables {
  hat: Customizable;
  bike: Customizable;
}

class Bridge {
  customizables: Customizables;
  unlocks: Unlocks;

  constructor() {
    document.getElementById("app")!.addEventListener("mousemove", (e) => {
      e.stopPropagation();
    });
    document.getElementById("app")!.addEventListener("mousedown", (e) => {
      e.stopPropagation();
    });
    document.getElementById("app")!.addEventListener("mouseup", (e) => {
      e.stopPropagation();
    });
    document.getElementById("app")!.addEventListener("keydown", (e) => {
      e.stopPropagation();
    });
    document.getElementById("app")!.addEventListener("keyup", (e) => {
      e.stopPropagation();
    });
    const template = html`
      <div>
        ${money()}
        <div class="${() => (state.shopVisible ? "" : "hidden")}" id="shop">
          <h1>Sal Mon's Customs</h1>
          <h2>Hat</h2>
          <div class="spacer">
            <p id="hat-name">Cat</p>
            <p id="hat-cost">Cost: 50</p>
          </div>
          <div class="w-75">
            <div class="flex-row">
              <div class="button" id="hat-prev">Prev</div>
              <div class="button" id="hat-next">Next</div>
            </div>
            <div class="button" id="hat-equip">Equip</div>
          </div>
          <h2>Bike</h2>
          <div class="spacer">
            <p id="bike-name">Cat</p>
            <p id="bike-cost">Cost: 50</p>
          </div>
          <div class="w-75">
            <div class="flex-row">
              <div class="button" id="bike-prev">Prev</div>
              <div class="button" id="bike-next">Next</div>
            </div>
            <div class="button" id="bike-equip">Equip</div>
          </div>
        </div>
        ${phone()} ${raceSummary()}
      </div>
    `;
    template(document.getElementById("app")!);

    this.customizables = {
      hat: {
        items: [],
        index: 0,
        equipped: 0,
      },
      bike: {
        items: [],
        index: 0,
        equipped: 0,
      },
    };

    this.unlocks = {
      hats: [],
      bikes: [],
    };

    document.querySelector("#hat-next")!.addEventListener("click", () => {
      this.next_custom("hat");
    });
    document.querySelector("#hat-prev")!.addEventListener("click", () => {
      this.prev_custom("hat");
    });

    document.querySelector("#bike-next")!.addEventListener("click", () => {
      this.next_custom("bike");
    });
    document.querySelector("#bike-prev")!.addEventListener("click", () => {
      this.prev_custom("bike");
    });

    document.querySelector("#bike-equip")!.addEventListener("click", () => {
      const kind = "bike";
      const index = this.customizables[kind].index;
      this.customizables[kind].equipped = index;
      bridge_reply({
        type: "equip_and_buy",
        kind,
        index,
      });
    });

    document.querySelector("#hat-equip")!.addEventListener("click", () => {
      const kind = "hat";
      this.customizables[kind].equipped = this.customizables[kind].index;
      bridge_reply({
        type: "equip_and_buy",
        kind,
        index: this.customizables[kind].index,
      });
    });

    // this.phone.addEventListener("mousemove", (e: any) => {
    //   if (document?.activeElement?.id === "name_input") {
    //     e.stopPropagation();
    //   }
    // });

    // this.phone.addEventListener("keydown", (e: any) => {
    //   if (e.target.id === "name_input") {
    //     e.stopPropagation();
    //     if (e.key === "Enter") {
    //       phone_remove_task("change_name");
    //       bridge_reply({ type: "change_name", name: e.target.value });
    //       (e as any).target.blur();
    //     }
    //   }
    // });
    // this.phone.addEventListener("keyup", (e: any) => {
    //   if (e.target.id === "name_input") {
    //     e.stopPropagation();
    //   }
    // });
  }

  prev_custom(kind: "hat" | "bike"): void {
    const { length } = this.customizables[kind].items;
    let { index } = this.customizables[kind];
    index -= 1;
    if (index < 0) {
      index = length - 1;
    }
    this.customizables[kind].index = index;
    this.render_custom(kind, index);
  }

  next_custom(kind: "hat" | "bike"): void {
    const { length } = this.customizables[kind].items;
    let { index } = this.customizables[kind];
    index += 1;
    if (index >= length) {
      index = 0;
    }
    this.customizables[kind].index = index;
    this.render_custom(kind, index);
  }

  render_custom(
    kind: "hat" | "bike",
    index: number,
    update: boolean = true,
  ): void {
    if (index < 0 || index >= this.customizables[kind].items.length) {
      console.error(`early access of ${kind} at ${index}`);
      return;
    }
    const { name, cost } = this.customizables[kind].items[index] || {
      name: "None",
      cost: 0,
    };
    const owned = this.unlocks[`${kind}s`].includes(index);
    document.getElementById(`${kind}-name`)!.innerHTML = name;
    if (cost === 0) {
      document.getElementById(`${kind}-cost`)!.innerHTML = `Free!`;
    } else {
      document.getElementById(`${kind}-cost`)!.innerHTML = `Cost: $${cost}`;
    }
    document.getElementById(`${kind}-equip`)!.innerHTML =
      `${cost === 0 || owned ? "Equip" : "Buy"}`;
    if (update) {
      bridge_reply({ type: "preview_cosmetic", kind, index });
    }
  }

  receive(event: OutboundUiMessage): void {
    switch (event.type) {
      case "sync_money":
        moneyState.money = event.amount;
        break;
      case "phone_show_invite":
        phoneState.inviter = event.from;
        phone_add_task("invite");
        break;
      case "unlocks":
        this.unlocks = event;
        this.render_custom("hat", this.customizables.hat.index, false);
        this.render_custom("bike", this.customizables.bike.index, false);
        break;
      case "customization_info":
        this.customizables.hat.items = event.hat_names;
        this.customizables.bike.items = event.bike_names;
        break;
      case "show_shop":
        if (event.visible) {
          this.render_custom("hat", this.customizables.hat.index, false);
          this.render_custom("bike", this.customizables.bike.index, false);
          state.shopVisible = true;
        } else {
          state.shopVisible = false;
          this.customizables["hat"].index = this.customizables["hat"].equipped;
          this.customizables["bike"].index =
            this.customizables["bike"].equipped;
        }
        break;
      case "phone_change_name":
        phone_add_task("change_name");
        break;
      case "phone_new_job":
        // TODO: use server's quest text
        this.quest();
        break;
      case "phone_accept_invite":
      case "phone_reject_invite":
        phone_remove_task("invite");
        break;
      case "phone_interact_key":
        phone_interact_key(event.mouse);
        break;
      case "phone_dismiss_notification":
        phone_remove_task("job");
        break;
      case "sync_team_leader":
        phoneState.teamLeader = event.name;
        phoneState.isSelfLeader = event.is_self;
        break;

      case "show_race_summary":
        raceSummaryState.showing = true;
        break;
      case "update_race_summary":
        handleStatistic(event.statistic);
        break;
      case "clear_race_summary":
        raceSummaryState.stats = [];
        raceSummaryState.showing = false;
        break;
      case "exit_race_circle":
        phone_remove_task("race_circle");
        break;
      case "enter_race_circle":
        phone_add_task("race_circle");
        break;
      case "update_ready_count":
        phoneState.readyCount = event.ready;
        phoneState.totalCount = event.total;
        break;
      case "phone_alert":
        phoneState.alertText = event.msg;
        phone_add_task("alert");
        break;
      default:
        console.error("Unexpected message received!", event);
        assertUnreachable(event);
    }
  }

  // accept() {
  //   bridge_reply({ type: "accept_quest" });
  //   document.removeEventListener("keydown", this.boundAcceptHandler);
  //   phone_remove_task("job");
  // }

  // acceptHandler(e: KeyboardEvent) {
  //   if (e.key === "e") {
  //     e.stopPropagation();
  //     this.accept();
  //   }
  // }
  quest(): void {
    const prompts: string[] = [
      "Can you take my books back to the library?",
      "Bring me my food now!!!!",
      "Please pick up my dry cleaning",
      "I need 3 gerbils ASAP. No questions please",
      "Can you deliver my groceries? I need tomato",
      "I AM OUT OF TOILET PAPER GO FAST PLEASE",
      "i want spaghetti",
      "HUNGRY!!!!!!",
      "bring me some flowers.",
      "please do not look in this bag. just deliver",
      "i would like 1 newspaper please",
      "its me, pgorley",
      "please serve these court summons for me",
      "i ran out of coffee creamer. can you bring me some butter?",
      "i need 37 cans of soup. no time to explain",
      "can you deliver sushi",
      "deliver this mail for me",
      "can you take this trash away",
      "i need a new kidney",
      "PLEASE DELIVER MY TELEGRAM STOP DONT STOP STOP",
      "find my pet turtle",
      "let's go bowling cousin",
      "listen, you just drive. to point B. simple.",
      "2 Number 9's, a number 9 large, a number 6 with extra dip, 2 number 45's (one with cheese) and a large soda",
    ];
    const prompt = prompts[Math.floor(Math.random() * prompts.length)];
    phoneState.questText = prompt;
    phone_add_task("job");
    // document.addEventListener("keydown", this.boundAcceptHandler);
  }
}

// the rest of this file is boilerplate; please ignore
// thank you!

let bridge: Bridge | undefined;
(window as any).bridge_init = () => {
  bridge = new Bridge();
};

(window as any).bridge_send = function () {
  return (
    (bridge || (console.warn("Bridge accessed before init!"), 0)) &&
    (Bridge as any).prototype["receive"].apply(bridge, arguments)
  );
};

if (import.meta.env.DEV) {
  (window as any).bridge_init();
  // simulate geng events
  document.addEventListener("keydown", (e) => {
    e.preventDefault();
    document.getElementById("fakecanvas")!.focus();
  });
  document.addEventListener("keyup", (e) => {
    e.preventDefault();
    document.getElementById("fakecanvas")!.focus();
  });
  document.getElementById("fakecanvas")!.addEventListener("mousemove", (e) => {
    e.preventDefault();
    document.getElementById("fakecanvas")!.focus();
  });
  document.getElementById("fakecanvas")!.addEventListener("mousedown", (e) => {
    e.preventDefault();
    document.getElementById("fakecanvas")!.focus();
  });
  document.getElementById("fakecanvas")!.addEventListener("mouseup", (e) => {
    e.preventDefault();
    document.getElementById("fakecanvas")!.focus();
  });
}
