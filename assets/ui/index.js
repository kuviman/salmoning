var _=Object.defineProperty;var w=(o,t,s)=>t in o?_(o,t,{enumerable:!0,configurable:!0,writable:!0,value:s}):o[t]=s;var p=(o,t,s)=>(w(o,typeof t!="symbol"?t+"":t,s),s);(function(){const t=document.createElement("link").relList;if(t&&t.supports&&t.supports("modulepreload"))return;for(const e of document.querySelectorAll('link[rel="modulepreload"]'))l(e);new MutationObserver(e=>{for(const n of e)if(n.type==="childList")for(const r of n.addedNodes)r.tagName==="LINK"&&r.rel==="modulepreload"&&l(r)}).observe(document,{childList:!0,subtree:!0});function s(e){const n={};return e.integrity&&(n.integrity=e.integrity),e.referrerPolicy&&(n.referrerPolicy=e.referrerPolicy),e.crossOrigin==="use-credentials"?n.credentials="include":e.crossOrigin==="anonymous"?n.credentials="omit":n.credentials="same-origin",n}function l(e){if(e.ep)return;e.ep=!0;const n=s(e);fetch(e.href,n)}})();const E="modulepreload",L=function(o){return"/"+o},g={},b=function(t,s,l){let e=Promise.resolve();if(s&&s.length>0){const n=document.getElementsByTagName("link"),r=document.querySelector("meta[property=csp-nonce]"),y=(r==null?void 0:r.nonce)||(r==null?void 0:r.getAttribute("nonce"));e=Promise.all(s.map(i=>{if(i=L(i),i in g)return;g[i]=!0;const c=i.endsWith(".css"),v=c?'[rel="stylesheet"]':"";if(!!l)for(let d=n.length-1;d>=0;d--){const u=n[d];if(u.href===i&&(!c||u.rel==="stylesheet"))return}else if(document.querySelector(`link[href="${i}"]${v}`))return;const a=document.createElement("link");if(a.rel=c?"stylesheet":E,c||(a.as="script",a.crossOrigin=""),a.href=i,y&&a.setAttribute("nonce",y),document.head.appendChild(a),c)return new Promise((d,u)=>{a.addEventListener("load",d),a.addEventListener("error",()=>u(new Error(`Unable to preload CSS for ${i}`)))})}))}return e.then(()=>t()).catch(n=>{const r=new Event("vite:preloadError",{cancelable:!0});if(r.payload=n,window.dispatchEvent(r),!r.defaultPrevented)throw n})};let h;const P=async()=>{try{const t="../../salmoning.js";h=(await b(()=>import(t),[])).send_message_to_world}catch(o){console.error("salmoning.js module is not available",o),h=()=>console.log("Fallback function")}};P();class m{constructor(){p(this,"app");p(this,"money");p(this,"shop");p(this,"phone");var t,s,l;document.querySelector("#app").innerHTML=`
  <div>
    <div id="money">$0</div>
    <h1 class="hidden" id="shop">SHOPPING</h1>
    <div id="phone" class="phone_down">
      <div id="name">
        Enter your name:
        <input type="text" id="name_input" placeholder="sam"></input>
        </div>
    </div>
  </div>
`,this.app=document.querySelector("#app"),this.money=(t=this.app)==null?void 0:t.querySelector("#money"),this.shop=(s=this.app)==null?void 0:s.querySelector("#shop"),this.phone=(l=this.app)==null?void 0:l.querySelector("#phone"),this.phone.addEventListener("mousemove",e=>{var n;((n=document==null?void 0:document.activeElement)==null?void 0:n.id)==="name_input"&&e.stopPropagation()}),this.phone.addEventListener("keydown",e=>{e.target.id==="name_input"&&(e.stopPropagation(),console.log(e),e.key==="Enter"&&(h({ChangeName:{name:e.target.value}}),e.target.blur()))}),this.phone.addEventListener("keyup",e=>{e.target.id==="name_input"&&(e.stopPropagation(),console.log(e))})}sync_money(t){this.money.innerHTML=`$${t}`}show_phone(t){t?this.phone.classList.remove("phone_down"):this.phone.classList.add("phone_down")}show_shop(t){t?this.shop.classList.remove("hidden"):this.shop.classList.add("hidden")}}let f;window.bridge_init=()=>{f=new m};Object.getOwnPropertyNames(m.prototype).forEach(o=>{window[`bridge_${o}`]=function(){return f&&m.prototype[o].apply(f,arguments)}});
