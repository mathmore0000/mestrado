# CQRS: Separando Leituras e Escritas para Escalar sem Quebrar o Banco

**Disciplina:** Tópicos Avançados de Arquitetura\
**Autor:** Matheus Moreira e Lucas Aguiar\
**Instituição:** Universidade Presbiteriana Mackenzie — Programa de Pós-Graduação em Computação Aplicada\
**Data:** 2026

---

## Contexto e Motivação

### O Sonho

Jorge é um desenvolvedor que usa IA generativa para acelerar sua produtividade. Em um fim de semana, ele decide tirar do papel uma ideia: criar uma rede social focada em comunidades de nicho — um espaço autêntico para o público jovem.

A estratégia é simples: construir rápido, publicar cedo, validar com usuários reais antes de investir em infraestrutura robusta.

> "Se funcionar neste fim de semana, publico na segunda. O mercado vai me dizer se a ideia presta."

Frontend em React, backend em Node.js, PostgreSQL em terceira forma normal. Roda na máquina local. Funciona.

### A Realidade

Jorge publica o MVP. Compartilha com amigos, grupos de tecnologia e comunidades de desenvolvedores. O feedback inicial valida o conceito.

Duas semanas depois, os primeiros 100 usuários aparecem. No primeiro mês, o boca a boca supera qualquer estratégia de marketing. Em dois meses, o tráfego cresce 500%.

> @dev_gabriel: "Que app incrível! As comunidades de nicho foram uma sacada genial."
> @mari_jardim: "Acredita que até minha vó está usando? Ela ama a comunidade de jardinagem!"

O backend escala sem dificuldade — é só adicionar containers. Jorge respira aliviado.

### A Crise

Três meses depois. O app tem 10.000 usuários ativos. Os primeiros sinais de instabilidade aparecem. Notificações de erro acordam Jorge de madrugada.

> @ana_juju: "Tentei adicionar um comentário e ele simplesmente sumiu."
> @dev_pedro: "Não consigo editar meus posts — o botão trava por vários segundos."
> @carlos_m: "O feed demora uma eternidade pra carregar. Já pensei em desinstalar."

O servidor de aplicação não está sobrecarregado. Jorge analisa os logs do PostgreSQL *(cenário ilustrativo)*:

```
$ top -p $(pgrep postgres)
→ CPU: 98.7% · I/O Wait: 64%

SELECT count(*) FROM pg_stat_activity WHERE wait_event_type = 'Lock';
→ 342 conexões aguardando liberação de locks
```

O culpado é a query. Para montar o feed de um usuário, a aplicação executa isso toda vez que alguém abre o app:

```sql
SELECT
    p.id, p.content, p.created_at,
    u.name, u.avatar_url,
    COUNT(DISTINCT l.id)  AS likes_count,
    COUNT(DISTINCT c.id)  AS comments_count
FROM posts p
JOIN users u ON u.id = p.author_id
LEFT JOIN likes    l ON l.post_id = p.id
LEFT JOIN comments c ON c.post_id = p.id
WHERE p.author_id IN (
    SELECT followed_id FROM follows WHERE follower_id = :user_id
)
ORDER BY p.created_at DESC
LIMIT 20;
```

Com 10.000 usuários simultâneos, cada um abrindo o feed, cada um disparando JOINs e `COUNT`s sobre as mesmas tabelas — o banco engasga. Adicionar réplicas de leitura adia o problema, mas não o resolve: a query continua cara, e a concorrência continua crescendo.

O backend foi escalado horizontalmente (múltiplos containers), mas o banco de dados permaneceu como ponto único de gargalo. O problema não é de código — é arquitetural.

### O problema de latência custa dinheiro

Latência não é só uma métrica técnica — tem impacto direto em receita. Marissa Mayer, então vice-presidente de Produtos de Busca do Google, relatou em 2006 que um atraso de 500ms nas buscas resultou em 20% menos pesquisas realizadas (Mayer, 2006). Em um estudo controlado posterior, Brutlag (2009) confirmou a relação: atrasos adicionais de 400ms levaram a reduções mensuráveis no número de buscas por usuário. Na Amazon, Greg Linden documentou que cada 100ms adicionais de latência correspondiam a uma queda de 1% nas vendas (Linden, 2006).

Interrupções de serviço causadas por gargalos de infraestrutura têm consequências financeiras diretas e documentadas. Durante o Amazon Prime Day de 2018, falhas de escalonamento de infraestrutura travaram o site por aproximadamente duas horas; estimativas independentes apontaram perdas entre US$ 72 e US$ 99 milhões em vendas não realizadas (Digital Commerce 360, 2018). O Gartner estima que o custo médio de indisponibilidade de sistemas de TI é de US$ 5.600 por minuto — valor que pode ser substancialmente maior em plataformas de e-commerce e streaming sob alta carga (Lerner, 2014).

Brutlag, J. (2009). *Speed Matters for Google Web Search*. Google Research.\
Digital Commerce 360. (2018). *Amazon Prime Day 2018 outage cost*. digitalcommerce360.com.\
Lerner, A. (2014). *The Cost of Downtime*. Gartner Research.\
Linden, G. (2006). *Make Data Useful*. Apresentação na Stanford University.\
Mayer, M. (2006). *Improving User Experience Through Browser Responsiveness*. Apresentação na O'Reilly Web 2.0 Expo.

---

## O Que é CQRS

### A Solução

Jorge propõe reprojetar a arquitetura separando os caminhos de leitura e escrita. O padrão que formaliza essa separação é o CQRS.

CQRS — *Command Query Responsibility Segregation* — é um padrão arquitetural que separa as operações de **escrita** (*commands*) das operações de **leitura** (*queries*) em modelos distintos (Young, 2010; Fowler, 2011).

O princípio é uma extensão do CQS (*Command-Query Separation*) proposto por Bertrand Meyer (Meyer, 1988): um método deve ser ou um *command* (que muda estado e não retorna valor) ou uma *query* (que retorna valor e não muda estado) — nunca os dois. CQRS eleva esse princípio ao nível arquitetural: modelos, armazenamentos e pipelines separados para cada responsabilidade.

### Linha do tempo do padrão

| Ano | Marco |
|-----|-------|
| 1988 | Bertrand Meyer introduz o CQS em *Object-Oriented Software Construction* |
| 2009 | Udi Dahan publica *Clarified CQRS*, socializando o padrão em sistemas distribuídos |
| 2010 | Greg Young formaliza o termo CQRS e o integra ao Domain-Driven Design |
| 2011 | Martin Fowler publica análise definitiva em martinfowler.com |

Dahan, U. (2009). *Clarified CQRS*. udidahan.com.\
Meyer, B. (1988). *Object-Oriented Software Construction*. Prentice Hall.\
Young, G. (2010). *CQRS Documents*. Disponível em: https://cqrs.files.wordpress.com/2010/11/cqrs_documents.pdf\
Fowler, M. (2011). *CQRS*. martinfowler.com. Disponível em: https://martinfowler.com/bliki/CQRS.html

---

## O Modelo de Escrita (Command Side)

O lado de escrita mantém um modelo **normalizado**, consistente, voltado para integridade de dados. Comandos — `CurtirPost`, `PublicarPost`, `SeguirUsuário` — passam por validação de regras de negócio e modificam o estado canônico da aplicação (Young, 2010).

No contexto do feed do Jorge, o modelo de escrita reflete fielmente o domínio:

```
users    (id, name, avatar_url, created_at)
posts    (id, author_id, content, created_at)
likes    (id, user_id, post_id, created_at)
follows  (id, follower_id, followed_id)
comments (id, post_id, author_id, content, created_at)
```

A normalização aqui é intencional: cada fato é armazenado em um único lugar, o que evita anomalias de atualização e mantém a consistência de dados (Codd, 1970). Se o nome de um usuário muda, só a tabela `users` precisa ser atualizada. Este lado opera sob garantias **ACID** — Atomicidade, Consistência, Isolamento e Durabilidade — que protegem a integridade do estado canônico (Haerder & Reuter, 1983).

Codd, E. F. (1970). A relational model of data for large shared data banks. *Communications of the ACM*, 13(6), 377–387.\
Haerder, T., & Reuter, A. (1983). Principles of transaction-oriented database recovery. *ACM Computing Surveys*, 15(4), 287–317. https://doi.org/10.1145/289.291

---

## O Modelo de Leitura (Query Side)

O lado de leitura mantém um modelo **desnormalizado**, otimizado para as queries que a aplicação realmente executa. Em vez de JOINs em tempo de execução, os dados chegam pré-computados e prontos para exibição.

Para o feed do Jorge, o modelo de leitura é uma tabela materializada:

```
feed_items (
    user_id,        -- dono do feed
    post_id,
    content,
    created_at,
    author_id,
    author_name,    -- desnormalizado de users
    author_avatar,  -- desnormalizado de users
    likes_count,    -- pré-computado, atualizado por eventos
    comments_count  -- pré-computado, atualizado por eventos
)
```

A query do feed vira:

```sql
SELECT * FROM feed_items
WHERE user_id = :user_id
ORDER BY created_at DESC
LIMIT 20;
```

Sem JOINs. Sem `COUNT`s em tempo de execução. O custo de computação foi deslocado do momento da leitura para o momento da escrita — e em sistemas de feed, leituras superam escritas por uma larga margem (Kleppmann, 2017).

A desnormalização reduz latência porque elimina operações de JOIN em tempo de leitura — que em bancos relacionais sob alta carga são um dos principais gargalos de performance (Stonebraker et al., 2007). Este lado opera sob o modelo **BASE** — *Basically Available, Soft State, Eventual Consistency* — priorizando disponibilidade e velocidade de resposta (Pritchett, 2008).

Kleppmann, M. (2017). *Designing Data-Intensive Applications*. O'Reilly Media.\
Pritchett, D. (2008). BASE: An Acid Alternative. *ACM Queue*, 6(3), 48–55.\
Stonebraker, M., Madden, S., Abadi, D. J., Harizopoulos, S., Hachem, N., & Helland, P. (2007). The end of an architectural era (it's time for a complete rewrite). *Proceedings of the 33rd VLDB Conference*.

---

## Sincronização: Domain Events + Message Queue

A separação entre os dois modelos cria um problema: como manter o modelo de leitura atualizado quando o modelo de escrita muda?

A resposta são **domain events** — eventos que representam algo que aconteceu no domínio (Evans, 2003). Quando o modelo de escrita muda de estado, ele publica um evento em um *message broker*. Consumidores assinantes atualizam o modelo de leitura de forma assíncrona.

Evans, E. (2003). *Domain-Driven Design: Tackling Complexity in the Heart of Software*. Addison-Wesley.

No contexto do feed do Jorge, o fluxo de sincronização funciona assim:

1. Um usuário curte um post → comando `CurtirPost` chega ao serviço de escrita
2. O serviço persiste o registro na tabela `likes` (modelo de escrita)
3. O serviço publica um evento `PostCurtido { post_id, user_id, timestamp }` no broker
4. Um consumidor assíncrono recebe o evento e incrementa `likes_count` no modelo de leitura
5. A próxima leitura do feed já reflete o valor atualizado

O mesmo padrão se aplica para `PostPublicado`, `PostComentado`, `UsuárioSeguido` — cada mudança de estado no domínio gera um evento, e cada evento mantém o modelo de leitura atualizado.

### O problema do dual write

Entre os passos 2 e 3 existe uma janela de falha: se o processo crashar após gravar em `likes` mas antes de publicar o evento, o modelo de leitura nunca é atualizado — e o banco de escrita e o modelo de leitura ficam divergentes de forma silenciosa.

A solução padrão é o **Transactional Outbox Pattern**: em vez de publicar o evento diretamente no broker, o serviço grava o evento em uma tabela `outbox` na mesma transação da escrita principal. Um processo separado lê essa tabela e publica os eventos no broker de forma confiável (Richardson, 2018). A atomicidade da transação garante que a escrita e o evento ou ocorrem juntos ou não ocorrem.

Richardson, C. (2018). *Microservices Patterns*. Manning.

### Apache Kafka

Kafka é uma plataforma de streaming distribuído baseada em **log imutável e particionado** (Kreps et al., 2011). Diferente de filas tradicionais, as mensagens não são deletadas após o consumo — ficam persistidas no disco por um período configurável.

Isso tem duas consequências importantes para o cenário de CQRS:

- **Múltiplos consumidores independentes**: um evento `PostCurtido` pode ser consumido simultaneamente pelo serviço que atualiza `likes_count` no feed, pelo serviço de notificações e pelo serviço de analytics — sem coordenação entre eles.
- **Reprocessamento**: se o modelo de leitura precisar ser reconstruído (por uma mudança de schema, por exemplo), os consumidores podem reler os eventos a partir de qualquer offset histórico.

Vale notar que Kafka garante ordenação **por partição**, não globalmente. Para o caso de curtidas, eventos do mesmo `post_id` devem ser roteados para a mesma partição — uma decisão de design que garante que `likes_count` seja incrementado na ordem correta (Narkhede et al., 2017).

Kreps, J., Narkhede, N., & Rao, J. (2011). *Kafka: A distributed messaging system for log processing*. Proceedings of NetDB.

### Garantias de Entrega e Idempotência

Sistemas de mensageria assíncrona operam sob garantias de entrega que impactam diretamente a consistência do modelo de leitura (Kleppmann, 2017):

- ***At-least-once delivery***: o broker garante que o evento será entregue, mas pode entregar mais de uma vez em caso de falha. O consumidor precisa ser **idempotente** — processar o mesmo evento duas vezes não deve produzir resultado diferente de processá-lo uma vez.
- ***Exactly-once delivery***: garantia mais forte, com custo de performance maior. Kafka suporta isso via transações e *idempotent producers* desde a versão 0.11 (Narkhede et al., 2017).

No feed do Jorge, idempotência pode ser implementada registrando os IDs de eventos já processados antes de aplicar a atualização no modelo de leitura.

Narkhede, N., Shapira, G., & Palino, T. (2017). *Kafka: The Definitive Guide*. O'Reilly Media.

---

## Consistência Eventual: O Tradeoff Honesto

Adotar CQRS com sincronização assíncrona implica aceitar **consistência eventual**: o modelo de leitura pode estar momentaneamente desatualizado em relação ao modelo de escrita (Vogels, 2009).

Isso não é um defeito do padrão — é um tradeoff explícito. O Teorema CAP (provado formalmente por Gilbert e Lynch em 2002 a partir da conjectura de Brewer) estabelece que sistemas distribuídos não podem garantir simultaneamente Consistência, Disponibilidade e Tolerância a Partições. O próprio Brewer revisitou e refinou o teorema em 2012, reconhecendo que a escolha raramente é binária — na prática, trata-se de gerenciar partições quando elas ocorrem (Brewer, 2012). A sincronização assíncrona adotada pelo CQRS implica uma escolha AP: prioriza disponibilidade e tolerância a partições, aceitando consistência eventual no modelo de leitura (Kleppmann, 2017).

Uma analogia cotidiana: quando alguém posta uma foto no Instagram, seus seguidores não a veem no mesmo milissegundo. O feed de cada um converge para o estado correto em poucos segundos. Isso é consistência eventual — e é aceitável para interações humanas. O mesmo vale para o feed do Jorge: se o `likes_count` de um post levar 200ms para refletir a última curtida, nenhum usuário perceberá.

| Modelo | Garantia | Propriedades |
|--------|----------|--------------|
| Escrita | Consistência forte | ACID (Haerder & Reuter, 1983) |
| Leitura | Consistência eventual | BASE (Pritchett, 2008) |

Brewer, E. (2012). CAP twelve years later: How the 'rules' have changed. *IEEE Computer*, 45(2), 23–29. https://doi.org/10.1109/MC.2012.37\
Gilbert, S., & Lynch, N. (2002). Brewer's conjecture and the feasibility of consistent, available, partition-tolerant web services. *ACM SIGACT News*, 33(2), 51–59.\
Vogels, W. (2009). Eventually consistent. *Communications of the ACM*, 52(1), 40–44.

---

## Event Sourcing: O Complemento Purista

Enquanto CQRS separa os modelos de leitura e escrita, *Event Sourcing* vai além: em vez de armazenar o estado atual, armazena a **sequência de eventos que produziu esse estado** (Young, 2010; Fowler, 2005).

O estado atual é derivado reproduzindo os eventos em ordem. Isso torna o log de eventos a fonte da verdade (*source of truth*), e o modelo de leitura uma projeção derivada desse log.

```
Event Store (append-only):
001 · UsuarioCriado   { id: 42, nome: "Jorge" }
002 · ComunidadeCriada { id: 7, titulo: "Jardinagem" }
003 · MembroAderido   { userId: 42, commId: 7 }
004 · PostPublicado   { postId: 101, texto: "Olá!" }
005 · PostCurtido     { postId: 101, userId: 99 }
```

A combinação CQRS + Event Sourcing é natural: os eventos do *event store* são exatamente os domain events que alimentam o message broker e atualizam o modelo de leitura (Young, 2010). A vantagem adicional é o **replay**: se o modelo de leitura for corrompido ou precisar ser reprojetado, basta reproduzir todos os eventos desde o início — o que também implica um custo operacional significativo em sistemas com histórico longo.

Fowler, M. (2005). *Event Sourcing*. martinfowler.com. Disponível em: https://martinfowler.com/eaaDev/EventSourcing.html

---

## Adoção na Indústria

O problema de Jorge não é novo. Empresas que operam em escala global enfrentaram o mesmo gargalo e adotaram soluções baseadas nos mesmos princípios — separação de responsabilidades, consistência eventual, logs de eventos e modelos de leitura otimizados. Vale notar que nenhuma delas descreve formalmente suas soluções como "CQRS"; o padrão emerge da análise dos princípios aplicados (DeCandia et al., 2007; Izrailevsky & Meshenberg, 2016; Crunkilton et al., 2016).

**Amazon — Dynamo (2004–2007):** O sistema de carrinho de compras crescia sob carga exponencial. O banco Oracle centralizado sofria com locks severos de leitura/escrita, especialmente durante picos como a Black Friday. A solução foi o Dynamo — sistema interno de chave-valor com separação completa dos fluxos de leitura e escrita, consistência eventual como premissa de design e sem ponto único de falha. Seus princípios influenciaram diretamente o DynamoDB, lançado publicamente em 2012 (DeCandia et al., 2007).

**Netflix (2009–2016):** O monólito Oracle não sustentava a escala global de streaming. A migração para microserviços adotou CQRS: Cassandra e Elasticsearch para leitura de catálogo; MySQL particionado para escritas transacionais (Izrailevsky & Meshenberg, 2016).

**Uber (2014–2016):** PostgreSQL monolítico não suportava a demanda global simultânea. Contenção entre escritas (pedidos de corrida) e leituras (cálculo de ETA e tarifas) gerava timeouts. A solução foi CQRS com Schemaless (baseado em MySQL) para escritas e Cassandra para leituras, com propagação via logs de eventos assíncronos (Crunkilton et al., 2016).

Crunkilton, B. et al. (2016). *Designing Schemaless, Uber Engineering's Scalable Datastore Using MySQL*. Uber Engineering Blog.\
DeCandia, G., Hastorun, D., Jampani, M., Kakulapati, G., Lakshman, A., Pilchin, A., Sivasubramanian, S., Vosshall, P., & Vogels, W. (2007). Dynamo: Amazon's highly available key-value store. *Proceedings of SOSP '07*, 205–220.\
Izrailevsky, Y., & Meshenberg, R. (2016). *Completing the Netflix Cloud Migration*. Netflix Technology Blog.

---

## Quando Usar (e Quando Não Usar)

CQRS não é uma solução universal. Fowler (2011) alerta que o padrão adiciona complexidade significativa — mais componentes para operar, dados duplicados no modelo de leitura, sincronização assíncrona para gerenciar — e deve ser aplicado apenas onde os benefícios justificam esse custo.

O padrão é adequado quando há:
- Alta disparidade entre volume de leituras e escritas
- Requisitos de performance de leitura muito distintos dos de escrita
- Necessidade de escalar os dois lados de forma independente
- Domínios complexos onde diferentes visões dos dados precisam ser otimizadas separadamente

O caso do Jorge se encaixa nesse perfil: o feed é a operação mais frequente e mais cara, leituras superam escritas por larga margem, e a audiência cresce de forma previsível.

Para um CRUD simples com baixo volume de acesso, o overhead de CQRS não se justifica.

---

## Conclusão

O problema de Jorge é o problema de qualquer sistema que cresce além do que seu modelo de dados foi projetado para suportar. Um banco relacional normalizado é a escolha certa para um MVP — ele garante integridade, facilita o desenvolvimento e é bem compreendido. Mas quando leituras e escritas começam a competir pelo mesmo recurso físico, a normalização se torna o gargalo.

CQRS resolve isso de forma cirúrgica: separa os modelos, otimiza cada um para sua responsabilidade, e usa domain events para manter os dois em sincronia. O custo é a consistência eventual e a complexidade operacional adicional — um tradeoff explícito e bem documentado (Fowler, 2011; Vogels, 2009).

A adoção de CQRS por Amazon, Netflix e Uber não foi uma escolha ideológica — foi uma resposta a gargalos concretos, com impacto financeiro mensurável. O mesmo raciocínio se aplica ao feed do Jorge: quando a arquitetura vira prejuízo, é hora de separar as responsabilidades.

---

## Referências

Brutlag, J. (2009). *Speed Matters for Google Web Search*. Google Research.

Codd, E. F. (1970). A relational model of data for large shared data banks. *Communications of the ACM*, 13(6), 377–387.

Crunkilton, B. et al. (2016). *Designing Schemaless, Uber Engineering's Scalable Datastore Using MySQL*. Uber Engineering Blog.

Dahan, U. (2009). *Clarified CQRS*. udidahan.com.

DeCandia, G., Hastorun, D., Jampani, M., Kakulapati, G., Lakshman, A., Pilchin, A., Sivasubramanian, S., Vosshall, P., & Vogels, W. (2007). Dynamo: Amazon's highly available key-value store. *Proceedings of SOSP '07*, 205–220.

Digital Commerce 360. (2018). *Amazon Prime Day 2018 outage cost*. digitalcommerce360.com.

Evans, E. (2003). *Domain-Driven Design: Tackling Complexity in the Heart of Software*. Addison-Wesley.

Fowler, M. (2005). *Event Sourcing*. martinfowler.com.

Fowler, M. (2011). *CQRS*. martinfowler.com.

Gilbert, S., & Lynch, N. (2002). Brewer's conjecture and the feasibility of consistent, available, partition-tolerant web services. *ACM SIGACT News*, 33(2), 51–59.

Haerder, T., & Reuter, A. (1983). Principles of transaction-oriented database recovery. *ACM Computing Surveys*, 15(4), 287–317. https://doi.org/10.1145/289.291

Izrailevsky, Y., & Meshenberg, R. (2016). *Completing the Netflix Cloud Migration*. Netflix Technology Blog.

Kleppmann, M. (2017). *Designing Data-Intensive Applications*. O'Reilly Media.

Kreps, J., Narkhede, N., & Rao, J. (2011). *Kafka: A distributed messaging system for log processing*. Proceedings of NetDB.

Lerner, A. (2014). *The Cost of Downtime*. Gartner Research.

Linden, G. (2006). *Make Data Useful*. Apresentação na Stanford University.

Mayer, M. (2006). *Improving User Experience Through Browser Responsiveness*. Apresentação na O'Reilly Web 2.0 Expo.

Meyer, B. (1988). *Object-Oriented Software Construction*. Prentice Hall.

Narkhede, N., Shapira, G., & Palino, T. (2017). *Kafka: The Definitive Guide*. O'Reilly Media.

Pritchett, D. (2008). BASE: An Acid Alternative. *ACM Queue*, 6(3), 48–55.

Richardson, C. (2018). *Microservices Patterns*. Manning.

Stonebraker, M., Madden, S., Abadi, D. J., Harizopoulos, S., Hachem, N., & Helland, P. (2007). The end of an architectural era (it's time for a complete rewrite). *Proceedings of the 33rd VLDB Conference*.

Vogels, W. (2009). Eventually consistent. *Communications of the ACM*, 52(1), 40–44.

Young, G. (2010). *CQRS Documents*. cqrs.files.wordpress.com.
