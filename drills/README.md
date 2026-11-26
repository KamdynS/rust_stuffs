# Drills

Patterns you should be able to write cold. Each one is a module in `src/` that you retype from memory on a schedule, with a test that checks it.

## How a drill works

**Learning it.** Run `/pattern <name>` (or read the pattern in its project doc). Claude names a toy task. Close the doc and write the signature from memory in `src/<name>.rs` with a `todo!()` body, then add `pub mod <name>;` to `src/lib.rs`. Claude checks the signature: if it's off, you're told where and how much it matters, not what to write, and you fix it. Then Claude writes the `#[cfg(test)] mod tests` against your signature, and you write the body. You don't write the tests: the signature and the body are the pattern, and the `assert_eq!`s aren't. Add a row to the table below. If the first attempt needed the doc open, that's fine; the rebuilds are what make it stick.

**Rebuilding it.** Delete everything in the file above `#[cfg(test)]`, signature included. Keep the tests: they now also check that you remembered the same API. Retype the code with no docs and no editor completion beyond what you'd have in an interview. Then:

```
cargo test -p drills <name>
git diff drills/src/<name>.rs
```

The test says whether it works; the diff against the last version shows what you remembered differently. Aim for about five minutes per drill. Then mark the date done in the table.

If a drill keeps failing, the pattern isn't learned yet: reread it in the doc, and treat the next rebuild as a first one.

Drills follow the same rules as the projects: no I/O in tests (pass in readers and writers), clippy clean, and only the crates the source doc allows.

## Schedule

Rebuild 1 day, 3 days, 1 week and 2 weeks after first learning it, then once more when you start a project that uses it. Write the four dates when you add the row (`MM-DD`) and add ✓ after each one you complete. Late is fine; don't shift the later dates.

Learn patterns just in time: when the milestone you're on needs them, one or two a night. Each new pattern adds four rebuilds over the next two weeks, so learning a whole primer in one sitting makes them all come due on the same nights. `/tonight` spends up to 10 minutes on drills, most overdue first, and rolls the rest to the next night.

Example: learned 11-23 → `11-24 · 11-26 · 11-30 · 12-07`.

A row with no "first learned" date is queued: it's in a primer but you haven't learned it yet.

The larger structure rebuilds in project 02 ("Rebuild drills" in its doc) keep their own weekly rotation and stay out of this crate.

## Drills

| Pattern | Source doc | First learned | Rebuild dates |
|---------|------------|---------------|---------------|
| Reading lines from a file or stdin | [01](../docs/01-grep.md#1--reading-lines-from-a-file-or-stdin) | 11-24 | 11-25 · 11-27 · 12-01 · 12-08 |
| Writing to `&mut impl Write` | [01](../docs/01-grep.md#2--writing-to-mut-impl-write) | | |
| Collecting `Result`s into `Result<Vec<_>, _>` | [01](../docs/01-grep.md#3--collecting-results-into-resultvec_-_) | | |
| Iterator chains, and when a loop is clearer | [01](../docs/01-grep.md#4--iterator-chains-and-when-a-loop-is-clearer) | | |
| `impl Trait`, generics and `dyn` in arguments | [01](../docs/01-grep.md#5--impl-trait-generics-and-dyn-in-arguments) | | |
| A `thiserror` enum in the library, `anyhow` in the binary | [01](../docs/01-grep.md#6--a-thiserror-enum-in-the-library-anyhow-in-the-binary) | | |
| A bounded window with `VecDeque` | [01](../docs/01-grep.md#7--a-bounded-window-with-vecdeque) | | |
