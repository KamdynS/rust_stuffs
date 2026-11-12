# 01 · Guessing Game and Unit Converter

> Level 1 of 12 · 3–4 nights · crates: `projects/guess`, `projects/convert`

## Why this project

Two tiny programs to get the toolchain working and meet the syntax. Nothing here is hard for an experienced developer, so use the spare attention to notice where Rust differs from what you know.

## Rust concepts

- `cargo new`, `cargo run`, `cargo check`, `cargo add`, `cargo fmt`, `cargo clippy`
- `let` vs `let mut`, shadowing, constants
- Scalar types, and the absence of implicit numeric conversion
- Expressions vs statements; the last expression is the return value
- `match`, `loop`, `while`, `for`, ranges
- `String` vs `&str` (first contact)
- `Result` and `Option` as ordinary enums you match on
- First taste of ownership: passing `&mut String` to `read_line`
- A first `enum` and a first `#[test]`

## Dev skills

Reading stdin, parsing command-line arguments, exit codes, writing to stderr.

## Allowed crates

`rand` (guessing game only). Everything else from `std`.

## Spec

**Guessing game** (`guess`)

- Picks a random integer from 1 to 100.
- Prompts for a guess and answers "too low", "too high", or "correct".
- Non-numeric input prints a message and asks again. It never crashes.
- `q` quits.
- On a win, prints how many guesses it took.

**Unit converter** (`convert`)

- `convert 100 c f` prints `212 f`. Arguments: value, from-unit, to-unit.
- Supports temperature (c, f, k), length (m, km, mi, ft), mass (kg, lb).
- Converting between incompatible units (km to kg) is an error.
- Bad input prints a usage message to stderr and exits with a non-zero code.

## Milestones

- [x] **M1 · Toolchain.** Install Rust. `cargo new projects/guess`. Read a line from stdin and echo it back. Run `cargo fmt` and `cargo clippy` once so you know what they do.
- [x] **M2 · The game.** Random number, parse the guess, compare with `match`, loop until correct.
- [ ] **M3 · Harden it.** Replace every `unwrap`/`expect` on user input with a `match` on the `Result`. Add the guess counter and `q` to quit.
- [ ] **M4 · Converter core.** `cargo new projects/convert`. Model units as an `enum`. Write a pure function that converts a value between two units and returns an `Option` or `Result`. Write tests for it before wiring up the CLI.
- [ ] **M5 · Converter CLI.** Parse `std::env::args`, call the function, print the result. Usage message and exit code on bad input.

## Traps when coming from other languages

- Variables are immutable unless you say `mut`. Shadowing with a second `let` is a different thing from mutation.
- `if` is an expression; there is no ternary operator.
- A trailing semicolon turns a value into `()`. "Expected `i32`, found `()`" usually means a stray semicolon.
- No implicit conversion between `i32`, `u32`, `f64`. Convert explicitly with `as` or `From`/`into`.
- `read_line` appends to the buffer and keeps the newline. Trim it, and clear the buffer between reads.
- Integer overflow panics in debug builds and wraps in release builds.
- The `rand` API has changed between major versions. Follow the docs on docs.rs for the version `cargo add` gave you; older tutorials may not compile.

## Done when

- [ ] Typing letters, an empty line, or `-5` into the game never panics.
- [ ] The converter's logic has tests and no I/O inside it.
- [ ] `cargo clippy -- -D warnings` is clean for both crates.
- [ ] You can explain why shadowing lets `guess` change type from `String` to `u32`.
- [ ] You can explain why `read_line` takes `&mut guess` and not `guess`.

## Stretch

- Difficulty levels with a limited number of guesses.
- Converter: accept `100c` and `100 C` forms; add a `--list` flag.
- Make invalid states unrepresentable: can the type system stop `km` to `kg` at compile time?

## Reading

- The Rust Book, ch. 1–3, and ch. 2 (Programming a Guessing Game): <https://doc.rust-lang.org/book/>
- Skim ch. 4 (Understanding Ownership). It will make more sense after project 02.
- Rustlings exercises: `variables`, `functions`, `if`, `primitive_types`.
