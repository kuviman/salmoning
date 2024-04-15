import "./style.css";

class Bridge {
  app: Element;
  money: Element;

  constructor() {
    document.querySelector<HTMLDivElement>("#app")!.innerHTML = `
  <div>
    <h1 id="money">$0</h1>
  </div>
  <button>Hello</button>
`;
    this.app = document.querySelector("#app")!;
    this.money = this.app?.querySelector("#money")!;
  }

  sync_money(amt: number): boolean {
    this.money.innerHTML = `$${amt}`;
    return true;
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
