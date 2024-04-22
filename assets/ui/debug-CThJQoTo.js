import{r as p,h as c}from"./index-BkIOamZK.js";function g(){const e=p({visible:!!localStorage.getItem("debugVisible")||!1}),t=[{name:"Join Team",action:{type:"sync_team_leader",name:"leader",is_self:!0}},{name:"BIG Money",action:{type:"sync_money",amount:5e3}},{name:"Add Money",action:{type:"sync_money",amount:100}},{name:"Remove Money",action:{type:"sync_money",amount:0}},{name:"Show Shop",action:{type:"show_shop",visible:!0}},{name:"Hide Shop",action:{type:"show_shop",visible:!1}},{name:"Change Name",action:{type:"phone_change_name"}},{name:"Team Invite",action:{type:"phone_show_invite",from:"Pomo"}},{name:"New Job",action:{type:"phone_new_job",prompt:"hello i have a job for you"}},{name:"Phone Interact Key",action:{type:"phone_interact_key",mouse:!1}}];c`<div id="debug-handle">
      debug
      <div
        id="debug-collapse"
        @click="${()=>{e.visible=!e.visible,e.visible?localStorage.setItem("debugVisible","true"):localStorage.removeItem("debugVisible")}}"
      >
        ${()=>e.visible?"⮟":"⮞"}
      </div>
    </div>
    ${()=>e.visible?t.map((o,n)=>c`<li
              @click="${()=>{window.bridge_send(o.action)}}"
            >
              ${o.name}
            </li>`.key(n)):void 0} `(document.getElementById("debug")),u(document.getElementById("debug"),document.getElementById("debug-handle"))}function u(e,t){let i=t||e;["mousedown","touchstart"].forEach(o=>{e.style.left=localStorage.getItem("debugX")||"0px",e.style.top=localStorage.getItem("debugY")||"0px",i.addEventListener(o,n=>{var l=n.clientX-parseInt(getComputedStyle(e).left),m=n.clientY-parseInt(getComputedStyle(e).top);function s(a){e.style.top=a.clientY-m+"px",e.style.left=a.clientX-l+"px",localStorage.setItem("debugX",`${a.clientX-l}px`),localStorage.setItem("debugY",`${a.clientY-m}px`)}function d(){removeEventListener("mousemove",s),removeEventListener("mouseup",d)}addEventListener("mousemove",s),addEventListener("mouseup",d)})})}export{g as activate};
