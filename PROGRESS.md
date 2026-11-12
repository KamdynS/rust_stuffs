# Progress

**Current project:** 01 · Guessing game and unit converter
**Next step:** Answer the two open questions (why `secret_number` infers `u32`; why removing `use rand::prelude::*` breaks `random_range`). Then M3 in `projects/guess/src/main.rs`: print a message on bad input instead of silent `continue`, replace the `read_line` `expect` with a `match`, add a guess counter and `q` to quit. Then M4: `cargo new projects/convert`, unit `enum`, pure convert fn, tests first.

## Ladder

| #  | Project | Status | Started | Finished |
|----|---------|--------|---------|----------|
| 01 | Guessing game and unit converter | in progress | 2026-11-10 | |
| 02 | grep clone | not started | | |
| 03 | Todo CLI with JSON storage | not started | | |
| 04 | Calculator with tokenizer and parser | not started | | |
| 05 | Your own data structures | not started | | |
| 06 | HTTP server from scratch | not started | | |
| 07 | REST API with axum, sqlx, tokio | not started | | |
| 08 | WebSocket chat server or web crawler | not started | | |
| 09 | Redis clone | not started | | |
| 10 | Interpreter plus a derive macro | not started | | |
| 11 | Own `Vec`, arena allocator, C binding | not started | | |
| 12 | Capstone | not started | | |

Milestone checkboxes live in each project's doc under `docs/`.

## Concepts I can explain without looking anything up

Add a line when a concept clicks. If you cannot explain it to someone else, it does not go here yet.

- `read_line(&mut buf)`: borrow, don't move, so the caller keeps ownership of `buf`; `mut` because the callee writes into it. It also keeps the trailing `\n`.

## Session log

Newest first. One line per night: date, what was done, what was learned, exact next step.

- **2026-11-11** · Done: M2. Added `rand` 0.10.3; game loops until correct using `cmp` + `match` on `Ordering`; bad input doesn't panic (silently retries). Code closely follows Book ch. 2. Learned: rand 0.10 API (`rand::rng()`, `random_range`, trait `RngExt`). Open questions carried over: `u32` inference of `secret_number`; trait-in-scope for `random_range`. Next: answer those, then M3, then M4.
- **2026-11-10** · Done: M1. Created `projects/guess`, reads a line from stdin and echoes it; fmt/clippy/test clean; .gitignore set up. Learned: `println!` format strings are checked at compile time (`{ input }` is a build error, `{input}` is not); `_n` binds-but-silences vs `_` doesn't bind; `read_line` borrows the buffer via `&mut` and keeps the newline. Next: cleanup (prompt, `eprintln!`, trim), then M2 with `rand`.
