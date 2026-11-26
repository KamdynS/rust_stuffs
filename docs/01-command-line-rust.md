# 01 · Command-Line Rust

> Level 1 of 5 · about 11 nights · one crate per chapter under `projects/`
> Book: Ken Youens-Clark, *Command-Line Rust* (O'Reilly, 2024 update: clap v4, derive). Companion repo: <https://github.com/kyclark/command-line-rust>, branch `main`.

## Why this project

This is the warm-up before the hard projects, done as four chapters of the book (6–9; you did 1–3 earlier and they repeat what `rgrep` covered) instead of one home-grown grep. Each chapter is a small, real Unix tool with a ready-made test suite, so you get many reps of the same groundwork: clap, reading from a file or stdin, writing to stdout or a file, `anyhow` errors, iterator chains, enums for options, and binary-level tests. The book repeats its core patterns in almost every chapter, so that repetition replaces the drill schedule for this project.

`projects/rgrep` (M1–M3 of the old grep project) is retired; it's in git history before 2026-11-26. Chapter 9 builds grep again, properly.

## Rust concepts

- Clap derive structs, validation, and turning arguments into a config type
- `fn main` that calls `run` and maps its `Result` to an exit code
- `anyhow::Result` and `?` in a binary
- File-or-stdin input behind one type; stdout-or-file output behind one type
- `BufRead` line reading that keeps or drops line endings, and why that matters for byte counts
- State carried across loop iterations
- Enums for options; `match` over them
- Closures and iterator adaptors (`filter`, `filter_map`, `map`, `collect`)
- Parsing user input into your own types, with errors that say what was wrong
- Ranges and slicing by bytes vs chars
- Regular expressions; walking a directory tree
- Integration tests that run the compiled binary (`assert_cmd`, `predicates`)

## Dev skills

CLI design, exit codes, stdout vs stderr, test-driven work against someone else's test suite, reading a reference solution critically.

## Allowed crates

Whatever the chapter's `Cargo.toml` in the companion repo lists, and nothing else:

| Chapter | Dependencies | Dev-dependencies |
|---------|--------------|------------------|
| 6 uniqr | `anyhow`, `clap` | `assert_cmd`, `predicates`, `pretty_assertions`, `rand`, `tempfile` |
| 7 findr | `anyhow`, `clap`, `regex`, `walkdir` | as ch. 6, minus `tempfile` |
| 8 cutr | `anyhow`, `clap`, `csv`, `regex` | as ch. 6, minus `tempfile` |
| 9 grepr | `anyhow`, `clap`, `regex`, `walkdir` | as ch. 6, minus `tempfile`, plus `sys-info` |

Copy the dev-dependency versions exactly as the repo pins them (`rand` is 0.8 there), since the book's tests are written against those APIs. For runtime dependencies use the current release; the workspace already has clap 4.6.

## How to work a chapter

1. The crate already exists (`projects/uniqr`, `findr`, `cutr`, `grepr`), each with `cargo new`'s placeholder `main.rs`, the book's `tests/` copied from the companion repo, its dev-dependencies, and `anyhow` (the tests import it). Add the other runtime dependencies yourself with `cargo add` when the chapter calls for them. Don't open the repo's `src/`.
2. `cargo test -p <tool>` fails until you write the tool. That's the starting point.
3. Read the chapter **up to the point where the author starts writing the solution**. Stop there. Concept explanations and the spec are fair game; the author's code for the tool isn't, yet.
4. Write your own version until `cargo test -p <tool>` passes. The 20-minute rule and the hint ladder in `CLAUDE.md` apply. Run the tool by hand next to the real Unix one too.
5. Clean up: `cargo fmt`, `cargo clippy -p <tool> --all-targets -- -D warnings`, no `unwrap`/`expect` outside tests.
6. **Now** read the author's solution. Write one or two lines in the session log: what they did differently, and which version you think is better and why. This comparison is the code review.
7. Tick the milestone.

Note: the repo's crates use edition 2021 and yours use 2024. Almost nothing differs at this level, but if a snippet from the book behaves differently for you, the edition is one suspect.

Beyond the book's tests: in every chapter, add at least one unit test of your own for a pure function (parsing, counting, formatting), with no file or terminal I/O.

## Patterns for this project

Reference, not a drill list: the book teaches its own versions of most of these, and its repetition across chapters does the drill's job, so there are no drills for project 01. Read an entry when the book's explanation doesn't click, or when you're comparing your solution with the author's. The examples are toy programs, none of them a piece of any chapter's tool. Patterns 6 (`thiserror`) and 7 (`VecDeque`) aren't used by the chosen chapters; 6 comes back in project 02 and 7 in the optional tailr.

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

## Milestones

Each milestone is one chapter: the chapter's tests pass, fmt and clippy are clean, you've compared your version with the author's, and you can answer its question without looking anything up.

- [ ] **M1 · Ch. 6, uniqr** (`projects/uniqr`). Input and output both chosen at run time, `-c`, state across lines. Question: what's the ownership story of the "previous line" you keep between iterations?
- [ ] **M2 · Ch. 7, findr** (`projects/findr`). Enums for entry types, regex names, walking directories, iterator chains. Question: where did you pick a chain over a loop, or the reverse, and why?
- [ ] **M3 · Ch. 8, cutr** (`projects/cutr`). Parsing position lists into your own type, bytes vs chars vs fields. Question: what does your parsed type rule out that a `Vec<String>` of raw arguments wouldn't?
- [ ] **M4 · Ch. 9, grepr** (`projects/grepr`). Regex matching, recursion, invert, count, many files. Question: compare grepr with your old `rgrep` (in git history): what would you design differently now?

Optional, after M4 or whenever you want one more: ch. 11 **tailr** (seeking, reading from the end, performance). The other chapters (4 headr, 5 wcr, 10 commr, 12–14) are skippable; 1–3 are done.

## Done when

- [ ] All four milestones ticked.
- [ ] No `unwrap` or `expect` outside tests in any of the crates.
- [ ] Each chapter has at least one unit test of your own that needs no file system.
- [ ] Every chapter has a log line comparing your solution with the author's.
- [ ] You can explain `Box<dyn BufRead>` vs `impl BufRead` vs a generic, and when each is right.
- [ ] You can explain how `?` turns an `io::Error` into an `anyhow::Error`.

## Reading

- The book, chapters 6–9 (1–3 already done).
- Command Line Applications in Rust: <https://rust-cli.github.io/book/>
- READMEs of `clap` (derive tutorial), `anyhow`, `regex`, `walkdir`.
- The `std::iter::Iterator` method list, once, so you know what exists.
