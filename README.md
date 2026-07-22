# Wallet Live

Bem-vindo ao **Wallet Live**, uma aplicação web de carteira digital construída com **Rust**. Este documento serve como o guia principal para desenvolvedores que desejam entender, configurar e contribuir para o projeto.

## 🚀 Visão Geral e Tecnologias

A aplicação utiliza uma arquitetura robusta e assíncrona, focada em performance e segurança. As principais tecnologias da stack são:

- **[Rust](https://www.rust-lang.org/)**: Linguagem de programação focada em segurança, velocidade e concorrência.
- **[Axum](https://github.com/tokio-rs/axum)**: Framework web modular e ergonômico projetado para o ecossistema Tokio. Utilizado tanto para a API RESTful quanto para as rotas do frontend.
- **[SQLx](https://github.com/launchbadge/sqlx)**: Toolkit assíncrono para interação com banco de dados em Rust, com verificação de queries em tempo de compilação.
- **[PostgreSQL](https://www.postgresql.org/)**: Banco de dados relacional.
- **[Askama](https://github.com/djc/askama)**: Motor de templates seguro e rápido (tipo Jinja) para renderização HTML no servidor (Server-Side Rendering).
- **[Tokio](https://tokio.rs/)**: Runtime assíncrono para a execução eficiente do código.
- **Docker & Docker Compose**: Ferramentas de containerização para garantir um ambiente de desenvolvimento limpo e padronizado.

## 📁 Estrutura do Projeto

A organização de diretórios do projeto segue boas práticas de separação de responsabilidades (Clean Architecture e Repository Pattern):

```text
wallet_live/
├── src/
│   ├── main.rs          # Ponto de entrada da aplicação. Delega o boot para app.rs.
│   ├── app.rs           # Configuração do AppState, Router do Axum, Logs (tracing) e listener HTTP.
│   ├── error.rs         # Estrutura centralizada de tratamento de erros personalizados.
│   ├── repository.rs    # Camada de acesso ao banco de dados (Repository Pattern) usando SQLx.
│   ├── auth/            # Regras e middlewares de autenticação (JWT, verificação de senhas).
│   ├── models/          # Entidades de domínio (structs) e DTOs.
│   └── routes/          # Definição dos handlers e controladores (dividido em api e frontend).
├── templates/           # Arquivos de visualização HTML processados pelo Askama.
├── migrations/          # Scripts SQL criados pelo SQLx para controle de versão do banco.
├── Dockerfile           # Receita para construção da imagem do container da aplicação.
├── docker-compose.yaml  # Orquestração do ambiente de dev (app + banco de dados).
└── Cargo.toml           # Manifesto do projeto e dependências (pacotes e versão).
```

## 🛠️ Configurando o Ambiente de Desenvolvimento

### Pré-requisitos

Para rodar este projeto, você precisa ter instalados:
- [Docker e Docker Compose](https://docs.docker.com/get-docker/)
- [Rust & Cargo](https://rustup.rs/) (Opcional para rodar local, mas fortemente recomendado para ferramentas como `sqlx-cli`)

### Passo a Passo

1. **Clone o repositório:**
   ```bash
   git clone <url-do-repositorio>
   cd wallet_live
   ```

2. **Configuração de Variáveis de Ambiente:**
   O projeto utiliza um arquivo `.env` para gerenciar segredos e configurações locais.
   ```bash
   cp .env.example .env
   ```
   Se estiver usando o Docker Compose para o banco, o `DATABASE_URL` padrão no `.env.example` costuma servir: `postgres://root:root@db:5432/postgres`.

3. **Subindo a Infraestrutura com Docker Compose:**
   O `docker-compose.yaml` define dois serviços: o banco `db` (Postgres) e o serviço `app`. 
   
   O serviço `app` mapeia os arquivos locais para dentro do container e roda um comando de bloqueio (`sleep infinity`). Isso permite manter o container online, reutilizando volumes para o cache do `cargo` (o que deixa as próximas compilações absurdamente mais rápidas).
   
   ```bash
   docker-compose up -d db app
   ```

4. **Rodando a Aplicação:**
   Você pode rodar a aplicação "entrando" no container ou a rodando localmente (caso tenha o Rust e dependências do Postgres na sua máquina).
   
   - **Via Container (Recomendado):**
     ```bash
     docker-compose exec app bash
     
     # Dentro do bash do container:
     cargo run
     ```
     *(Dica: Considere instalar `cargo-watch` dentro do container para recarregamento automático no desenvolvimento: `cargo install cargo-watch && cargo watch -x run`)*.

   - **Localmente (Com Postgres via Docker):**
     Garanta que o serviço `db` está de pé:
     ```bash
     docker-compose up -d db
     ```
     Altere a variável `DATABASE_URL` no seu `.env` local para apontar para `localhost`:
     `DATABASE_URL=postgres://root:root@localhost:5432/postgres`
     E em seguida rode:
     ```bash
     cargo run
     ```

## 🗄️ Banco de Dados e Migrations

As interações com banco de dados no Rust são feitas com o **SQLx**. Durante a compilação, o SQLx valida se suas queries SQL fazem sentido no banco configurado via `$DATABASE_URL`.

### Ferramenta CLI (sqlx-cli)

Para gerenciar o banco via terminal local, instale a ferramenta do sqlx (caso não tenha):
```bash
cargo install sqlx-cli --no-default-features --features rustls,postgres
```

### Comandos Úteis

- **Criar o banco de dados:** `cargo sqlx database create`
- **Rodar migrações:** `cargo sqlx migrate run`
- **Reverter a última migração:** `cargo sqlx migrate revert`
- **Criar uma nova migração:** `cargo sqlx migrate add <nome_da_migracao>`

Se você alterar qualquer query nas structs ou arquivos do projeto que interajam com o SQLx, **será necessário preparar as queries para que a compilação offline ou pipelines de CI funcionem**. Para isso, utilize:
```bash
cargo sqlx prepare
```
Esse comando gera um arquivo `sqlx-data.json` na raiz do projeto (que deve ser "commitado" para o git).

## 🔒 Autenticação e Segurança

A aplicação adota mecanismos de autenticação com segurança de mercado:
- Utiliza **Argon2** (via `password-auth`) para hashes de senha robustos.
- Utiliza **JSON Web Tokens (JWT)** (via `jwt-simple`) para gerenciar as sessões dos usuários de forma "stateless".
- Utiliza Cookies assinados (`axum-extra`) para prover armazenamento seguro de tokens e preferências no cliente.
- Configure apropriadamente `ADMIN_SECRET_KEY` no seu `.env` local, que pode ser utilizado em rotas restritas ou como pimenta para o seu hash/tokens.

## 🧪 Testes

Os testes na aplicação são conduzidos integrando o framework oficial de testes do Rust juntamente com [Insta](https://insta.rs/) (Snapshot Testing). Para rodar a suíte de testes do repositório:

```bash
cargo test
```

## 🤝 Contribuindo
- Sempre rode `cargo fmt` antes de realizar um commit.
- Garanta que seu código passe pelo validador rodando `cargo clippy`.
- Verifique se o `sqlx-data.json` está devidamente gerado após alterar queries no banco executando `cargo sqlx prepare`.
