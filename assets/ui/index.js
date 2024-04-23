var ae=Object.defineProperty;var le=(e,t,n)=>t in e?ae(e,t,{enumerable:!0,configurable:!0,writable:!0,value:n}):e[t]=n;var V=(e,t,n)=>(le(e,typeof t!="symbol"?t+"":t,n),n);(function(){const t=document.createElement("link").relList;if(t&&t.supports&&t.supports("modulepreload"))return;for(const o of document.querySelectorAll('link[rel="modulepreload"]'))s(o);new MutationObserver(o=>{for(const r of o)if(r.type==="childList")for(const i of r.addedNodes)i.tagName==="LINK"&&i.rel==="modulepreload"&&s(i)}).observe(document,{childList:!0,subtree:!0});function n(o){const r={};return o.integrity&&(r.integrity=o.integrity),o.referrerPolicy&&(r.referrerPolicy=o.referrerPolicy),o.crossOrigin==="use-credentials"?r.credentials="include":o.crossOrigin==="anonymous"?r.credentials="omit":r.credentials="same-origin",r}function s(o){if(o.ep)return;o.ep=!0;const r=n(o);fetch(o.href,r)}})();const ue="modulepreload",de=function(e){return"/"+e},J={},Q=function(t,n,s){let o=Promise.resolve();if(n&&n.length>0){const r=document.getElementsByTagName("link"),i=document.querySelector("meta[property=csp-nonce]"),a=(i==null?void 0:i.nonce)||(i==null?void 0:i.getAttribute("nonce"));o=Promise.all(n.map(u=>{if(u=de(u),u in J)return;J[u]=!0;const f=u.endsWith(".css"),y=f?'[rel="stylesheet"]':"";if(!!s)for(let p=r.length-1;p>=0;p--){const c=r[p];if(c.href===u&&(!f||c.rel==="stylesheet"))return}else if(document.querySelector(`link[href="${u}"]${y}`))return;const d=document.createElement("link");if(d.rel=f?"stylesheet":ue,f||(d.as="script",d.crossOrigin=""),d.href=u,a&&d.setAttribute("nonce",a),document.head.appendChild(d),f)return new Promise((p,c)=>{d.addEventListener("load",p),d.addEventListener("error",()=>c(new Error(`Unable to preload CSS for ${u}`)))})}))}return o.then(()=>t()).catch(r=>{const i=new Event("vite:preloadError",{cancelable:!0});if(i.payload=r,window.dispatchEvent(i),!i.defaultPrevented)throw r})};let ee=(...e)=>console.log("SENDING BRIDGE REPLY",...e);(async()=>{try{const t="../../salmoning.js";ee=(await Q(()=>import(t),[])).bridge_reply}catch{console.warn("activating ui debug"),(await Q(()=>import("./debug-CeGruBaV.js"),[])).activate()}})();function $(){return ee(...arguments)}const P=new Set,X=new Set;function z(e){return typeof e=="function"&&!!e.isT}function C(e){return typeof e=="object"&&e!==null&&"$on"in e&&typeof e.$on=="function"}function me(e){return"$on"in e}function pe(e){return(t,n)=>{function s(){const o=Array.from(P);P.clear();const r=Array.from(X);X.clear(),o.forEach(i=>i(t,n)),r.forEach(i=>i()),P.size&&queueMicrotask(s)}P.size||queueMicrotask(s),P.add(e)}}const D={};function fe(e,t){const n=performance.now(),s=typeof t=="function";e=s?`${e} (ms)`:`${e} (calls)`;const o=s?t():t,r=s?performance.now()-n:t;return D[e]?D[e].push(r):D[e]=[r],o}const L=new Map;function T(e,t={}){if(C(e)||typeof e!="object")return e;const n=t.o||new Map,s=t.op||new Map,o=Array.isArray(e),r=[],i=o?[]:Object.create(e,{});for(const c in e){const l=e[c];typeof l=="object"&&l!==null?(i[c]=C(l)?l:T(l),r.push(c)):i[c]=l}const a=c=>(l,h)=>{let m=n.get(l),_=s.get(h);m||(m=new Set,n.set(l,m)),_||(_=new Set,s.set(h,_)),m[c](h),_[c](l)},u=a("add"),f=a("delete"),y=(c,l,h)=>{n.has(c)&&n.get(c).forEach(m=>m(l,h))},d={$on:u,$off:f,_em:y,_st:()=>({o:n,op:s,r:i,p:p._p}),_p:void 0},p=new Proxy(i,{has(c,l){return l in d||l in c},get(...c){const[,l]=c;if(Reflect.has(d,l))return Reflect.get(d,l);const h=Reflect.get(...c);return he(p,l),o&&l in Array.prototype?ve(l,i,p,h):h},set(...c){const[l,h,m]=c,_=Reflect.get(l,h);if(Reflect.has(d,h))return Reflect.set(d,h,m);if(m&&C(_)){const A=_,O=A._st(),B=C(m)?ye(m,A):T(m,O);return Reflect.set(l,h,B),y(h,B),O.o.forEach((De,j)=>{const U=Reflect.get(_,j),K=Reflect.get(B,j);U!==K&&A._em(j,K,U)}),!0}const k=Reflect.set(...c);return k&&(_!==m&&y(h,m,_),p._p&&p._p[1]._em(...p._p)),k}});return t.p&&(p._p=t.p),r.map(c=>{p[c]._p=[c,p]}),p}function he(e,t){L.forEach(n=>{let s=n.get(e);s||(s=new Set,n.set(e,s)),s.add(t)})}function ve(e,t,n,s){const o=(...r)=>{const i=Array.prototype[e].call(t,...r);if(t.forEach((a,u)=>n._em(String(u),a)),n._p){const[a,u]=n._p;u._em(a,n)}return i};switch(e){case"shift":case"pop":case"sort":case"reverse":case"copyWithin":return o;case"unshift":case"push":case"fill":return(...r)=>o(...r.map(i=>T(i)));case"splice":return function(r,i,...a){return arguments.length===1?o(r):o(r,i,...a.map(u=>T(u)))};default:return s}}function ye(e,t){const n=t._st();return n.o&&n.o.forEach((s,o)=>{s.forEach(r=>{e.$on(o,r)})}),n.p&&(e._p=n.p),e}function H(e,t){const n=Symbol();L.has(n)||L.set(n,new Map);let s=new Map;const o=pe(r);function r(){L.set(n,new Map);const i=e(),a=L.get(n);return L.delete(n),s.forEach((u,f)=>{const y=a.get(f);y&&y.forEach(S=>u.delete(S)),u.forEach(S=>f.$off(S,o))}),a.forEach((u,f)=>{u.forEach(y=>f.$on(y,o))}),s=a,t?t(i):i}return me(e)&&e.$on(r),r()}const I=new WeakMap,Z={},te="➳❍",ne="❍⇚",se=`<!--${te}-->`,be=`<!--${ne}-->`;function ie(e,...t){const n=[];let s="";const o=(a,u)=>{if(typeof a=="function"){let f=()=>{};return n.push(Object.assign((...y)=>a(...y),{e:a,$on:y=>{f=y},_up:y=>{a=y,f()}})),u+se}return Array.isArray(a)?a.reduce((f,y)=>o(y,f),u):u+a},r=()=>(s||(!t.length&&e.length===1&&e[0]===""?s="<!---->":s=e.reduce(function(u,f,y){return u+=f,t[y]!==void 0?o(t[y],u):u},"")),s),i=a=>{const u=oe(r()),f=Y(u,{i:0,e:n});return a?f(a):f()};return i.isT=!0,i._k=0,i._h=()=>[r(),n,i._k],i.key=a=>(i._k=a,i),i}function Y(e,t){let n,s=0;const o=e.childNodes;for(;n=o.item(s++);){if(n.nodeType===8&&n.nodeValue===te){we(n,t);continue}n instanceof Element&&ge(n,t),n.hasChildNodes()&&Y(n,t),n instanceof HTMLOptionElement&&(n.selected=n.defaultSelected)}return r=>r?(r.appendChild(e),r):e}function ge(e,t){var n;const s=[];let o=0,r;for(;r=e.attributes[o++];){if(t.i>=t.e.length)return;if(r.value!==se)continue;let i=r.name;const a=t.e[t.i++];if(i.charAt(0)==="@"){const u=i.substring(1);e.addEventListener(u,a),I.has(e)||I.set(e,new Map),(n=I.get(e))===null||n===void 0||n.set(u,a),s.push(i)}else{const u=i==="value"&&"value"in e||i==="checked"||i.startsWith(".")&&(i=i.substring(1));H(a,f=>{u&&(e[i]=f,e.getAttribute(i)!=f&&(f=!1)),f!==!1?e.setAttribute(i,f):(e.removeAttribute(i),o--)})}}s.forEach(i=>e.removeAttribute(i))}function _e(e){e.forEach(ke)}function ke(e){var t;e.remove(),(t=I.get(e))===null||t===void 0||t.forEach((n,s)=>e.removeEventListener(s,n))}function we(e,t){var n;const s=t.e[t.i++];let o;if(s&&z(s.e))o=G().add(s.e)();else{let r;o=(r=H(s,i=>$e(i,r)))()}(n=e.parentNode)===null||n===void 0||n.replaceChild(o,e)}function $e(e,t){const n=typeof t=="function",s=n?t:G();return Array.isArray(e)?e.forEach(o=>fe("partialAdd",()=>s.add(o))):s.add(e),n&&s._up(),s}function oe(e){var t;const s=((t=Z[e])!==null&&t!==void 0?t:(()=>{const o=document.createElement("template");return o.innerHTML=e,Z[e]=o})()).content.cloneNode(!0);return s.normalize(),s}function G(e=Symbol()){let t="",n={i:0,e:[]},s=[],o=[];const r=new Map,i=[],a=()=>{let d;if(s.length||f(),s.length===1&&!z(s[0].tpl)){const p=s[0];p.dom.length?p.dom[0].nodeValue=p.tpl:p.dom.push(document.createTextNode(p.tpl)),d=p.dom[0]}else d=y(Y(oe(t),n)());return u(),d};a.ch=()=>o,a.l=0,a.add=d=>{if(!d&&d!==0)return a;let p=[],c,l="";z(d)&&([l,p,c]=d._h()),t+=l,t+=be;const h=c&&r.get(c),m=h||{html:l,exp:p,dom:[],tpl:d,key:c};return s.push(m),c&&(h?h.exp.forEach((_,k)=>_._up(p[k].e)):r.set(c,m)),n.e.push(...p),a.l++,a},a._up=()=>{const d=G(e);let p=0,c=o[0].dom[0];s.length||f(document.createComment(""));const l=()=>{if(!d.l)return;const m=d(),_=m.lastChild;c[p?"after":"before"](m),S(d,s,p),c=_};s.forEach((m,_)=>{const k=o[_];m.key&&m.dom.length?(l(),(!k||k.dom!==m.dom)&&c[_?"after":"before"](...m.dom),c=m.dom[m.dom.length-1]):k&&m.html===k.html&&!k.key?(l(),k.exp.forEach((A,O)=>A._up(m.exp[O].e)),m.exp=k.exp,m.dom=k.dom,c=m.dom[m.dom.length-1],Ee(m)&&c instanceof Text&&(c.nodeValue=m.tpl)):(k&&m.html!==k.html&&!k.key&&i.push(...k.dom),d.l||(p=_),d.add(m.tpl))}),l();let h=c==null?void 0:c.nextSibling;for(;h&&e in h;)i.push(h),h=h.nextSibling;_e(i),u()};const u=()=>{i.length=0,t="",a.l=0,n={i:0,e:[]},o=[...s],s=[]},f=d=>{t="<!---->",s.push({html:t,exp:[],dom:d?[d]:[],tpl:ie`${t}`,key:0})},y=d=>{let p=0;const c=[];return d.childNodes.forEach(l=>{if(l.nodeType===8&&l.data===ne){p++,c.push(l);return}Object.defineProperty(l,e,{value:e}),s[p].dom.push(l)}),c.forEach(l=>l.remove()),d},S=(d,p,c)=>{d.ch().forEach((l,h)=>{p[c+h].dom=l.dom})};return a}function Ee(e){return e.dom.length===1&&!z(e.tpl)}const g=ie,re=H;function M(e){return T(e)}const v=M({tasks:[],questText:"spaghetti",inviter:"kuviman",teamLeader:null,isSelfLeader:!1,alertText:""}),N={base:{priority:0,template:Re,closeOnInteract:!0},settings:{priority:1,template:ze,closeOnInteract:!0},races:{priority:1,template:Ie,closeOnInteract:!0},race_list:{priority:2,template:Oe,closeOnInteract:!0},race_editor:{priority:3,template:Ce},change_name:{priority:2,template:Ne},job:{priority:5,template:qe,closeOnInteract:!0},invite:{priority:10,template:Te},race_circle:{priority:15,template:Ae},alert:{priority:20,template:Pe,closeOnInteract:!0}};function Se(e){v.alertText=e,x("alert")}function R(e){v.tasks.shift(),x(e)}function w(e){v.tasks=v.tasks.filter(t=>t!==e)}function xe(e){if(v.tasks.length===0){e||x("base");return}if(N[v.tasks[0]].closeOnInteract){v.tasks.shift();return}}function x(e){v.tasks.includes(e)||v.tasks.push(e),v.tasks.sort((t,n)=>N[n].priority-N[t].priority)}function Le(){return g`
    <div
      id="phone"
      class="${()=>v.tasks.length?"":"phone_down"}"
    >
      ${()=>v.tasks.length?N[v.tasks[0]].template():void 0}
    </div>
  `}function Ae(){return g`
    <div class="screen">
      <p>
        ${()=>v.isSelfLeader?"Ring your bell to start the race.":"Wait for the leader's bell to start!"}
      </p>
    </div>
  `}function Pe(){function e(){w("alert")}return g`
    <div class="screen">
      <p>${()=>v.alertText}</p>
      <div class="flex-row">
        <div class="button accept" @click="${e}">Ok</div>
      </div>
    </div>
  `}function qe(){function e(){w("job")}return g`
    <div class="screen">
      <p>Someone is summoning you!</p>
      <p>"${()=>v.questText}"</p>
      <div class="flex-row">
        <div class="button accept" @click="${e}">Nice</div>
        <div class="button decline" @click="${e}">OK</div>
      </div>
    </div>
  `}function Te(){function e(t){w("invite"),$({type:t})}return g`
    <div class="screen" id="invite">
      <p>New Message</p>
      <p>"yo, wanna join my racing crew?"</p>
      <p id="inviter">- ${()=>v.inviter}</p>
      <div class="flex-row">
        <div class="button accept" @click="${()=>e("accept_invite")}">
          (Y)es
        </div>
        <div class="button decline" @click="${()=>e("decline_invite")}">
          (N)o
        </div>
      </div>
    </div>
  `}function Re(){function e(){$({type:"leave_team"}),w("base")}let t="./icons";return t="assets/icons",g`
    <div class="screen">
      ${()=>v.teamLeader&&g`${()=>v.isSelfLeader?"You are a leader":`Leader: ${v.teamLeader}`}
          <div class="button decline" @click="${e}">Leave Team</div>`}
      <div class="flex-row">
        <div
          class="button secondary"
          @click="${()=>R("settings")}"
        >
          Settings
        </div>
        <div class="button" @click="${()=>R("races")}">
          Races
        </div>
      </div>
      <div class="phone-grid">
        <img src="${t}/msg.png" />
        <img src="${t}/salmazon.png" />
        <img src="${t}/firefish.png" />
        <img src="${t}/fishmail.png" />
        <img src="${t}/phone.png" />
        <img src="${t}/fishmaps.png" />
        <img src="${t}/fishbook.png" />
        <img src="${t}/fishdonalds.png" />
        <img src="${t}/fishcord.png" />
        <img src="${t}/samneats.png" />
        <img src="${t}/fwich.png" />
      </div>
    </div>
  `}function Oe(){let e=JSON.parse(localStorage.getItem("./races")||'{ "races": {} }');function t(n){return function(){$({type:"race_start",name:n}),w("race_list")}}return g`<div class="screen">
    Pick a Race:
    <div class="race-list">
      ${Object.entries(e.races).map(([n,s])=>g`<div class="race-option" @click="${t(n)}">
          ${n}<br />
          ${s.track.length} checkpoints
        </div>`)}
    </div>
  </div>`}function Ce(){function e(){$({type:"race_edit_cancel"}),w("race_editor")}function t(){const n=document.getElementById("race-name").value;$({type:"race_edit_submit",name:n}),w("race_editor")}return g`
    <div class="screen">
      Race Editor
      <input type="text" placeholder="name" id="race-name" />
      <div class="button decline" @click="${e}">Cancel</div>
      <div class="button accept" @click="${t}">Save</div>
    </div>
  `}function Ie(){function e(){if(!v.isSelfLeader){Se("You must be the leader of a race crew.");return}R("race_list")}function t(){R("race_editor"),$({type:"race_create"})}return g`
    <div class="screen">
      Racing
      <div class="button accept" @click="${e}">Start Race</div>
      <div class="button secondary" @click="${t}">Create Track</div>
    </div>
  `}function ze(){return g`
    <div class="screen">
      Settings
      <div
        class="button secondary"
        @click="${()=>R("change_name")}"
      >
        Change Name
      </div>
    </div>
  `}function Ne(){return g`
      <div class="screen" id="choose_name">
        Enter your name:
        <input type="text" autocomplete=off id="name_input" @keydown="${e=>{e.key==="Enter"&&e.target.value&&(w("change_name"),$({type:"change_name",name:e.target.value}))}}" placeholder="sam"></input>
      </div>
      `}let Me=0;const b=M({money:0,diffs:[],moneyAnimated:0,moneyWas:0});re(()=>{if(b.money!==b.moneyWas){const e=b.money-b.moneyWas;b.moneyWas=b.money,b.diffs.push({id:Me++,amt:e}),setTimeout(()=>{b.diffs.shift()},3e3)}});let q=null;re(()=>{if(b.money,typeof q=="number")return;const e=()=>{if(b.money===b.moneyAnimated){q=null;return}const t=Math.abs(b.money-b.moneyAnimated),n=Math.max(Math.floor(t/30),1);if(t<=1){b.moneyAnimated=b.money,q=null;return}b.money>b.moneyAnimated?b.moneyAnimated+=n:b.moneyAnimated-=n,q=setTimeout(e,10)};q=setTimeout(e,10)});function Be(){return g`<div id="money" class="no-mouse">
    ${()=>g`<div>$${()=>b.moneyAnimated}</div>`.key("money")}
    ${()=>b.diffs.map(({amt:e,id:t})=>g`<div
          id="${`diff-${t}`}"
          class="diff ${e<0?"negative":""}"
        >
          ${e<0?"-":"+"}$${Math.abs(e)}
        </div>`.key(`diff-${t}`))}
  </div>`}const E=M({showing:!1,stats:[]});function je(e){E.stats.push(e),E.stats.sort((t,n)=>t.place-n.place)}function Ve(){return g`${()=>E.showing?g`<div id="summary">
          <div class="flex-row space-between">
            <h1>Race Summary</h1>
            <div>
              <div
                class="button padded"
                @click="${()=>E.showing=!1}"
              >
                Close
              </div>
            </div>
          </div>
          ${()=>E.stats.map(e=>g`<div class="statistic">
                  ${e.place+1}.
                  ${e.who.replaceAll("<","&lt").replaceAll(">","&gt")||"&lt;salmoner&gt;"}
                  <div>${e.duration.toFixed(3)}s</div>
                </div>`)}
        </div>`:""}`}const F=M({shopVisible:!1});class ce{constructor(){V(this,"customizables");V(this,"unlocks");document.getElementById("app").addEventListener("mousemove",n=>{n.stopPropagation()}),document.getElementById("app").addEventListener("mousedown",n=>{n.stopPropagation()}),document.getElementById("app").addEventListener("mouseup",n=>{n.stopPropagation()}),document.getElementById("app").addEventListener("keydown",n=>{n.stopPropagation()}),document.getElementById("app").addEventListener("keyup",n=>{n.stopPropagation()}),g`
      <div>
        ${Be()}
        <div class="${()=>F.shopVisible?"":"hidden"}" id="shop">
          <h1>Sal Mon's Customs</h1>
          <h2>Hat</h2>
          <div class="spacer">
            <p id="hat-name">Cat</p>
            <p id="hat-cost">Cost: 50</p>
          </div>
          <div class="w-75">
            <div class="flex-row">
              <div class="button" id="hat-prev">Prev</div>
              <div class="button" id="hat-next">Next</div>
            </div>
            <div class="button" id="hat-equip">Equip</div>
          </div>
          <h2>Bike</h2>
          <div class="spacer">
            <p id="bike-name">Cat</p>
            <p id="bike-cost">Cost: 50</p>
          </div>
          <div class="w-75">
            <div class="flex-row">
              <div class="button" id="bike-prev">Prev</div>
              <div class="button" id="bike-next">Next</div>
            </div>
            <div class="button" id="bike-equip">Equip</div>
          </div>
        </div>
        ${Le()} ${Ve()}
      </div>
    `(document.getElementById("app")),this.customizables={hat:{items:[],index:0,equipped:0},bike:{items:[],index:0,equipped:0}},this.unlocks={hats:[],bikes:[]},document.querySelector("#hat-next").addEventListener("click",()=>{this.next_custom("hat")}),document.querySelector("#hat-prev").addEventListener("click",()=>{this.prev_custom("hat")}),document.querySelector("#bike-next").addEventListener("click",()=>{this.next_custom("bike")}),document.querySelector("#bike-prev").addEventListener("click",()=>{this.prev_custom("bike")}),document.querySelector("#bike-equip").addEventListener("click",()=>{const n="bike",s=this.customizables[n].index;this.customizables[n].equipped=s,$({type:"equip_and_buy",kind:n,index:s})}),document.querySelector("#hat-equip").addEventListener("click",()=>{const n="hat";this.customizables[n].equipped=this.customizables[n].index,$({type:"equip_and_buy",kind:n,index:this.customizables[n].index})})}prev_custom(t){const{length:n}=this.customizables[t].items;let{index:s}=this.customizables[t];s-=1,s<0&&(s=n-1),this.customizables[t].index=s,this.render_custom(t,s)}next_custom(t){const{length:n}=this.customizables[t].items;let{index:s}=this.customizables[t];s+=1,s>=n&&(s=0),this.customizables[t].index=s,this.render_custom(t,s)}render_custom(t,n,s=!0){if(n<0||n>=this.customizables[t].items.length){console.error(`early access of ${t} at ${n}`);return}const{name:o,cost:r}=this.customizables[t].items[n]||{name:"None",cost:0},i=this.unlocks[`${t}s`].includes(n);document.getElementById(`${t}-name`).innerHTML=o,r===0?document.getElementById(`${t}-cost`).innerHTML="Free!":document.getElementById(`${t}-cost`).innerHTML=`Cost: $${r}`,document.getElementById(`${t}-equip`).innerHTML=`${r===0||i?"Equip":"Buy"}`,s&&$({type:"preview_cosmetic",kind:t,index:n})}receive(t){switch(t.type){case"sync_money":b.money=t.amount;break;case"phone_show_invite":v.inviter=t.from,x("invite");break;case"unlocks":this.unlocks=t,this.render_custom("hat",this.customizables.hat.index,!1),this.render_custom("bike",this.customizables.bike.index,!1);break;case"customization_info":this.customizables.hat.items=t.hat_names,this.customizables.bike.items=t.bike_names;break;case"show_shop":t.visible?(this.render_custom("hat",this.customizables.hat.index,!1),this.render_custom("bike",this.customizables.bike.index,!1),F.shopVisible=!0):(F.shopVisible=!1,this.customizables.hat.index=this.customizables.hat.equipped,this.customizables.bike.index=this.customizables.bike.equipped);break;case"phone_change_name":x("change_name");break;case"phone_new_job":this.quest();break;case"phone_accept_invite":case"phone_reject_invite":w("invite");break;case"phone_interact_key":xe(t.mouse);break;case"phone_dismiss_notification":w("job");break;case"sync_team_leader":v.teamLeader=t.name,v.isSelfLeader=t.is_self;break;case"show_race_summary":E.showing=!0;break;case"update_race_summary":je(t.statistic);break;case"clear_race_summary":E.stats=[],E.showing=!1;break;case"exit_race_circle":w("race_circle");break;case"enter_race_circle":x("race_circle");break;default:console.error("Unexpected message received!",t)}}quest(){const t=["Can you take my books back to the library?","Bring me my food now!!!!","Please pick up my dry cleaning","I need 3 gerbils ASAP. No questions please","Can you deliver my groceries? I need tomato","I AM OUT OF TOILET PAPER GO FAST PLEASE","i want spaghetti","HUNGRY!!!!!!","bring me some flowers.","please do not look in this bag. just deliver","i would like 1 newspaper please","its me, pgorley","please serve these court summons for me","i ran out of coffee creamer. can you bring me some butter?","i need 37 cans of soup. no time to explain","can you deliver sushi","deliver this mail for me","can you take this trash away","i need a new kidney","PLEASE DELIVER MY TELEGRAM STOP DONT STOP STOP","find my pet turtle","let's go bowling cousin","listen, you just drive. to point B. simple.","2 Number 9's, a number 9 large, a number 6 with extra dip, 2 number 45's (one with cheese) and a large soda"],n=t[Math.floor(Math.random()*t.length)];v.questText=n,x("job")}}let W;window.bridge_init=()=>{W=new ce};window.bridge_send=function(){return(W||(console.warn("Bridge accessed before init!"),0))&&ce.prototype.receive.apply(W,arguments)};export{g as h,M as r};
