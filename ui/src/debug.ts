import { html } from "@arrow-js/core";
import { OutboundUiMessage } from "./salmoning";
import { reactive } from "./hack";

interface DebugMsg {
  name: string;
  action: OutboundUiMessage;
}

export function activate() {
  const data = reactive({
    visible: !!localStorage.getItem("debugVisible") || false,
  });
  const items: DebugMsg[] = [
    {
      name: "Join Team",
      action: {
        type: "sync_team_leader",
        name: "leader",
        is_self: true,
      },
    },
    {
      name: "BIG Money",
      action: {
        type: "sync_money",
        amount: 5000,
      },
    },
    {
      name: "Add Money",
      action: {
        type: "sync_money",
        amount: 100,
      },
    },
    {
      name: "Remove Money",
      action: {
        type: "sync_money",
        amount: 0,
      },
    },
    {
      name: "Show Shop",
      action: {
        type: "show_shop",
        visible: true,
      },
    },
    {
      name: "Hide Shop",
      action: {
        type: "show_shop",
        visible: false,
      },
    },
    {
      name: "Change Name",
      action: {
        type: "phone_change_name",
      },
    },
    {
      name: "Team Invite",
      action: {
        type: "phone_show_invite",
        from: "Pomo",
      },
    },
    {
      name: "New Job",
      action: {
        type: "phone_new_job",
        prompt: "hello i have a job for you",
      },
    },
    {
      name: "Phone Interact Key",
      action: {
        type: "phone_interact_key",
        mouse: false,
      },
    },
  ];
  const template = html`<div id="debug-handle">
      debug
      <div
        id="debug-collapse"
        @click="${() => {
          data.visible = !data.visible;
          if (data.visible) {
            localStorage.setItem("debugVisible", "true");
          } else {
            localStorage.removeItem("debugVisible");
          }
        }}"
      >
        ${() => (data.visible ? "⮟" : "⮞")}
      </div>
    </div>
    ${() =>
      data.visible
        ? items.map((item, id) =>
            html`<li
              @click="${() => {
                (window as any).bridge_send(item.action);
              }}"
            >
              ${item.name}
            </li>`.key(id),
          )
        : undefined} `;

  template(document.getElementById("debug")!);
  draggable(
    document.getElementById("debug")!,
    document.getElementById("debug-handle")!,
  );
}

function draggable(container: HTMLElement, handle?: HTMLElement) {
  let movable = handle ? handle : container;
  ["mousedown", "touchstart"].forEach((event: string) => {
    container.style.left = localStorage.getItem("debugX") || "0px";
    container.style.top = localStorage.getItem("debugY") || "0px";
    movable.addEventListener(event, (e: any) => {
      var offsetX = e.clientX - parseInt(getComputedStyle(container).left);
      var offsetY = e.clientY - parseInt(getComputedStyle(container).top);

      function mouseMoveHandler(e: any) {
        container.style.top = e.clientY - offsetY + "px";
        container.style.left = e.clientX - offsetX + "px";
        localStorage.setItem("debugX", `${e.clientX - offsetX}px`);
        localStorage.setItem("debugY", `${e.clientY - offsetY}px`);
      }

      function reset() {
        removeEventListener("mousemove", mouseMoveHandler);
        removeEventListener("mouseup", reset);
      }

      addEventListener("mousemove", mouseMoveHandler);
      addEventListener("mouseup", reset);
    });
  });
}
