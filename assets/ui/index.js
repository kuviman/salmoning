var l=Object.defineProperty;var u=(o,e,s)=>e in o?l(o,e,{enumerable:!0,configurable:!0,writable:!0,value:s}):o[e]=s;var i=(o,e,s)=>(u(o,typeof e!="symbol"?e+"":e,s),s);(function(){const e=document.createElement("link").relList;if(e&&e.supports&&e.supports("modulepreload"))return;for(const t of document.querySelectorAll('link[rel="modulepreload"]'))p(t);new MutationObserver(t=>{for(const r of t)if(r.type==="childList")for(const n of r.addedNodes)n.tagName==="LINK"&&n.rel==="modulepreload"&&p(n)}).observe(document,{childList:!0,subtree:!0});function s(t){const r={};return t.integrity&&(r.integrity=t.integrity),t.referrerPolicy&&(r.referrerPolicy=t.referrerPolicy),t.crossOrigin==="use-credentials"?r.credentials="include":t.crossOrigin==="anonymous"?r.credentials="omit":r.credentials="same-origin",r}function p(t){if(t.ep)return;t.ep=!0;const r=s(t);fetch(t.href,r)}})();class c{constructor(){i(this,"app");i(this,"money");i(this,"shop");var e,s;document.querySelector("#app").innerHTML=`
  <div>
    <h1 id="money">$0</h1>
    <h1 class="hidden" id="shop">SHOPPING</h1>
  </div>
`,this.app=document.querySelector("#app"),this.money=(e=this.app)==null?void 0:e.querySelector("#money"),this.shop=(s=this.app)==null?void 0:s.querySelector("#shop")}sync_money(e){this.money.innerHTML=`$${e}`}show_shop(e){e?this.shop.classList.remove("hidden"):this.shop.classList.add("hidden")}}let d;window.bridge_init=()=>{d=new c};Object.getOwnPropertyNames(c.prototype).forEach(o=>{window[`bridge_${o}`]=function(){return d&&c.prototype[o].apply(d,arguments)}});
