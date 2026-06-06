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

### 1. Diagramas de arquitetura (slide 2 + novo slide)
Três imagens disponíveis em `/Users/math/Downloads/taeca-arquitetura-software-seminario-main/assets/`:
- `Sistema vibe coded do George.png` → substituir placeholder CSS no slide 2 ("Conheçam Jorge")
- `Arquitetura do sistema SEM CQRS.png` → slide de problema/monolito
- `Arquitetura do sistema COM CQRS.png` → slide de solução CQRS

Copiar para `assets/` dentro do projeto e referenciar como `<img>`.

### 2. Animações "surgir" nos slides
Revisar todos os slides e aplicar animações `.reveal` nas partes que ainda não têm (listas, diagramas, blockquotes que aparecem em sequência lógica). O slide 10 (Kafka) já tem lista com reveals — avaliar se vale upgrade para animação CSS keyframe de pacote viajando.

**Spec Kafka (SESSION_NOTES.md):** `[Write Service] → [Kafka] → [Consumer] → [Read DB]`, auto-play ao entrar, executa uma vez, `likes_count: 42 → 43`.

### 3. Reordenar slide "Como vocês construiriam?"
Slide 6 atualmente — avaliar se faz mais sentido antes ou depois do slide de crise (slide 4/5).

### 4. Métrica faltando no slide de impacto financeiro (slide 5)
Verificar qual métrica está ausente e adicionar com referência acadêmica.

### 5. Disclaimer de uso de IA
Adicionar em algum slide (provavelmente capa ou obrigado) menção ao uso de IA generativa na construção do MVP do Jorge e/ou na preparação da apresentação.

### 6. Remover referências a "Atos"
Os slides de jornada do Jorge usam "Ato I", "Ato II" etc. — remover ou substituir por linguagem mais direta.

### 7. Layout para TV Full HD (visualização à distância)
Aumentar fontes base, espaçamentos e elementos visuais para legibilidade em TV 1920×1080 vista de longe numa sala de aula.

### 9. Fixes no cqrs.md (saídos do check-doc)

**BLOCKER — Dynamo vs DynamoDB (seção Adoção na Indústria, Amazon):**
O texto diz "a solução foi o DynamoDB (DeCandia et al., 2007)" mas o paper descreve o *Dynamo* (sistema interno, carrinho de compras), não o DynamoDB (serviço público, 2012). São sistemas distintos.
Fix: trocar por "Dynamo (sistema interno)" e adicionar nota que seus princípios influenciaram o DynamoDB em 2012.

**WARNING — Referência órfã Brewer (2000):**
`Brewer, E. (2000). Towards robust distributed systems. PODC.` está nas Referências mas nunca é citado no corpo. O body só cita Brewer (2012).
Fix: remover da seção de Referências.

**WARNING — Citação Netflix imprecisa:**
Izrailevsky & Meshenberg (2016) é sobre a migração para a nuvem, não sobre arquitetura CQRS com Cassandra/Elasticsearch. Os detalhes técnicos precisam de um post mais específico do Netflix Tech Blog.
Fix: buscar post técnico adequado ou qualificar o nível de detalhe da afirmação.

**WARNING — Autor Uber não verificado:**
`Crunkilton, B. et al. (2016)` — nome do autor não verificado no Uber Engineering Blog.
Fix: confirmar autoria antes da apresentação.

**WARNING — Stonebraker et al. (2007) usado para JOIN bottlenecks:**
O paper argumenta pelo fim do RDBMS monolítico em geral, não especificamente que JOINs são gargalo em feeds. Kleppmann (2017, cap. 3) cobre isso diretamente.
Fix: substituir ou complementar com Kleppmann (2017).

**WARNING — "réplicas de leitura" sem citação:**
"Adicionar réplicas de leitura adia o problema, mas não o resolve" — afirmação técnica sem referência.
Fix: adicionar "(Kleppmann, 2017, cap. 5)" ou qualificar a frase.

### 8. Divisão da apresentação entre os apresentadores
Ainda não decidida. Sugestões:
- **Opção A:** Lucas slides 1–6 (história + problema + discussão), Matheus slides 7–15 (solução técnica)
- **Opção B:** Lucas slides 1–5 (contexto), Matheus slides 6–15

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
