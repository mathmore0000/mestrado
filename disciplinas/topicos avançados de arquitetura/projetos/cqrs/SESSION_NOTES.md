# Session Notes — CQRS Presentation

## What we're building

A masters-level presentation on CQRS for the course "Tópicos Avançados de Arquitetura" at Universidade Presbiteriana Mackenzie. Output: `cqrs.md` → `/make-slides --vibe acadêmico --lang pt` → `presentation.html`.

## Narrative approach

Storytelling hook: Jorge (a developer who vibecoded an app) builds a social feed. Backend scales fine — DB doesn't. Hook is economic: latency = revenue loss (Linden 2006, Mayer 2006). Told in Atos (I–IV). CQRS is introduced as the architectural response.

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

## Step animation system (✓ implemented — S4, S6, S11)

Three slides use step-by-step reveal controlled by `→`. The system is extensible.

**CSS:**
```css
.step-anim{opacity:0;transform:translateY(18px);transition:opacity .5s ease,transform .5s ease}
.step-anim.visible{opacity:1;transform:translateY(0)}
```

**JS pattern — generic item list (S4 Journey):**
```javascript
const JOURNEY_IDX = 3;  // 0-based slide index
const journeySteps = Array.from(document.querySelectorAll('#s4 .step-anim'));
let journeyStep = 0;
function stepJourney() {
  if (journeyStep < journeySteps.length) {
    journeySteps[journeyStep].classList.add('visible');
    journeyStep++;
  }
}
function resetJourney() {
  journeyStep = 0;
  journeySteps.forEach(s => s.classList.remove('visible'));
}
```

**JS pattern — grouped elements (S6 Metrics):**
HTML elements get group classes (`s6g1`, `s6g2`, `s6g3`, `s6g4`, `s6card`). JS collects them by group:
```javascript
const metricsGroups = [
  Array.from(document.querySelectorAll('#s6 .s6g1')),
  ...
  [document.querySelector('#s6 .s6card')],
];
// stepMetrics() reveals metricsGroups[metricsStep] and increments
// resetMetrics() removes .visible from all groups
```

**`goForward()` dispatches to the right handler:**
```javascript
function goForward() {
  if (cur === KAFKA_IDX && kafkaStep < 5) stepKafka();
  else if (cur === JOURNEY_IDX && journeyStep < journeySteps.length) stepJourney();
  else if (cur === METRICS_IDX && metricsStep < metricsGroups.length) stepMetrics();
  else go(1);
}
```

**Reset on slide exit** — inside `goTo()`'s setTimeout, before `cur = idx`:
```javascript
if (cur === KAFKA_IDX) resetKafka();
if (cur === JOURNEY_IDX) resetJourney();
if (cur === METRICS_IDX) resetMetrics();
```

---

## Animation — Kafka flow slide (✓ implemented)

Slide 11 has a step-by-step animated flow. Each `→` press advances one step; after the last step, `→` navigates to slide 12. Going back resets the animation.

**Steps:**
1. Packet appears at **Write Service**
2. Slides to **Kafka**
3. Slides to **Consumer**
4. Slides to **Read DB**
5. `likes_count: 42` fades out → `43` fades in (green)
6. → navigates to S12

**Implementation pattern — reusable for any step animation:**

```html
<!-- Track + dot in HTML -->
<div class="kflow-track">
  <div class="kflow-line"></div>
  <div class="kflow-dot"></div>  <!-- starts opacity:0 -->
</div>
<!-- Counter with two overlapping spans -->
<span class="kflow-n42">42</span><span class="kflow-n43">43</span>
```

```css
/* CSS: transitions only, no keyframes */
.kflow-dot { transition: left .55s cubic-bezier(.4,0,.2,1), opacity .3s ease; opacity: 0; }
.kflow-n42 { transition: opacity .3s ease; }
.kflow-n43 { opacity: 0; transition: opacity .3s ease; }
```

```javascript
// JS: step counter, goForward() intercepts navigation
const SLIDE_IDX = 10;          // 0-based index of the target slide
const positions = ['0%','33%','66%','100%'];
let step = 0;

function stepAnim() {
  step++;
  if (step === 1) { dot.style.opacity = '1'; dot.style.left = '0%'; }
  else if (step <= 4) { dot.style.left = positions[step - 1]; }
  else if (step === 5) { n42.style.opacity = '0'; n43.style.opacity = '1'; }
}

function resetAnim() {
  step = 0;
  dot.style.transition = 'none'; dot.style.opacity = '0'; dot.style.left = '0%';
  n42.style.opacity = '1'; n43.style.opacity = '0';
  requestAnimationFrame(() => dot.style.transition = '');
}

// In goTo's setTimeout (before cur = idx):
if (cur === SLIDE_IDX) resetAnim();

// Replace go(1) with:
function goForward() {
  if (cur === SLIDE_IDX && step < 5) { stepAnim(); } else { go(1); }
}
```

**Key insight:** `resetAnim()` sets `transition: none` before resetting position so the dot doesn't visibly slide back to start when navigating away. `requestAnimationFrame` re-enables the transition on the next paint.

## Files

| File | Purpose |
|------|---------|
| `cqrs.md` | Source document com todas as referências verificadas |
| `presentation.html` | 16 slides, tema Mackenzie PPGA, animações reveal + step |
| `SESSION_NOTES.md` | Este arquivo — contexto técnico para sessões futuras |
| `HANDOFF.md` | Estado do projeto, todo list, decisões tomadas |
| `assets/` | Imagens: mvp-jorge.png, arquitetura-sem-cqrs.png, arquitetura-com-cqrs.png |

## How to resume

1. Read `HANDOFF.md` for current state and todo list.
2. Read `SESSION_NOTES.md` for technical context.
3. Open `presentation.html` in browser to see current state.
