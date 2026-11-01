// Ticket 07 integrated audit. Run from the repository root with Vite on :1420 and a
// disposable Chromium CDP on :9222. Browser evidence only: it never proves native
// GNOME/Wayland behavior. Uses recorded fixtures; no credentials or collection.
import fs from 'node:fs/promises';

const BASE = 'http://127.0.0.1:1420/.scratch/compact-panel-implementation/validation/surface/index.html';
const SHOTS = process.env.SHOTS ?? '/tmp';
const page = (await (await fetch('http://127.0.0.1:9222/json/list')).json()).find(p => p.type === 'page');
const ws = new WebSocket(page.webSocketDebuggerUrl);
await new Promise(r => ws.addEventListener('open', r, { once: true }));
const sleep = (ms) => new Promise(r => setTimeout(r, ms));
let seq = 0;
const pending = new Map();
ws.addEventListener('message', e => {
  const m = JSON.parse(e.data);
  if (!m.id) return;
  const p = pending.get(m.id);
  pending.delete(m.id);
  m.error ? p.reject(new Error(JSON.stringify(m.error))) : p.resolve(m.result);
});
const call = (method, params = {}) => new Promise((resolve, reject) => {
  const n = ++seq; pending.set(n, { resolve, reject }); ws.send(JSON.stringify({ id: n, method, params }));
});
const evaluate = async (expression) => {
  const r = await Promise.race([call('Runtime.evaluate', { expression, returnByValue: true, awaitPromise: true }),
    sleep(15000).then(() => { throw new Error('evaluate timed out: ' + expression.slice(0, 120)); })]);
  if (r.exceptionDetails) throw new Error(JSON.stringify(r.exceptionDetails).slice(0, 600));
  return r.result.value;
};
const key = async (k, mods = 0) => {
  const code = k.length === 1 ? `Key${k.toUpperCase()}` : k;
  const text = { Enter: '\r', ' ': ' ' }[k];
  await call('Input.dispatchKeyEvent', { type: text ? 'keyDown' : 'rawKeyDown', key: k, code, modifiers: mods, text, windowsVirtualKeyCode: { Tab: 9, Escape: 27, Enter: 13, ' ': 32 }[k] ?? 0 });
  await call('Input.dispatchKeyEvent', { type: 'keyUp', key: k, code, modifiers: mods });
};
const frame = () => evaluate('new Promise(r => requestAnimationFrame(() => requestAnimationFrame(r)))');

await call('Page.enable');
await call('Runtime.enable');
await call('Accessibility.enable');
// Fixed clock at the AI fixture time, matching tickets 02/04.
await call('Page.addScriptToEvaluateOnNewDocument', { source: 'Date.now = () => 1793448000000' });

async function load({ width = 1100, height = 700, scale = 1, theme = 'dark', query = '', prefersDark = false }) {
  await call('Emulation.setDeviceMetricsOverride', { width, height, deviceScaleFactor: scale, mobile: false });
  await call('Emulation.setEmulatedMedia', { features: [{ name: 'prefers-color-scheme', value: prefersDark ? 'dark' : 'light' }] });
  await call('Page.navigate', { url: `${BASE}?theme=${theme}${query}` });
  for (let i = 0; i < 100; i++) {
    if (await evaluate('!!document.querySelector("#module-system") && document.documentElement.dataset.theme !== undefined').catch(() => false)) break;
    await sleep(50);
  }
  await frame();
}

// Shared in-page helpers: contrast, clipping, focus indicator, accessible name.
const HELPERS = `
window.__a = (() => {
  const rgb = s => { s = s.trim(); if (s.startsWith('#')) return [1,3,5].map(i => parseInt(s.slice(i,i+2),16)).concat(1); const m = s.match(/[\\d.]+/g)?.map(Number) ?? [0,0,0,0]; if (m.length === 3) m.push(1); return m; };
  const blend = (a, b) => { const t = a[3] ?? 1; return a.slice(0,3).map((v,i) => v*t + b[i]*(1-t)).concat(1); };
  const lum = c => c.slice(0,3).map(v => { v /= 255; return v <= .04045 ? v/12.92 : ((v+.055)/1.055)**2.4; }).reduce((s,v,i) => s + v*[.2126,.7152,.0722][i], 0);
  const ratio = (f, b) => { const x = lum(f), y = lum(b); return (Math.max(x,y)+.05)/(Math.min(x,y)+.05); };
  const backdrop = el => { const chain = []; for (let e = el; e; e = e.parentElement) chain.unshift(e); let bg = [255,255,255,1]; for (const e of chain) bg = blend(rgb(getComputedStyle(e).backgroundColor), bg); return bg; };
  const opacityOf = el => { let o = 1; for (let e = el; e; e = e.parentElement) o *= +getComputedStyle(e).opacity; return o; };
  const visible = el => { if (!el.getClientRects().length) return false; const s = getComputedStyle(el); return s.visibility !== 'hidden' && s.display !== 'none'; };
  const name = el => {
    const by = el.getAttribute('aria-labelledby');
    if (by) return by.split(/\\s+/).map(id => document.getElementById(id)?.textContent ?? '').join(' ').trim();
    return (el.getAttribute('aria-label') ?? (el.labels?.[0]?.textContent) ?? el.textContent ?? el.getAttribute('title') ?? '').trim();
  };
  function contrast(root) {
    const failures = []; let min = 100;
    for (const el of root.querySelectorAll('*')) {
      if (![...el.childNodes].some(n => n.nodeType === 3 && n.textContent.trim()) || !visible(el)) continue;
      if (el.closest('svg,:disabled,.sr-only,[aria-hidden=true],option')) continue;
      const style = getComputedStyle(el);
      const bg = backdrop(el);
      const o = opacityOf(el);
      const fg = blend([...blend(rgb(style.color), bg).slice(0,3), o], bg);
      const r = ratio(fg, bg);
      const size = parseFloat(style.fontSize), large = size >= 24 || (size >= 18.66 && +style.fontWeight >= 700);
      min = Math.min(min, r);
      if (r < (large ? 3 : 4.5)) failures.push({ text: el.textContent.trim().slice(0, 50), ratio: +r.toFixed(2), color: style.color, size: style.fontSize, where: el.closest('[id]')?.id });
    }
    return { min: +min.toFixed(2), failures };
  }
  // Non-text: icons inside controls and control boundaries need 3:1 against the backdrop.
  function graphics(root) {
    const failures = []; let min = 100;
    for (const b of root.querySelectorAll('button:not(:disabled), select, input, a[href], [role=button]')) {
      if (!visible(b)) continue;
      const bg = backdrop(b.parentElement ?? b);
      const own = rgb(getComputedStyle(b).backgroundColor);
      const shape = (own[3] ?? 1) > 0 ? blend(own, bg) : null;
      const border = getComputedStyle(b);
      const borderColor = parseFloat(border.borderTopWidth) > 0 && border.borderTopStyle !== 'none' ? blend(rgb(border.borderTopColor), bg) : null;
      const bounded = [shape, borderColor].filter(Boolean).map(c => ratio(c, bg));
      const icons = [...b.querySelectorAll('svg')].map(svg => { const s = getComputedStyle(svg); const c = s.stroke !== 'none' && !s.stroke.startsWith('url') ? s.stroke : s.fill; return ratio(blend(rgb(c === 'none' ? s.color : c), shape ?? bg), shape ?? bg); });
      const textOnly = !icons.length && b.textContent.trim();
      // A text-only control is identified by its text (already checked); otherwise a boundary or icon must reach 3:1.
      const best = Math.max(0, ...bounded, ...icons);
      if (!textOnly) { min = Math.min(min, best); if (best < 3) failures.push({ name: name(b).slice(0, 50), ratio: +best.toFixed(2), where: b.closest('[id]')?.id }); }
    }
    return { min: +min.toFixed(2), failures };
  }
  // Content cut by an ancestor that hides overflow, or text truncated with an ellipsis.
  function clipping(root) {
    const out = [];
    for (const el of root.querySelectorAll('*')) {
      if (!visible(el) || el.closest('svg,.sr-only,dialog:not([open])')) continue;
      const s = getComputedStyle(el);
      const hides = (v) => v === 'hidden' || v === 'clip';
      if ((hides(s.overflowX) && el.scrollWidth > el.clientWidth + 1) || (hides(s.overflowY) && el.scrollHeight > el.clientHeight + 1))
        out.push({ el: el.tagName.toLowerCase() + '.' + [...el.classList].join('.'), where: el.closest('[id]')?.id, w: [el.clientWidth, el.scrollWidth], h: [el.clientHeight, el.scrollHeight], text: el.textContent.trim().slice(0, 40) });
    }
    return out.filter(o => !o.el.startsWith('div.panel'));
  }
  function escapes(root) {
    const out = [];
    for (const mod of root.querySelectorAll('section.module')) {
      const box = mod.getBoundingClientRect();
      for (const el of mod.querySelectorAll('*')) {
        if (!visible(el) || el.closest('dialog,.sr-only,[role=tooltip]')) continue;
        const r = el.getBoundingClientRect();
        if (r.width && (r.right > box.right + 1 || r.left < box.left - 1)) { out.push({ module: mod.id, el: el.tagName.toLowerCase() + '.' + [...el.classList].join('.'), over: +(r.right - box.right).toFixed(1) }); break; }
      }
    }
    return out;
  }
  function focusRing(el) {
    const s = getComputedStyle(el);
    const outline = s.outlineStyle !== 'none' && parseFloat(s.outlineWidth) >= 1;
    const shadow = s.boxShadow && s.boxShadow !== 'none';
    const bg = backdrop(el.parentElement ?? el);
    const c = outline ? ratio(blend(rgb(s.outlineColor), bg), bg) : null;
    return { visible: outline || shadow, width: s.outlineWidth, contrast: c && +c.toFixed(2) };
  }
  return { contrast, graphics, clipping, escapes, focusRing, name, visible };
})();`;

const results = { meta: { date: new Date().toISOString(), browser: (await (await fetch('http://127.0.0.1:9222/json/version')).json()).Browser }, layout: [], states: [], keyboard: [], overlays: [], trello: [], target: [], motion: [], timing: [] };

process.on('SIGTERM', () => { console.log(JSON.stringify({ ...results, error: 'timeout' }, null, 2)); process.exit(1); });
process.on('uncaughtException', (error) => { console.log(JSON.stringify({ ...results, error: String(error) }, null, 2)); process.exit(1); });

async function measure(scope = 'document') {
  await evaluate(HELPERS);
  return evaluate(`(() => {
    const root = ${scope};
    const sc = document.querySelector('main.scroller');
    const a = window.__a;
    return { mainScroll: sc.scrollHeight - sc.clientHeight, columns: getComputedStyle(document.querySelector('.composition')).gridTemplateColumns.split(' ').length,
      contrast: a.contrast(root), graphics: a.graphics(root), clipping: a.clipping(root), escapes: a.escapes(document),
      horizontal: document.documentElement.scrollWidth > innerWidth || sc.scrollWidth > sc.clientWidth + 1 };
  })()`);
}

async function scaleText(factor) {
  await evaluate(`(() => { const all = [...document.querySelectorAll('.panel *')].map(e => [e, parseFloat(getComputedStyle(e).fontSize)]); all.forEach(([e, s]) => e.style.fontSize = (s * ${factor}) + 'px'); })()`);
  await frame();
}

console.error('section 1', new Date().toISOString());
// 1. Layout against B: target and fallback sizes, themes, text 125%, long names, dense lists, monitor scale.
const themes = [['dark', false], ['light', false], ['system', false], ['system', true]];
for (const [theme, prefersDark] of themes) {
  for (const width of [1100, 1000, 950, 650]) {
    await load({ width, theme, prefersDark });
    const m = await measure();
    results.layout.push({ case: 'default', theme, prefersDark, width, ...m });
    if (width >= 1000 && !(theme === 'system' && prefersDark)) {
      const shot = await call('Page.captureScreenshot', { format: 'png' });
      await fs.writeFile(`${SHOTS}/integrated-${theme}-${width}.png`, Buffer.from(shot.data, 'base64'));
    }
  }
  for (const width of [1100, 1000]) {
    await load({ width, theme, prefersDark });
    await scaleText(1.25);
    results.layout.push({ case: 'text-125', theme, prefersDark, width, ...(await measure()) });
  }
  await load({ theme, prefersDark, query: '&long=1' });
  results.layout.push({ case: 'long-names', theme, prefersDark, width: 1100, ...(await measure()) });
  await load({ theme, prefersDark, query: '&dense=1&agents=mixed' });
  results.layout.push({ case: 'dense', theme, prefersDark, width: 1100, ...(await measure()) });
}
for (const scale of [1.25, 1.5, 2]) {
  // Logical 1100 × 700 at fractional/integer monitor scale; CSS layout must not change.
  await load({ scale });
  results.layout.push({ case: `monitor-${scale}`, theme: 'dark', width: 1100, ...(await measure()) });
}
// Reduced work area: the native code clamps the window; the webview then lays out at the smaller size.
for (const [w, h] of [[1100, 560], [900, 600], [760, 500]]) {
  await load({ width: w, height: h });
  const m = await measure();
  const reach = await evaluate(`(() => { const sc = document.querySelector('main.scroller'); sc.scrollTop = sc.scrollHeight; const last = document.querySelector('#module-system').getBoundingClientRect(); return last.bottom <= sc.getBoundingClientRect().bottom + 1; })()`);
  results.layout.push({ case: `work-area-${w}x${h}`, theme: 'dark', width: w, height: h, ...m, lastModuleReachable: reach });
}

console.error('section 2', new Date().toISOString());
// 2. State matrix per Module, scoped to the Module.
const fixtureNames = async (m) => (await fs.readdir(`src/lib/${m}/fixtures`)).filter(n => n.endsWith('.json')).map(n => n.slice(0, -5));
for (const module of ['ai', 'agents', 'github', 'network', 'trello', 'system']) {
  for (const name of [...await fixtureNames(module), 'none']) {
    for (const theme of ['dark', 'light']) {
      await load({ theme, query: `&${module}=${name}` });
      const m = await measure(`document.getElementById('module-${module}')`);
      const text = await evaluate(`document.getElementById('module-${module}').innerText.replace(/\\s+/g, ' ').trim().slice(0, 160)`);
      results.states.push({ module, fixture: name, theme, mainScroll: m.mainScroll, contrast: m.contrast, graphics: m.graphics, clipping: m.clipping, escapes: m.escapes.filter(e => e.module === `module-${module}`), text });
    }
  }
}

console.error('section 3', new Date().toISOString());
// 3. Keyboard: tab order, accessible names, visible focus, everything reachable.
for (const theme of ['dark', 'light']) {
  await load({ theme });
  await evaluate(HELPERS);
  await evaluate('document.activeElement?.blur(); document.body.focus()');
  const visited = [];
  for (let i = 0; i < 160; i++) {
    await key('Tab');
    const info = await evaluate(`(() => { const el = document.activeElement; if (!el || el === document.body) return null; el.dataset.auditId ??= String(Math.random()).slice(2);
      return { id: el.dataset.auditId, tag: el.tagName.toLowerCase(), name: __a.name(el).slice(0, 60), module: el.closest('section.module')?.dataset.module ?? 'toolbar', ring: __a.focusRing(el) }; })()`);
    if (!info) continue;
    if (visited.length && info.id === visited[0].id) break;
    visited.push(info);
  }
  const focusable = await evaluate(`[...document.querySelectorAll('.panel button:not(:disabled), .panel a[href], .panel select, .panel input, .panel [tabindex]:not([tabindex="-1"])')].filter(e => __a.visible(e) && !e.closest('dialog,[inert]')).length`);
  const order = visited.map(v => v.module).filter((m, i, all) => m !== all[i - 1]);
  results.keyboard.push({ theme, reached: visited.length, focusable, order, unnamed: visited.filter(v => !v.name), noRing: visited.filter(v => !v.ring.visible), lowRing: visited.filter(v => v.ring.contrast !== null && v.ring.contrast < 3) });
  // Accessibility tree: named controls and charts.
  const { nodes } = await call('Accessibility.getFullAXTree');
  const roles = ['button', 'link', 'combobox', 'img', 'image', 'meter', 'progressbar', 'figure'];
  const nameless = nodes.filter(n => !n.ignored && roles.includes(n.role?.value) && !(n.name?.value ?? '').trim()).map(n => n.role.value);
  const charts = nodes.filter(n => !n.ignored && ['img', 'image', 'meter', 'figure'].includes(n.role?.value)).map(n => ({ role: n.role.value, name: (n.name?.value ?? '').slice(0, 90) }));
  results.keyboard.at(-1).ax = { nameless, charts };
}

console.error('section 4', new Date().toISOString());
// 4. Overlays: geometry inside the window, focus confinement, Escape and focus restoration.
const overlays = [
  ['agents', `document.querySelector('#module-agents button.ring')`],
  ['network', `document.querySelector('#module-network button.apps-open')`],
  ['trello', `document.querySelector('#module-trello button.open-board')`],
];
for (const [width, height] of [[1100, 700], [1000, 700], [650, 700], [1100, 560]]) {
  for (const theme of ['dark', 'light']) {
    for (const [module, trigger] of overlays) {
      await load({ width, height, theme, query: '&dense=1&agents=mixed' });
      await evaluate(HELPERS);
      await evaluate(`${trigger}.focus()`);
      await key('Enter');
      await frame();
      const geo = await evaluate(`(() => { const d = document.querySelector('#module-${module} dialog[open]'); if (!d) return null; const r = d.getBoundingClientRect();
        return { w: Math.round(r.width), h: Math.round(r.height), inside: r.left >= 0 && r.top >= 0 && r.right <= innerWidth && r.bottom <= innerHeight, focusInside: d.contains(document.activeElement),
          contrast: __a.contrast(d), graphics: __a.graphics(d), clipping: __a.clipping(d) }; })()`);
      let escaped = 0;
      for (let i = 0; i < 40; i++) { await key('Tab'); if (await evaluate(`(() => { const a = document.activeElement; return !!a && a !== document.body && !a.closest('#module-${module} dialog[open]'); })()`)) escaped++; }
      await key('Escape');
      await frame();
      const afterFirst = await evaluate(`({ open: !!document.querySelector('dialog[open]'), restored: document.activeElement === ${trigger}, closed: __coreCalls.includes('close') })`);
      await key('Escape');
      const afterSecond = await evaluate(`__coreCalls.includes('close')`);
      results.overlays.push({ module, width, height, theme, geo, focusEscapes: escaped, afterFirst, panelClosedOnSecondEscape: afterSecond });
    }
  }
}

console.error('section 5', new Date().toISOString());
// 5. Trello: keyboard move, summary sync, revert on failure, selection on reopen, out-of-view lists, external open.
for (const mode of ['ok', 'fail']) {
  await load({ query: mode === 'fail' ? '&move=fail' : '' });
  const before = await evaluate(`document.querySelector('#module-trello ul.counts').innerText.replace(/\\s+/g, ' ')`);
  await evaluate(`document.querySelector('#module-trello button.open-board').focus()`);
  await key('Enter');
  await frame();
  const listInfo = await evaluate(`(() => { const cols = [...document.querySelectorAll('#module-trello .columns > .column')]; const box = document.querySelector('#module-trello .columns').getBoundingClientRect();
    return { lists: cols.length, outOfView: cols.filter(c => c.getBoundingClientRect().left >= box.right).length }; })()`);
  // Select the first card by keyboard, choose the last (out-of-view) list and confirm.
  await evaluate(`document.querySelector('#module-trello .column .card').focus()`);
  await key('Enter');
  await frame();
  const options = await evaluate(`[...document.querySelectorAll('#module-trello dialog select option')].map(o => o.value).filter(Boolean)`);
  await evaluate(`(() => { const s = document.querySelector('#module-trello dialog select'); s.focus(); s.value = s.options[s.options.length - 1].value; s.dispatchEvent(new Event('change', { bubbles: true })); })()`);
  await frame();
  await evaluate(`document.querySelector('#module-trello dialog button.primary').focus()`);
  await key('Enter');
  await frame();
  const optimistic = await evaluate(`document.querySelector('#module-trello ul.counts').innerText.replace(/\\s+/g, ' ')`);
  await sleep(150);
  await frame();
  const settled = await evaluate(`({ counts: document.querySelector('#module-trello ul.counts').innerText.replace(/\\s+/g, ' '), message: document.querySelector('#module-trello dialog [role=status], #module-trello dialog [role=alert], #module-trello dialog .notice')?.textContent.trim() ?? null, calls: __coreCalls.filter(c => c.startsWith('move')) })`);
  // Selection preserved across close/reopen; double click opens externally.
  // Choose a card that is not selected yet: clicking a selected card toggles it off.
  await evaluate(`document.querySelector('#module-trello .column .card:not([aria-pressed=true])').click()`);
  await key('Escape');
  await frame();
  await evaluate(`document.querySelector('#module-trello button.open-board').click()`);
  await frame();
  const reopened = await evaluate(`({ selected: !!document.querySelector('#module-trello dialog [aria-pressed=true], #module-trello dialog .card.selected, #module-trello dialog [aria-selected=true]'), focusOnCard: !!document.activeElement?.closest('.card') })`);
  await evaluate(`document.querySelector('#module-trello .column .card').dispatchEvent(new MouseEvent('dblclick', { bubbles: true }))`);
  const opened = await evaluate(`__coreCalls.some(c => c.startsWith('open:'))`);
  const draggable = await evaluate(`document.querySelector('#module-trello .column .card').getAttribute('draggable')`);
  results.trello.push({ mode, before, listInfo, destinations: options.length, optimistic, settled, reopened, opened, draggable });
}

console.error('section 6', new Date().toISOString());
// 6. Notification target: retained closed session and unavailable session.
for (const target of ['s-closed', 's-waiting', 'missing-session']) {
  await load({ query: `&target=${target}` });
  await sleep(100);
  results.target.push({ target, ...(await evaluate(`(() => { const d = document.querySelector('#module-agents dialog'); const hl = d?.querySelector('[aria-current], .notified, .target');
    return { open: !!d?.open, highlighted: hl?.textContent.replace(/\\s+/g, ' ').trim().slice(0, 80) ?? null, focus: document.activeElement?.textContent?.replace(/\\s+/g, ' ').trim().slice(0, 60) ?? null, notice: d?.querySelector('.notice, [role=status]')?.textContent.trim() ?? null }; })()`)) });
}

console.error('section 7', new Date().toISOString());
// 7. Motion and hidden panel: decoration policy and running animations (browser instrumentation).
for (const motion of ['full', 'reduced']) {
  await load({ query: `&motion=${motion}` });
  const count = () => evaluate(`document.getAnimations().filter(a => a.playState === 'running').length`);
  const shown = { decoration: await evaluate('document.documentElement.dataset.decoration'), running: await count() };
  await evaluate('__setVisible(false)');
  await frame();
  const hidden = { decoration: await evaluate('document.documentElement.dataset.decoration'), running: await count() };
  await evaluate('__setVisible(true)');
  await frame();
  const reopened = { decoration: await evaluate('document.documentElement.dataset.decoration'), running: await count() };
  results.motion.push({ motion, shown, hidden, reopened });
}

console.error('section 8', new Date().toISOString());
// 8. Browser response: Event Timing (input → next paint) and long animation frames on dense lists.
await load({ query: '&dense=1&agents=mixed&motion=full' });
await evaluate(`window.__events = []; window.__loaf = [];
  new PerformanceObserver(l => l.getEntries().forEach(e => __events.push({ name: e.name, duration: e.duration }))).observe({ type: 'event', durationThreshold: 16, buffered: true });
  try { new PerformanceObserver(l => l.getEntries().forEach(e => __loaf.push(e.duration))).observe({ type: 'long-animation-frame', buffered: true }); } catch {}`);
const responses = [];
for (let i = 0; i < 20; i++) {
  for (const [module, trigger] of overlays) {
    const t = await evaluate(`new Promise(r => { const t0 = performance.now(); ${trigger}.click(); requestAnimationFrame(() => requestAnimationFrame(() => r(performance.now() - t0))); })`);
    responses.push({ module, ms: t });
    await key('Escape');
    await frame();
  }
}
await evaluate(`document.querySelector('#module-trello button.open-board').click()`);
await frame();
// Scroll a dense list for ~2 s and record rAF intervals; resolves even if frames stop.
const scrollFrames = await evaluate(`new Promise(r => { const col = document.querySelector('#module-trello dialog .column ul') ?? document.querySelector('#module-trello dialog .columns'); const cols = document.querySelector('#module-trello dialog .columns');
  const times = []; let last = performance.now(); const end = last + 2000;
  const step = (t) => { times.push(t - last); last = t; col.scrollTop += 40; cols.scrollLeft += 20; if (t < end) requestAnimationFrame(step); else r(times.slice(1)); };
  requestAnimationFrame(step); setTimeout(() => r(times.slice(1)), 4000); })`);
const p = (arr, q) => { if (!arr.length) return null; const s = [...arr].sort((a, b) => a - b); return +s[Math.min(s.length - 1, Math.floor(q * s.length))].toFixed(1); };
results.timing.push({
  note: 'Headless Chromium, no CPU throttling; programmatic clicks. Not WebKitGTK and not native.',
  response: Object.fromEntries(overlays.map(([m]) => { const v = responses.filter(r => r.module === m).map(r => r.ms); return [m, { n: v.length, p95: p(v, .95), max: +Math.max(...v).toFixed(1) }]; })),
  eventTiming: await evaluate('__events.length ? { n: __events.length, max: Math.max(...__events.map(e => e.duration)) } : { n: 0 }'),
  longAnimationFrames: await evaluate('__loaf.length'),
  scrollFrames: { n: scrollFrames.length, p95: p(scrollFrames, .95), max: scrollFrames.length ? +Math.max(...scrollFrames).toFixed(1) : null, over20ms: scrollFrames.filter(t => t > 20).length },
});

console.log(JSON.stringify(results, null, 2));
ws.close();
