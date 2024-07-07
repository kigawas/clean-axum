# clean-axum

Axum scaffold with clean architecture.

You probably don't need [Rust on Rails](https://github.com/loco-rs/loco).

## Features

- [Axum](https://github.com/tokio-rs/axum) framework
- [SeaORM](https://github.com/SeaQL/sea-orm) domain models
- Completely separated API routers and DB-related logic (named "services")
- Completely separated input parameters, queries and output schemas
- OpenAPI documentation ([Swagger UI](https://clean-axum.shuttleapp.rs/docs) and [Scalar](https://clean-axum.shuttleapp.rs/scalar)) powered by [Utoipa](https://github.com/juhaku/utoipa)
- Optional [Shuttle](https://www.shuttle.rs/) runtime
- Error handling with [Anyhow](https://github.com/dtolnay/anyhow)

## Module hierarchy

- `api`: Axum logic
  - `api::routers`: Axum endpoints
  - `api::doc`: Utoipa doc declaration
  - `api::error`: Error handling
- `api::models`: Non domain model API models
  - `api::models::response`: JSON error response
  - `api::models::request`: Custom Axum Json extractor for error handling
- `app`: DB/API-agnostic logic
  - `app::services`: DB manipulation (CRUD) functions
  - `app::config`: DB or API configuration
  - `app::state`: APP state, e.g. DB connection
- `models`: DB/API-agnostic models
  - `models::domains`: SeaORM domain models
  - `models::params`: Serde input parameters for creating/updating domain models in DB
  - `models::schemas`: Serde output schemas for combining different domain models
  - `models::queries`: Serde queries for filtering domain models
- `migration`: SeaORM migration files
- `main`: Tokio and Shuttle conditional entry point

## Run

### Start server

```bash
cp .env.example .env
# touch dev.db
# cargo install sea-orm-cli
# sea-orm-cli migrate up
cargo run
```

### Call API

```bash
curl -X POST http://localhost:3000/users -H "Content-Type: application/json" -d '{"username":"aaa"}'
curl -X POST http://localhost:3000/users -H "Content-Type: application/json" -d '{"username":"abc"}'
curl http://localhost:3000/users\?username\=a
```

### OpenAPI doc (Swagger UI/Scalar)

```bash
open http://localhost:3000/docs
open http://localhost:3000/scalar
```

## Start Shuttle local server

```bash
# cargo install cargo-shuttle
cargo shuttle run
```

Make sure docker engine is running, otherwise:

```bash
brew install colima docker
colima start
sudo ln -sf $HOME/.colima/default/docker.sock /var/run/docker.sock
```

## Shuttle deployment

```bash
cargo shuttle login
cargo shuttle deploy
```

## Benchmark

```bash
# edit .env to use Postgres
cargo run --release
wrk --latency -t20 -c50 -d10s http://localhost:3000/users\?username\=
```
