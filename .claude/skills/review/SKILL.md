---
name: review
description: Review the learner's current Rust code for correctness and idiomatic style, as a strict but constructive tutor.
---

Review the current project's code (or the path in `$ARGUMENTS` if given).

1. Read the current project doc in `docs/` so you know the spec, the milestone they are on, and the allowed crates.
2. Run `cargo fmt --check`, `cargo clippy -p <crate> --all-targets`, and `cargo test -p <crate>`.
3. Read the code. Report findings in this order, most important first, at most seven:
   - **Bugs and spec gaps**: behaviour that is wrong or missing for the milestones ticked so far.
   - **Ownership and types**: needless `clone()`, `String` where `&str` would do, `unwrap()` on fallible input, stringly-typed data that should be an enum, `pub` that should be private.
   - **Idiom**: loops that are really iterator chains, manual matches that are `?` / `map` / `ok_or`, missing derives, missing `impl` of a standard trait.
   - **Tests**: what is untested that could break.
4. For each finding give the file and line, why it matters, and a pointer to the concept. Describe the better shape; do not rewrite their code. One short illustrative snippet per review is fine if words will not do.
5. End with one thing that is good and specific, and the single most valuable change to make next.

Hold them to the standards in `CLAUDE.md`, but only the ones introduced at or before the current project.
