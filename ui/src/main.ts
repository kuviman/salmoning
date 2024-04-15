import "./style.css";

(window as any).bridge_init = () => {
  document.querySelector<HTMLDivElement>("#app")!.innerHTML = `
  <div>
    <h1 id="money">$0</h1>
  </div>
`;
  const app = document.querySelector("#app")!;
  const money = app.querySelector("#money")!;

  (window as any).bridge_sync_money = (amt: number) => {
    money.innerHTML = `$${amt}`;
  };
};


