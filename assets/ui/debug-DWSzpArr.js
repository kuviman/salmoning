import{r as p,h as d}from"./index-Hr60HwG0.js";function y(){const e=p({visible:!!localStorage.getItem("debugVisible")||!1}),t=[{name:"Join Team",action:{type:"sync_team_leader",name:"leader",is_self:!0}},{name:"BIG Money",action:{type:"sync_money",amount:5e3}},{name:"Add Money",action:{type:"sync_money",amount:100}},{name:"Remove Money",action:{type:"sync_money",amount:0}},{name:"Show Shop",action:{type:"show_shop",visible:!0}},{name:"Hide Shop",action:{type:"show_shop",visible:!1}},{name:"Change Name",action:{type:"phone_change_name"}},{name:"Team Invite",action:{type:"phone_show_invite",from:"Pomo"}},{name:"New Job",action:{type:"phone_new_job",prompt:"hello i have a job for you"}},{name:"Phone Interact Key",action:{type:"phone_interact_key",mouse:!1}},{name:"Show Summary",action:{type:"show_race_summary"}},{name:"Clear Summary",action:{type:"clear_race_summary"}},{name:"Add Summary Entry",action:{type:"update_race_summary",statistic:{type:"RaceStatistic",who:"badcop_",duration:30,place:1,total:5}}}];d`<div id="debug-handle">
      debug
      <div
        id="debug-collapse"
        @click="${()=>{e.visible=!e.visible,e.visible?localStorage.setItem("debugVisible","true"):localStorage.removeItem("debugVisible")}}"
      >
        ${()=>e.visible?"⮟":"⮞"}
      </div>
    </div>
    ${()=>e.visible?t.map((o,a)=>d`<li
              @click="${()=>{window.bridge_send(o.action)}}"
            >
              ${o.name}
            </li>`.key(a)):void 0} `(document.getElementById("debug")),r(document.getElementById("debug"),document.getElementById("debug-handle"))}function r(e,t){let m=t||e;["mousedown","touchstart"].forEach(o=>{e.style.left=localStorage.getItem("debugX")||"0px",e.style.top=localStorage.getItem("debugY")||"0px",m.addEventListener(o,a=>{var i=a.clientX-parseInt(getComputedStyle(e).left),l=a.clientY-parseInt(getComputedStyle(e).top);function s(n){e.style.top=n.clientY-l+"px",e.style.left=n.clientX-i+"px",localStorage.setItem("debugX",`${n.clientX-i}px`),localStorage.setItem("debugY",`${n.clientY-l}px`)}function c(){removeEventListener("mousemove",s),removeEventListener("mouseup",c)}addEventListener("mousemove",s),addEventListener("mouseup",c)})})}export{y as activate};
