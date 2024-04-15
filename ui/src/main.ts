import "./style.css";

class Bridge {
  app: Element;
  money: Element;
  shop: Element;
  phone: Element;

  constructor() {
    document.querySelector<HTMLDivElement>("#app")!.innerHTML = `
  <div>
    <div id="money">$0</div>
    <h1 class="hidden" id="shop">SHOPPING</h1>
    <div id="phone" class="phone_down">
      <div id="name">
        Enter your name:
        <input type="text" id="name_input" placeholder="sam"></input>
        </div>
    </div>
  </div>
`;
    this.app = document.querySelector("#app")!;
    this.money = this.app?.querySelector("#money")!;
    this.shop = this.app?.querySelector("#shop")!;
    this.phone = this.app?.querySelector("#phone")!;

    this.phone.addEventListener("keydown", (e: any) => {
      if (e.target.id === "name_input") {
        e.stopPropagation();
        console.log(e);
        if (e.key === "Enter") {
          (window as any).send_message_to_world({
            ChangeName: { name: e.target.value },
          });
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

  show_phone(visible: boolean): void {
    if (visible) {
      this.shop.classList.remove("phone_down");
    } else {
      this.shop.classList.add("phone_down");
    }
  }
  show_shop(visible: boolean): void {
    if (visible) {
      this.shop.classList.remove("hidden");
    } else {
      this.shop.classList.add("hidden");
    }
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
