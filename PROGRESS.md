# Progress

**Current project:** 01 · grep clone
**Next step:** Start 01 M2. Add `clap` (derive feature) to `projects/rgrep`, create `src/main.rs` with a derive struct for PATTERN and a single PATH, read the file, and print what `search` returns. Exit codes 0/1/2 and stderr come after that.

## Ladder

| #  | Project | Status | Started | Finished |
|----|---------|--------|---------|----------|
| 01 | grep clone | in progress (M1 done) | 2026-11-18 | |
| 02 | Your own data structures | not started | | |
| 03 | HTTP server from scratch | not started | | |
| 04 | Monkey interpreter | not started | | |
| 05 | Redis clone | not started | | |

Milestone checkboxes live in each project's doc under `docs/`.

## Concepts I can explain without looking anything up

Add a line when a concept clicks. If you cannot explain it to someone else, it does not go here yet.

- `read_line(&mut buf)`: borrow, don't move, so the caller keeps ownership of `buf`; `mut` because the callee writes into it. It also keeps the trailing `\n`.
- Lifetimes in return types: in `fn search<'a>(input: &'a str, query: &str) -> Vec<&'a str>`, `'a` says the returned `&str`s point into `input`, so they may not outlive it. The query gets no `'a` because nothing in the output points into it, which lets the caller drop or reuse it right after the call. The annotation changes nothing at run time; it describes a relationship the compiler checks. You can't return a `&str` to a `String` you created inside the function, because that `String` is dropped before the caller could use the reference.

## Session log

Newest first. One line per night: date, what was done, what was learned, exact next step.

- **2026-11-20** · Expanded project 02 from 4 structures / 10 milestones to the full interview list: arrays, matrix, singly/doubly/circular lists, stack, linear/circular/double-ended/priority queues, hash map, binary/N-ary/BST/AVL/red-black trees, heap, trie, union-find, graphs. Data structures only (algorithms live in a separate repo). 23 milestones, ~28 nights, weekly speed drills. Next: unchanged, finish 01 first.
- **2026-11-18 (late)** · Done: 01 M1. Created `projects/rgrep`; `search(input: &'a str, matching_string: &str) -> Vec<&'a str>` with 3 unit tests; empty query matches every line, like grep. Went from returning one concatenated `String` to returning `Vec<&str>` borrowed from the input. Learned: `String` vs `&str` (owned buffer vs view); the lifetime goes only on the input the output borrows from, so the caller can drop the query early; `Vec<T>` is a type, `vec![]` is a value; `vec![]` needs a type annotation in `assert_eq!` because `Vec`'s `PartialEq` is generic; compiler `help:` suggestions can lead you down the wrong path. Also wrote `notes/how-to-toil.md`. Next: start M2 (see top).
- **2026-11-18** · Restructured the ladder from 12 projects to 5: grep (extended warm-up), data structures, HTTP server, Monkey interpreter (Ball, ch. 1–4), Redis clone with an async on-ramp. Dropped the guessing game as too easy and deleted `projects/guess`. Next: start 01 M1.
- **2026-11-11** · Done: M2. Added `rand` 0.10.3; game loops until correct using `cmp` + `match` on `Ordering`; bad input doesn't panic (silently retries). Code closely follows Book ch. 2. Learned: rand 0.10 API (`rand::rng()`, `random_range`, trait `RngExt`). Open questions carried over: `u32` inference of `secret_number`; trait-in-scope for `random_range`. Next: answer those, then M3, then M4.
- **2026-11-10** · Done: M1. Created `projects/guess`, reads a line from stdin and echoes it; fmt/clippy/test clean; .gitignore set up. Learned: `println!` format strings are checked at compile time (`{ input }` is a build error, `{input}` is not); `_n` binds-but-silences vs `_` doesn't bind; `read_line` borrows the buffer via `&mut` and keeps the newline. Next: cleanup (prompt, `eprintln!`, trim), then M2 with `rand`.
