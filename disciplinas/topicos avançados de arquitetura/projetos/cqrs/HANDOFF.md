# Handoff — Apresentação CQRS
**Disciplina:** Tópicos Avançados de Arquitetura — Universidade Presbiteriana Mackenzie  
**Apresentadores:** Matheus Moreira · Lucas Aguiar  
**Formato:** 30 min de apresentação + 5 min de Q&A  
**Última atualização:** 2026-06-05

---

## O que foi produzido

| Arquivo | Descrição |
|---------|-----------|
| `cqrs.md` | Documento-fonte da apresentação — narrativa completa, referências acadêmicas verificadas |
| `presentation.html` | Slides HTML gerados a partir do `cqrs.md` — 13 slides, vibe acadêmico, PT-BR |
| `SESSION_NOTES.md` | Notas técnicas de sessão para continuação via Claude Code |
| `HANDOFF.md` | Este arquivo |

Abra `presentation.html` diretamente no browser. Navegação por seta ← →, Space ou clique nos dots.

---

## Estado atual dos slides

| # | Slide | Status |
|---|-------|--------|
| 1 | Capa | ✓ Completo |
| 2 | A Jornada de Jorge (Atos I–II) | ✓ Completo |
| 3 | A Crise — diagnóstico + query culpada | ✓ Completo |
| 4 | Arquitetura Vira Prejuízo — dados de latência/receita | ✓ Completo |
| 5 | O Padrão CQRS — definição + linha do tempo | ✓ Completo |
| 6 | Modelo de Escrita vs Modelo de Leitura | ✓ Completo |
| 7 | Query Antes e Depois do CQRS | ✓ Completo |
| 8 | Domain Events + Kafka — fluxo + dual write + outbox | ✓ Completo |
| 9 | Apache Kafka — propriedades + garantias de entrega | ✓ Completo |
| 10 | Consistência Eventual — CAP + ACID vs BASE | ✓ Completo |
| 11 | Event Sourcing | ✓ Completo |
| 12 | Adoção na Indústria — Amazon, Netflix, Uber | ✓ Completo |
| 13 | Obrigado | ✓ Completo |

---

## O que ainda falta

### 1. Animação no slide 8 (Kafka flow) — não implementada
O slide 8 tem o fluxo de sincronização como lista estática. A intenção era ter uma animação CSS que auto-executa quando o slide aparece, mostrando o pacote de evento viajando do Write Service → Kafka → Consumer → Read DB com `likes_count: 42 → 43`.

**Plano acordado:**
- CSS keyframes, auto-play ao entrar no slide, executa uma vez
- Sequência: Write Service (0s) → Kafka (0.8s) → Consumer (1.8s) → Read DB (2.8s → 3.8s)
- Implementar manualmente no HTML depois de gerar os slides base

### 2. Divisão da apresentação entre os apresentadores — não definida
Ainda não foi decidido quem apresenta qual parte. Sugestões possíveis:

**Opção A — Narrativa / Técnico:**
- Lucas: slides 1–5 (história do Jorge, problema, impacto financeiro, definição do CQRS)
- Matheus: slides 6–13 (modelos, Kafka, consistência, Event Sourcing, indústria, encerramento)

**Opção B — Problema / Solução:**
- Lucas: slides 1–4 (contexto, crise, custo)
- Matheus: slides 5–13 (CQRS e tudo que vem depois)

### 3. Verificar referência do Mayer (2006)
A apresentação de Mayer ("Improving User Experience Through Browser Responsiveness") é citada como O'Reilly Web 2.0 Expo 2006. Vale confirmar o venue exato antes da apresentação — é uma fonte cinza (apresentação oral) e pode ser questionada.

---

## Decisões tomadas (não reverter sem discussão)

- **Personagem:** Jorge (não George) — apresentação em PT-BR para plateia brasileira
- **App:** rede social de feed com comunidades de nicho — curtidas (`likes_count`) como exemplo de desnormalização
- **Foco técnico:** Domain Events + Kafka (profundo) + Event Sourcing (introdutório)
- **Referência BASE:** Pritchett (2008), não Fox & Brewer (1999) — correção feita após verificação
- **CAP theorem:** Gilbert & Lynch (2002) como prova formal, Brewer (2000) como conjectura original
- **Dual write:** incluído explicitamente com solução Transactional Outbox (Richardson, 2018)
- **Vibe dos slides:** `acadêmico` — dark, denso, formal
- **Idioma dos slides:** `pt` — PT-BR

---

## Como regerar os slides

Se `cqrs.md` for editado, rodar no Claude Code:

```
/make-slides /Users/math/Documents/mestrado/disciplinas/topicos avançados de arquitetura/projetos/cqrs/cqrs.md --vibe acadêmico --lang pt
```

Depois de gerar, implementar manualmente a animação do slide 8 (ver SESSION_NOTES.md).

## Como verificar o documento

```
/check-doc /Users/math/Documents/mestrado/disciplinas/topicos avançados de arquitetura/projetos/cqrs/cqrs.md --lang pt
```

---

## Preparação para Q&A — perguntas esperadas

| Pergunta provável | Onde está a resposta |
|-------------------|----------------------|
| "E se o processo cair entre a escrita e a publicação no Kafka?" | Slide 8 — Transactional Outbox |
| "Como garantir que o consumidor não processe o mesmo evento duas vezes?" | Slide 9 — idempotência + at-least-once |
| "O modelo de leitura pode ficar para sempre desatualizado?" | Slide 10 — consistência eventual, janela de milissegundos |
| "CQRS não adiciona muita complexidade?" | Slide 5 (Fowler 2011 alerta) + "Quando Usar" em cqrs.md |
| "Por que Kafka e não RabbitMQ?" | Não está nos slides — resposta: Kafka é log persistente (reprocessamento), RabbitMQ é fila descartável. Para CQRS com replay, Kafka é mais adequado |
| "Event Sourcing é obrigatório com CQRS?" | Slide 11 — não, são padrões independentes que se complementam |
| "Como funciona o rebuild do modelo de leitura?" | Slide 11 — replay dos eventos do event store |

---

## Referências principais para estudo rápido

- **Young, G. (2010).** CQRS Documents — leitura obrigatória, 30 min
- **Fowler, M. (2011).** CQRS — martinfowler.com, 10 min
- **Kleppmann, M. (2017).** Designing Data-Intensive Applications — caps. 5 e 11 para consistência e streams
- **Richardson, C. (2018).** Microservices Patterns — cap. sobre Saga e Outbox para dual write
- **Pritchett, D. (2008).** BASE: An Acid Alternative — ACM Queue, 5 min
