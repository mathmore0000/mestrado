# Session Notes — CQRS Presentation

## What we're building

A masters-level presentation on CQRS for the course "Tópicos Avançados de Arquitetura" at Universidade Presbiteriana Mackenzie. Output: `cqrs.md` → `/make-slides --vibe acadêmico --lang pt` → `presentation.html`.

## Narrative approach

Storytelling hook: Jorge (a developer who vibecoded an app) builds a social feed. Backend scales fine — DB doesn't. Hook is economic: latency = revenue loss (Linden 2006, Mayer 2006). CQRS is introduced as the architectural response. (Note: "Atos" framing was removed — narrative flows as a continuous story.)

## Content priorities

1. **Domain events + message queue (Kafka)** — deep dive. Primary sync mechanism.
2. **Event Sourcing** — touched as the purist CQRS companion, not the main focus.
3. **Denormalization on the read side** — the tangible payoff.
4. **Eventual consistency** — addressed head-on as the honest tradeoff.

## Hard constraints

- **Every claim must have a reference.** No standalone assertions. Non-negotiable for academic context.
- Presentation is in **Portuguese (PT-BR)**, Brazilian crowd at Mackenzie.
- Character name is **Jorge** (not George).

## Current state of `cqrs.md`

**Document is complete.** All sections written, all references in place. Ready to run `/make-slides`.

- [x] Contexto e Motivação — Jorge, Atos I–IV, user quotes, debug metrics, latency/money data
- [x] O Que é CQRS — definition, CQS origin, timeline table (Meyer/Dahan/Young/Fowler)
- [x] O Modelo de Escrita — normalized schema, ACID framing
- [x] O Modelo de Leitura — denormalized feed_items, before/after query, BASE framing
- [x] Sincronização / Kafka — domain events flow, Kafka log, at-least-once vs exactly-once, idempotência
- [x] Consistência Eventual — CAP theorem (Gilbert & Lynch 2002), ACID/BASE table, Instagram analogy
- [x] Event Sourcing — event store example with Jorge's events, replay concept
- [x] Adoção na Indústria — Amazon DynamoDB, Netflix, Uber with full references
- [x] Quando Usar — criteria + Jorge's case + counter-case
- [x] Conclusão — written

## Decisions made

- Character: **Jorge** — social feed app, communities of niche interests
- Crisis trigger: 10k active users, 3 months in, errors waking Jorge up at night
- Audience: mixed (backend, AI-focused, product-oriented) at Mackenzie
- Duration: 30 min with Lucas Aguiar + 5 min Q&A
- Split between presenters: TBD (to decide with Lucas)

## Step animation system (✓ generic — all content slides)

**Every content slide** uses `→` / `←` to step through groups. The system is data-driven.

### CSS
```css
.step-anim { opacity:0; transform:translateY(18px); transition:opacity .5s ease,transform .5s ease }
.step-anim.visible { opacity:1; transform:translateY(0) }
```

### HTML — markup elements with data-group
```html
<p class="step-anim" data-group="1">First thing to appear</p>
<div class="step-anim" data-group="2">Second group</div>
<!-- Elements sharing the same data-group number appear together -->
<p class="step-anim" data-group="2">Also appears with group 2</p>
```

### JS — generic engine
```javascript
function buildSteps(id) {
  const map = {};
  document.querySelectorAll('#' + id + ' [data-group]').forEach(el => {
    const g = el.dataset.group;
    (map[g] = map[g] || []).push(el);
  });
  return { groups: Object.keys(map).sort((a,b) => +a-+b).map(k => map[k]), step: 0 };
}

// Register slides: index → slide id
const STEP_IDS = {
  1:'s2', 2:'s3', 3:'smono', 4:'s4', 5:'s5', 6:'s6',
  7:'s7', 8:'s8', 9:'s9', 10:'s10', 12:'s12', 13:'s13', 14:'s14', 15:'s15'
};
const SS = {};
Object.entries(STEP_IDS).forEach(([idx, id]) => { SS[+idx] = buildSteps(id); });

// Forward / backward / reset
function doFwd(i) { const s=SS[i]; s.groups[s.step].forEach(el=>el.classList.add('visible')); s.step++; }
function doBk(i)  { const s=SS[i]; s.step--; s.groups[s.step].forEach(el=>el.classList.remove('visible')); }
function resetSteps(i) { if(!SS[i])return; SS[i].step=0; SS[i].groups.flat().forEach(el=>el.classList.remove('visible')); }

function goForward()  { if(cur===KAFKA_IDX&&kafkaStep<5) stepKafka(); else if(canFwd(cur)) doFwd(cur); else go(1); }
function goBackward() { if(cur===KAFKA_IDX&&kafkaStep>0) stepKafkaBack(); else if(canBk(cur)) doBk(cur); else go(-1); }
```

Reset on slide exit inside `goTo()`'s setTimeout:
```javascript
if (cur === KAFKA_IDX) resetKafka();
resetSteps(cur);   // covers all generic slides
```

---

## Kafka flow slide — S11 (index 11, KAFKA_IDX=11)

Special handler: dot animation + left-side steps move in sync. `←` reverses both.

**Steps (5 total, then → navigates):**
1. Dot appears at Write Service + left step 1 reveals
2. Dot → Kafka + left step 2 reveals
3. Dot → Consumer + left step 3 reveals
4. Dot → Read DB + left step 4 reveals
5. Counter 42 → 43 + left step 5 reveals
6. → navigates to S12

**Left steps:** `#s11 .step-anim` (no `data-group` — handled exclusively by Kafka JS).

```javascript
const KAFKA_IDX = 11;
const kLeftSteps = Array.from(document.querySelectorAll('#s11 .step-anim'));
let kafkaStep = 0;

function stepKafka() {
  kafkaStep++;
  if (kafkaStep===1) { kDot.style.opacity='1'; kDot.style.left='0%'; }
  else if (kafkaStep<=4) { kDot.style.left = kPositions[kafkaStep-1]; }
  else if (kafkaStep===5) { kN42.style.opacity='0'; kN43.style.opacity='1'; }
  if (kLeftSteps[kafkaStep-1]) kLeftSteps[kafkaStep-1].classList.add('visible');
}

function stepKafkaBack() {
  if (kLeftSteps[kafkaStep-1]) kLeftSteps[kafkaStep-1].classList.remove('visible');
  kafkaStep--;
  if (kafkaStep===0) { kDot.style.opacity='0'; }
  else if (kafkaStep<4) { kDot.style.left = kPositions[kafkaStep-1]; }
  else if (kafkaStep===4) { kN42.style.opacity='1'; kN43.style.opacity='0'; }
}

function resetKafka() {
  kafkaStep=0;
  kDot.style.transition='none'; kDot.style.opacity='0'; kDot.style.left='0%';
  kN42.style.opacity='1'; kN43.style.opacity='0';
  kLeftSteps.forEach(s => s.classList.remove('visible'));
  requestAnimationFrame(() => kDot.style.transition='');
}
```

**Key insight:** `resetKafka()` sets `transition:none` before resetting so the dot doesn't animate back on slide exit. `requestAnimationFrame` re-enables transition on next paint.

## Files

| File | Purpose |
|------|---------|
| `cqrs.md` | Source document — narrativa completa, 24 referências verificadas |
| `presentation.html` | 18 slides, tema Mackenzie PPGA, step animation em todos os slides de conteúdo |
| `SESSION_NOTES.md` | Este arquivo — contexto técnico para sessões futuras |
| `HANDOFF.md` | Estado do projeto, todo list, decisões tomadas |
| `assets/` | mvp-jorge.png, arquitetura-sem-cqrs.png, arquitetura-com-cqrs.png |

## Slide index map (0-based)

| Idx | ID | Slide |
|-----|----|-------|
| 0 | s1 | Capa |
| 1 | s2 | Conheçam Jorge |
| 2 | s3 | Como vocês construiriam? |
| 3 | smono | Arquitetura Monolítica Clássica |
| 4 | s4 | A Jornada de Jorge |
| 5 | s5 | A Crise |
| 6 | s6 | Arquitetura Vira Prejuízo |
| 7 | s7 | O Padrão CQRS |
| 8 | s8 | Arquitetura CQRS |
| 9 | s9 | Separação de Modelos 1/2 |
| 10 | s10 | Separação de Modelos 2/2 |
| **11** | **s11** | **Domain Events + Kafka ← KAFKA_IDX** |
| 12 | s12 | Apache Kafka |
| 13 | s13 | Consistência Eventual |
| 14 | s14 | Event Sourcing |
| 15 | s15 | Adoção na Indústria |
| 16 | srefs | Referências |
| 17 | s16 | Obrigado |

## Mouse navigation (✓ implemented)

```javascript
// Left click anywhere on slide → forward; right click → backward
document.getElementById('deck').addEventListener('click', () => goForward());
document.getElementById('deck').addEventListener('contextmenu', e => { e.preventDefault(); goBackward(); });
```

`#nav` is outside `#deck` in the DOM — clicks on buttons/dots don't bubble through and are unaffected.

---

## How to resume

1. Read `HANDOFF.md` for current state and todo list.
2. Read `SESSION_NOTES.md` for technical context.
3. Open `presentation.html` in browser to see current state.
