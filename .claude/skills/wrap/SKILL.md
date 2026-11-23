---
name: wrap
description: End a nightly Rust session. Runs checks, updates PROGRESS.md, the milestone checklist and the drills table, and asks recall questions.
---

Wrap up tonight's session.

1. Run for the current crate and report pass/fail in one line each:
   - `cargo fmt --check`
   - `cargo clippy -p <crate> --all-targets -- -D warnings`
   - `cargo test -p <crate>`
   If a drill was written or rebuilt tonight, also run `cargo test -p drills`.
2. Decide with them which milestones are complete. A milestone is complete only if the checks pass and its behaviour works. Tick those boxes in the project doc under `docs/`.
3. New patterns. Ask whether a new pattern came up tonight. Also name any you noticed yourself: an idiom they reached for after a compiler error, something you taught with `/pattern`, or a std or crate idiom they'll need again. For each one they agree is a pattern (same in any codebase, not a design decision for this project), add a row to `drills/README.md`: the pattern name, the source doc (link to its primer entry if it has one; otherwise the doc where it came up), today as "first learned", and the rebuild dates at +1, +3, +7 and +14 days. If it isn't in that doc's primer yet, add it to the "Patterns for this project" list there. Don't write the drill module; the first rebuild creates it.
4. Update `PROGRESS.md`:
   - "Current project" and "Next step" at the top. The next step records where they stopped and what comes next (milestone, open question, a half-finished piece), specific enough to pick up cold tomorrow. Say *what* comes next, not *how* to go about it: no procedures or rituals ("write a note first", "read X before touching code") unless they asked for one tonight.
   - The ladder table status and dates if a project started or finished.
   - A new line at the top of the session log: date, what was done (including tonight's drill and its result), what was learned, next step.
5. Ask one or two short recall questions about tonight's main concept. If they answer well, offer to add a line under "Concepts I can explain".
6. Remind them to commit, and suggest a commit message.

These are the only files you edit: `PROGRESS.md`, `drills/README.md`, the checkboxes in the current project doc, and its "Patterns for this project" list.
