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

## Patterns for this project

Idioms a fluent Rust developer writes without thinking. Learn each one here, rebuild it from memory in `drills/`, then use it in the milestones. The examples use toy programs on purpose: none of them is a piece of rgrep, and fitting them to rgrep's needs is still your job.

1. Reading lines from a file or stdin
2. Writing to `&mut impl Write`
3. Collecting `Result`s into `Result<Vec<_>, _>`
4. Iterator chains, and when a loop is clearer
5. `impl Trait`, generics and `dyn` in arguments
6. A `thiserror` enum in the library, `anyhow` in the binary
7. A bounded window with `VecDeque`

### 1 · Reading lines from a file or stdin

```rust
use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::path::Path;

fn longest_line(path: &Path) -> io::Result<usize> {
    let reader = BufReader::new(File::open(path)?);
    let mut longest = 0;
    for line in reader.lines() {
        let line = line?;
        longest = longest.max(line.len());
    }
    Ok(longest)
}

fn count_blank_stdin() -> io::Result<usize> {
    let mut blank = 0;
    for line in io::stdin().lock().lines() {
        if line?.trim().is_empty() {
            blank += 1;
        }
    }
    Ok(blank)
}
```

`File` does no buffering, so each small read is a system call; `BufReader` reads in large chunks and gives you `BufRead`, which is where `lines()` lives. Stdin already has a buffer inside it, and `lock()` hands you a `StdinLock` that implements `BufRead` directly. Each item from `lines()` is an `io::Result<String>`: a new owned `String` with the `\n` or `\r\n` removed, or an error (including invalid UTF-8), so you need `?` on every line, not just on the open. The common mistake is `.map(|l| l.unwrap())` to make the `Result` go away. When allocating a `String` per line matters, the alternative is one buffer reused with `read_line`; that one appends, so the buffer needs `clear()` each time round.

### 2 · Writing to `&mut impl Write`

```rust
use std::io::{self, BufWriter, Write};

fn write_scores(out: &mut impl Write, scores: &[(&str, u32)]) -> io::Result<()> {
    for (name, points) in scores {
        writeln!(out, "{name}: {points}")?;
    }
    Ok(())
}

fn main() -> io::Result<()> {
    let mut out = BufWriter::new(io::stdout().lock());
    write_scores(&mut out, &[("ada", 3), ("lin", 5)])?;
    out.flush()
}

#[test]
fn writes_one_line_per_score() {
    let mut buf = Vec::new();
    write_scores(&mut buf, &[("ada", 3)]).unwrap();
    assert_eq!(buf, b"ada: 3\n");
}
```

A function that calls `println!` can only write to stdout, and a test can't see what it wrote. Taking a writer lets the caller choose: stdout in `main`, a `Vec<u8>` in a test, a file or a socket later. It takes `&mut` rather than the writer by value because writing needs mutable access but shouldn't consume the writer: the caller still owns it afterwards, can write more, and (in the test) can inspect the bytes. std does the same (`io::copy` takes `&mut W`). `writeln!` returns an `io::Result`, and forgetting the `?` gives an "unused `Result`" warning, not an error, so writes can fail silently. `BufWriter` flushes when dropped, but an error during that flush is silently discarded, so call `flush()` yourself at the end.

### 3 · Collecting `Result`s into `Result<Vec<_>, _>`

```rust
use std::io::{self, BufRead};
use std::num::ParseIntError;

fn parse_ports(words: &[&str]) -> Result<Vec<u16>, ParseIntError> {
    words.iter().map(|w| w.parse::<u16>()).collect()
}

fn all_lines(reader: impl BufRead) -> io::Result<Vec<String>> {
    reader.lines().collect()
}
```

`collect` builds whatever the target type says, and `Result<V, E>` implements `FromIterator<Result<T, E>>` whenever `V` can be collected from `T`s. It stops at the first `Err` and returns it; otherwise you get `Ok` with every value. The same works for `Option`, and for `sum` and `product`. The common mistakes are collecting into `Vec<Result<_, _>>` and then looping to unwrap, and leaving out the target type: `collect` cannot guess it, so it comes from the return type, a `let` annotation, or a turbofish (`.collect::<Result<Vec<_>, _>>()`).

### 4 · Iterator chains, and when a loop is clearer

```rust
let text = "the quick brown fox jumped over everything lazily";
let long_words: Vec<(usize, &str)> = text
    .split_whitespace()
    .enumerate()
    .filter(|(_, word)| word.len() > 5)
    .map(|(i, word)| (i + 1, word))
    .take(2)
    .collect();
assert_eq!(long_words, [(5, "jumped"), (7, "everything")]);
```

Read it top to bottom as a pipeline. Nothing runs until `collect` (or `for`, `count`, `sum`...) pulls items through, and `take(2)` stops pulling after two, so the rest of the input is never looked at. `filter`'s closure receives a reference to each item (`&(usize, &str)` here), while `map`'s receives the item itself; that is why the two closures look alike but bind different types, and why `filter` closures sometimes need `|&x|` or a `*`. When the filter and the map are one step ("keep it if this parse works"), use `filter_map`. Prefer a plain `for` loop when the body has several fallible steps that each need `?`, updates more than one accumulator, or breaks on a condition that's awkward to express as `take_while`. A chain that needs comments to follow is worse than the loop it replaced.

### 5 · `impl Trait`, generics and `dyn` in arguments

```rust
trait Notify {
    fn send(&self, msg: &str) -> String;
}

fn ping_impl(n: &impl Notify) -> String { n.send("ping") }
fn ping_generic<N: Notify>(n: &N) -> String { n.send("ping") }
fn ping_dyn(n: &dyn Notify) -> String { n.send("ping") }

fn ping_all(all: &[Box<dyn Notify>]) -> Vec<String> {
    all.iter().map(|n| n.send("ping")).collect()
}
```

The first two are the same thing: `impl Trait` in argument position is an anonymous generic parameter. The compiler makes a separate copy of the function for each concrete type (monomorphisation), so calls are direct and can be inlined; the cost is compile time and binary size. Name the generic when you need to refer to it: two arguments of the same type, a `where` clause, or a caller choosing the type with a turbofish (which `impl Trait` arguments don't allow). `dyn Notify` is one compiled function that calls through a vtable. The concrete type can be decided at run time, and one `Vec<Box<dyn Notify>>` can hold an `Email` next to an `Sms`, which no generic can do. `dyn` always sits behind a pointer (`&dyn`, `Box<dyn>`) because its size isn't known at compile time, and only dyn-compatible traits qualify (no generic methods, no returning `Self`). The common mistakes are reaching for `Box<dyn>` by Java or C# reflex when one type per call site would do, and writing `Vec<impl Trait>` expecting it to hold mixed types.

### 6 · A `thiserror` enum in the library, `anyhow` in the binary

Library side (thiserror 2):

```rust
use std::{fs, io, num::ParseIntError, path::Path};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum ConfigError {
    #[error("could not read config file")]
    Io(#[from] io::Error),
    #[error("port {value:?} is not a number")]
    BadPort {
        value: String,
        #[source]
        source: ParseIntError,
    },
}

pub fn load_port(path: &Path) -> Result<u16, ConfigError> {
    let text = fs::read_to_string(path)?;
    let value = text.trim();
    value.parse().map_err(|source| ConfigError::BadPort {
        value: value.to_string(),
        source,
    })
}
```

Binary side (anyhow 1):

```rust
use anyhow::{Context, Result};
use std::path::Path;

fn main() -> Result<()> {
    let path = Path::new("server.conf");
    let port = load_port(path)
        .with_context(|| format!("loading {}", path.display()))?;
    println!("listening on {port}");
    Ok(())
}
```

A library returns an enum so callers can `match` on what went wrong. A binary mostly wants a readable report, and `anyhow::Error` holds any error plus a chain of context messages. `#[error(...)]` writes the `Display` impl; `#[from]` also generates a `From` impl, which is what lets `?` convert; `#[source]` records the cause without the `From`. `with_context` takes a closure so the message is only built if there is an error (`context` takes a value and builds it every time). The common mistake is repeating the source's text in your own message (`"could not read config: {0}"`): the chain already prints the cause, so it shows up twice. Printed with `{:#}`, the chain is `loading server.conf: could not read config file: No such file or directory (os error 2)`.

### 7 · A bounded window with `VecDeque`

```rust
use std::collections::VecDeque;

struct Recent {
    items: VecDeque<String>,
    cap: usize,
}

impl Recent {
    fn new(cap: usize) -> Self {
        Recent { items: VecDeque::with_capacity(cap), cap }
    }

    fn push(&mut self, item: String) {
        self.items.push_back(item);
        if self.items.len() > self.cap {
            self.items.pop_front();
        }
    }

    fn iter(&self) -> impl Iterator<Item = &String> {
        self.items.iter()
    }
}
```

`VecDeque` is a ring buffer: pushing or popping at either end is O(1), and with the capacity reserved up front the window never reallocates. It keeps the last `cap` items in arrival order, and iterating gives oldest first (`.iter().rev()` for newest first). The common mistake is a `Vec` with `remove(0)`, which shifts every element on each push.

## Spec

- `rgrep PATTERN [PATH...]` prints every line containing PATTERN (a fixed string).
- With no PATH, reads stdin.
- With more than one PATH, or with `-r`, prefixes each line with the file name.
- Flags: `-i` ignore case, `-n` line numbers, `-v` invert, `-c` count only, `-r` recurse into directories, `-C N` print N lines of context around each match (with `--` between non-adjacent groups, as real grep does).
- Exit code 0 if anything matched, 1 if nothing matched, 2 if any error occurred.
- Errors go to stderr; a bad file does not stop the others from being searched.
- Binary files and invalid UTF-8 do not crash the program. Decide what to do and document it.

## Milestones

- [x] **M1 · Library and lifetime.** `cargo new --lib`. A search function in `lib.rs` that takes the query and the contents and returns the matching lines **without copying them**. Write the tests first. Work out what your lifetime annotation is telling the compiler.
- [x] **M2 · Thin binary and errors.** A `clap` derive struct, a `run` function returning `Result<_, Box<dyn Error>>`, `?` everywhere, exit codes 0/1/2, errors on stderr. `main.rs` holds no search logic.
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
