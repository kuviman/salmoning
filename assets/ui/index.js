var ce=Object.defineProperty;var ae=(e,t,n)=>t in e?ce(e,t,{enumerable:!0,configurable:!0,writable:!0,value:n}):e[t]=n;var B=(e,t,n)=>(ae(e,typeof t!="symbol"?t+"":t,n),n);(function(){const t=document.createElement("link").relList;if(t&&t.supports&&t.supports("modulepreload"))return;for(const o of document.querySelectorAll('link[rel="modulepreload"]'))i(o);new MutationObserver(o=>{for(const r of o)if(r.type==="childList")for(const s of r.addedNodes)s.tagName==="LINK"&&s.rel==="modulepreload"&&i(s)}).observe(document,{childList:!0,subtree:!0});function n(o){const r={};return o.integrity&&(r.integrity=o.integrity),o.referrerPolicy&&(r.referrerPolicy=o.referrerPolicy),o.crossOrigin==="use-credentials"?r.credentials="include":o.crossOrigin==="anonymous"?r.credentials="omit":r.credentials="same-origin",r}function i(o){if(o.ep)return;o.ep=!0;const r=n(o);fetch(o.href,r)}})();const le="modulepreload",ue=function(e){return"/"+e},K={},J=function(t,n,i){let o=Promise.resolve();if(n&&n.length>0){const r=document.getElementsByTagName("link"),s=document.querySelector("meta[property=csp-nonce]"),a=(s==null?void 0:s.nonce)||(s==null?void 0:s.getAttribute("nonce"));o=Promise.all(n.map(u=>{if(u=ue(u),u in K)return;K[u]=!0;const f=u.endsWith(".css"),y=f?'[rel="stylesheet"]':"";if(!!i)for(let p=r.length-1;p>=0;p--){const c=r[p];if(c.href===u&&(!f||c.rel==="stylesheet"))return}else if(document.querySelector(`link[href="${u}"]${y}`))return;const d=document.createElement("link");if(d.rel=f?"stylesheet":le,f||(d.as="script",d.crossOrigin=""),d.href=u,a&&d.setAttribute("nonce",a),document.head.appendChild(d),f)return new Promise((p,c)=>{d.addEventListener("load",p),d.addEventListener("error",()=>c(new Error(`Unable to preload CSS for ${u}`)))})}))}return o.then(()=>t()).catch(r=>{const s=new Event("vite:preloadError",{cancelable:!0});if(s.payload=r,window.dispatchEvent(s),!s.defaultPrevented)throw r})};let Z=(...e)=>console.log("SENDING BRIDGE REPLY",...e);(async()=>{try{const t="../../salmoning.js";Z=(await J(()=>import(t),[])).bridge_reply}catch{console.warn("activating ui debug"),(await J(()=>import("./debug-CThJQoTo.js"),[])).activate()}})();function E(){return Z(...arguments)}const P=new Set,Q=new Set;function C(e){return typeof e=="function"&&!!e.isT}function R(e){return typeof e=="object"&&e!==null&&"$on"in e&&typeof e.$on=="function"}function de(e){return"$on"in e}function me(e){return(t,n)=>{function i(){const o=Array.from(P);P.clear();const r=Array.from(Q);Q.clear(),o.forEach(s=>s(t,n)),r.forEach(s=>s()),P.size&&queueMicrotask(i)}P.size||queueMicrotask(i),P.add(e)}}const j={};function pe(e,t){const n=performance.now(),i=typeof t=="function";e=i?`${e} (ms)`:`${e} (calls)`;const o=i?t():t,r=i?performance.now()-n:t;return j[e]?j[e].push(r):j[e]=[r],o}const S=new Map;function q(e,t={}){if(R(e)||typeof e!="object")return e;const n=t.o||new Map,i=t.op||new Map,o=Array.isArray(e),r=[],s=o?[]:Object.create(e,{});for(const c in e){const l=e[c];typeof l=="object"&&l!==null?(s[c]=R(l)?l:q(l),r.push(c)):s[c]=l}const a=c=>(l,h)=>{let m=n.get(l),g=i.get(h);m||(m=new Set,n.set(l,m)),g||(g=new Set,i.set(h,g)),m[c](h),g[c](l)},u=a("add"),f=a("delete"),y=(c,l,h)=>{n.has(c)&&n.get(c).forEach(m=>m(l,h))},d={$on:u,$off:f,_em:y,_st:()=>({o:n,op:i,r:s,p:p._p}),_p:void 0},p=new Proxy(s,{has(c,l){return l in d||l in c},get(...c){const[,l]=c;if(Reflect.has(d,l))return Reflect.get(d,l);const h=Reflect.get(...c);return fe(p,l),o&&l in Array.prototype?he(l,s,p,h):h},set(...c){const[l,h,m]=c,g=Reflect.get(l,h);if(Reflect.has(d,h))return Reflect.set(d,h,m);if(m&&R(g)){const L=g,O=L._st(),N=R(m)?ve(m,L):q(m,O);return Reflect.set(l,h,N),y(h,N),O.o.forEach((Me,M)=>{const G=Reflect.get(g,M),U=Reflect.get(N,M);G!==U&&L._em(M,U,G)}),!0}const _=Reflect.set(...c);return _&&(g!==m&&y(h,m,g),p._p&&p._p[1]._em(...p._p)),_}});return t.p&&(p._p=t.p),r.map(c=>{p[c]._p=[c,p]}),p}function fe(e,t){S.forEach(n=>{let i=n.get(e);i||(i=new Set,n.set(e,i)),i.add(t)})}function he(e,t,n,i){const o=(...r)=>{const s=Array.prototype[e].call(t,...r);if(t.forEach((a,u)=>n._em(String(u),a)),n._p){const[a,u]=n._p;u._em(a,n)}return s};switch(e){case"shift":case"pop":case"sort":case"reverse":case"copyWithin":return o;case"unshift":case"push":case"fill":return(...r)=>o(...r.map(s=>q(s)));case"splice":return function(r,s,...a){return arguments.length===1?o(r):o(r,s,...a.map(u=>q(u)))};default:return i}}function ve(e,t){const n=t._st();return n.o&&n.o.forEach((i,o)=>{i.forEach(r=>{e.$on(o,r)})}),n.p&&(e._p=n.p),e}function F(e,t){const n=Symbol();S.has(n)||S.set(n,new Map);let i=new Map;const o=me(r);function r(){S.set(n,new Map);const s=e(),a=S.get(n);return S.delete(n),i.forEach((u,f)=>{const y=a.get(f);y&&y.forEach(w=>u.delete(w)),u.forEach(w=>f.$off(w,o))}),a.forEach((u,f)=>{u.forEach(y=>f.$on(y,o))}),i=a,t?t(s):s}return de(e)&&e.$on(r),r()}const I=new WeakMap,X={},ee="➳❍",te="❍⇚",ne=`<!--${ee}-->`,ye=`<!--${te}-->`;function ie(e,...t){const n=[];let i="";const o=(a,u)=>{if(typeof a=="function"){let f=()=>{};return n.push(Object.assign((...y)=>a(...y),{e:a,$on:y=>{f=y},_up:y=>{a=y,f()}})),u+ne}return Array.isArray(a)?a.reduce((f,y)=>o(y,f),u):u+a},r=()=>(i||(!t.length&&e.length===1&&e[0]===""?i="<!---->":i=e.reduce(function(u,f,y){return u+=f,t[y]!==void 0?o(t[y],u):u},"")),i),s=a=>{const u=se(r()),f=H(u,{i:0,e:n});return a?f(a):f()};return s.isT=!0,s._k=0,s._h=()=>[r(),n,s._k],s.key=a=>(s._k=a,s),s}function H(e,t){let n,i=0;const o=e.childNodes;for(;n=o.item(i++);){if(n.nodeType===8&&n.nodeValue===ee){ke(n,t);continue}n instanceof Element&&be(n,t),n.hasChildNodes()&&H(n,t),n instanceof HTMLOptionElement&&(n.selected=n.defaultSelected)}return r=>r?(r.appendChild(e),r):e}function be(e,t){var n;const i=[];let o=0,r;for(;r=e.attributes[o++];){if(t.i>=t.e.length)return;if(r.value!==ne)continue;let s=r.name;const a=t.e[t.i++];if(s.charAt(0)==="@"){const u=s.substring(1);e.addEventListener(u,a),I.has(e)||I.set(e,new Map),(n=I.get(e))===null||n===void 0||n.set(u,a),i.push(s)}else{const u=s==="value"&&"value"in e||s==="checked"||s.startsWith(".")&&(s=s.substring(1));F(a,f=>{u&&(e[s]=f,e.getAttribute(s)!=f&&(f=!1)),f!==!1?e.setAttribute(s,f):(e.removeAttribute(s),o--)})}}i.forEach(s=>e.removeAttribute(s))}function ge(e){e.forEach(_e)}function _e(e){var t;e.remove(),(t=I.get(e))===null||t===void 0||t.forEach((n,i)=>e.removeEventListener(i,n))}function ke(e,t){var n;const i=t.e[t.i++];let o;if(i&&C(i.e))o=W().add(i.e)();else{let r;o=(r=F(i,s=>Ee(s,r)))()}(n=e.parentNode)===null||n===void 0||n.replaceChild(o,e)}function Ee(e,t){const n=typeof t=="function",i=n?t:W();return Array.isArray(e)?e.forEach(o=>pe("partialAdd",()=>i.add(o))):i.add(e),n&&i._up(),i}function se(e){var t;const i=((t=X[e])!==null&&t!==void 0?t:(()=>{const o=document.createElement("template");return o.innerHTML=e,X[e]=o})()).content.cloneNode(!0);return i.normalize(),i}function W(e=Symbol()){let t="",n={i:0,e:[]},i=[],o=[];const r=new Map,s=[],a=()=>{let d;if(i.length||f(),i.length===1&&!C(i[0].tpl)){const p=i[0];p.dom.length?p.dom[0].nodeValue=p.tpl:p.dom.push(document.createTextNode(p.tpl)),d=p.dom[0]}else d=y(H(se(t),n)());return u(),d};a.ch=()=>o,a.l=0,a.add=d=>{if(!d&&d!==0)return a;let p=[],c,l="";C(d)&&([l,p,c]=d._h()),t+=l,t+=ye;const h=c&&r.get(c),m=h||{html:l,exp:p,dom:[],tpl:d,key:c};return i.push(m),c&&(h?h.exp.forEach((g,_)=>g._up(p[_].e)):r.set(c,m)),n.e.push(...p),a.l++,a},a._up=()=>{const d=W(e);let p=0,c=o[0].dom[0];i.length||f(document.createComment(""));const l=()=>{if(!d.l)return;const m=d(),g=m.lastChild;c[p?"after":"before"](m),w(d,i,p),c=g};i.forEach((m,g)=>{const _=o[g];m.key&&m.dom.length?(l(),(!_||_.dom!==m.dom)&&c[g?"after":"before"](...m.dom),c=m.dom[m.dom.length-1]):_&&m.html===_.html&&!_.key?(l(),_.exp.forEach((L,O)=>L._up(m.exp[O].e)),m.exp=_.exp,m.dom=_.dom,c=m.dom[m.dom.length-1],$e(m)&&c instanceof Text&&(c.nodeValue=m.tpl)):(_&&m.html!==_.html&&!_.key&&s.push(..._.dom),d.l||(p=g),d.add(m.tpl))}),l();let h=c==null?void 0:c.nextSibling;for(;h&&e in h;)s.push(h),h=h.nextSibling;ge(s),u()};const u=()=>{s.length=0,t="",a.l=0,n={i:0,e:[]},o=[...i],i=[]},f=d=>{t="<!---->",i.push({html:t,exp:[],dom:d?[d]:[],tpl:ie`${t}`,key:0})},y=d=>{let p=0;const c=[];return d.childNodes.forEach(l=>{if(l.nodeType===8&&l.data===te){p++,c.push(l);return}Object.defineProperty(l,e,{value:e}),i[p].dom.push(l)}),c.forEach(l=>l.remove()),d},w=(d,p,c)=>{d.ch().forEach((l,h)=>{p[c+h].dom=l.dom})};return a}function $e(e){return e.dom.length===1&&!C(e.tpl)}const k=ie,oe=F;function Y(e){return q(e)}const v=Y({tasks:[],questText:"spaghetti",inviter:"kuviman",teamLeader:null,isSelfLeader:!1,alertText:""}),z={base:{priority:0,template:qe,closeOnInteract:!0},settings:{priority:1,template:Ie,closeOnInteract:!0},races:{priority:1,template:Re,closeOnInteract:!0},race_list:{priority:2,template:Te,closeOnInteract:!0},race_editor:{priority:3,template:Oe},change_name:{priority:2,template:Ce},job:{priority:5,template:Pe,closeOnInteract:!0},invite:{priority:10,template:Ae},alert:{priority:20,template:Le,closeOnInteract:!0}};function we(e){v.alertText=e,x("alert")}function T(e){v.tasks.shift(),x(e)}function $(e){v.tasks=v.tasks.filter(t=>t!==e)}function Se(e){if(v.tasks.length===0){e||x("base");return}if(z[v.tasks[0]].closeOnInteract){v.tasks.shift();return}}function x(e){v.tasks.includes(e)||v.tasks.push(e),v.tasks.sort((t,n)=>z[n].priority-z[t].priority)}function xe(){return k`
    <div
      id="phone"
      class="${()=>v.tasks.length?"":"phone_down"}"
    >
      ${()=>v.tasks.length?z[v.tasks[0]].template():void 0}
    </div>
  `}function Le(){function e(){$("alert")}return k`
    <div class="screen">
      <p>${()=>v.alertText}</p>
      <div class="flex-row">
        <div class="button accept" @click="${e}">Ok</div>
      </div>
    </div>
  `}function Pe(){function e(){$("job")}return k`
    <div class="screen">
      <p>Someone is summoning you!</p>
      <p>"${()=>v.questText}"</p>
      <div class="flex-row">
        <div class="button accept" @click="${e}">Nice</div>
        <div class="button decline" @click="${e}">OK</div>
      </div>
    </div>
  `}function Ae(){function e(t){$("invite"),E({type:t})}return k`
    <div class="screen" id="invite">
      <p>New Message</p>
      <p>"yo, wanna join my team?"</p>
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
  `}function qe(){function e(){E({type:"leave_team"}),$("base")}let t="./icons";return t="assets/icons",k`
    <div class="screen">
      ${()=>v.teamLeader&&k`${()=>v.isSelfLeader?"You are a leader":`Leader: ${v.teamLeader}`}
          <div class="button decline" @click="${e}">Leave Team</div>`}
      <div class="flex-row">
        <div
          class="button secondary"
          @click="${()=>T("settings")}"
        >
          Settings
        </div>
        <div class="button" @click="${()=>T("races")}">
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
  `}function Te(){let e=JSON.parse(localStorage.getItem("./races")||'{ "races": {} }');function t(n){return function(){E({type:"race_start",name:n}),$("race_list")}}return k`<div class="screen">
    Pick a Race:
    <div class="race-list">
      ${Object.entries(e.races).map(([n,i])=>k`<div class="race-option" @click="${t(n)}">
          ${n}<br />
          ${i.track.length} checkpoints
        </div>`)}
    </div>
  </div>`}function Oe(){function e(){E({type:"race_edit_cancel"}),$("race_editor")}function t(){const n=document.getElementById("race-name").value;E({type:"race_edit_submit",name:n}),$("race_editor")}return k`
    <div class="screen">
      Race Editor
      <input type="text" placeholder="name" id="race-name" />
      <div class="button decline" @click="${e}">Cancel</div>
      <div class="button accept" @click="${t}">Save</div>
    </div>
  `}function Re(){function e(){if(!v.isSelfLeader){we("You must be the leader of a race club.");return}T("race_list")}function t(){T("race_editor"),E({type:"race_create"})}return k`
    <div class="screen">
      Racing
      <div class="button accept" @click="${e}">Start Race</div>
      <div class="button secondary" @click="${t}">Create Track</div>
    </div>
  `}function Ie(){return k`
    <div class="screen">
      Settings
      <div
        class="button secondary"
        @click="${()=>T("change_name")}"
      >
        Change Name
      </div>
    </div>
  `}function Ce(){return k`
      <div class="screen" id="choose_name">
        Enter your name:
        <input type="text" autocomplete=off id="name_input" @keydown="${e=>{e.key==="Enter"&&($("change_name"),E({type:"change_name",name:e.target.value}))}}" placeholder="sam"></input>
      </div>
      `}let ze=0;const b=Y({money:0,diffs:[],moneyAnimated:0,moneyWas:0});oe(()=>{if(b.money!==b.moneyWas){const e=b.money-b.moneyWas;b.moneyWas=b.money,b.diffs.push({id:ze++,amt:e}),setTimeout(()=>{b.diffs.shift()},3e3)}});let A=null;oe(()=>{if(b.money,typeof A=="number")return;const e=()=>{if(b.money===b.moneyAnimated){A=null;return}const t=Math.abs(b.money-b.moneyAnimated),n=Math.max(Math.floor(t/30),1);if(t<=1){b.moneyAnimated=b.money,A=null;return}b.money>b.moneyAnimated?b.moneyAnimated+=n:b.moneyAnimated-=n,A=setTimeout(e,10)};A=setTimeout(e,10)});function Ne(){return k`<div id="money" class="no-mouse">
    ${()=>k`<div>$${()=>b.moneyAnimated}</div>`.key("money")}
    ${()=>b.diffs.map(({amt:e,id:t})=>k`<div
          id="${`diff-${t}`}"
          class="diff ${e<0?"negative":""}"
        >
          ${e<0?"-":"+"}$${Math.abs(e)}
        </div>`.key(`diff-${t}`))}
  </div>`}const V=Y({shopVisible:!1});class re{constructor(){B(this,"customizables");B(this,"unlocks");document.getElementById("app").addEventListener("mousemove",n=>{n.stopPropagation()}),document.getElementById("app").addEventListener("mousedown",n=>{n.stopPropagation()}),document.getElementById("app").addEventListener("mouseup",n=>{n.stopPropagation()}),document.getElementById("app").addEventListener("keydown",n=>{n.stopPropagation()}),document.getElementById("app").addEventListener("keyup",n=>{n.stopPropagation()}),k`
      <div>
        ${Ne()}
        <div class="${()=>V.shopVisible?"":"hidden"}" id="shop">
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
        ${xe()}
      </div>
    `(document.getElementById("app")),this.customizables={hat:{items:[],index:0,equipped:0},bike:{items:[],index:0,equipped:0}},this.unlocks={hats:[],bikes:[]},document.querySelector("#hat-next").addEventListener("click",()=>{this.next_custom("hat")}),document.querySelector("#hat-prev").addEventListener("click",()=>{this.prev_custom("hat")}),document.querySelector("#bike-next").addEventListener("click",()=>{this.next_custom("bike")}),document.querySelector("#bike-prev").addEventListener("click",()=>{this.prev_custom("bike")}),document.querySelector("#bike-equip").addEventListener("click",()=>{const n="bike",i=this.customizables[n].index;this.customizables[n].equipped=i,E({type:"equip_and_buy",kind:n,index:i})}),document.querySelector("#hat-equip").addEventListener("click",()=>{const n="hat";this.customizables[n].equipped=this.customizables[n].index,E({type:"equip_and_buy",kind:n,index:this.customizables[n].index})})}prev_custom(t){const{length:n}=this.customizables[t].items;let{index:i}=this.customizables[t];i-=1,i<0&&(i=n-1),this.customizables[t].index=i,this.render_custom(t,i)}next_custom(t){const{length:n}=this.customizables[t].items;let{index:i}=this.customizables[t];i+=1,i>=n&&(i=0),this.customizables[t].index=i,this.render_custom(t,i)}render_custom(t,n,i=!0){if(n<0||n>=this.customizables[t].items.length){console.error(`early access of ${t} at ${n}`);return}const{name:o,cost:r}=this.customizables[t].items[n]||{name:"None",cost:0},s=this.unlocks[`${t}s`].includes(n);document.getElementById(`${t}-name`).innerHTML=o,r===0?document.getElementById(`${t}-cost`).innerHTML="Free!":document.getElementById(`${t}-cost`).innerHTML=`Cost: $${r}`,document.getElementById(`${t}-equip`).innerHTML=`${r===0||s?"Equip":"Buy"}`,i&&E({type:"preview_cosmetic",kind:t,index:n})}receive(t){switch(t.type){case"sync_money":b.money=t.amount;break;case"phone_show_invite":v.inviter=t.from,x("invite");break;case"unlocks":this.unlocks=t,this.render_custom("hat",this.customizables.hat.index,!1),this.render_custom("bike",this.customizables.bike.index,!1);break;case"customization_info":this.customizables.hat.items=t.hat_names,this.customizables.bike.items=t.bike_names;break;case"show_shop":t.visible?(this.render_custom("hat",this.customizables.hat.index,!1),this.render_custom("bike",this.customizables.bike.index,!1),V.shopVisible=!0):(V.shopVisible=!1,this.customizables.hat.index=this.customizables.hat.equipped,this.customizables.bike.index=this.customizables.bike.equipped);break;case"phone_change_name":x("change_name");break;case"phone_new_job":this.quest();break;case"phone_accept_invite":case"phone_reject_invite":$("invite");break;case"phone_interact_key":Se(t.mouse);break;case"phone_dismiss_notification":$("job");break;case"sync_team_leader":v.teamLeader=t.name,v.isSelfLeader=t.is_self;break;default:console.error("Unexpected message received!",t)}}quest(){const t=["Can you take my books back to the library?","Bring me my food now!!!!","Please pick up my dry cleaning","I need 3 gerbils ASAP. No questions please","Can you deliver my groceries? I need tomato","I AM OUT OF TOILET PAPER GO FAST PLEASE","i want spaghetti","HUNGRY!!!!!!","bring me some flowers.","please do not look in this bag. just deliver","i would like 1 newspaper please","its me, pgorley","please serve these court summons for me","i ran out of coffee creamer. can you bring me some butter?","i need 37 cans of soup. no time to explain","can you deliver sushi","deliver this mail for me","can you take this trash away","i need a new kidney","PLEASE DELIVER MY TELEGRAM STOP DONT STOP STOP","find my pet turtle","let's go bowling cousin","listen, you just drive. to point B. simple.","2 Number 9's, a number 9 large, a number 6 with extra dip, 2 number 45's (one with cheese) and a large soda"],n=t[Math.floor(Math.random()*t.length)];v.questText=n,x("job")}}let D;window.bridge_init=()=>{D=new re};window.bridge_send=function(){return(D||(console.warn("Bridge accessed before init!"),0))&&re.prototype.receive.apply(D,arguments)};export{k as h,Y as r};
