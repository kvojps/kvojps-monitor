// Run from the repository root with Vite on :1420 and a disposable Chromium CDP on :9222.
// Text-only WCAG scan: excludes disabled controls, hidden/decorative content and SVG.
// Graphical controls/series and native behavior need separate checks. No production data.
import fs from 'node:fs/promises';
const page = (await (await fetch('http://127.0.0.1:9222/json/list')).json()).find(p => p.type === 'page');
const ws = new WebSocket(page.webSocketDebuggerUrl);
await new Promise(r => ws.addEventListener('open', r, { once: true }));
let id = 0;
const pending = new Map();
ws.addEventListener('message', e => { const m = JSON.parse(e.data); if (m.id) {
    const p = pending.get(m.id);
    pending.delete(m.id);
    m.error ? p.reject(m.error) : p.resolve(m.result);
} });
const call = (method, params = {}) => new Promise((resolve, reject) => { const n = ++id; pending.set(n, { resolve, reject }); ws.send(JSON.stringify({ id: n, method, params })); });
const evaluate = async (expression) => { const result = await call('Runtime.evaluate', { expression, returnByValue: true, awaitPromise: true }); if (result.exceptionDetails)
    throw new Error(JSON.stringify(result.exceptionDetails)); return result.result.value; };
await call('Page.enable');
await call('Runtime.enable');
const results = [];
const scenarios = [];
for (const theme of ['dark', 'light', 'system']) {
    for (const width of [1440, 800, 560, 900])
        scenarios.push({ theme, width, query: '', textScale: 1 });
    for (const file of await fs.readdir('src/lib')) {
        let names;
        try {
            names = await fs.readdir(`src/lib/${file}/fixtures`);
        }
        catch {
            continue;
        }
        for (const name of names.filter(n => n.endsWith('.json')))
            scenarios.push({ theme, width: 560, query: `&${file}=${name.slice(0, -5)}`, textScale: 1 });
    }
    scenarios.push({ theme, width: 560, query: '', textScale: 2 });
    scenarios.push({ theme, width: 560, query: '', textScale: 1, settings: true });
}
for (const { theme, width, query, textScale, settings } of scenarios) {
    await call('Emulation.setDeviceMetricsOverride', { width, height: 1000, deviceScaleFactor: width === 900 ? 2 : 1, mobile: false });
    await call('Page.navigate', { url: `http://127.0.0.1:1420/.scratch/cinematic-panel-implementation/validation/index.html?theme=${theme}${query}` });
    await new Promise(r => setTimeout(r, 150));
    for (let i = 0; i < 80; i++) {
        if (await evaluate('!!document.querySelector(".network")'))
            break;
        await new Promise(r => setTimeout(r, 100));
    }
    if (!await evaluate('!!document.querySelector(".network")'))
        throw new Error(`Panel did not mount: ${theme}${query}`);
    if (settings)
        await evaluate(`document.querySelector('.settings-item').click()`);
    if (textScale !== 1)
        await evaluate(`[...document.querySelectorAll('body *')].map(e=>[e,parseFloat(getComputedStyle(e).fontSize)]).forEach(([e,size])=>e.style.fontSize=(size*${textScale})+'px')`);
    const result = await evaluate(`(() => {
 const overflows=[...document.querySelectorAll('.module, .network, .settings, .station')].filter(e=>e.scrollWidth>e.clientWidth+1).map(e=>({name:e.className,client:e.clientWidth,scroll:e.scrollWidth}));
 function rgb(s){s=s.trim();if(s.startsWith('#'))return [1,3,5].map(i=>parseInt(s.slice(i,i+2),16)).concat(1);return s.match(/[\\d.]+/g)?.map(Number)??[0,0,0,0]}
 function blend(a,b){let t=a[3]??1;return a.slice(0,3).map((v,i)=>v*t+b[i]*(1-t)).concat(1)}
 function lum(c){return c.slice(0,3).map(v=>{v/=255;return v<=.04045?v/12.92:((v+.055)/1.055)**2.4}).reduce((s,v,i)=>s+v*[.2126,.7152,.0722][i],0)}
 const failures=[];let min=100;
 for(const el of document.querySelectorAll('body *')){
 if(![...el.childNodes].some(n=>n.nodeType===3&&n.textContent.trim())||!el.getClientRects().length||el.closest('svg,button:disabled,.sr-only,[aria-hidden=true]'))continue;
 const style=getComputedStyle(el);if(style.display==='none'||style.visibility==='hidden')continue;
 const chain=[];for(let e=el;e;e=e.parentElement)chain.unshift(e);
 let bg=[255,255,255,1];for(const e of chain)bg=blend(rgb(getComputedStyle(e).backgroundColor),bg);
 let fg=blend(rgb(style.color),bg);
 // Composite group opacity over the ancestor background, including stale content.
 for(let e=el;e;e=e.parentElement){const opacity=+getComputedStyle(e).opacity;if(opacity<1){const backdrop=rgb(getComputedStyle(e.parentElement??e).backgroundColor);const under=(backdrop[3]??1)===0?rgb(getComputedStyle(document.documentElement).getPropertyValue('--module-bg')):backdrop;fg=blend([...fg.slice(0,3),opacity],under);bg=blend([...bg.slice(0,3),opacity],under)}}
 const a=lum(fg),b=lum(bg),ratio=(Math.max(a,b)+.05)/(Math.min(a,b)+.05);
 if (!Number.isFinite(ratio)) throw new Error('Unsupported rendered color: '+style.color);
 const large=parseFloat(style.fontSize)>=24||(parseFloat(style.fontSize)>=18.66&&+style.fontWeight>=700);min=Math.min(min,ratio);
 if(ratio<(large?3:4.5))failures.push({text:el.textContent.trim().slice(0,65),ratio:+ratio.toFixed(2),color:style.color,size:style.fontSize});
 }
 return {overflows,min,failures,ready:!!document.querySelector('.network')};})()`);
    const shot = (!query && textScale === 1 && !settings) ? await call('Page.captureScreenshot', { format: 'png', captureBeyondViewport: false }) : {};
    if (!query && textScale === 1 && !settings)
        await fs.writeFile(`/tmp/panel-${theme}-${width}.png`, Buffer.from(shot.data, 'base64'));
    results.push({ theme, width, query, textScale, settings: !!settings, ...result });
}
console.log(JSON.stringify(results, null, 2));
ws.close();
if (results.some(r => !r.ready || r.failures.length || r.overflows.length))
    process.exitCode = 1;
