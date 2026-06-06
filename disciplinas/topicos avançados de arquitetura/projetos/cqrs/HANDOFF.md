# Handoff — Apresentação CQRS
**Disciplina:** Tópicos Avançados de Arquitetura — Universidade Presbiteriana Mackenzie  
**Apresentadores:** Matheus Moreira · Lucas Aguiar  
**Formato:** 30 min de apresentação + 5 min de Q&A  
**Última atualização:** 2026-06-06 (sessão 3)

---

## O que foi produzido

| Arquivo | Descrição |
|---------|-----------|
| `cqrs.md` | Documento-fonte — narrativa completa, referências acadêmicas verificadas |
| `presentation.html` | 17 slides, tema Mackenzie PPGA (light, navy/red), responsivo com `clamp()` |
| `SESSION_NOTES.md` | Notas técnicas para continuação via Claude Code |
| `HANDOFF.md` | Este arquivo |

Abra `presentation.html` no browser. Navegação: ← → / Space / clique nos dots.

---

## Estado atual dos slides

| # | ID | Slide | Status |
|---|-----|-------|--------|
| 1 | s1 | Capa | ✓ Auto-reveal (cover slide) |
| 2 | s2 | Conheçam Jorge | ✓ **Step** — card+img / navy card / quote |
| 3 | s3 | Como vocês construiriam? | ✓ **Step** — discussion card / warn card · coluna única |
| 4 | smono | Arquitetura Monolítica Clássica | ✓ **NOVO** slide dedicado ao diagrama monolito |
| 5 | s4 | A Jornada de Jorge | ✓ **Step** — 4 passos individuais |
| 6 | s5 | A Crise — diagnóstico + query culpada | ✓ **Step** — 4 grupos sincronizados (L/R) |
| 7 | s6 | Arquitetura Vira Prejuízo — latência/receita | ✓ **Step** — 4 métricas + card |
| 8 | s7 | O Padrão CQRS — definição + timeline | ✓ **Step** — 3 grupos (L/R sincronizados) |
| 9 | s8 | Arquitetura CQRS — diagrama geral | ✓ **Step** — diagrama / pills |
| 10 | s9 | Modelos Especializados · 1/2 | ✓ **Step** — 4 layers (L/R sincronizados) |
| 11 | s10 | A Query Antes e Depois do CQRS · 2/2 | ✓ **Step** — 4 layers (L/R sincronizados) |
| 12 | s11 | Domain Events + Kafka + Dual Write | ✓ **Step Kafka** — dot + left steps sincronizados; ← reverte |
| 13 | s12 | Apache Kafka — propriedades + garantias | ✓ **Step** — 3 grupos (L/R sincronizados) |
| 14 | s13 | Consistência Eventual — CAP + ACID/BASE | ✓ **Step** — 3 grupos (L/R sincronizados) |
| 15 | s14 | Event Sourcing | ✓ **Step** — 3 grupos (L/R sincronizados) |
| 16 | s15 | Adoção na Indústria — Amazon/Netflix/Uber | ✓ **Step** — disclaimer + 3 cards individualmente |
| 17 | s16 | Obrigado | ✓ Auto-reveal (closing slide) |

---

## O que ainda falta

### 1. Verificações manuais antes da apresentação

> ⚠ **VERIFICAÇÃO MANUAL NECESSÁRIA @lucas**
>
> **Citação Netflix:** Izrailevsky & Meshenberg (2016) é sobre migração para a nuvem, não sobre arquitetura CQRS/Cassandra. Buscar post mais específico no Netflix Tech Blog ou substituir em cqrs.md e slide 15.
>
> **Autor Uber:** `Crunkilton, B. et al. (2016)` — confirmar autoria no Uber Engineering Blog. URL: https://www.uber.com/en-BR/blog/engineering/

### 2. Oportunidades opcionais (check-doc 2ª rodada)

> 💡 **SUGESTÕES — baixo risco, podem ficar para outra sessão**
>
> **Snapshots em Event Sourcing:** mencionar que replay em históricos longos exige snapshots periódicos (Young, 2010 já cobre isso). Uma frase no slide 14 eliminaria a Q&A mais provável.
>
> **SLA da janela de inconsistência:** o exemplo de 200ms para `likes_count` é ilustrativo — uma frase sobre latência de propagação típica do Kafka (single-digit ms a poucos segundos) tornaria o argumento mais defensável.
>
> **Fonte Lerner (2014):** Gartner Research Note atrás de paywall. Se questionado, mencionar que o número é amplamente reproduzido por Atlassian e IBM em publicações abertas.

### 3. Divisão da apresentação entre os apresentadores

> ⚠ **DECISÃO PENDENTE @lucas + @matheus**
>
> Agora são 16 slides. Sugestões:
> - **Opção A:** Lucas S1–S7 (contexto + problema + discussão + definição), Matheus S8–S16 (arquitetura + técnico + indústria)
> - **Opção B:** Lucas S1–S6, Matheus S7–S16

---

## O que foi feito — sessão 1 (2026-06-06)

- ✓ **TV Full HD layout** — `html { font-size: clamp(15px,1.3vw,22px) }` + caps tipográficas ajustadas
- ✓ **Diagramas de arquitetura** — 3 imagens em `assets/`; slides 2, 3, 8 (total: 16 slides)
- ✓ **Slide reorder** — "Como construiriam?" inserido como S3
- ✓ **Métricas impacto financeiro** — Gartner $5,6k/min + ratio 89%/11%
- ✓ **Animações reveal** — todas auditadas
- ✓ **Animação Kafka step-by-step** — S11: cada → avança um nó, 5º → troca 42→43, 6º → navega
- ✓ **Disclaimer de IA** — S16
- ✓ **Remove "Atos"**, corrige Dynamo/DynamoDB, remove Brewer (2000) órfão, substitui Stonebraker

## O que foi feito — sessão 2 (2026-06-06)

- ✓ **Título/subtítulo S1** — "Computação aplicada em Arquitetura de Software" + subtítulo completo; CQRS visível como linha de rodapé
- ✓ **Step animation S4** — 4 passos da Jornada de Jorge revelados um a um via →; 5º → navega
- ✓ **Step animation S6** — 4 métricas de impacto financeiro (com separadores) + card revelados em grupo via →; 6º → navega
- ✓ **Título + breadcrumb S9** — "Separação de Modelos · 1 / 2 · Esquemas Especializados por Responsabilidade"
- ✓ **Título + breadcrumb S10** — "Separação de Modelos · 2 / 2 · A Query Antes e Depois do CQRS"
- ✓ **check-doc 3ª rodada** — sem novos blockers; mesmas pendências manuais de antes

## O que foi feito — sessão 3 (2026-06-06)

- ✓ **goBackward()** — ← reverte step animations (doBk/stepKafkaBack) antes de navegar; botão esquerdo e ArrowLeft atualizados
- ✓ **Step animation em todos os slides de conteúdo** — sistema genérico data-group + buildSteps() substitui handlers específicos; S2, S3, smono, S4, S5, S6, S7, S8, S9, S10, S12, S13, S14, S15 cobertos
- ✓ **Slide smono** — "Arquitetura Monolítica Clássica" como slide dedicado full-width (espelho de S8/CQRS); S3 virou coluna única de discussão
- ✓ **Kafka sync** — 5 left steps de S11 agora revelam em sincronia com o dot; ← reverte dot + left step
- ✓ **CSS fix S12** — `ul.lst li code{background:none;padding:0}` remove o fundo azul de PostCurtido, post_id, likes_count
- ✓ **N=17**, KAFKA_IDX=11; todos os índices JS atualizados

---

## Decisões tomadas (não reverter sem discussão)

- **Personagem:** Jorge (não George — apresentação em PT-BR)
- **App:** rede social de feed com comunidades de nicho
- **Tema:** Mackenzie PPGA — light, navy `#0f2d59`, red `#da291c`, paper `#f7f8fa`
- **Layout:** responsivo com `clamp()` — não fixed stage (evita espaço em branco)
- **Referências chave corrigidas:**
  - BASE → Pritchett (2008), não Fox & Brewer
  - CAP → Brewer (2012) revisão, não Brewer (2000)
  - ACID → Haerder & Reuter (1983), não Gray & Reuter
  - Kafka → "log processing", não "log aggregation"
  - Indústria: qualificado como "princípios relacionados, não CQRS formal"
- **Dual write + transactional outbox:** incluído explicitamente
- **Animações:** `.reveal` com delays `.d1`–`.d9`, disparam ao entrar no slide
- **Step animation:** sistema genérico `data-group` + `buildSteps()`; `goForward()` / `goBackward()` despacham para slide correto; Kafka (S11) permanece handler especial sincronizando dot + left steps — ver SESSION_NOTES.md
- **Slide smono:** slide dedicado ao monolito (id="smono"), entre s3 e s4, N=17, KAFKA_IDX=11

---

## Como regerar os slides

Se `cqrs.md` for editado:
```
/make-slides /Users/math/Documents/mestrado/disciplinas/topicos avançados de arquitetura/projetos/cqrs/cqrs.md --vibe light-acadêmico --lang pt
```
Depois: reimplementar as animações reveal e os slides novos (2 e 6) manualmente — eles foram feitos à mão.

## Como verificar o documento

```
/check-doc /Users/math/Documents/mestrado/disciplinas/topicos avançados de arquitetura/projetos/cqrs/cqrs.md --lang pt
```

---

## Preparação para Q&A

| Pergunta provável | Onde está a resposta |
|-------------------|----------------------|
| "E se o processo cair entre a escrita e a publicação no Kafka?" | Slide 10 — Transactional Outbox |
| "Como garantir que o consumidor não processe o mesmo evento duas vezes?" | Slide 11 — idempotência + at-least-once |
| "O modelo de leitura pode ficar para sempre desatualizado?" | Slide 12 — consistência eventual |
| "CQRS não adiciona muita complexidade?" | Slide 7 (Fowler 2011 alerta) |
| "Por que Kafka e não RabbitMQ?" | Não está nos slides — Kafka é log persistente (reprocessamento), RabbitMQ é fila descartável |
| "Event Sourcing é obrigatório com CQRS?" | Slide 13 — não, são complementares |
| "Essas empresas realmente usam CQRS?" | Slide 14 — qualificado: princípios relacionados, não adoção formal |

---

## Referências principais para estudo rápido

- **Young, G. (2010).** CQRS Documents — 30 min
- **Fowler, M. (2011).** CQRS — martinfowler.com, 10 min
- **Kleppmann, M. (2017).** Designing Data-Intensive Applications — caps. 5 e 11
- **Richardson, C. (2018).** Microservices Patterns — cap. Saga e Outbox
- **Pritchett, D. (2008).** BASE: An Acid Alternative — ACM Queue, 5 min
- **Brewer, E. (2012).** CAP Twelve Years Later — IEEE Computer
