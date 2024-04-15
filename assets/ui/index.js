var g=Object.defineProperty;var w=(c,e,t)=>e in c?g(c,e,{enumerable:!0,configurable:!0,writable:!0,value:t}):c[e]=t;var l=(c,e,t)=>(w(c,typeof e!="symbol"?e+"":e,t),t);(function(){const e=document.createElement("link").relList;if(e&&e.supports&&e.supports("modulepreload"))return;for(const s of document.querySelectorAll('link[rel="modulepreload"]'))n(s);new MutationObserver(s=>{for(const o of s)if(o.type==="childList")for(const r of o.addedNodes)r.tagName==="LINK"&&r.rel==="modulepreload"&&n(r)}).observe(document,{childList:!0,subtree:!0});function t(s){const o={};return s.integrity&&(o.integrity=s.integrity),s.referrerPolicy&&(o.referrerPolicy=s.referrerPolicy),s.crossOrigin==="use-credentials"?o.credentials="include":s.crossOrigin==="anonymous"?o.credentials="omit":o.credentials="same-origin",o}function n(s){if(s.ep)return;s.ep=!0;const o=t(s);fetch(s.href,o)}})();const L="modulepreload",E=function(c){return"/"+c},_={},k=function(e,t,n){let s=Promise.resolve();if(t&&t.length>0){const o=document.getElementsByTagName("link"),r=document.querySelector("meta[property=csp-nonce]"),m=(r==null?void 0:r.nonce)||(r==null?void 0:r.getAttribute("nonce"));s=Promise.all(t.map(a=>{if(a=E(a),a in _)return;_[a]=!0;const u=a.endsWith(".css"),b=u?'[rel="stylesheet"]':"";if(!!n)for(let p=o.length-1;p>=0;p--){const i=o[p];if(i.href===a&&(!u||i.rel==="stylesheet"))return}else if(document.querySelector(`link[href="${a}"]${b}`))return;const d=document.createElement("link");if(d.rel=u?"stylesheet":L,u||(d.as="script",d.crossOrigin=""),d.href=a,m&&d.setAttribute("nonce",m),document.head.appendChild(d),u)return new Promise((p,i)=>{d.addEventListener("load",p),d.addEventListener("error",()=>i(new Error(`Unable to preload CSS for ${a}`)))})}))}return s.then(()=>e()).catch(o=>{const r=new Event("vite:preloadError",{cancelable:!0});if(r.payload=o,window.dispatchEvent(r),!r.defaultPrevented)throw o})};let h;const S=async()=>{try{const e="../../salmoning.js";h=(await k(()=>import(e),[])).send_message_to_world}catch(c){console.error("salmoning.js module is not available",c),h=()=>console.log("Fallback function")}};S();class y{constructor(){l(this,"app");l(this,"money");l(this,"shop");l(this,"phone");l(this,"ques");l(this,"job");l(this,"tasks");l(this,"boundAcceptHandler");l(this,"customizables");var e,t,n,s,o,r,m,a,u,b,v,d,p;document.querySelector("#app").innerHTML=`
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
`,this.app=document.querySelector("#app"),this.money=(e=this.app)==null?void 0:e.querySelector("#money"),this.shop=(t=this.app)==null?void 0:t.querySelector("#shop"),this.phone=(n=this.app)==null?void 0:n.querySelector("#phone"),this.ques=(s=this.app)==null?void 0:s.querySelector("#quest"),this.job=(o=this.app)==null?void 0:o.querySelector("job"),this.boundAcceptHandler=this.acceptHandler.bind(this),this.tasks=new Set,this.customizables={hat:{items:[],index:0,equipped:0},bike:{items:[],index:0,equipped:0}},(r=this.app)==null||r.querySelector("#quest-accept").addEventListener("click",()=>{this.accept()}),(m=this.app)==null||m.querySelector("#quest-decline").addEventListener("click",()=>{this.accept()}),(a=this.app)==null||a.querySelector("#hat-next").addEventListener("click",()=>{this.next_custom("hat")}),(u=this.app)==null||u.querySelector("#hat-prev").addEventListener("click",()=>{this.prev_custom("hat")}),(b=this.app)==null||b.querySelector("#bike-next").addEventListener("click",()=>{this.next_custom("bike")}),(v=this.app)==null||v.querySelector("#bike-prev").addEventListener("click",()=>{this.prev_custom("bike")}),(d=this.app)==null||d.querySelector("#bike-equip").addEventListener("click",()=>{const i="bike";this.customizables[i].equipped=this.customizables[i].index,h({type:"EquipAndBuy",kind:i,index:this.customizables[i].index})}),(p=this.app)==null||p.querySelector("#hat-equip").addEventListener("click",()=>{const i="hat";this.customizables[i].equipped=this.customizables[i].index,h({type:"EquipAndBuy",kind:i,index:this.customizables[i].index})}),this.phone.addEventListener("mousemove",i=>{var q;((q=document==null?void 0:document.activeElement)==null?void 0:q.id)==="name_input"&&i.stopPropagation()}),this.phone.addEventListener("keydown",i=>{i.target.id==="name_input"&&(i.stopPropagation(),i.key==="Enter"&&(this.remove_task("choose_name"),h({type:"ChangeName",name:i.target.value}),i.target.blur()))}),this.phone.addEventListener("keyup",i=>{i.target.id==="name_input"&&i.stopPropagation()})}sync_money(e){this.money.innerHTML=`$${e}`}add_task(e){var t;this.phone.classList.remove("phone_down"),(t=this.phone.querySelector(`#${e}`))==null||t.classList.remove("hidden"),this.tasks.add(e)}remove_task(e){var t;(t=this.phone.querySelector(`#${e}`))==null||t.classList.add("hidden"),this.tasks.delete(e),this.tasks.size||this.phone.classList.add("phone_down")}prev_custom(e){const{length:t}=this.customizables[e].items;let{index:n}=this.customizables[e];n-=1,n<0&&(n=t-1),this.customizables[e].index=n,this.render_custom(e,n)}next_custom(e){const{length:t}=this.customizables[e].items;let{index:n}=this.customizables[e];n+=1,n>=t&&(n=0),this.customizables[e].index=n,this.render_custom(e,n)}render_custom(e,t){if(console.warn({kind:e,index:t,c:this.customizables}),t<0||t>=this.customizables[e].items.length){console.error(`early access of ${e} at ${t}`);return}const{name:n,cost:s,owned:o}=this.customizables[e].items[t]||{name:"None",cost:0};this.app.querySelector(`#${e}-name`).innerHTML=n,s===0?this.app.querySelector(`#${e}-cost`).innerHTML="Free!":this.app.querySelector(`#${e}-cost`).innerHTML=`Cost: $${s}`,this.app.querySelector(`#${e}-equip`).innerHTML=`${s===0||o?"Equip":"Buy"}`,h({type:"PreviewCosmetic",kind:e,index:t})}send_customizations(e){console.warn(e),this.customizables.hat.items=e.hat_names,this.customizables.bike.items=e.bike_names}show_shop(e){e?(this.render_custom("hat",this.customizables.hat.index),this.render_custom("bike",this.customizables.bike.index),this.shop.classList.remove("hidden")):(this.shop.classList.add("hidden"),this.customizables.hat.index=this.customizables.hat.equipped,this.customizables.bike.index=this.customizables.bike.equipped)}accept(){h({type:"AcceptQuest"}),document.removeEventListener("keydown",this.boundAcceptHandler),this.remove_task("job")}acceptHandler(e){e.key==="e"&&(e.stopPropagation(),this.accept())}quest(e){this.add_task("job"),this.ques.innerHTML=`"${e}"`,document.addEventListener("keydown",this.boundAcceptHandler)}}let f;window.bridge_init=()=>{f=new y};Object.getOwnPropertyNames(y.prototype).forEach(c=>{window[`bridge_${c}`]=function(){return f&&y.prototype[c].apply(f,arguments)}});
