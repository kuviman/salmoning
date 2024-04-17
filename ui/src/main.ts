import "./style.css";

import { bridge_reply } from "salmoning";
import type { OutboundUiMessage } from "salmoning";

function assertUnreachable(_: never): never {
  return _;
}

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
  app: HTMLElement;
  money: HTMLElement;
  shop: HTMLElement;
  phone: HTMLElement;
  ques: HTMLElement;
  tasks: Set<string>;
  boundAcceptHandler: any;
  customizables: Customizables;
  unlocks: Unlocks;

  constructor() {
    document.querySelector<HTMLDivElement>("#app")!.innerHTML = `
  <div>
    <div id="money">$0</div>
    <div class="hiddenTODO" id="shop"><h1>Sal Mon's Customs</h1>
      <h2>Hat</h2>
      <div class="spacer">
        <p id="hat-name">Cat</p>
        <p id="hat-cost">Cost: 50</p>
        </div>
      <div class="w-75">
      <div class="flex-row">
      <button id="hat-prev">Prev</button>
      <button id="hat-next">Next</button>
        </div>
      <button id="hat-equip">Equip</button>
        </div>
      <h2>Bike</h2>
      <div class="spacer">
        <p id="bike-name">Cat</p>
        <p id="bike-cost">Cost: 50</p>
        </div>
      <div class="w-75">
      <div class="flex-row">
      <button id="bike-prev">Prev</button>
      <button id="bike-next">Next</button>
        </div>
      <button id="bike-equip">Equip</button>
        </div>
    </div>
    <div id="phone" class="phone_down">
      <div class="screen hidden" id="choose_name">
        Enter your name:
        <input type="text" autocomplete=off id="name_input" placeholder="sam"></input>
        </div>
      <div class="screen hidden" id="job">
        <p>Someone is summoning you!</p>
        <p id="quest">"I need my groceries delivered."</p>
        <div class="flex-row">
        <button id="quest-accept" class="accept">Nice</button>
          <button id="quest-decline" class="decline">OK</button>
          </div>
        </div>
      <div class="screen hidden" id="invite">
        <p>New Message</p>
        <p>"yo, wanna join my team?"</p>
        <p id="inviter">- kuviman</p>
        <div class="flex-row">
        <button id="invite-accept" class="accept">(Y)es</button>
          <button id="invite-decline" class="decline">(N)o</button>
          </div>
        </div>
    </div>
  </div>
`;
    this.app = document.querySelector("#app")!;
    this.money = this.app?.querySelector("#money")!;
    this.shop = this.app?.querySelector("#shop")!;
    this.phone = this.app?.querySelector("#phone")!;
    this.ques = this.app?.querySelector("#quest")!;
    this.boundAcceptHandler = this.acceptHandler.bind(this);

    this.tasks = new Set();
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

    this.app?.querySelector("#invite-accept")!.addEventListener("click", () => {
      bridge_reply({ type: "accept_invite" });
      this.remove_task("invite");
    });
    this.app
      ?.querySelector("#invite-decline")!
      .addEventListener("click", () => {
        bridge_reply({ type: "decline_invite" });
        this.remove_task("invite");
      });

    this.app?.querySelector("#quest-accept")!.addEventListener("click", () => {
      this.accept();
    });
    this.app?.querySelector("#quest-decline")!.addEventListener("click", () => {
      this.accept();
    });

    this.app?.querySelector("#hat-next")!.addEventListener("click", () => {
      this.next_custom("hat");
    });
    this.app?.querySelector("#hat-prev")!.addEventListener("click", () => {
      this.prev_custom("hat");
    });

    this.app?.querySelector("#bike-next")!.addEventListener("click", () => {
      this.next_custom("bike");
    });
    this.app?.querySelector("#bike-prev")!.addEventListener("click", () => {
      this.prev_custom("bike");
    });

    this.app?.querySelector("#bike-equip")!.addEventListener("click", () => {
      const kind = "bike";
      const index = this.customizables[kind].index;
      this.customizables[kind].equipped = index;
      bridge_reply({
        type: "equip_and_buy",
        kind,
        index,
      });
    });

    this.app?.querySelector("#hat-equip")!.addEventListener("click", () => {
      const kind = "hat";
      this.customizables[kind].equipped = this.customizables[kind].index;
      bridge_reply({
        type: "equip_and_buy",
        kind,
        index: this.customizables[kind].index,
      });
    });

    this.phone.addEventListener("mousemove", (e: any) => {
      if (document?.activeElement?.id === "name_input") {
        e.stopPropagation();
      }
    });

    this.phone.addEventListener("keydown", (e: any) => {
      if (e.target.id === "name_input") {
        e.stopPropagation();
        if (e.key === "Enter") {
          this.remove_task("choose_name");
          bridge_reply({ type: "change_name", name: e.target.value });
          (e as any).target.blur();
        }
      }
    });
    this.phone.addEventListener("keyup", (e: any) => {
      if (e.target.id === "name_input") {
        e.stopPropagation();
      }
    });
  }

  remove_task(task: string): void {
    this.phone.querySelector(`#${task}`)?.classList.add("hidden");
    this.tasks.delete(task);
    if (!this.tasks.size) {
      this.phone.classList.add("phone_down");
    }
  }
  add_task(task: string): void {
    this.phone.classList.remove("phone_down");
    this.phone.querySelector(`#${task}`)?.classList.remove("hidden");
    this.tasks.add(task);
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
    console.warn({ kind, index, c: this.customizables });
    if (index < 0 || index >= this.customizables[kind].items.length) {
      console.error(`early access of ${kind} at ${index}`);
      return;
    }
    const { name, cost } = this.customizables[kind].items[index] || {
      name: "None",
      cost: 0,
    };
    const owned = this.unlocks[`${kind}s`].includes(index);
    this.app.querySelector(`#${kind}-name`)!.innerHTML = name;
    if (cost === 0) {
      this.app.querySelector(`#${kind}-cost`)!.innerHTML = `Free!`;
    } else {
      this.app.querySelector(`#${kind}-cost`)!.innerHTML = `Cost: $${cost}`;
    }
    this.app.querySelector(`#${kind}-equip`)!.innerHTML =
      `${cost === 0 || owned ? "Equip" : "Buy"}`;
    if (update) {
      bridge_reply({ type: "preview_cosmetic", kind, index });
    }
  }

  receive(event: OutboundUiMessage): void {
    switch (event.type) {
      case "sync_money":
        this.money.innerHTML = `$${event.amount}`;
        break;
      case "phone_show_invite":
        this.app.querySelector("#inviter")!.innerHTML = `- ${event.from}`;
        this.add_task("invite");
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
          this.shop.classList.remove("hidden");
        } else {
          this.shop.classList.add("hidden");
          this.customizables["hat"].index = this.customizables["hat"].equipped;
          this.customizables["bike"].index =
            this.customizables["bike"].equipped;
        }
        break;
      case "phone_change_name":
        this.add_task("choose_name");
        break;
      case "phone_new_job":
        // TODO: use server's quest text
        this.quest();
        break;
      case "phone_accept_invite":
      case "phone_reject_invite":
        this.remove_task("invite");
        break;
      case "phone_dismiss_notification":
        this.remove_task("job");
        break;
      default:
        console.error("Unexpected message received!", event);
        assertUnreachable(event);
    }
  }

  accept() {
    bridge_reply({ type: "accept_quest" });
    document.removeEventListener("keydown", this.boundAcceptHandler);
    this.remove_task("job");
  }

  acceptHandler(e: KeyboardEvent) {
    if (e.key === "e") {
      e.stopPropagation();
      this.accept();
    }
  }
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
    this.add_task("job");
    this.ques.innerHTML = `"${prompt}"`;
    document.addEventListener("keydown", this.boundAcceptHandler);
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
    (e as any).target.blur();
  });
  document.addEventListener("keyup", (e) => {
    e.preventDefault();
    (e as any).target.blur();
  });
}
