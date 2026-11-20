# Rust Ladder

Five projects, worked on a little every night. One warm-up, then four hard ones: data structures, a threaded HTTP server, an interpreter, and an async Redis clone. Between them they cover ownership, lifetimes, traits, smart pointers, concurrency, async and performance work, plus the standard interview set of data structures.

## The ladder

| #  | Project | Main Rust topics | Nights |
|----|---------|------------------|--------|
| 01 | [grep clone](docs/01-grep.md) | lifetimes, iterators, traits, `thiserror`/`anyhow`, tests | 6 |
| 02 | [Your own data structures](docs/02-data-structures.md) | generics, lifetimes in structs, `Rc<RefCell>`, std traits, const generics; every common structure, plus speed drills | 28 |
| 03 | [HTTP server from scratch](docs/03-http-server.md) | TCP, threads, `Send`/`Sync`, `Arc<Mutex>`, channels | 11 |
| 04 | [Monkey interpreter](docs/04-monkey-interpreter.md) | large enums, Pratt parsing, shared environments, golden tests | 14 |
| 05 | [Redis clone](docs/05-redis-clone.md) | async/tokio, byte parsing, zero-copy, persistence, profiling | 20 |

At 30–60 minutes a night, five nights a week, that is roughly four months.

Project 04 follows Thorsten Ball's *Writing An Interpreter In Go* (chapters 1–4), ported to idiomatic Rust.

## Setup (once)

1. Install Rust with rustup: <https://rustup.rs>. The `rust-toolchain.toml` here pulls in `rustfmt` and `clippy`.
2. Install rust-analyzer in your editor.
3. Optional but useful: `cargo install cargo-watch cargo-expand`.

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
- Finish the core milestones before stretch goals.
