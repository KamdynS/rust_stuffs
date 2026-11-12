# Rust Ladder

Twelve projects, easiest to hardest, built a little every night. By the end you will have used nearly every part of Rust and built the things working developers actually build: CLIs, an HTTP server, a REST API, a database, an interpreter.

## The ladder

| #  | Project | Main Rust topics | Nights |
|----|---------|------------------|--------|
| 01 | [Guessing game and unit converter](docs/01-guessing-game-and-converter.md) | cargo, variables, `match`, first ownership | 3–4 |
| 02 | [grep clone](docs/02-grep-clone.md) | borrowing, `Result` and `?`, iterators, tests | 6 |
| 03 | [Todo CLI with JSON storage](docs/03-todo-cli.md) | structs, enums, modules, serde, error types | 6 |
| 04 | [Calculator with tokenizer and parser](docs/04-calculator.md) | recursive enums, `Box`, pattern matching | 7 |
| 05 | [Your own data structures](docs/05-data-structures.md) | generics, traits, lifetimes, `Rc<RefCell>` | 10 |
| 06 | [HTTP server from scratch](docs/06-http-server.md) | TCP, threads, `Arc<Mutex>`, channels | 11 |
| 07 | [REST API with axum, sqlx, tokio](docs/07-rest-api.md) | async/await, database, middleware, auth | 13 |
| 08 | [WebSocket chat server or web crawler](docs/08-chat-or-crawler.md) | tasks, `select!`, backpressure, cancellation | 10 |
| 09 | [Redis clone](docs/09-redis-clone.md) | byte parsing, zero-copy, persistence, profiling | 18 |
| 10 | [Interpreter plus a derive macro](docs/10-interpreter-and-macros.md) | trait objects, lifetimes, macros | 22 |
| 11 | [Own `Vec`, arena allocator, C binding](docs/11-unsafe-and-ffi.md) | `unsafe`, raw pointers, FFI, Miri | 14 |
| 12 | [Capstone](docs/12-capstone.md) | Raft KV store, SQLite clone, or async runtime | 40+ |

At 30–60 minutes a night, five nights a week, the first eleven take roughly six months.

## Setup (once)

1. Install Rust with rustup: <https://rustup.rs>. The `rust-toolchain.toml` here pulls in `rustfmt` and `clippy`.
2. Install rust-analyzer in your editor.
3. `git init`, commit, push somewhere. Commit at the end of every session.
4. Optional but useful: `cargo install cargo-watch cargo-expand`.

## How a night goes

1. Open `PROGRESS.md`. It says which milestone is next.
2. Open that project's doc. Read the milestone, not the whole doc.
3. Code for 30–60 minutes. Stop mid-task on purpose and write the next step down, so tomorrow starts fast.
4. `cargo fmt`, `cargo clippy`, `cargo test`, commit.

Start each project with `cargo new projects/<name>` from the repo root; the workspace picks it up automatically.

## Working with Claude Code

`CLAUDE.md` sets Claude up as a tutor: it gives hints, explains errors and reviews code, and does not write the solutions unless you explicitly ask. Four commands are included:

- `/tonight` picks up where you left off and proposes tonight's slice.
- `/hint` gives the smallest nudge that could unblock you. Run it again for a bigger one.
- `/review` reviews your current work for correctness and idiomatic Rust.
- `/wrap` runs the checks, updates `PROGRESS.md`, and quizzes you briefly.

## Rules of the ladder

- Type every line yourself.
- Try before you read. The docs name a concept; attempt the milestone, then read the chapter when you hit the wall.
- Each doc has an "Allowed crates" list. Stick to it; the limits are where the learning is.
- A project is done when its "Done when" list is true, not when it compiles.
- Finish the core milestones before stretch goals. Come back to stretch goals when a later project feels too steep.
