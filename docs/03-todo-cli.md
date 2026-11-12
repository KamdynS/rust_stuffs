# 03 · Todo CLI with JSON Storage

> Level 3 of 12 · about 6 nights · crate: `projects/todo`

## Why this project

Data modelling the Rust way: structs, enums that carry data, `Option` instead of null, and errors as types. You also organise code into modules, write your first trait, and meet the two crates almost every Rust program uses: `serde` and `clap`.

## Rust concepts

- Structs, `impl` blocks, `&self` vs `&mut self` vs `self`
- Enums with data; `Option`; `match` and `if let`
- `Vec`, `HashMap`, iterator adapters for filtering and sorting
- Modules, `pub`, `use`, visibility as a design tool
- `#[derive(...)]`: `Debug`, `Clone`, `PartialEq`, `Serialize`, `Deserialize`
- Implementing `Display`
- Custom error enums with `thiserror`; `anyhow` at the top level
- How `?` converts errors through `From`
- Defining a trait and implementing it twice
- `Path` / `PathBuf`

## Dev skills

Serialization, schema evolution, safe file writes, subcommand-style CLIs, dependency injection for tests, continuous integration.

## Allowed crates

`clap` (derive), `serde` (derive), `serde_json`, `thiserror`, `anyhow`, one date crate of your choice (`chrono`, `time`, or `jiff`), `dirs`. Dev: `tempfile`, `assert_cmd`.

## Spec

- `todo add "Buy milk" [--due 2026-12-01] [--priority low|med|high]`
- `todo list` shows open tasks, sorted by priority then due date. `--all` includes done tasks.
- `todo done ID`, `todo rm ID`, `todo edit ID --text ... --due ... --priority ...`
- Tasks are stored in one JSON file in the user's data directory; `--file PATH` overrides it.
- IDs are stable: removing task 2 does not renumber task 3.
- A missing file means an empty list. A corrupt file produces a clear error and is **never overwritten**.
- Unknown ID produces an error message and exit code 1.

## Milestones

- [ ] **M1 · Model.** `Task`, `Priority`, and a `TaskList` with add / complete / remove / find methods. No I/O. Unit tests for each method. Notice which methods need `&mut self`.
- [ ] **M2 · Persistence.** Derive serde traits. Save to and load from a JSON file. Write a round-trip test using `tempfile`.
- [ ] **M3 · CLI.** `clap` subcommands as an enum; one `match` dispatches them.
- [ ] **M4 · Errors.** A `TodoError` enum with `thiserror` (not found, I/O, parse). Library functions return `Result<_, TodoError>`; `main` uses `anyhow` and adds context. Look at what `#[from]` generates.
- [ ] **M5 · Modules and a trait.** Split into `model`, `storage`, `cli` modules. Put storage behind a `Store` trait with a JSON-file implementation and an in-memory implementation used by tests. Make everything private that can be.
- [ ] **M6 · Polish and CI.** `Display` for `Task`. Sorting and filtering with iterator adapters. Atomic save: write a temp file, then rename. Add a GitHub Actions workflow that runs fmt, clippy with `-D warnings`, and tests.

## Traps when coming from other languages

- There is no null. A due date that may be absent is `Option<Date>`, and the compiler makes you handle `None`.
- There are no exceptions. Every fallible function says so in its return type.
- There is no inheritance. Shared behaviour comes from traits; shared data from composition.
- "Find the task, then change it" hits the borrow checker if you hold a shared borrow while mutating. Look at `iter_mut` and `find`.
- Indexing a `Vec` with `v[i]` moves nothing out; you get a place. To take ownership of an element use `remove`, `swap_remove`, or `mem::take`.
- A field added to `Task` later breaks old JSON files unless you plan for it (`#[serde(default)]`).

## Done when

- [ ] All commands in the spec work and survive a restart.
- [ ] Hand-corrupting the JSON file gives a helpful error and leaves the file untouched.
- [ ] `main.rs` contains no logic beyond parsing, dispatch and error printing.
- [ ] Tests use the in-memory store; nothing in the unit tests touches disk.
- [ ] CI is green.
- [ ] You can explain how `?` turned an `io::Error` into a `TodoError`.
- [ ] You can explain when to choose a generic `impl Store` parameter over `Box<dyn Store>`.

## Stretch

- `--json` output for scripting.
- `todo undo` (keep the previous state).
- Shell completions with `clap_complete`.
- A SQLite implementation of `Store` with `rusqlite`.
- Recurring tasks.

## Reading

- The Rust Book: ch. 5 (Structs), ch. 6 (Enums), ch. 7 (Modules), ch. 8 (Collections), ch. 9 (Error Handling), ch. 10.2 (Traits)
- serde: <https://serde.rs>
- READMEs of `thiserror` and `anyhow`
- Rust API Guidelines, the checklist page: <https://rust-lang.github.io/api-guidelines/>
