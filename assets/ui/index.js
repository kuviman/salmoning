var w=Object.defineProperty;var E=(a,e,t)=>e in a?w(a,e,{enumerable:!0,configurable:!0,writable:!0,value:t}):a[e]=t;var d=(a,e,t)=>(E(a,typeof e!="symbol"?e+"":e,t),t);(function(){const e=document.createElement("link").relList;if(e&&e.supports&&e.supports("modulepreload"))return;for(const i of document.querySelectorAll('link[rel="modulepreload"]'))o(i);new MutationObserver(i=>{for(const s of i)if(s.type==="childList")for(const r of s.addedNodes)r.tagName==="LINK"&&r.rel==="modulepreload"&&o(r)}).observe(document,{childList:!0,subtree:!0});function t(i){const s={};return i.integrity&&(s.integrity=i.integrity),i.referrerPolicy&&(s.referrerPolicy=i.referrerPolicy),i.crossOrigin==="use-credentials"?s.credentials="include":i.crossOrigin==="anonymous"?s.credentials="omit":s.credentials="same-origin",s}function o(i){if(i.ep)return;i.ep=!0;const s=t(i);fetch(i.href,s)}})();const L="modulepreload",S=function(a){return"/"+a},k={},z=function(e,t,o){let i=Promise.resolve();if(t&&t.length>0){const s=document.getElementsByTagName("link"),r=document.querySelector("meta[property=csp-nonce]"),b=(r==null?void 0:r.nonce)||(r==null?void 0:r.getAttribute("nonce"));i=Promise.all(t.map(c=>{if(c=S(c),c in k)return;k[c]=!0;const p=c.endsWith(".css"),v=p?'[rel="stylesheet"]':"";if(!!o)for(let h=s.length-1;h>=0;h--){const m=s[h];if(m.href===c&&(!p||m.rel==="stylesheet"))return}else if(document.querySelector(`link[href="${c}"]${v}`))return;const l=document.createElement("link");if(l.rel=p?"stylesheet":L,p||(l.as="script",l.crossOrigin=""),l.href=c,b&&l.setAttribute("nonce",b),document.head.appendChild(l),p)return new Promise((h,m)=>{l.addEventListener("load",h),l.addEventListener("error",()=>m(new Error(`Unable to preload CSS for ${c}`)))})}))}return i.then(()=>e()).catch(s=>{const r=new Event("vite:preloadError",{cancelable:!0});if(r.payload=s,window.dispatchEvent(r),!r.defaultPrevented)throw s})};let u;const P=async()=>{try{const e="../../salmoning.js";u=(await z(()=>import(e),[])).send_message_to_world}catch(a){console.error("salmoning.js module is not available",a),u=()=>console.log("Fallback function")}};P();class f{constructor(){d(this,"app");d(this,"money");d(this,"shop");d(this,"phone");d(this,"ques");d(this,"job");d(this,"tasks");d(this,"boundAcceptHandler");d(this,"customizables");d(this,"unlocks");var e,t,o,i,s,r,b,c,p,v,y,l,h,m,q;document.querySelector("#app").innerHTML=`
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
`,this.app=document.querySelector("#app"),this.money=(e=this.app)==null?void 0:e.querySelector("#money"),this.shop=(t=this.app)==null?void 0:t.querySelector("#shop"),this.phone=(o=this.app)==null?void 0:o.querySelector("#phone"),this.ques=(i=this.app)==null?void 0:i.querySelector("#quest"),this.job=(s=this.app)==null?void 0:s.querySelector("job"),this.boundAcceptHandler=this.acceptHandler.bind(this),this.tasks=new Set,this.customizables={hat:{items:[],index:0,equipped:0},bike:{items:[],index:0,equipped:0}},this.unlocks={hats:[],bikes:[]},(r=this.app)==null||r.querySelector("#invite-accept").addEventListener("click",()=>{u({type:"AcceptInvite"}),this.remove_task("invite")}),(b=this.app)==null||b.querySelector("#invite-decline").addEventListener("click",()=>{u({type:"DeclineInvite"}),this.remove_task("invite")}),(c=this.app)==null||c.querySelector("#quest-accept").addEventListener("click",()=>{this.accept()}),(p=this.app)==null||p.querySelector("#quest-decline").addEventListener("click",()=>{this.accept()}),(v=this.app)==null||v.querySelector("#hat-next").addEventListener("click",()=>{this.next_custom("hat")}),(y=this.app)==null||y.querySelector("#hat-prev").addEventListener("click",()=>{this.prev_custom("hat")}),(l=this.app)==null||l.querySelector("#bike-next").addEventListener("click",()=>{this.next_custom("bike")}),(h=this.app)==null||h.querySelector("#bike-prev").addEventListener("click",()=>{this.prev_custom("bike")}),(m=this.app)==null||m.querySelector("#bike-equip").addEventListener("click",()=>{const n="bike";this.customizables[n].equipped=this.customizables[n].index,u({type:"EquipAndBuy",kind:n,index:this.customizables[n].index})}),(q=this.app)==null||q.querySelector("#hat-equip").addEventListener("click",()=>{const n="hat";this.customizables[n].equipped=this.customizables[n].index,u({type:"EquipAndBuy",kind:n,index:this.customizables[n].index})}),this.phone.addEventListener("mousemove",n=>{var _;((_=document==null?void 0:document.activeElement)==null?void 0:_.id)==="name_input"&&n.stopPropagation()}),this.phone.addEventListener("keydown",n=>{n.target.id==="name_input"&&(n.stopPropagation(),n.key==="Enter"&&(this.remove_task("choose_name"),u({type:"ChangeName",name:n.target.value}),n.target.blur()))}),this.phone.addEventListener("keyup",n=>{n.target.id==="name_input"&&n.stopPropagation()})}sync_money(e){this.money.innerHTML=`$${e}`}set_inviter(e){this.app.querySelector("#inviter").innerHTML=`- ${e}`}add_task(e){var t;this.phone.classList.remove("phone_down"),(t=this.phone.querySelector(`#${e}`))==null||t.classList.remove("hidden"),this.tasks.add(e)}remove_task(e){var t;(t=this.phone.querySelector(`#${e}`))==null||t.classList.add("hidden"),this.tasks.delete(e),this.tasks.size||this.phone.classList.add("phone_down")}prev_custom(e){const{length:t}=this.customizables[e].items;let{index:o}=this.customizables[e];o-=1,o<0&&(o=t-1),this.customizables[e].index=o,this.render_custom(e,o)}next_custom(e){const{length:t}=this.customizables[e].items;let{index:o}=this.customizables[e];o+=1,o>=t&&(o=0),this.customizables[e].index=o,this.render_custom(e,o)}render_custom(e,t,o=!0){if(console.warn({kind:e,index:t,c:this.customizables}),t<0||t>=this.customizables[e].items.length){console.error(`early access of ${e} at ${t}`);return}const{name:i,cost:s}=this.customizables[e].items[t]||{name:"None",cost:0},r=this.unlocks[`${e}s`].includes(t);this.app.querySelector(`#${e}-name`).innerHTML=i,s===0?this.app.querySelector(`#${e}-cost`).innerHTML="Free!":this.app.querySelector(`#${e}-cost`).innerHTML=`Cost: $${s}`,this.app.querySelector(`#${e}-equip`).innerHTML=`${s===0||r?"Equip":"Buy"}`,o&&u({type:"PreviewCosmetic",kind:e,index:t})}send_unlocks(e){this.unlocks=e}send_customizations(e){console.warn(e),this.customizables.hat.items=e.hat_names,this.customizables.bike.items=e.bike_names}show_shop(e){e?(this.render_custom("hat",this.customizables.hat.index,!1),this.render_custom("bike",this.customizables.bike.index,!1),this.shop.classList.remove("hidden")):(this.shop.classList.add("hidden"),this.customizables.hat.index=this.customizables.hat.equipped,this.customizables.bike.index=this.customizables.bike.equipped)}accept(){u({type:"AcceptQuest"}),document.removeEventListener("keydown",this.boundAcceptHandler),this.remove_task("job")}acceptHandler(e){e.key==="e"&&(e.stopPropagation(),this.accept())}quest(){const e=["Can you take my books back to the library?","Bring me my food now!!!!","Please pick up my dry cleaning","I need 3 gerbils ASAP. No questions please","Can you deliver my groceries? I need tomato","I AM OUT OF TOILET PAPER GO FAST PLEASE","i want spaghetti","HUNGRY!!!!!!","bring me some flowers.","please do not look in this bag. just deliver","i would like 1 newspaper please","its me, pgorley","please serve these court summons for me","i ran out of coffee creamer. can you bring me some butter?","i need 37 cans of soup. no time to explain","can you deliver sushi","deliver this mail for me","can you take this trash away","i need a new kidney","PLEASE DELIVER MY TELEGRAM STOP DONT STOP STOP","find my pet turtle","let's go bowling cousin","listen, you just drive. to point B. simple.","2 Number 9's, a number 9 large, a number 6 with extra dip, 2 number 45's (one with cheese) and a large soda"],t=e[Math.floor(Math.random()*e.length)];this.add_task("job"),this.ques.innerHTML=`"${t}"`,document.addEventListener("keydown",this.boundAcceptHandler)}}let g;window.bridge_init=()=>{g=new f};Object.getOwnPropertyNames(f.prototype).forEach(a=>{window[`bridge_${a}`]=function(){return g&&f.prototype[a].apply(g,arguments)}});
