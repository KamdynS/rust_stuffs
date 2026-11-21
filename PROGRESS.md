# Progress

**Current project:** 01 · grep clone
**Next step:** Start 01 M3 (streaming). Before touching code, write 3–4 sentences in `notes/` on why the M1 signature `search<'a>(input: &'a str, pattern: &str) -> Vec<&'a str>` can't work when the input is an `impl BufRead`. Then sketch the new signature taking `impl BufRead` and `impl Write`, and read the `BufRead` and `Write` trait pages (`rustup doc --path std::io::BufRead`).

## Ladder

| #  | Project | Status | Started | Finished |
|----|---------|--------|---------|----------|
| 01 | grep clone | in progress (M1–M2 done) | 2026-11-18 | |
| 02 | Your own data structures | not started | | |
| 03 | HTTP server from scratch | not started | | |
| 04 | Monkey interpreter | not started | | |
| 05 | Redis clone | not started | | |

Milestone checkboxes live in each project's doc under `docs/`.

## Concepts I can explain without looking anything up

Add a line when a concept clicks. If you cannot explain it to someone else, it does not go here yet.

- `read_line(&mut buf)`: borrow, don't move, so the caller keeps ownership of `buf`; `mut` because the callee writes into it. It also keeps the trailing `\n`.
- Lifetimes in return types: in `fn search<'a>(input: &'a str, query: &str) -> Vec<&'a str>`, `'a` says the returned `&str`s point into `input`, so they may not outlive it. The query gets no `'a` because nothing in the output points into it, which lets the caller drop or reuse it right after the call. The annotation changes nothing at run time; it describes a relationship the compiler checks. You can't return a `&str` to a `String` you created inside the function, because that `String` is dropped before the caller could use the reference.
- `PathBuf` vs `String` for paths: `String` must be valid UTF-8, but OS paths needn't be (Linux filenames are arbitrary bytes; `$'\xff.txt'` is legal). With a `String` field, clap rejects such an argument before `main`'s logic runs; with `PathBuf` (an `OsString` underneath) it passes through and only the OS can reject it. `fs::read_to_string` has one bound, `P: AsRef<Path>`, which `String`, `&str`, `PathBuf` and `&Path` all satisfy; a bare `Path` is unsized, like `str`, so you pass `&Path`.
- `?` on an `Err` returns early after converting the error with `From::from(err)` (same effect as `err.into()`, since `Into` comes free with every `From`). std has a blanket `impl<E: Error> From<E> for Box<dyn Error>`, so any error type boxes itself automatically in a function returning `Box<dyn Error>`.
- `match` is exhaustive: the compiler requires an arm for every value the type can hold. So model outcomes with a type that holds exactly the states you have. `Result<RunOutput, Box<dyn Error>>` with a two-variant enum has exactly three cases (`Ok(FoundLine)`, `Ok(NotFound)`, `Err(_)`), so `main` needs no `_` arm. An integer would force a catch-all for values that should never happen.

## Session log

Newest first. One line per night: date, what was done, what was learned, exact next step.

- **2026-11-20 (late)** · Done: 01 M2. `main() -> ExitCode` maps `run`'s result: `Ok(RunOutput::FoundLine)` → 0, `Ok(RunOutput::NotFound)` → 1, `Err` → message on stderr + 2. `run(cli) -> Result<RunOutput, Box<dyn Error>>` uses `?` on `read_to_string`; PATH is now required (dropped `default_value = ""`, clap exits 2 with usage). Exit codes checked by hand with `echo $?` (match/no-match/missing file/no PATH); automated binary tests wait for M6 under `tests/`. Learned: `main`'s return type must implement `Termination`; `ExitCode::FAILURE` is 1, so grep's 2 needs `ExitCode::from(2)`; tail expression vs early `return`; `match` must be exhaustive, so pick a type with exactly the states you need (enum over integer); `run` says *what happened*, `main` decides *how to tell the OS*, so the enum variants carry no `ExitCode`; `Error` is a trait, not a type: use a concrete type, `impl Error`, or `Box<dyn Error>`; `?` calls `From::from`, which boxes any `E: Error`. Also added "Toil is for Rust only" to CLAUDE.md. Next: see top.
- **2026-11-20 (evening)** · 01 M2, part 1: replaced the clap tutorial code with a `Cli` derive struct (`pattern: String`, `file_name: PathBuf`); `main` reads one file and prints what `search` returns; no search logic in `main.rs`. Added `projects/rgrep/fixtures/` (poem.txt, log.txt) for manual runs. Learned: `fs::read_to_string` takes `impl AsRef<Path>`, which is why `String` compiled; `String` must be UTF-8 but OS paths needn't be (`OsString`/`PathBuf`), so `PathBuf` is the honest type; `main` can return `Result<(), Box<dyn Error>>` without anyhow; anyhow/thiserror wait until M5 on purpose; offline std docs via `rustup doc --path <item>` + `w3m`. Open: what should a missing PATH do, and what exit code does `main` use when it returns `Err`? Next: see top.
- **2026-11-20** · Expanded project 02 from 4 structures / 10 milestones to the full interview list: arrays, matrix, singly/doubly/circular lists, stack, linear/circular/double-ended/priority queues, hash map, LRU cache, binary/BST/AVL trees, heap and priority queue, trie, union-find, graphs. Data structures only, aimed at longer Rust interview rounds (timed DSA practice is in Python, in a separate repo). 21 milestones, ~24 nights, weekly rebuild drills with narrated ownership choices. Red-black and N-ary trees moved to stretch. Next: unchanged, finish 01 first.
- **2026-11-18 (late)** · Done: 01 M1. Created `projects/rgrep`; `search(input: &'a str, matching_string: &str) -> Vec<&'a str>` with 3 unit tests; empty query matches every line, like grep. Went from returning one concatenated `String` to returning `Vec<&str>` borrowed from the input. Learned: `String` vs `&str` (owned buffer vs view); the lifetime goes only on the input the output borrows from, so the caller can drop the query early; `Vec<T>` is a type, `vec![]` is a value; `vec![]` needs a type annotation in `assert_eq!` because `Vec`'s `PartialEq` is generic; compiler `help:` suggestions can lead you down the wrong path. Also wrote `notes/how-to-toil.md`. Next: start M2 (see top).
- **2026-11-18** · Restructured the ladder from 12 projects to 5: grep (extended warm-up), data structures, HTTP server, Monkey interpreter (Ball, ch. 1–4), Redis clone with an async on-ramp. Dropped the guessing game as too easy and deleted `projects/guess`. Next: start 01 M1.
- **2026-11-11** · Done: M2. Added `rand` 0.10.3; game loops until correct using `cmp` + `match` on `Ordering`; bad input doesn't panic (silently retries). Code closely follows Book ch. 2. Learned: rand 0.10 API (`rand::rng()`, `random_range`, trait `RngExt`). Open questions carried over: `u32` inference of `secret_number`; trait-in-scope for `random_range`. Next: answer those, then M3, then M4.
- **2026-11-10** · Done: M1. Created `projects/guess`, reads a line from stdin and echoes it; fmt/clippy/test clean; .gitignore set up. Learned: `println!` format strings are checked at compile time (`{ input }` is a build error, `{input}` is not); `_n` binds-but-silences vs `_` doesn't bind; `read_line` borrows the buffer via `&mut` and keeps the newline. Next: cleanup (prompt, `eprintln!`, trim), then M2 with `rand`.
