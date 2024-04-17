var q=Object.defineProperty;var g=(c,e,t)=>e in c?q(c,e,{enumerable:!0,configurable:!0,writable:!0,value:t}):c[e]=t;var a=(c,e,t)=>(g(c,typeof e!="symbol"?e+"":e,t),t);import{bridge_reply as d}from"../../salmoning.js";(function(){const e=document.createElement("link").relList;if(e&&e.supports&&e.supports("modulepreload"))return;for(const i of document.querySelectorAll('link[rel="modulepreload"]'))s(i);new MutationObserver(i=>{for(const n of i)if(n.type==="childList")for(const r of n.addedNodes)r.tagName==="LINK"&&r.rel==="modulepreload"&&s(r)}).observe(document,{childList:!0,subtree:!0});function t(i){const n={};return i.integrity&&(n.integrity=i.integrity),i.referrerPolicy&&(n.referrerPolicy=i.referrerPolicy),i.crossOrigin==="use-credentials"?n.credentials="include":i.crossOrigin==="anonymous"?n.credentials="omit":n.credentials="same-origin",n}function s(i){if(i.ep)return;i.ep=!0;const n=t(i);fetch(i.href,n)}})();class k{constructor(){a(this,"app");a(this,"money");a(this,"shop");a(this,"phone");a(this,"ques");a(this,"tasks");a(this,"boundAcceptHandler");a(this,"customizables");a(this,"unlocks");var e,t,s,i,n,r,h,l,m,b,v,y,f,_;document.querySelector("#app").innerHTML=`
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
`,this.app=document.querySelector("#app"),this.money=(e=this.app)==null?void 0:e.querySelector("#money"),this.shop=(t=this.app)==null?void 0:t.querySelector("#shop"),this.phone=(s=this.app)==null?void 0:s.querySelector("#phone"),this.ques=(i=this.app)==null?void 0:i.querySelector("#quest"),this.boundAcceptHandler=this.acceptHandler.bind(this),this.tasks=new Set,this.customizables={hat:{items:[],index:0,equipped:0},bike:{items:[],index:0,equipped:0}},this.unlocks={hats:[],bikes:[]},(n=this.app)==null||n.querySelector("#invite-accept").addEventListener("click",()=>{d({type:"accept_invite"}),this.remove_task("invite")}),(r=this.app)==null||r.querySelector("#invite-decline").addEventListener("click",()=>{d({type:"decline_invite"}),this.remove_task("invite")}),(h=this.app)==null||h.querySelector("#quest-accept").addEventListener("click",()=>{this.accept()}),(l=this.app)==null||l.querySelector("#quest-decline").addEventListener("click",()=>{this.accept()}),(m=this.app)==null||m.querySelector("#hat-next").addEventListener("click",()=>{this.next_custom("hat")}),(b=this.app)==null||b.querySelector("#hat-prev").addEventListener("click",()=>{this.prev_custom("hat")}),(v=this.app)==null||v.querySelector("#bike-next").addEventListener("click",()=>{this.next_custom("bike")}),(y=this.app)==null||y.querySelector("#bike-prev").addEventListener("click",()=>{this.prev_custom("bike")}),(f=this.app)==null||f.querySelector("#bike-equip").addEventListener("click",()=>{const o="bike",p=this.customizables[o].index;this.customizables[o].equipped=p,d({type:"equip_and_buy",kind:o,index:p})}),(_=this.app)==null||_.querySelector("#hat-equip").addEventListener("click",()=>{const o="hat";this.customizables[o].equipped=this.customizables[o].index,d({type:"equip_and_buy",kind:o,index:this.customizables[o].index})}),this.phone.addEventListener("mousemove",o=>{var p;((p=document==null?void 0:document.activeElement)==null?void 0:p.id)==="name_input"&&o.stopPropagation()}),this.phone.addEventListener("keydown",o=>{o.target.id==="name_input"&&(o.stopPropagation(),o.key==="Enter"&&(this.remove_task("choose_name"),d({type:"change_name",name:o.target.value}),o.target.blur()))}),this.phone.addEventListener("keyup",o=>{o.target.id==="name_input"&&o.stopPropagation()})}remove_task(e){var t;(t=this.phone.querySelector(`#${e}`))==null||t.classList.add("hidden"),this.tasks.delete(e),this.tasks.size||this.phone.classList.add("phone_down")}add_task(e){var t;this.phone.classList.remove("phone_down"),(t=this.phone.querySelector(`#${e}`))==null||t.classList.remove("hidden"),this.tasks.add(e)}prev_custom(e){const{length:t}=this.customizables[e].items;let{index:s}=this.customizables[e];s-=1,s<0&&(s=t-1),this.customizables[e].index=s,this.render_custom(e,s)}next_custom(e){const{length:t}=this.customizables[e].items;let{index:s}=this.customizables[e];s+=1,s>=t&&(s=0),this.customizables[e].index=s,this.render_custom(e,s)}render_custom(e,t,s=!0){if(console.warn({kind:e,index:t,c:this.customizables}),t<0||t>=this.customizables[e].items.length){console.error(`early access of ${e} at ${t}`);return}const{name:i,cost:n}=this.customizables[e].items[t]||{name:"None",cost:0},r=this.unlocks[`${e}s`].includes(t);this.app.querySelector(`#${e}-name`).innerHTML=i,n===0?this.app.querySelector(`#${e}-cost`).innerHTML="Free!":this.app.querySelector(`#${e}-cost`).innerHTML=`Cost: $${n}`,this.app.querySelector(`#${e}-equip`).innerHTML=`${n===0||r?"Equip":"Buy"}`,s&&d({type:"preview_cosmetic",kind:e,index:t})}receive(e){switch(e.type){case"sync_money":this.money.innerHTML=`$${e.amount}`;break;case"phone_show_invite":this.app.querySelector("#inviter").innerHTML=`- ${e.from}`,this.add_task("invite");break;case"unlocks":this.unlocks=e,this.render_custom("hat",this.customizables.hat.index,!1),this.render_custom("bike",this.customizables.bike.index,!1);break;case"customization_info":this.customizables.hat.items=e.hat_names,this.customizables.bike.items=e.bike_names;break;case"show_shop":e.visible?(this.render_custom("hat",this.customizables.hat.index,!1),this.render_custom("bike",this.customizables.bike.index,!1),this.shop.classList.remove("hidden")):(this.shop.classList.add("hidden"),this.customizables.hat.index=this.customizables.hat.equipped,this.customizables.bike.index=this.customizables.bike.equipped);break;case"phone_change_name":this.add_task("choose_name");break;case"phone_new_job":this.quest();break;case"phone_accept_invite":case"phone_reject_invite":this.remove_task("invite");break;case"phone_dismiss_notification":this.remove_task("job");break;default:console.error("Unexpected message received!",e)}}accept(){d({type:"accept_quest"}),document.removeEventListener("keydown",this.boundAcceptHandler),this.remove_task("job")}acceptHandler(e){e.key==="e"&&(e.stopPropagation(),this.accept())}quest(){const e=["Can you take my books back to the library?","Bring me my food now!!!!","Please pick up my dry cleaning","I need 3 gerbils ASAP. No questions please","Can you deliver my groceries? I need tomato","I AM OUT OF TOILET PAPER GO FAST PLEASE","i want spaghetti","HUNGRY!!!!!!","bring me some flowers.","please do not look in this bag. just deliver","i would like 1 newspaper please","its me, pgorley","please serve these court summons for me","i ran out of coffee creamer. can you bring me some butter?","i need 37 cans of soup. no time to explain","can you deliver sushi","deliver this mail for me","can you take this trash away","i need a new kidney","PLEASE DELIVER MY TELEGRAM STOP DONT STOP STOP","find my pet turtle","let's go bowling cousin","listen, you just drive. to point B. simple.","2 Number 9's, a number 9 large, a number 6 with extra dip, 2 number 45's (one with cheese) and a large soda"],t=e[Math.floor(Math.random()*e.length)];this.add_task("job"),this.ques.innerHTML=`"${t}"`,document.addEventListener("keydown",this.boundAcceptHandler)}}let u;window.bridge_init=()=>{u=new k};window.bridge_send=function(){return(u||(console.warn("Bridge accessed before init!"),0))&&k.prototype.receive.apply(u,arguments)};
