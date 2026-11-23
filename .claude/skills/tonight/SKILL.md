---
name: tonight
description: Start a nightly Rust session. Runs the drills that are due (up to 10 minutes), then finds the next milestone and proposes a 30–60 minute slice of work.
---

Start tonight's session.

**Drills first (up to 10 minutes).**

1. Read `drills/README.md`. A drill is due if one of its rebuild dates is today or earlier and has no ✓. If tonight starts a new project, every drill that project's "Patterns for this project" list builds on is due too.
2. If nothing is due, say so in one line and go to the session below.
3. If several are due, take them most overdue first, one at a time, until about 10 minutes are used (usually two; a drill takes about 5). Whatever doesn't fit rolls to tomorrow; say which ones. If the backlog is more than about three nights' worth, say so and suggest learning fewer new patterns until it clears.
4. For each drill: name it and nothing else: no hints, no reminder of the pattern's shape. Ask them to clear everything above `#[cfg(test)]` in `drills/src/<module>.rs` (or create the file if this is the first rebuild of a pattern learned during project work) and write it from memory, docs closed.
5. When they say they're done, run `cargo test -p drills <module>` and `git diff drills/src/<module>.rs`, and compare what they wrote against the canonical form in the source doc's primer. Report: tests pass or fail, and each difference that matters (a missing `?`, a needless `clone`, a different but equally good form). Patterns are taught directly, so show the canonical line where theirs differs.
6. Tick the date (add ✓) in `drills/README.md`. If they needed help to finish, don't tick it; say it stays due.

**Then the session.**

1. Read `PROGRESS.md` for the current project and the last "next step".
2. Read the current project's doc in `docs/` and find the first unchecked milestone.
3. Look at the current crate under `projects/` (if it exists) to see where the code actually stands. Run `cargo check -p <crate>` and note any errors.
4. Reply with, in this order and briefly:
   - Where we are: project, milestone, one line on the state of the code.
   - Tonight's slice: one concrete piece of work that fits 30–60 minutes, and what "done for tonight" looks like.
   - The patterns from the doc's primer this slice applies, and whether any of them is still queued (not yet learned). If one is, offer `/pattern <name>` before they start. Only these: patterns are learned just in time, one or two a night, so their rebuilds come due staggered. Don't suggest learning ahead.
   - The one Rust concept this slice will exercise, in a sentence, with the Book chapter or doc page to read if they get stuck.
5. Then stop and let them code. Do not write the code. Follow the tutor rules in `CLAUDE.md`.

If `$ARGUMENTS` is given, treat it as how much time or energy they have tonight and size the slice to fit (the drill still comes first unless they say skip it).
