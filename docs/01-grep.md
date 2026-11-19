# 01 · grep Clone

> Level 1 of 5 · about 6 nights · crate: `projects/rgrep`

## Why this project

This is the one warm-up before the hard projects. It is a small, real tool, and it makes you use the groundwork everything later relies on: borrowed return values and their lifetimes, iterators over streaming input, a trait with two implementations, typed errors with `thiserror` and `anyhow`, and tests at both the unit and the binary level. You have read the Book, so the milestones skip the naive version and expect you to make design decisions early.

## Rust concepts

- Borrowing from inputs: returning `&str` slices of the caller's data, and the lifetime that says so
- Why that design breaks when the input becomes a stream of owned lines
- Iterators: `lines`, `filter`, `map`, `enumerate`, `take`, laziness, `collect` and its type annotation
- Closures and the three ways they capture (`Fn`, `FnMut`, `FnOnce`)
- Writing over any `impl BufRead` / `impl Write` instead of files and stdout
- Defining a trait and implementing it twice; generic `impl Trait` vs `Box<dyn Trait>`
- `Box<dyn Error>` first, then a `thiserror` enum in the library and `anyhow` with context in `main`
- How `?` converts errors through `From`
- Library/binary split: logic in `src/lib.rs`, a thin `src/main.rs`
- `VecDeque` as a bounded buffer
- Unit tests in the module, integration tests in `tests/`

## Dev skills

CLI design, exit codes that scripts can rely on, stdout vs stderr, working in a pipe, testing a compiled binary, GitHub Actions CI.

## Allowed crates

`clap` (derive), `thiserror`, `anyhow`. Dev: `assert_cmd`, `predicates`, `tempfile`.
Not allowed until stretch: `regex`, `walkdir`, `ignore`.

## Spec

- `rgrep PATTERN [PATH...]` prints every line containing PATTERN (a fixed string).
- With no PATH, reads stdin.
- With more than one PATH, or with `-r`, prefixes each line with the file name.
- Flags: `-i` ignore case, `-n` line numbers, `-v` invert, `-c` count only, `-r` recurse into directories, `-C N` print N lines of context around each match (with `--` between non-adjacent groups, as real grep does).
- Exit code 0 if anything matched, 1 if nothing matched, 2 if any error occurred.
- Errors go to stderr; a bad file does not stop the others from being searched.
- Binary files and invalid UTF-8 do not crash the program. Decide what to do and document it.

## Milestones

- [ ] **M1 · Library and lifetime.** `cargo new --lib`. A search function in `lib.rs` that takes the query and the contents and returns the matching lines **without copying them**. Write the tests first. Work out what your lifetime annotation is telling the compiler.
- [ ] **M2 · Thin binary and errors.** A `clap` derive struct, a `run` function returning `Result<_, Box<dyn Error>>`, `?` everywhere, exit codes 0/1/2, errors on stderr. `main.rs` holds no search logic.
- [ ] **M3 · Streaming.** Search over any `impl BufRead` and write to any `impl Write`, so stdin and huge files work and tests need no files. Rewrite the loop as an iterator chain. Your M1 signature cannot survive this; work out why before changing it.
- [ ] **M4 · Flags and a trait.** `-i`, `-n`, `-v`, `-c`. Put "does this line match?" behind a `Matcher` trait with at least two implementations. Choose generic or `dyn` for how the search takes a matcher, and be able to defend the choice.
- [ ] **M5 · Many files and typed errors.** Multiple paths and `-r` using `std::fs::read_dir`. Replace `Box<dyn Error>` with a `thiserror` enum in the library and `anyhow` with `.context(...)` in `main`. Look at what `#[from]` generates (`cargo expand` helps).
- [ ] **M6 · Context, integration tests, CI.** `-C N` using a bounded buffer of previous lines. Tests under `tests/` that run the binary and assert on stdout, stderr and exit code. A GitHub Actions workflow running fmt, clippy with `-D warnings`, and tests.

## Traps when coming from other languages

- You cannot return a reference to something created inside the function. When the compiler says something "does not live long enough", ask who owns it.
- A lifetime annotation changes nothing at run time. It describes a relationship that the compiler then checks.
- `String` cannot be indexed with `s[0]`. Strings are UTF-8; think in bytes, chars or slices.
- `to_lowercase` allocates and is not a byte-for-byte mapping for all of Unicode. Decide whether that matters for `-i`.
- `BufRead::lines` returns `io::Result<String>` per line and fails on invalid UTF-8. Look at `read_until` or `split` if you want bytes.
- Iterator chains do nothing until consumed.
- With `-C`, after-context and before-context overlap. Get the edge cases into unit tests before touching the binary.

## Done when

- [ ] `cat file | rgrep -n -i foo` works.
- [ ] `rgrep foo missing.txt real.txt; echo $?` reports the missing file on stderr, still searches `real.txt`, and exits 2.
- [ ] `rgrep -r -C 2 fn src/` output matches real `grep -r -C 2 -F fn src/` on your own source tree, apart from ordering.
- [ ] No `unwrap` or `expect` outside tests.
- [ ] Search logic is unit-tested without touching the file system.
- [ ] CI is green.
- [ ] You can explain what `'a` meant in your M1 signature and why M3 removed it.
- [ ] You can explain how `?` turned an `io::Error` into your error type.
- [ ] You can say when you would pick `impl Matcher` over `Box<dyn Matcher>`, and the reverse.

## Stretch

- Regular expressions with `regex`, as a third `Matcher`.
- Respect `.gitignore` with `ignore`.
- Highlight matches in colour when stdout is a terminal (`std::io::IsTerminal`).
- Benchmark against real `grep` with `hyperfine` on a large file, in `--release`.

## Reading

- The Rust Book: ch. 10.3 (Lifetimes), ch. 12 (I/O Project), ch. 13 (Iterators and Closures). Skim; you have read them.
- Command Line Applications in Rust: <https://rust-cli.github.io/book/>
- READMEs of `thiserror` and `anyhow`
- The `std::iter::Iterator` method list, once, so you know what exists
