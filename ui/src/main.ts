import "./style.css";
//@ts-ignore
let send_message_to_world: any;

const stuff = async () => {
  try {
    const based = "salmoning.js";
    const path = `${based}`;
    const salmoning = await import(path);
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

  constructor() {
    document.querySelector<HTMLDivElement>("#app")!.innerHTML = `
  <div>
    <div id="money">$0</div>
    <h1 class="hidden" id="shop">SHOPPING</h1>
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

    this.app?.querySelector("#quest-accept")!.addEventListener("click", () => {
      this.accept();
    });
    this.app?.querySelector("#quest-decline")!.addEventListener("click", () => {
      this.accept();
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
          send_message_to_world({ type: "ChangeName", name: e.target.value });
          (e as any).target.blur();
        }
      }
    });
    this.phone.addEventListener("keyup", (e: any) => {
      if (e.target.id === "name_input") {
        e.stopPropagation();
        console.log(e);
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

  show_shop(visible: boolean): void {
    if (visible) {
      this.shop.classList.remove("hidden");
    } else {
      this.shop.classList.add("hidden");
    }
  }

  accept() {
    send_message_to_world({ type: "AcceptQuest" });
    document.removeEventListener("keydown", this.boundAcceptHandler);
    this.remove_task("job");
  }

  acceptHandler(e: KeyboardEvent) {
    console.log(e);
    if (e.key === "e") {
      e.stopPropagation();
      console.log("Accepted!");
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
