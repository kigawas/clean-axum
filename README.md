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

## Module hierarchy

- `api`: Axum logic
  - `api::routers`: Axum endpoints
  - `api::doc`: Utopia doc declaration
- `app`: DB/API-agnostic logic
  - `app::services`: DB manipulation (CRUD) functions
  - `app::config`: DB or API configuration
  - `app::state`: APP state, e.g. DB connection
- `models`: DB/API-agnostic models
  - `models::domains`: SeaORM models
  - `models::params`: Serde input parameters for creating domain models in DB
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

## Shuttle deployment

Add `Secrets.toml`

```toml
DATABASE_URL = "sqlite://dev.db"
```

Then

```bash
# cargo install cargo-shuttle
cargo shuttle deploy
```

## Benchmark

```bash
cargo run --release
wrk --latency -t20 -c50 -d10s http://localhost:3000/users\?username\=
```
