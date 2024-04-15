import "./style.css";
//@ts-ignore
let send_message_to_world: any;

interface Customizable {
  items: Array<
    | {
        name: string;
        cost: number;
        owned?: boolean;
      }
    | undefined
  >;
  index: number;
  equipped: number;
}

interface Customizables {
  hat: Customizable;
  bike: Customizable;
}

const stuff = async () => {
  try {
    const based = "salmoning.js";
    const path = `${based}`;
    const salmoning = await import(/* @vite-ignore */ path);
    send_message_to_world = salmoning.send_message_to_world;
  } catch (e) {
    console.error("salmoning.js module is not available", e);
    send_message_to_world = () => console.log("Fallback function");
  }
};
stuff();

class Bridge {
  app: HTMLElement;
  money: HTMLElement;
  shop: HTMLElement;
  phone: HTMLElement;
  ques: HTMLElement;
  job: HTMLElement;
  tasks: Set<string>;
  boundAcceptHandler: any;
  customizables: Customizables;

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
    this.job = this.app?.querySelector("job")!;
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
      this.customizables[kind].equipped = this.customizables[kind].index;
      send_message_to_world({
        type: "EquipAndBuy",
        kind,
        index: this.customizables[kind].index,
      });
    });

    this.app?.querySelector("#hat-equip")!.addEventListener("click", () => {
      const kind = "hat";
      this.customizables[kind].equipped = this.customizables[kind].index;
      send_message_to_world({
        type: "EquipAndBuy",
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
          send_message_to_world({ type: "ChangeName", name: e.target.value });
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

  sync_money(amt: number): void {
    this.money.innerHTML = `$${amt}`;
  }

  add_task(task: string): void {
    this.phone.classList.remove("phone_down");
    this.phone.querySelector(`#${task}`)?.classList.remove("hidden");
    this.tasks.add(task);
  }

  remove_task(task: string): void {
    this.phone.querySelector(`#${task}`)?.classList.add("hidden");
    this.tasks.delete(task);
    if (!this.tasks.size) {
      this.phone.classList.add("phone_down");
    }
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
  render_custom(kind: "hat" | "bike", index: number): void {
    console.warn({ kind, index, c: this.customizables });
    if (index < 0 || index >= this.customizables[kind].items.length) {
      console.error(`early access of ${kind} at ${index}`);
      return;
    }
    const { name, cost, owned } = this.customizables[kind].items[index] || {
      name: "None",
      cost: 0,
    };
    this.app.querySelector(`#${kind}-name`)!.innerHTML = name;
    if (cost === 0) {
      this.app.querySelector(`#${kind}-cost`)!.innerHTML = `Free!`;
    } else {
      this.app.querySelector(`#${kind}-cost`)!.innerHTML = `Cost: $${cost}`;
    }
    this.app.querySelector(`#${kind}-equip`)!.innerHTML =
      `${cost === 0 || owned ? "Equip" : "Buy"}`;
    send_message_to_world({ type: "PreviewCosmetic", kind, index });
  }

  send_customizations(data: any): void {
    console.warn(data);
    this.customizables.hat.items = data.hat_names;
    this.customizables.bike.items = data.bike_names;
  }

  show_shop(visible: boolean): void {
    if (visible) {
      this.render_custom("hat", this.customizables.hat.index);
      this.render_custom("bike", this.customizables.bike.index);
      this.shop.classList.remove("hidden");
    } else {
      this.shop.classList.add("hidden");
      this.customizables["hat"].index = this.customizables["hat"].equipped;
      this.customizables["bike"].index = this.customizables["bike"].equipped;
    }
  }

  accept() {
    send_message_to_world({ type: "AcceptQuest" });
    document.removeEventListener("keydown", this.boundAcceptHandler);
    this.remove_task("job");
  }

  acceptHandler(e: KeyboardEvent) {
    if (e.key === "e") {
      e.stopPropagation();
      this.accept();
    }
  }
  quest(prompt: string): void {
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

Object.getOwnPropertyNames((Bridge as any).prototype).forEach((key) => {
  (window as any)[`bridge_${key}`] = function () {
    return bridge && (Bridge as any).prototype[key].apply(bridge, arguments);
  };
});

if (import.meta.env.DEV) {
  (window as any).bridge_init();
  // simulate geng shit
  document.addEventListener("keydown", (e) => {
    e.preventDefault();
    (e as any).target.blur();
  });
  document.addEventListener("keyup", (e) => {
    e.preventDefault();
    (e as any).target.blur();
  });
}
