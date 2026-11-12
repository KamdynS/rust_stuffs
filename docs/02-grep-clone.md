# 02 · grep Clone

> Level 2 of 12 · about 6 nights · crate: `projects/rgrep`

## Why this project

A small, real tool that forces you through ownership and borrowing properly, introduces Rust's error handling, and gets you fluent with iterators. The Rust Book builds a simpler version in chapter 12; yours goes further. Try each milestone before reading the chapter.

## Rust concepts

- Ownership, moves, `&` and `&mut` borrows, slices
- Your first lifetime annotation, and why the compiler needs it
- `Result<T, E>`, the `?` operator, `Box<dyn Error>`
- Library/binary split: `src/lib.rs` plus a thin `src/main.rs`
- Iterators: `lines`, `filter`, `map`, `enumerate`, `collect`; laziness
- Closures and what they capture
- Traits as bounds: writing a function over any `impl BufRead`
- Unit tests in the module, integration tests in `tests/`
- `std::fs`, `std::io`, `std::env`, `std::process`

## Dev skills

CLI design, exit codes that scripts can rely on, stdout vs stderr, reading from stdin so the tool works in a pipe, argument parsing with `clap`.

## Allowed crates

- `clap` with the `derive` feature (from M4)
- Dev-dependencies for testing the binary: `assert_cmd`, `predicates`, `tempfile`
- Not allowed until stretch: `regex`, `walkdir`

## Spec

- `rgrep PATTERN [PATH...]` prints every line containing PATTERN.
- With no PATH, reads stdin.
- With more than one PATH, prefixes each line with the file name.
- Flags: `-i` ignore case, `-n` show line numbers, `-v` invert match, `-c` print only a count.
- Exit code 0 if anything matched, 1 if nothing matched, 2 on error. This is what real grep does.
- Errors (missing file, unreadable file) go to stderr; a bad file does not stop the others from being searched.

## Milestones

- [ ] **M1 · Naive version.** Parse two arguments by hand, read the file into a `String`, print matching lines. All in `main`.
- [ ] **M2 · Library and lifetime.** Move the search into `lib.rs` as a function that takes the query and the contents and returns the matching lines **without copying them**. Write the tests first. You will need a lifetime annotation; work out what it is telling the compiler.
- [ ] **M3 · Errors.** `main` calls a `run` function that returns `Result<_, Box<dyn Error>>`. Use `?` everywhere. Map results to exit codes 0/1/2. Errors go to stderr.
- [ ] **M4 · clap and flags.** Replace hand parsing with a `clap` derive struct. Implement `-i`, `-n`, `-v`, `-c`.
- [ ] **M5 · Iterators and streaming.** Rewrite the search loop as an iterator chain. Then change the search to work line by line over any `impl BufRead` instead of a whole `String`, so stdin and huge files work.
- [ ] **M6 · Many files and integration tests.** Multiple paths with file-name prefixes. Add tests under `tests/` that run the compiled binary and assert on stdout, stderr and exit code.

## Traps when coming from other languages

- You cannot return a reference to something created inside the function. If the compiler says a value "does not live long enough", ask who owns it.
- A lifetime annotation does not change how long anything lives. It describes a relationship the compiler then checks.
- `String` cannot be indexed with `s[0]`. Strings are UTF-8; think in bytes, chars, or slices.
- Iterator chains do nothing until consumed. `collect` often needs a type annotation.
- `&String` coerces to `&str`. Take `&str` in function parameters.
- When M5 switches to streaming, each line is a fresh `String`, so the M2 "return borrowed lines" design no longer works as is. Working out why is the lesson.

## Done when

- [ ] `cat file | rgrep -n -i foo` works.
- [ ] `rgrep foo missing.txt real.txt; echo $?` reports the missing file on stderr, still searches `real.txt`, and exits 2.
- [ ] No `unwrap` or `expect` outside tests.
- [ ] Search logic is tested without touching the file system.
- [ ] You can explain what `'a` means in your M2 signature, and why M5 changed it.
- [ ] You can explain the three ways a closure can capture a variable.

## Stretch

- `-r` recursive search, first with `std::fs::read_dir`, then with `walkdir`.
- Regular expressions with the `regex` crate.
- Highlight the match in colour when stdout is a terminal (`std::io::IsTerminal`).
- Benchmark against real `grep` with `hyperfine`. Build with `--release` first.

## Reading

- The Rust Book: ch. 4 (Ownership), ch. 9 (Error Handling), ch. 10.3 (Lifetimes), ch. 11 (Tests), ch. 12 (I/O Project), ch. 13 (Iterators and Closures)
- Command Line Applications in Rust: <https://rust-cli.github.io/book/>
- `std::iter::Iterator` docs. Read the list of methods once so you know what exists.
