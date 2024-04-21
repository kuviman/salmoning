import { html } from "@arrow-js/core";

export function activate() {
  const template = html`<div>hi</div>`;
  template(document.getElementById("debug")!);
  draggable(document.getElementById("debug")!);
}

function draggable(container: HTMLElement, handle?: HTMLElement) {
  let movable = handle ? handle : container;
  ["mousedown", "touchstart"].forEach((event: string) => {
    movable.addEventListener(event, (e: any) => {
      var offsetX = e.clientX - parseInt(getComputedStyle(container).left);
      var offsetY = e.clientY - parseInt(getComputedStyle(container).top);

      function mouseMoveHandler(e: any) {
        container.style.top = e.clientY - offsetY + "px";
        container.style.left = e.clientX - offsetX + "px";
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
