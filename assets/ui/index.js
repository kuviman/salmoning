var ce=Object.defineProperty;var ae=(t,e,n)=>e in t?ce(t,e,{enumerable:!0,configurable:!0,writable:!0,value:n}):t[e]=n;var I=(t,e,n)=>(ae(t,typeof e!="symbol"?e+"":e,n),n);(function(){const e=document.createElement("link").relList;if(e&&e.supports&&e.supports("modulepreload"))return;for(const o of document.querySelectorAll('link[rel="modulepreload"]'))s(o);new MutationObserver(o=>{for(const r of o)if(r.type==="childList")for(const i of r.addedNodes)i.tagName==="LINK"&&i.rel==="modulepreload"&&s(i)}).observe(document,{childList:!0,subtree:!0});function n(o){const r={};return o.integrity&&(r.integrity=o.integrity),o.referrerPolicy&&(r.referrerPolicy=o.referrerPolicy),o.crossOrigin==="use-credentials"?r.credentials="include":o.crossOrigin==="anonymous"?r.credentials="omit":r.credentials="same-origin",r}function s(o){if(o.ep)return;o.ep=!0;const r=n(o);fetch(o.href,r)}})();const le="modulepreload",ue=function(t){return"/"+t},Y={},K=function(e,n,s){let o=Promise.resolve();if(n&&n.length>0){const r=document.getElementsByTagName("link"),i=document.querySelector("meta[property=csp-nonce]"),a=(i==null?void 0:i.nonce)||(i==null?void 0:i.getAttribute("nonce"));o=Promise.all(n.map(u=>{if(u=ue(u),u in Y)return;Y[u]=!0;const f=u.endsWith(".css"),v=f?'[rel="stylesheet"]':"";if(!!s)for(let p=r.length-1;p>=0;p--){const c=r[p];if(c.href===u&&(!f||c.rel==="stylesheet"))return}else if(document.querySelector(`link[href="${u}"]${v}`))return;const d=document.createElement("link");if(d.rel=f?"stylesheet":le,f||(d.as="script",d.crossOrigin=""),d.href=u,a&&d.setAttribute("nonce",a),document.head.appendChild(d),f)return new Promise((p,c)=>{d.addEventListener("load",p),d.addEventListener("error",()=>c(new Error(`Unable to preload CSS for ${u}`)))})}))}return o.then(()=>e()).catch(r=>{const i=new Event("vite:preloadError",{cancelable:!0});if(i.payload=r,window.dispatchEvent(i),!i.defaultPrevented)throw r})};let X=(...t)=>console.log("SENDING BRIDGE REPLY",...t);(async()=>{try{const e="../../salmoning.js";X=(await K(()=>import(e),[])).bridge_reply}catch{console.warn("activating ui debug"),(await K(()=>import("./debug-DjfHceCd.js"),[])).activate()}})();function L(){return X(...arguments)}const x=new Set,Q=new Set;function M(t){return typeof t=="function"&&!!t.isT}function z(t){return typeof t=="object"&&t!==null&&"$on"in t&&typeof t.$on=="function"}function de(t){return"$on"in t}function me(t){return(e,n)=>{function s(){const o=Array.from(x);x.clear();const r=Array.from(Q);Q.clear(),o.forEach(i=>i(e,n)),r.forEach(i=>i()),x.size&&queueMicrotask(s)}x.size||queueMicrotask(s),x.add(t)}}const O={};function pe(t,e){const n=performance.now(),s=typeof e=="function";t=s?`${t} (ms)`:`${t} (calls)`;const o=s?e():e,r=s?performance.now()-n:e;return O[t]?O[t].push(r):O[t]=[r],o}const $=new Map;function q(t,e={}){if(z(t)||typeof t!="object")return t;const n=e.o||new Map,s=e.op||new Map,o=Array.isArray(t),r=[],i=o?[]:Object.create(t,{});for(const c in t){const l=t[c];typeof l=="object"&&l!==null?(i[c]=z(l)?l:q(l),r.push(c)):i[c]=l}const a=c=>(l,h)=>{let m=n.get(l),g=s.get(h);m||(m=new Set,n.set(l,m)),g||(g=new Set,s.set(h,g)),m[c](h),g[c](l)},u=a("add"),f=a("delete"),v=(c,l,h)=>{n.has(c)&&n.get(c).forEach(m=>m(l,h))},d={$on:u,$off:f,_em:v,_st:()=>({o:n,op:s,r:i,p:p._p}),_p:void 0},p=new Proxy(i,{has(c,l){return l in d||l in c},get(...c){const[,l]=c;if(Reflect.has(d,l))return Reflect.get(d,l);const h=Reflect.get(...c);return fe(p,l),o&&l in Array.prototype?he(l,i,p,h):h},set(...c){const[l,h,m]=c,g=Reflect.get(l,h);if(Reflect.has(d,h))return Reflect.set(d,h,m);if(m&&z(g)){const S=g,T=S._st(),N=z(m)?ve(m,S):q(m,T);return Reflect.set(l,h,N),v(h,N),T.o.forEach((Ce,R)=>{const G=Reflect.get(g,R),U=Reflect.get(N,R);G!==U&&S._em(R,U,G)}),!0}const _=Reflect.set(...c);return _&&(g!==m&&v(h,m,g),p._p&&p._p[1]._em(...p._p)),_}});return e.p&&(p._p=e.p),r.map(c=>{p[c]._p=[c,p]}),p}function fe(t,e){$.forEach(n=>{let s=n.get(t);s||(s=new Set,n.set(t,s)),s.add(e)})}function he(t,e,n,s){const o=(...r)=>{const i=Array.prototype[t].call(e,...r);if(e.forEach((a,u)=>n._em(String(u),a)),n._p){const[a,u]=n._p;u._em(a,n)}return i};switch(t){case"shift":case"pop":case"sort":case"reverse":case"copyWithin":return o;case"unshift":case"push":case"fill":return(...r)=>o(...r.map(i=>q(i)));case"splice":return function(r,i,...a){return arguments.length===1?o(r):o(r,i,...a.map(u=>q(u)))};default:return s}}function ve(t,e){const n=e._st();return n.o&&n.o.forEach((s,o)=>{s.forEach(r=>{t.$on(o,r)})}),n.p&&(t._p=n.p),t}function D(t,e){const n=Symbol();$.has(n)||$.set(n,new Map);let s=new Map;const o=me(r);function r(){$.set(n,new Map);const i=t(),a=$.get(n);return $.delete(n),s.forEach((u,f)=>{const v=a.get(f);v&&v.forEach(w=>u.delete(w)),u.forEach(w=>f.$off(w,o))}),a.forEach((u,f)=>{u.forEach(v=>f.$on(v,o))}),s=a,e?e(i):i}return de(t)&&t.$on(r),r()}const C=new WeakMap,J={},Z="➳❍",ee="❍⇚",te=`<!--${Z}-->`,ye=`<!--${ee}-->`;function ne(t,...e){const n=[];let s="";const o=(a,u)=>{if(typeof a=="function"){let f=()=>{};return n.push(Object.assign((...v)=>a(...v),{e:a,$on:v=>{f=v},_up:v=>{a=v,f()}})),u+te}return Array.isArray(a)?a.reduce((f,v)=>o(v,f),u):u+a},r=()=>(s||(!e.length&&t.length===1&&t[0]===""?s="<!---->":s=t.reduce(function(u,f,v){return u+=f,e[v]!==void 0?o(e[v],u):u},"")),s),i=a=>{const u=se(r()),f=F(u,{i:0,e:n});return a?f(a):f()};return i.isT=!0,i._k=0,i._h=()=>[r(),n,i._k],i.key=a=>(i._k=a,i),i}function F(t,e){let n,s=0;const o=t.childNodes;for(;n=o.item(s++);){if(n.nodeType===8&&n.nodeValue===Z){ke(n,e);continue}n instanceof Element&&be(n,e),n.hasChildNodes()&&F(n,e),n instanceof HTMLOptionElement&&(n.selected=n.defaultSelected)}return r=>r?(r.appendChild(t),r):t}function be(t,e){var n;const s=[];let o=0,r;for(;r=t.attributes[o++];){if(e.i>=e.e.length)return;if(r.value!==te)continue;let i=r.name;const a=e.e[e.i++];if(i.charAt(0)==="@"){const u=i.substring(1);t.addEventListener(u,a),C.has(t)||C.set(t,new Map),(n=C.get(t))===null||n===void 0||n.set(u,a),s.push(i)}else{const u=i==="value"&&"value"in t||i==="checked"||i.startsWith(".")&&(i=i.substring(1));D(a,f=>{u&&(t[i]=f,t.getAttribute(i)!=f&&(f=!1)),f!==!1?t.setAttribute(i,f):(t.removeAttribute(i),o--)})}}s.forEach(i=>t.removeAttribute(i))}function ge(t){t.forEach(_e)}function _e(t){var e;t.remove(),(e=C.get(t))===null||e===void 0||e.forEach((n,s)=>t.removeEventListener(s,n))}function ke(t,e){var n;const s=e.e[e.i++];let o;if(s&&M(s.e))o=H().add(s.e)();else{let r;o=(r=D(s,i=>Ee(i,r)))()}(n=t.parentNode)===null||n===void 0||n.replaceChild(o,t)}function Ee(t,e){const n=typeof e=="function",s=n?e:H();return Array.isArray(t)?t.forEach(o=>pe("partialAdd",()=>s.add(o))):s.add(t),n&&s._up(),s}function se(t){var e;const s=((e=J[t])!==null&&e!==void 0?e:(()=>{const o=document.createElement("template");return o.innerHTML=t,J[t]=o})()).content.cloneNode(!0);return s.normalize(),s}function H(t=Symbol()){let e="",n={i:0,e:[]},s=[],o=[];const r=new Map,i=[],a=()=>{let d;if(s.length||f(),s.length===1&&!M(s[0].tpl)){const p=s[0];p.dom.length?p.dom[0].nodeValue=p.tpl:p.dom.push(document.createTextNode(p.tpl)),d=p.dom[0]}else d=v(F(se(e),n)());return u(),d};a.ch=()=>o,a.l=0,a.add=d=>{if(!d&&d!==0)return a;let p=[],c,l="";M(d)&&([l,p,c]=d._h()),e+=l,e+=ye;const h=c&&r.get(c),m=h||{html:l,exp:p,dom:[],tpl:d,key:c};return s.push(m),c&&(h?h.exp.forEach((g,_)=>g._up(p[_].e)):r.set(c,m)),n.e.push(...p),a.l++,a},a._up=()=>{const d=H(t);let p=0,c=o[0].dom[0];s.length||f(document.createComment(""));const l=()=>{if(!d.l)return;const m=d(),g=m.lastChild;c[p?"after":"before"](m),w(d,s,p),c=g};s.forEach((m,g)=>{const _=o[g];m.key&&m.dom.length?(l(),(!_||_.dom!==m.dom)&&c[g?"after":"before"](...m.dom),c=m.dom[m.dom.length-1]):_&&m.html===_.html&&!_.key?(l(),_.exp.forEach((S,T)=>S._up(m.exp[T].e)),m.exp=_.exp,m.dom=_.dom,c=m.dom[m.dom.length-1],we(m)&&c instanceof Text&&(c.nodeValue=m.tpl)):(_&&m.html!==_.html&&!_.key&&i.push(..._.dom),d.l||(p=g),d.add(m.tpl))}),l();let h=c==null?void 0:c.nextSibling;for(;h&&t in h;)i.push(h),h=h.nextSibling;ge(i),u()};const u=()=>{i.length=0,e="",a.l=0,n={i:0,e:[]},o=[...s],s=[]},f=d=>{e="<!---->",s.push({html:e,exp:[],dom:d?[d]:[],tpl:ne`${e}`,key:0})},v=d=>{let p=0;const c=[];return d.childNodes.forEach(l=>{if(l.nodeType===8&&l.data===ee){p++,c.push(l);return}Object.defineProperty(l,t,{value:t}),s[p].dom.push(l)}),c.forEach(l=>l.remove()),d},w=(d,p,c)=>{d.ch().forEach((l,h)=>{p[c+h].dom=l.dom})};return a}function we(t){return t.dom.length===1&&!M(t.tpl)}const k=ne,ie=D;function W(t){return q(t)}const b=W({tasks:[],questText:"spaghetti",inviter:"kuviman",teamLeader:null}),j={base:{priority:0,template:Ae},settings:{priority:1,template:Pe},change_name:{priority:2,template:qe},job:{priority:5,template:Se},invite:{priority:10,template:xe}};function oe(t){b.tasks.shift(),P(t)}function E(t){b.tasks=b.tasks.filter(e=>e!==t)}function $e(){if(b.tasks.length===0){P("base");return}if(b.tasks[0]==="job"){E("job");return}if(b.tasks[0]==="settings"){E("settings");return}if(b.tasks[0]==="base"){E("base");return}}function P(t){b.tasks.includes(t)||b.tasks.push(t),b.tasks.sort((e,n)=>j[n].priority-j[e].priority)}function Le(){return k`
    <div
      id="phone"
      class="${()=>b.tasks.length?"":"phone_down"}"
    >
      ${()=>b.tasks.length?j[b.tasks[0]].template():void 0}
    </div>
  `}function Se(){function t(){E("job")}return k`
    <div class="screen">
      <p>Someone is summoning you!</p>
      <p>"${()=>b.questText}"</p>
      <div class="flex-row">
        <div class="button accept" @click="${t}">Nice</div>
        <div class="button decline" @click="${t}">OK</div>
      </div>
    </div>
  `}function xe(){function t(e){E("invite"),L({type:e})}return k`
    <div class="screen" id="invite">
      <p>New Message</p>
      <p>"yo, wanna join my team?"</p>
      <p id="inviter">- ${()=>b.inviter}</p>
      <div class="flex-row">
        <div class="button accept" @click="${()=>t("accept_invite")}">
          (Y)es
        </div>
        <div class="button decline" @click="${()=>t("decline_invite")}">
          (N)o
        </div>
      </div>
    </div>
  `}function Ae(){function t(){L({type:"leave_team"}),E("base")}let e="./icons";return e="assets/icons",k`
    <div class="screen">
      ${()=>b.teamLeader&&k`Leader: ${()=>b.teamLeader}
          <div class="button decline" @click="${t}">Leave Team</div>`}
      <div
        class="button secondary"
        @click="${()=>oe("settings")}"
      >
        Settings
      </div>
      <div class="phone-grid">
        <img src="${e}/msg.png" />
        <img src="${e}/salmazon.png" />
        <img src="${e}/firefish.png" />
        <img src="${e}/fishmail.png" />
        <img src="${e}/phone.png" />
        <img src="${e}/fishmaps.png" />
        <img src="${e}/fishbook.png" />
        <img src="${e}/fishdonalds.png" />
        <img src="${e}/fishcord.png" />
        <img src="${e}/samneats.png" />
        <img src="${e}/fwich.png" />
      </div>
    </div>
  `}function Pe(){return k`
    <div class="screen">
      Settings
      <div
        class="button secondary"
        @click="${()=>oe("change_name")}"
      >
        Change Name
      </div>
    </div>
  `}function qe(){return k`
      <div class="screen" id="choose_name">
        Enter your name:
        <input type="text" autocomplete=off id="name_input" @keydown="${t=>{t.key==="Enter"&&(E("change_name"),L({type:"change_name",name:t.target.value}))}}" placeholder="sam"></input>
      </div>
      `}let Te=0;const y=W({money:0,diffs:[],moneyAnimated:0,moneyWas:0});ie(()=>{if(y.money!==y.moneyWas){const t=y.money-y.moneyWas;y.moneyWas=y.money,y.diffs.push({id:Te++,amt:t}),setTimeout(()=>{y.diffs.shift()},3e3)}});let A=null;ie(()=>{if(y.money,typeof A=="number")return;const t=()=>{if(y.money===y.moneyAnimated){A=null;return}const e=Math.abs(y.money-y.moneyAnimated),n=Math.max(Math.floor(e/30),1);if(e<=1){y.moneyAnimated=y.money,A=null;return}y.money>y.moneyAnimated?y.moneyAnimated+=n:y.moneyAnimated-=n,A=setTimeout(t,10)};A=setTimeout(t,10)});function ze(){return k`<div id="money" class="no-mouse">
    ${()=>k`<div>$${()=>y.moneyAnimated}</div>`.key("money")}
    ${()=>y.diffs.map(({amt:t,id:e})=>k`<div
          id="${`diff-${e}`}"
          class="diff ${t<0?"negative":""}"
        >
          ${t<0?"-":"+"}$${Math.abs(t)}
        </div>`.key(`diff-${e}`))}
  </div>`}const B=W({shopVisible:!1});class re{constructor(){I(this,"customizables");I(this,"unlocks");document.getElementById("app").addEventListener("mousemove",n=>{n.stopPropagation()}),document.getElementById("app").addEventListener("mousedown",n=>{n.stopPropagation()}),document.getElementById("app").addEventListener("mouseup",n=>{n.stopPropagation()}),document.getElementById("app").addEventListener("keydown",n=>{n.stopPropagation()}),document.getElementById("app").addEventListener("keyup",n=>{n.stopPropagation()}),k`
      <div>
        ${ze()}
        <div class="${()=>B.shopVisible?"":"hidden"}" id="shop">
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
        ${Le()}
      </div>
    `(document.getElementById("app")),this.customizables={hat:{items:[],index:0,equipped:0},bike:{items:[],index:0,equipped:0}},this.unlocks={hats:[],bikes:[]},document.querySelector("#hat-next").addEventListener("click",()=>{this.next_custom("hat")}),document.querySelector("#hat-prev").addEventListener("click",()=>{this.prev_custom("hat")}),document.querySelector("#bike-next").addEventListener("click",()=>{this.next_custom("bike")}),document.querySelector("#bike-prev").addEventListener("click",()=>{this.prev_custom("bike")}),document.querySelector("#bike-equip").addEventListener("click",()=>{const n="bike",s=this.customizables[n].index;this.customizables[n].equipped=s,L({type:"equip_and_buy",kind:n,index:s})}),document.querySelector("#hat-equip").addEventListener("click",()=>{const n="hat";this.customizables[n].equipped=this.customizables[n].index,L({type:"equip_and_buy",kind:n,index:this.customizables[n].index})})}prev_custom(e){const{length:n}=this.customizables[e].items;let{index:s}=this.customizables[e];s-=1,s<0&&(s=n-1),this.customizables[e].index=s,this.render_custom(e,s)}next_custom(e){const{length:n}=this.customizables[e].items;let{index:s}=this.customizables[e];s+=1,s>=n&&(s=0),this.customizables[e].index=s,this.render_custom(e,s)}render_custom(e,n,s=!0){if(console.warn({kind:e,index:n,c:this.customizables}),n<0||n>=this.customizables[e].items.length){console.error(`early access of ${e} at ${n}`);return}const{name:o,cost:r}=this.customizables[e].items[n]||{name:"None",cost:0},i=this.unlocks[`${e}s`].includes(n);document.getElementById(`${e}-name`).innerHTML=o,r===0?document.getElementById(`${e}-cost`).innerHTML="Free!":document.getElementById(`${e}-cost`).innerHTML=`Cost: $${r}`,document.getElementById(`${e}-equip`).innerHTML=`${r===0||i?"Equip":"Buy"}`,s&&L({type:"preview_cosmetic",kind:e,index:n})}receive(e){switch(e.type){case"sync_money":y.money=e.amount;break;case"phone_show_invite":b.inviter=e.from,P("invite");break;case"unlocks":this.unlocks=e,this.render_custom("hat",this.customizables.hat.index,!1),this.render_custom("bike",this.customizables.bike.index,!1);break;case"customization_info":this.customizables.hat.items=e.hat_names,this.customizables.bike.items=e.bike_names;break;case"show_shop":e.visible?(this.render_custom("hat",this.customizables.hat.index,!1),this.render_custom("bike",this.customizables.bike.index,!1),B.shopVisible=!0):(B.shopVisible=!1,this.customizables.hat.index=this.customizables.hat.equipped,this.customizables.bike.index=this.customizables.bike.equipped);break;case"phone_change_name":P("change_name");break;case"phone_new_job":this.quest();break;case"phone_accept_invite":case"phone_reject_invite":E("invite");break;case"phone_interact_key":$e();break;case"phone_dismiss_notification":E("job");break;case"sync_team_leader":b.teamLeader=e.name;break;default:console.error("Unexpected message received!",e)}}quest(){const e=["Can you take my books back to the library?","Bring me my food now!!!!","Please pick up my dry cleaning","I need 3 gerbils ASAP. No questions please","Can you deliver my groceries? I need tomato","I AM OUT OF TOILET PAPER GO FAST PLEASE","i want spaghetti","HUNGRY!!!!!!","bring me some flowers.","please do not look in this bag. just deliver","i would like 1 newspaper please","its me, pgorley","please serve these court summons for me","i ran out of coffee creamer. can you bring me some butter?","i need 37 cans of soup. no time to explain","can you deliver sushi","deliver this mail for me","can you take this trash away","i need a new kidney","PLEASE DELIVER MY TELEGRAM STOP DONT STOP STOP","find my pet turtle","let's go bowling cousin","listen, you just drive. to point B. simple.","2 Number 9's, a number 9 large, a number 6 with extra dip, 2 number 45's (one with cheese) and a large soda"],n=e[Math.floor(Math.random()*e.length)];b.questText=n,P("job")}}let V;window.bridge_init=()=>{V=new re};window.bridge_send=function(){return(V||(console.warn("Bridge accessed before init!"),0))&&re.prototype.receive.apply(V,arguments)};export{k as h,W as r};
