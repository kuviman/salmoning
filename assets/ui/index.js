var u=Object.defineProperty;var l=(o,e,i)=>e in o?u(o,e,{enumerable:!0,configurable:!0,writable:!0,value:i}):o[e]=i;var n=(o,e,i)=>(l(o,typeof e!="symbol"?e+"":e,i),i);(function(){const e=document.createElement("link").relList;if(e&&e.supports&&e.supports("modulepreload"))return;for(const t of document.querySelectorAll('link[rel="modulepreload"]'))s(t);new MutationObserver(t=>{for(const r of t)if(r.type==="childList")for(const d of r.addedNodes)d.tagName==="LINK"&&d.rel==="modulepreload"&&s(d)}).observe(document,{childList:!0,subtree:!0});function i(t){const r={};return t.integrity&&(r.integrity=t.integrity),t.referrerPolicy&&(r.referrerPolicy=t.referrerPolicy),t.crossOrigin==="use-credentials"?r.credentials="include":t.crossOrigin==="anonymous"?r.credentials="omit":r.credentials="same-origin",r}function s(t){if(t.ep)return;t.ep=!0;const r=i(t);fetch(t.href,r)}})();class p{constructor(){n(this,"app");n(this,"money");n(this,"shop");n(this,"phone");var e,i,s;document.querySelector("#app").innerHTML=`
  <div>
    <div id="money">$0</div>
    <h1 class="hidden" id="shop">SHOPPING</h1>
    <div id="phone">
      <div id="name">
        Enter your name:
        <input type="text" placeholder="sam"></input>
        </div>
    </div>
  </div>
`,this.app=document.querySelector("#app"),this.money=(e=this.app)==null?void 0:e.querySelector("#money"),this.shop=(i=this.app)==null?void 0:i.querySelector("#shop"),this.phone=(s=this.app)==null?void 0:s.querySelector("#phone")}sync_money(e){this.money.innerHTML=`$${e}`}show_shop(e){e?this.shop.classList.remove("hidden"):this.shop.classList.add("hidden")}}let c;window.bridge_init=()=>{c=new p};Object.getOwnPropertyNames(p.prototype).forEach(o=>{window[`bridge_${o}`]=function(){return c&&p.prototype[o].apply(c,arguments)}});
