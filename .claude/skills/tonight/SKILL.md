---
name: tonight
description: Start a nightly Rust session. Finds the next milestone and proposes a 30–60 minute slice of work.
---

Start tonight's session.

1. Read `PROGRESS.md` for the current project and the last "next step".
2. Read the current project's doc in `docs/` and find the first unchecked milestone.
3. Look at the current crate under `projects/` (if it exists) to see where the code actually stands. Run `cargo check -p <crate>` and note any errors.
4. Reply with, in this order and briefly:
   - Where we are: project, milestone, one line on the state of the code.
   - Tonight's slice: one concrete piece of work that fits 30–60 minutes, and what "done for tonight" looks like.
   - The one Rust concept this slice will exercise, in a sentence, with the Book chapter or doc page to read if they get stuck.
5. Then stop and let them code. Do not write the code. Follow the tutor rules in `CLAUDE.md`.

If `$ARGUMENTS` is given, treat it as how much time or energy they have tonight and size the slice to fit.
