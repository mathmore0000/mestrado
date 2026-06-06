# Handoff — Apresentação CQRS
**Disciplina:** Tópicos Avançados de Arquitetura — Universidade Presbiteriana Mackenzie  
**Apresentadores:** Matheus Moreira · Lucas Aguiar  
**Formato:** 30 min de apresentação + 5 min de Q&A  
**Última atualização:** 2026-06-06

---

## O que foi produzido

| Arquivo | Descrição |
|---------|-----------|
| `cqrs.md` | Documento-fonte — narrativa completa, referências acadêmicas verificadas |
| `presentation.html` | 15 slides, tema Mackenzie PPGA (light, navy/red), responsivo com `clamp()` |
| `SESSION_NOTES.md` | Notas técnicas para continuação via Claude Code |
| `HANDOFF.md` | Este arquivo |

Abra `presentation.html` no browser. Navegação: ← → / Space / clique nos dots.

---

## Estado atual dos slides

| # | Slide | Status |
|---|-------|--------|
| 1 | Capa | ✓ Com animações reveal |
| 2 | Conheçam Jorge | ✓ Novo — narrativa + mock MVP |
| 3 | A Jornada de Jorge | ✓ Com animações reveal por step |
| 4 | A Crise — diagnóstico + query culpada | ✓ |
| 5 | Arquitetura Vira Prejuízo — latência/receita | ✓ |
| 6 | Como vocês construiriam? — discussão | ✓ Novo — pausa + revela monolito |
| 7 | O Padrão CQRS — definição + timeline | ✓ |
| 8 | Modelo de Escrita vs Modelo de Leitura | ✓ |
| 9 | Query Antes e Depois do CQRS | ✓ |
| 10 | Domain Events + Kafka + Dual Write | ✓ Com animações por step |
| 11 | Apache Kafka — propriedades + garantias | ✓ |
| 12 | Consistência Eventual — CAP + ACID/BASE | ✓ |
| 13 | Event Sourcing | ✓ |
| 14 | Adoção na Indústria — Amazon/Netflix/Uber | ✓ |
| 15 | Obrigado | ✓ |

---

## O que ainda falta

### 1. Animações "surgir" nos slides
Revisar todos os slides e aplicar animações `.reveal` nas partes que ainda não têm. Slide 11 (Kafka) já tem lista com reveals — avaliar upgrade para animação CSS keyframe de pacote viajando.

**Spec Kafka:** `[Write Service] → [Kafka] → [Consumer] → [Read DB]`, auto-play ao entrar, executa uma vez, `likes_count: 42 → 43`.

### 2. Reordenar slide "Como vocês construiriam?" (slide 6)
Avaliar se faz mais sentido antes ou depois do slide de crise (slides 4/5). Decisão pendente.

### 3. Métrica faltando no slide de impacto financeiro (slide 5)
Identificar qual métrica está ausente e adicionar com referência acadêmica.

### 4. Fixes restantes no cqrs.md

**WARNING — Citação Netflix imprecisa:**
Izrailevsky & Meshenberg (2016) é sobre migração para a nuvem, não arquitetura CQRS/Cassandra. Buscar post técnico mais específico ou qualificar a afirmação.

**WARNING — Autor Uber não verificado:**
`Crunkilton, B. et al. (2016)` — confirmar autoria no Uber Engineering Blog antes da apresentação.

**WARNING — Stonebraker et al. (2007) para JOIN bottlenecks:**
Paper argumenta pelo fim do RDBMS em geral. Substituir ou complementar com Kleppmann (2017, cap. 3).

**WARNING — "réplicas de leitura" sem citação:**
Adicionar "(Kleppmann, 2017, cap. 5)" ou qualificar a frase.

### 5. Divisão da apresentação entre os apresentadores
Ainda não decidida. Agora são 16 slides. Sugestões:
- **Opção A:** Lucas slides 1–7 (história + problema + discussão + definição), Matheus slides 8–16 (arquitetura + técnico)
- **Opção B:** Lucas slides 1–6, Matheus slides 7–16

---

## O que foi feito nesta sessão (2026-06-06)

- ✓ **TV Full HD layout** — `html { font-size: clamp(15px,1.3vw,22px) }` + todas as caps de clamp() tipográficos aumentadas
- ✓ **Diagramas de arquitetura** — 3 imagens em `assets/`; slide 2 com imagem real do MVP; slide 6 com diagrama SEM CQRS; novo slide 8 com diagrama COM CQRS (total: 16 slides)
- ✓ **Disclaimer de IA** — slide 16 (Obrigado)
- ✓ **Remove "Atos"** — cqrs.md e presentation.html
- ✓ **Dynamo/DynamoDB** — corrigido em cqrs.md e slide 15
- ✓ **Brewer (2000) órfão** — removido das Referências

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
