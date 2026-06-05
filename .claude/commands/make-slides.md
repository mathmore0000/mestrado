Read the document at the path provided in $ARGUMENTS.

Parse $ARGUMENTS for optional flags:
- `--vibe <name>` — one of: acadêmico, profissional, pitch, casual
- `--lang <code>` — one of: pt, en, mirror (mirror = match the document's language)

If either flag is missing, ask the user ONE single message with BOTH questions before doing anything else. Do not start reading or planning until you have both answers. Example ask:

> Before I start, two quick questions:
>
> **Vibe** — which feel should the slides have?
> - `acadêmico` — dark, data-dense, formal (default for papers/dissertations)
> - `profissional` — clean corporate feel, more whitespace, restrained
> - `pitch` — spacious, 1 big idea per slide, high contrast
> - `casual` — relaxed tone, warmer accents, emoji welcome
>
> **Language** — slide labels, section tags, and UI text should be in:
> - `pt` — Portuguese (PT-BR)
> - `en` — English
> - `mirror` — match the document's own language

Once you have both answers (or they were already in the flags), proceed.

---

## Step 1 — Understand the document

Read the full document. Identify:
- **Title, subtitle, authors, institution, date**
- **Narrative arc**: what problem does it solve, how, with what result?
- **Key sections** (introduction, methodology, results, conclusion, etc.)
- **Quantitative highlights**: metrics, tables, percentages, comparisons — these make the best slides
- **Structured data** worth recreating as CSS bar charts or tables

---

## Step 2 — Apply the vibe

The vibe changes **only** the CSS palette, spacing scale, and content tone. The layout system, all component class names, and the JS block are identical across all vibes.

### acadêmico
```css
--bg: #070d1e;
--sur: rgba(255,255,255,.05);
--bdr: rgba(255,255,255,.09);
--acc1: #3b82f6; --acc1l: #60a5fa;   /* primary accent = blue  */
--acc2: #f59e0b; --acc2l: #fbbf24;   /* secondary     = gold   */
--pos:  #10b981; --posl:  #34d399;   /* positive      = green  */
--neg:  #ef4444; --negl:  #f87171;   /* negative      = red    */
--tx: #f1f5f9; --mt: #94a3b8; --dm: #475569;
```
- Dense layouts, 2-column grids preferred, `.tbl` and `.frow` used freely
- No decorative emoji in slide body (cover and thank-you slide only)
- Bullet points can be long and precise
- Eyebrow tags are noun phrases: "Metodologia", "Resultados", "Análise"

### profissional
```css
--bg: #0d1a30;
--sur: rgba(255,255,255,.06);
--bdr: rgba(255,255,255,.1);
--acc1: #2563eb; --acc1l: #60a5fa;
--acc2: #0891b2; --acc2l: #22d3ee;   /* teal instead of gold */
--pos:  #059669; --posl:  #34d399;
--neg:  #dc2626; --negl:  #f87171;
--tx: #f8fafc; --mt: #94a3b8; --dm: #475569;
```
- Slightly more padding inside cards (`padding:26px`)
- Prefer `.steps` and `ul.lst` over dense tables
- No emoji anywhere
- Eyebrow tags are crisp action phrases: "Context", "Approach", "Outcome"

### pitch
```css
--bg: #06091a;
--sur: rgba(255,255,255,.04);
--bdr: rgba(255,255,255,.08);
--acc1: #f59e0b; --acc1l: #fbbf24;   /* gold is now primary  */
--acc2: #3b82f6; --acc2l: #60a5fa;   /* blue is secondary    */
--pos:  #10b981; --posl:  #34d399;
--bg: #ef4444; --negl: #f87171;
--tx: #ffffff;  --mt: #94a3b8; --dm: #475569;
```
- Maximum 2 key points per slide — if you have more, split into another slide
- Use `.fnd` cards and `.mstrip` heavily; avoid dense tables
- `h1` on cover can be larger: `font-size: clamp(2.4rem, 4.5vw, 4rem)`
- Headlines should be bold statements, not neutral labels
- Emoji welcome on cover and section-opener slides

### casual
```css
--bg: #080f1e;
--sur: rgba(255,255,255,.05);
--bdr: rgba(255,255,255,.09);
--acc1: #10b981; --acc1l: #34d399;   /* green is now primary */
--acc2: #f59e0b; --acc2l: #fbbf24;
--pos:  #3b82f6; --posl:  #60a5fa;
--neg:  #ef4444; --negl:  #f87171;
--tx: #f1f5f9; --mt: #94a3b8; --dm: #475569;
```
- Short, punchy bullet points — cut anything that takes more than one line
- Emoji welcome throughout
- Prefer `.steps` and `ul.lst` over tables
- Eyebrow tags can be casual: "Wait, why?", "The data", "What we found"
- Cover subtitle can be a one-liner quip if the content allows

**In all vibes:** replace `var(--bl)` / `var(--bll)` in the CSS reference below with `var(--acc1)` / `var(--acc1l)`, and `var(--gd)` / `var(--gdl)` with `var(--acc2)` / `var(--acc2l)`. The component markup stays identical.

---

## Step 3 — Apply the language

Use the chosen language for all UI chrome (eyebrow labels, section tags, nav hint, button labels) and for any copy you write (bullet points, card titles, insight callouts). Do not translate content quoted directly from the source document.

**PT-BR chrome reference:**
- Nav hint: `← → navegar · Space avançar`
- Final slide heading: `Obrigado!` / `Obrigada!` (match document authors if determinable)
- Positive label: `✓` · Warning label: `⚠️`

**EN chrome reference:**
- Nav hint: `← → navigate · Space advance`
- Final slide heading: `Thank you!`
- Positive label: `✓` · Warning label: `⚠️`

---

## Step 4 — Plan the slides

Design 9–13 slides that tell a coherent story. Typical structure:

| # | Purpose |
|---|---------|
| 1 | Cover — title, authors, institution |
| 2 | Context / Motivation |
| 3 | Data / Problem setup |
| 4–N-3 | Methodology / approaches / experiments |
| N-2 | Results — quantitative comparison |
| N-1 | Key findings — the "so what?" |
| N-0 | Conclusion + future work |
| Last | Thank you + contacts + key numbers |

Adapt freely. A short report may need 8; a complex thesis 14. Every slide must earn its place.

---

## Step 5 — Build the HTML

Save the file as `presentation.html` in the same directory as the source document. Overwrite if it exists.

### Complete CSS block (paste verbatim, substituting the vibe palette at the top)

```css
*,*::before,*::after{box-sizing:border-box;margin:0;padding:0}
:root{
  /* ← PASTE VIBE PALETTE HERE */
  --bl:var(--acc1);--bll:var(--acc1l);
  --gd:var(--acc2);--gdl:var(--acc2l);
  --gn:var(--pos); --gnl:var(--posl);
  --rd:var(--neg); --rdl:var(--negl);
}
html,body{width:100%;height:100%;overflow:hidden;background:var(--bg);
  font-family:-apple-system,'Segoe UI',system-ui,sans-serif;color:var(--tx);
  -webkit-font-smoothing:antialiased}
#deck{position:relative;width:100%;height:100%;overflow:hidden}
.slide{position:absolute;inset:0;display:flex;flex-direction:column;
  justify-content:center;align-items:center;padding:48px 68px 80px;
  opacity:0;pointer-events:none;will-change:transform,opacity}
.slide.on{opacity:1;pointer-events:auto}
#prog{position:fixed;top:0;left:0;right:0;height:3px;background:rgba(255,255,255,.05);z-index:999}
#progf{height:100%;background:linear-gradient(90deg,var(--bl),var(--gd));
  border-radius:0 2px 2px 0;transition:width .4s ease}
#nav{position:fixed;bottom:22px;left:50%;transform:translateX(-50%);
  display:flex;align-items:center;gap:13px;z-index:999;
  background:rgba(7,13,30,.88);backdrop-filter:blur(16px);-webkit-backdrop-filter:blur(16px);
  border:1px solid var(--bdr);border-radius:100px;padding:7px 16px}
.nb{width:30px;height:30px;border-radius:50%;border:1px solid var(--bdr);
  background:var(--sur);color:var(--tx);cursor:pointer;
  display:flex;align-items:center;justify-content:center;font-size:17px;line-height:1;
  transition:background .2s,border-color .2s,transform .1s;user-select:none}
.nb:hover:not(:disabled){background:var(--bl);border-color:var(--bl);transform:scale(1.06)}
.nb:active:not(:disabled){transform:scale(.94)}
.nb:disabled{opacity:.22;cursor:default}
#dots{display:flex;gap:5px;align-items:center}
.dot{width:6px;height:6px;border-radius:3px;background:var(--dm);cursor:pointer;transition:all .28s}
.dot.on{background:var(--bll);width:17px}
#ctr{font-size:11px;color:var(--mt);font-variant-numeric:tabular-nums;min-width:34px}
#hint{position:fixed;top:14px;right:18px;font-size:11px;color:var(--dm);
  display:flex;gap:5px;align-items:center;pointer-events:none}
.key{border:1px solid var(--dm);border-radius:4px;padding:1px 5px;font-size:10px}
.eye{font-size:11px;font-weight:600;letter-spacing:.14em;text-transform:uppercase;
  color:var(--bll);margin-bottom:9px}
h1{font-size:clamp(1.9rem,3.6vw,3.2rem);font-weight:800;line-height:1.12}
h2{font-size:clamp(1.3rem,2.6vw,2.1rem);font-weight:700;line-height:1.2}
h3{font-size:clamp(.9rem,1.5vw,1.2rem);font-weight:600;line-height:1.35}
p{font-size:clamp(.78rem,1vw,.92rem);line-height:1.7;color:var(--mt)}
.grad{background:linear-gradient(135deg,var(--bll),var(--gdl));
  -webkit-background-clip:text;-webkit-text-fill-color:transparent;background-clip:text}
.rule{width:50px;height:3px;background:linear-gradient(90deg,var(--bl),transparent);
  border-radius:2px;margin:11px 0 20px}
.w{width:100%;max-width:1060px}
.g2{display:grid;grid-template-columns:1fr 1fr;gap:20px;width:100%}
.g3{display:grid;grid-template-columns:repeat(3,1fr);gap:16px;width:100%}
.card{background:var(--sur);border:1px solid var(--bdr);border-radius:13px;padding:22px}
.pill{display:inline-flex;align-items:center;padding:3px 9px;border-radius:20px;
  font-size:11px;font-weight:600;margin-bottom:9px}
.pb{background:rgba(59,130,246,.15);color:var(--bll);border:1px solid rgba(59,130,246,.3)}
.pg{background:rgba(16,185,129,.15);color:var(--gnl);border:1px solid rgba(16,185,129,.3)}
.po{background:rgba(245,158,11,.15);color:var(--gdl);border:1px solid rgba(245,158,11,.3)}
.pr{background:rgba(239,68,68,.15);color:var(--rdl);border:1px solid rgba(239,68,68,.3)}
ul.lst{list-style:none;display:flex;flex-direction:column;gap:9px}
ul.lst li{display:flex;gap:8px;font-size:clamp(.76rem,.98vw,.86rem);color:var(--mt);line-height:1.55}
ul.lst li::before{content:'→';color:var(--bll);font-weight:700;flex-shrink:0}
.steps{display:flex;flex-direction:column;gap:13px;width:100%}
.step{display:flex;gap:13px;align-items:flex-start}
.sn{width:25px;height:25px;border-radius:50%;background:var(--bl);color:#fff;
  font-size:11px;font-weight:700;display:flex;align-items:center;justify-content:center;
  flex-shrink:0;margin-top:2px}
.sn.g{background:var(--gn)}
.sb h4{font-size:clamp(.8rem,1.05vw,.9rem);font-weight:600;margin-bottom:2px}
.sb p{font-size:clamp(.72rem,.92vw,.8rem)}
.blbl{font-size:11px;color:var(--mt);margin-bottom:6px}
.brow{display:flex;align-items:center;gap:9px;margin-bottom:6px}
.btag{font-size:11px;color:var(--mt);width:54px;flex-shrink:0}
.btrk{flex:1;height:27px;background:rgba(255,255,255,.04);border-radius:6px;overflow:hidden}
.bf{height:100%;border-radius:6px;display:flex;align-items:center;padding-left:8px;
  font-size:11px;font-weight:700;color:#fff}
.bfb{background:linear-gradient(90deg,#3b82f6,#60a5fa)}
.bfg{background:linear-gradient(90deg,#10b981,#34d399)}
.bfo{background:linear-gradient(90deg,#f59e0b,#fbbf24)}
.bfr{background:linear-gradient(90deg,#ef4444,#f87171)}
.frow{display:flex;align-items:center;gap:9px;margin-bottom:8px}
.fnm{font-size:11px;color:var(--mt);width:138px;flex-shrink:0}
.fbar{flex:1;height:6px;background:rgba(255,255,255,.05);border-radius:3px;overflow:hidden}
.fbf{height:100%;border-radius:3px;background:linear-gradient(90deg,var(--bl),var(--bll))}
.tbl{width:100%;border-collapse:collapse}
.tbl th{padding:9px 14px;text-align:left;font-size:11px;text-transform:uppercase;
  letter-spacing:.07em;color:var(--mt);border-bottom:1px solid var(--bdr)}
.tbl td{padding:12px 14px;font-size:13px;border-bottom:1px solid rgba(255,255,255,.04)}
.tbl tr:last-child td{border-bottom:none}
.win{color:var(--gnl);font-weight:700}.lose{color:var(--rdl)}
.mstrip{display:flex;gap:28px;align-items:center;justify-content:center;flex-wrap:wrap}
.mitem{text-align:center}
.mval{font-size:clamp(1.7rem,2.8vw,2.4rem);font-weight:800;line-height:1;margin-bottom:2px}
.mlbl{font-size:10px;color:var(--mt);text-transform:uppercase;letter-spacing:.07em}
.msep{width:1px;height:48px;background:var(--bdr)}
.fnd{background:linear-gradient(135deg,rgba(59,130,246,.12),rgba(16,185,129,.06));
  border:1px solid rgba(59,130,246,.22);border-radius:13px;padding:20px;text-align:center}
.fnum{font-size:clamp(2rem,3.2vw,3rem);font-weight:900;
  background:linear-gradient(135deg,var(--bll),var(--gnl));
  -webkit-background-clip:text;-webkit-text-fill-color:transparent;
  background-clip:text;line-height:1;margin-bottom:5px}
.fdsc{font-size:11px;color:var(--mt);line-height:1.5}
.pbox{background:#050c1c;border:1px solid rgba(59,130,246,.2);border-radius:9px;
  padding:16px;font-family:'SF Mono',Consolas,monospace;
  font-size:clamp(9px,.95vw,11px);color:#7dd3fc;line-height:1.75;
  white-space:pre;overflow-x:auto}
.mbox{border-radius:13px;padding:18px;border:1px solid var(--bdr)}
.mbox.a{background:linear-gradient(135deg,rgba(59,130,246,.09),rgba(16,185,129,.04));
  border-color:rgba(59,130,246,.28)}
.mbox.b{background:linear-gradient(135deg,rgba(245,158,11,.09),rgba(239,68,68,.04));
  border-color:rgba(245,158,11,.28)}
.cmeta{display:flex;gap:18px;align-items:center;justify-content:center;flex-wrap:wrap;margin-top:26px}
.cm{font-size:13px;color:var(--mt)}
.cdot{width:3px;height:3px;border-radius:50%;background:var(--dm)}
```

### Nav HTML (verbatim)

```html
<div id="hint"><span class="key">←</span><span class="key">→</span>&nbsp;[nav hint per lang]</div>
<nav id="nav">
  <button class="nb" id="bp" onclick="go(-1)" disabled>&#8249;</button>
  <div id="dots"></div>
  <span id="ctr">1 / N</span>
  <button class="nb" id="bn" onclick="go(1)">&#8250;</button>
</nav>
```

### JS block (verbatim — replace `__N__` with actual slide count)

```js
const N=__N__;let cur=0,busy=false;
const dotsEl=document.getElementById('dots');
for(let i=0;i<N;i++){
  const d=document.createElement('div');
  d.className='dot'+(i===0?' on':'');
  d.onclick=()=>goTo(i);
  dotsEl.appendChild(d);
}
function syncUI(){
  document.getElementById('ctr').textContent=`${cur+1} / ${N}`;
  document.getElementById('bp').disabled=cur===0;
  document.getElementById('bn').disabled=cur===N-1;
  document.querySelectorAll('.dot').forEach((d,i)=>d.classList.toggle('on',i===cur));
  document.getElementById('progf').style.width=(cur/(N-1)*100)+'%';
}
function goTo(idx){
  if(busy||idx===cur||idx<0||idx>=N)return;
  busy=true;
  const dir=idx>cur?1:-1;
  const slides=document.querySelectorAll('.slide');
  const from=slides[cur],to=slides[idx];
  to.style.cssText='transition:none;transform:translateX('+dir*100+'%);opacity:1;pointer-events:none';
  requestAnimationFrame(()=>requestAnimationFrame(()=>{
    const T='transform .46s cubic-bezier(.4,0,.2,1),opacity .46s ease';
    from.style.cssText='transition:'+T+';transform:translateX('+(-dir*100)+'%);opacity:0';
    to.style.cssText='transition:'+T+';transform:translateX(0);opacity:1;pointer-events:none';
    setTimeout(()=>{
      from.classList.remove('on');from.style.cssText='';
      to.classList.add('on');to.style.cssText='';
      cur=idx;syncUI();busy=false;
    },470);
  }));
}
function go(d){goTo(cur+d);}
document.addEventListener('keydown',e=>{
  if(['ArrowRight','ArrowDown',' '].includes(e.key)){e.preventDefault();go(1);}
  else if(['ArrowLeft','ArrowUp'].includes(e.key)){e.preventDefault();go(-1);}
});
let tx=0;
document.addEventListener('touchstart',e=>{tx=e.touches[0].clientX;},{passive:true});
document.addEventListener('touchend',e=>{const dx=e.changedTouches[0].clientX-tx;if(Math.abs(dx)>50)go(dx<0?1:-1);});
syncUI();
```

### Cover slide special treatment

```html
<div class="slide on" id="s1" style="text-align:center;
  background:
    radial-gradient(ellipse 70% 60% at 75% 25%, rgba(59,130,246,.12) 0%, transparent 55%),
    radial-gradient(ellipse 50% 50% at 20% 80%, rgba(245,158,11,.07) 0%, transparent 50%),
    var(--bg)">
```

For **pitch** vibe, increase the radial opacity to `.18` and `.11` for more drama.

### Thank-you slide special treatment

```html
<div class="slide" id="sN" style="text-align:center;
  background:radial-gradient(ellipse 70% 60% at 50% 50%, rgba(59,130,246,.13) 0%, transparent 65%), var(--bg)">
```

---

## Step 6 — Content guidelines

- Every slide needs a `.eye` eyebrow label and a `.rule` after `h2`
- Wrap all slide content in `<div class="w">`
- Pick the right component for the content type:
  - Rankings / importance → `.frow` feature bars
  - A vs B comparison → `.brow/.btrk/.bf` bar chart
  - Structured comparison → `.tbl`
  - Top 3–4 numbers → `.mstrip`
  - 3 big findings → `.g3` + `.fnd`
  - Sequential steps → `.steps`
  - Unordered points → `ul.lst`
  - Code / prompts / formulas → `.pbox`
  - Two named approaches → `.mbox.a` and `.mbox.b`
- The most important insight on any slide goes in a coloured callout card: `border-left:3px solid var(--bl/gd/gn)` + matching `background:rgba(...,.04)`
- `.grad` text only on the cover slide title — nowhere else
- Colour consistency: acc1 = primary approach/model, acc2 = comparison, green = positive, red = negative

---

## Step 7 — Output

Write the complete HTML file to `presentation.html` next to the source document. Then report:
- Full path to the saved file
- Slide count
- Vibe and language used
- One sentence on the narrative logic chosen
