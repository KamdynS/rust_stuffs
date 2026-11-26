# Rust Ladder

Five projects, worked on a little every night. One warm-up, then four hard ones: data structures, a threaded HTTP server, an interpreter, and an async Redis clone. Between them they cover ownership, lifetimes, traits, smart pointers, concurrency, async and performance work, plus the standard interview set of data structures.

## The ladder

| #  | Project | Main Rust topics | Nights |
|----|---------|------------------|--------|
| 01 | [Command-Line Rust](docs/01-command-line-rust.md) (book, ch. 6–9) | clap, `BufRead`/`Write`, `anyhow`, iterators, enums, regex, binary tests | 11 |
| 02 | [Your own data structures](docs/02-data-structures.md) | generics, lifetimes in structs, `Rc<RefCell>`, std traits, const generics; every common structure, plus rebuild drills for Rust interview rounds | 24 |
| 03 | [HTTP server from scratch](docs/03-http-server.md) | TCP, threads, `Send`/`Sync`, `Arc<Mutex>`, channels | 11 |
| 04 | [Monkey interpreter](docs/04-monkey-interpreter.md) | large enums, Pratt parsing, shared environments, golden tests | 14 |
| 05 | [Redis clone](docs/05-redis-clone.md) | async/tokio, byte parsing, zero-copy, persistence, profiling | 20 |

At 30–60 minutes a night, five nights a week, that is roughly four months.

Project 04 follows Thorsten Ball's *Writing An Interpreter In Go* (chapters 1–4), ported to idiomatic Rust.

## Setup (once)

1. Install Rust with rustup: <https://rustup.rs>. The `rust-toolchain.toml` here pulls in `rustfmt` and `clippy`.
2. Install rust-analyzer in your editor.
3. Optional but useful: `cargo install cargo-watch cargo-expand`.

## Learn → drill → apply

Fluent Rust developers know a vocabulary of patterns cold: how to read a file line by line, how to write to any writer, how to collect a list of `Result`s. Discovering those through compiler errors is slow and doesn't stick, so the ladder teaches them like algorithm templates:

1. **Learn.** Each project doc opens with "Patterns for this project": a canonical example of each idiom in a toy context, why it's written that way, and the common mistake. Project 01's is written; the others list the pattern names and get written up when you start them. Learn each pattern when a milestone needs it, not the whole primer at once.
2. **Drill.** Rebuild each pattern from memory as a module in `drills/`, with a test, on a spaced schedule (1 day, 3 days, 1 week, 2 weeks, then when a new project uses it). `drills/README.md` tracks what's due.
3. **Apply.** Use the patterns in the milestones. Choosing between them and fitting them to the project is the hard part, and that's where you work things out yourself.

## How a night goes

1. Up to ten minutes on whatever drills are due.
2. Open `PROGRESS.md`. It says which milestone is next.
3. Open that project's doc. Read the milestone, not the whole doc, and learn any pattern it needs that you don't know yet.
4. Code for 30–60 minutes. Stop mid-task on purpose and write the next step down, so tomorrow starts fast.
5. `cargo fmt`, `cargo clippy`, `cargo test`, commit.

Start each project with `cargo new projects/<name>` from the repo root; the workspace picks it up automatically.

## Working with Claude Code

`CLAUDE.md` sets Claude up as a tutor: it teaches patterns directly, gives hints on design, explains errors and reviews code, and does not write your project or drill code unless you explicitly ask. Five commands are included:

- `/tonight` runs the drills that are due, then picks up where you left off and proposes tonight's slice.
- `/pattern <name>` teaches one pattern in the primer format and has you rebuild it as a drill.
- `/hint` gives the smallest nudge that could unblock you. Run it again for a bigger one.
- `/review` reviews your current work for correctness and idiomatic Rust.
- `/wrap` runs the checks, updates `PROGRESS.md` and the drills table, and quizzes you briefly.

## Rules of the ladder

- Type every line yourself, drills included.
- Learn patterns first, then try the milestone. Design decisions are yours to work out; when you hit a wall that isn't a pattern, read the chapter the doc points to.
- Each doc has an "Allowed crates" list. Stick to it; the limits are where the learning is.
- A project is done when its "Done when" list is true, not when it compiles.
- Finish the core milestones before stretch goals.
