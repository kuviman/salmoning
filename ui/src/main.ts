import "./style.css";

class Bridge {
  app: Element;
  money: Element;
  shop: Element;

  constructor() {
    document.querySelector<HTMLDivElement>("#app")!.innerHTML = `
  <div>
    <h1 id="money">$0</h1>
    <h1 class="hidden" id="shop">SHOPPING</h1>
  </div>
`;
    this.app = document.querySelector("#app")!;
    this.money = this.app?.querySelector("#money")!;
    this.shop = this.app?.querySelector("shop")!;
  }

  sync_money(amt: number): void {
    this.money.innerHTML = `$${amt}`;
  }

  show_shop(visible: boolean): void {
    if (visible) {
      this.shop.classList.add("hidden");
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
}
