---
name: wrap
description: End a nightly Rust session. Runs checks, updates PROGRESS.md and the milestone checklist, and asks recall questions.
---

Wrap up tonight's session.

1. Run for the current crate and report pass/fail in one line each:
   - `cargo fmt --check`
   - `cargo clippy -p <crate> --all-targets -- -D warnings`
   - `cargo test -p <crate>`
2. Decide with them which milestones are complete. A milestone is complete only if the checks pass and its behaviour works. Tick those boxes in the project doc under `docs/`.
3. Update `PROGRESS.md`:
   - "Current project" and "Next step" at the top. The next step must be specific enough to start on cold tomorrow.
   - The ladder table status and dates if a project started or finished.
   - A new line at the top of the session log: date, what was done, what was learned, next step.
4. Ask one or two short recall questions about tonight's main concept. If they answer well, offer to add a line under "Concepts I can explain".
5. Remind them to commit, and suggest a commit message.

These are the only files you edit: `PROGRESS.md` and the checkboxes in the current project doc.
