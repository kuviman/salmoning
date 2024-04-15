var c=Object.defineProperty;var h=(i,t,n)=>t in i?c(i,t,{enumerable:!0,configurable:!0,writable:!0,value:n}):i[t]=n;var s=(i,t,n)=>(h(i,typeof t!="symbol"?t+"":t,n),n);(function(){const t=document.createElement("link").relList;if(t&&t.supports&&t.supports("modulepreload"))return;for(const e of document.querySelectorAll('link[rel="modulepreload"]'))r(e);new MutationObserver(e=>{for(const o of e)if(o.type==="childList")for(const d of o.addedNodes)d.tagName==="LINK"&&d.rel==="modulepreload"&&r(d)}).observe(document,{childList:!0,subtree:!0});function n(e){const o={};return e.integrity&&(o.integrity=e.integrity),e.referrerPolicy&&(o.referrerPolicy=e.referrerPolicy),e.crossOrigin==="use-credentials"?o.credentials="include":e.crossOrigin==="anonymous"?o.credentials="omit":o.credentials="same-origin",o}function r(e){if(e.ep)return;e.ep=!0;const o=n(e);fetch(e.href,o)}})();class p{constructor(){s(this,"app");s(this,"money");s(this,"shop");s(this,"phone");var t,n,r;document.querySelector("#app").innerHTML=`
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
`,this.app=document.querySelector("#app"),this.money=(t=this.app)==null?void 0:t.querySelector("#money"),this.shop=(n=this.app)==null?void 0:n.querySelector("#shop"),this.phone=(r=this.app)==null?void 0:r.querySelector("#phone"),this.phone.addEventListener("keydown",e=>{e.target.id==="name_input"&&(e.stopPropagation(),console.log(e),e.key==="Enter"&&(window.send_message_to_world({ChangeName:{name:e.target.value}}),e.target.blur()))}),this.phone.addEventListener("keyup",e=>{e.target.id==="name_input"&&(e.stopPropagation(),console.log(e))})}sync_money(t){this.money.innerHTML=`$${t}`}show_phone(t){t?this.phone.classList.remove("phone_down"):this.phone.classList.add("phone_down")}show_shop(t){t?this.shop.classList.remove("hidden"):this.shop.classList.add("hidden")}}let a;window.bridge_init=()=>{a=new p};Object.getOwnPropertyNames(p.prototype).forEach(i=>{window[`bridge_${i}`]=function(){return a&&p.prototype[i].apply(a,arguments)}});
