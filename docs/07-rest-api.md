# 07 · REST API with axum, sqlx and tokio

> Level 7 of 12 · about 13 nights · crate: `projects/api`

## Why this project

This is the most job-relevant project on the ladder: a production-shaped web service. You move from threads to async, from files to a database, and from "it works" to logging, configuration, auth and tests. You built the layer underneath in project 06, so you know what the framework is doing for you.

You are building a **bookmarks API**: users save links, tag them, and search them.

## Rust concepts

- `async` / `.await`; futures are lazy; what a runtime does
- `#[tokio::main]`, `tokio::spawn`, `spawn_blocking`
- Why spawned futures must be `Send + 'static`
- axum extractors and handlers: traits doing work at the type level
- Shared state with `State` and `Arc`; std `Mutex` vs tokio `Mutex`
- `IntoResponse` for your own error type
- Implementing an extractor trait (`FromRequestParts`)
- Tower layers and middleware
- The newtype pattern (`UserId`), `TryFrom` for validated input
- Compile-time checked SQL with `sqlx` macros
- Cargo features
- `tracing` spans and `#[instrument]`

## Dev skills

REST design and status codes, SQL schema and migrations, connection pooling, password hashing, token auth, input validation, pagination, structured logging, configuration from the environment, integration testing, Docker, graceful shutdown.

## Allowed crates

`tokio`, `axum`, `tower`, `tower-http`, `serde`, `serde_json`, `sqlx` (SQLite first), `thiserror`, `anyhow`, `tracing`, `tracing-subscriber`, `uuid`, `argon2`, `jsonwebtoken`, `dotenvy`. Dev: `reqwest`, `axum-macros` (for `#[debug_handler]`).

These crates evolve quickly. Use the versions `cargo add` gives you and read the docs for that version on docs.rs. Blog posts older than a year often show APIs that no longer compile.

## Spec

| Method | Path | Behaviour |
|--------|------|-----------|
| GET | `/health` | 200 if the database is reachable |
| POST | `/users` | Register with email and password. 409 if the email is taken |
| POST | `/login` | Returns a bearer token. 401 on bad credentials |
| GET | `/bookmarks` | The caller's bookmarks. Filters: `tag`, `q`; pagination: `limit`, `cursor` |
| POST | `/bookmarks` | Create: `url`, `title`, `tags[]`. 422 if the URL is invalid |
| GET | `/bookmarks/{id}` | 404 if missing **or owned by someone else** |
| PATCH | `/bookmarks/{id}` | Partial update |
| DELETE | `/bookmarks/{id}` | 204 |

- Every error response is JSON with the same shape: `{"error": {"code": "...", "message": "..."}}`.
- Passwords are stored only as argon2 hashes.
- Every request is logged with method, path, status, latency and a request ID.
- Requests time out; bodies are size-limited.
- Configuration (database URL, port, token secret) comes from environment variables.
- Migrations run at startup.
- The server finishes in-flight requests on SIGTERM / Ctrl-C.

## Milestones

- [ ] **M1 · Hello, async.** tokio and axum serving `/health`. Run `cargo expand` on `main` to see what `#[tokio::main]` generates.
- [ ] **M2 · In-memory CRUD.** Bookmarks in a `HashMap` behind shared state. Decide between std and tokio `Mutex` and be able to defend the choice.
- [ ] **M3 · Request and response types.** Separate DTOs from the domain model. Validate on the way in with `TryFrom`. Correct status codes.
- [ ] **M4 · Error type.** An `AppError` enum that implements `IntoResponse`. Handlers return `Result<_, AppError>` and use `?`.
- [ ] **M5 · Database.** sqlx with SQLite: migrations, a pool in state, replace the `HashMap`. Use the checked query macros.
- [ ] **M6 · Users.** Registration with argon2. Hashing is slow on purpose; work out where it must run so it does not stall the runtime.
- [ ] **M7 · Auth.** Login issues a token. Write an `AuthUser` extractor so protected handlers simply take it as a parameter.
- [ ] **M8 · Ownership, tags, search, pagination.** Many-to-many tags. Cursor pagination. No query may return another user's rows.
- [ ] **M9 · Middleware.** tower-http layers for tracing, timeout, request ID, body limit, CORS. Order matters; find out why.
- [ ] **M10 · Observability and config.** `#[instrument]` on handlers, with secrets kept out of the logs. One typed `Config` struct loaded at startup.
- [ ] **M11 · Integration tests.** Each test gets a fresh database, starts the app, and drives it over HTTP. Cover the happy path and every error code in the spec.
- [ ] **M12 · Ship it.** Graceful shutdown. A multi-stage Dockerfile producing a small image.
- [ ] **M13 · Postgres.** Switch the database to Postgres with docker compose. Notice what changed in your code and what did not.

## Traps when coming from other languages

- Calling an `async fn` does nothing. You get a future; it runs when awaited or spawned.
- Blocking inside async code (`std::thread::sleep`, heavy CPU work, synchronous file or network I/O) stalls every task on that worker thread.
- Holding a std `MutexGuard` across an `.await` makes the future non-`Send`, and the error message is long. Shorten the guard's scope.
- Handler signature mistakes produce huge trait-bound errors. `#[debug_handler]` turns them into readable ones.
- The sqlx query macros talk to a real database at compile time. Learn the offline mode before setting up CI.
- "404 vs 403 for someone else's resource" is a security decision, not a style choice.

## Done when

- [ ] Every row of the spec table behaves as described, verified by integration tests.
- [ ] A second user cannot read, change or delete the first user's bookmarks.
- [ ] `docker run` starts the service from the image with only environment variables.
- [ ] Logs are structured and contain no passwords or tokens.
- [ ] No `unwrap` in handlers; every failure maps to a deliberate status code.
- [ ] You can explain what happens, step by step, between `.await` on a database query and the handler resuming.
- [ ] You can explain how axum decides that your function is a valid handler.

## Stretch

- Refresh tokens and logout.
- Rate limiting per user as a tower layer you write yourself.
- OpenAPI docs generated from the code.
- A background task that fetches each bookmark's page title.
- Metrics endpoint for Prometheus.

## Reading

- The Rust Book, ch. 17 (Fundamentals of Asynchronous Programming)
- The Tokio tutorial: <https://tokio.rs/tokio/tutorial>
- The axum docs on docs.rs and the `examples/` directory in the axum repository
- Zero To Production in Rust by Luca Palmieri. It uses a different framework; the engineering practices carry over.
- The sqlx README, especially the sections on migrations and offline mode
