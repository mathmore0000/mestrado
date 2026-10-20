Encontro_02_Fichas_Praticas_Alunos 2) 2.1) A plataforma é destinada a cerca de 2.000 usuários e 60 equipamentos. O sistema deve permitir consultar recursos, solicitar um intervalo de uso, analisar uma solicitação quando necessário, confirmar uma reserva e avisar interessados. A meta de consulta é 95% das respostas em até 2 s, sob carga a especificar. Pesquisadores precisam consultar e solicitar equipamentos compartilhados; técnicos registram manutenção e indisponibilidade; gestores acompanham o uso.

2.2) Consultas gerenciais, funcionalidades das reservas, disparo de avisos.

.3.2) - módulo .

2.3.3) consultas-gerenciais - módulo que centraliza consultas gerenciais, acessa banco de dados levemente dezatualizados performáticos para grandes consultas - necessidades técnicas de consultas complexas.

2.3.4) -

<table style="min-width: 75px;">
<colgroup><col style="min-width: 25px;"><col style="min-width: 25px;"><col style="min-width: 25px;"></colgroup><tbody><tr><td colspan="1" rowspan="1"><p>Nome do módulo</p></td><th colspan="1" rowspan="1"><p>Responsabilidade principal</p></th><th colspan="1" rowspan="1"><p>O que deve esconder</p></th></tr><tr><td colspan="1" rowspan="1"><p>reservas</p></td><td colspan="1" rowspan="1"><p>Resonsável por gerenciar criação, consulta e gerenciamento geral das reservas</p></td><td colspan="1" rowspan="1"><p>Ciclo de vida das reservas.</p></td></tr><tr><td colspan="1" rowspan="1"><p>avisos</p></td><td colspan="1" rowspan="1"><p>Responsável pela centralização de disparo de avisos, para inicio de reservas, confirmação de reservas ou e-mail genéricos</p></td><td colspan="1" rowspan="1"><p>Orquestração e controle de envio de e-mails.</p></td></tr><tr><td colspan="1" rowspan="1"><p>consultas-gerenciais</p></td><td colspan="1" rowspan="1"><p>Responsável por centralizar consultas gerenciais</p></td><td colspan="1" rowspan="1"><p>Necessidades técnicas de consultas complexas. Acessando banco de dados levemente dezatualizados performáticos para grandes consultas.</p></td></tr><tr><td colspan="1" rowspan="1"><p>auth</p></td><td colspan="1" rowspan="1"><p>Responsável por garantir autenticação e autorização correta dos usuários do sistema.</p></td><td colspan="1" rowspan="1"><p>Lógicas de negócio complexas baseado em diferentes perfis.</p></td></tr></tbody>
</table>

3.4

![](./Encontro_02_Fichas_Praticas_Alunos.assets/Screenshot-2026-10-19-at-21.17.47.png)3.B)

| De → Para | O que a interface oferece (em linguagem neutra) |
| --- | --- |
| reservas→avisos | Oferece disparo notificações para usuários em situações diferentes, ex.: reserva confirmada. |
| consultas-gerenciais→auth | Confirma credenciais do cliente e que ele tem acesso para acessar o que deseja. |
| reservas→banco de dados | Produz o ciclo de vida confiável da reserva. |
