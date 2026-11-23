---
name: pattern
description: Teach one Rust pattern or idiom in the primer format (canonical toy example, why it's written that way, the common mistake), then have the learner rebuild it from memory as a drill.
---

Teach the pattern named in `$ARGUMENTS`. If no name is given, use the next queued pattern for the current project (first row in `drills/README.md` with no "first learned" date whose source doc is the current project's).

1. **Find it.** Read `PROGRESS.md` for the current project, then look for the pattern in that project's doc under "Patterns for this project", and in the other docs if it's not there.
   - Written up already: teach from that entry.
   - Listed but not written up (the 02–05 checklists): write the entry into that doc's primer first, in the same format as `docs/01-grep.md`, then teach from it.
   - Not listed anywhere: teach it, and add it to the current project's list once they agree it's worth drilling.
2. **Check for spoilers** before showing anything. The example must be a toy program, never their crate, and must not answer any milestone in any project doc. If the pattern is so close to the current milestone that showing it would hand over a design decision, say so and ask how they want to handle it. If the request is really a design question for their project ("should my search take `dyn`?"), say it's design, teach the mechanics only, and leave the decision to them.
3. **Check the API.** For a third-party crate, check the version in `Cargo.lock` (or the latest release if it isn't a dependency yet) and the docs for that version. Compile the example in a scratch crate outside the workspace before showing it.
4. **Teach it**, briefly, in this order:
   - The canonical example: 10–25 lines, a toy context, compiled.
   - Why it's written that way: 2–3 sentences on the Rust reasons (ownership, the trait involved, what the types guarantee). Map it to what they'd write in another language where that helps, and say where the analogy breaks.
   - The common mistake, and what it costs.
   This is direct teaching: no Socratic questions here, no hint ladder.
5. **Rebuild from memory.** Ask them to close the example and write `drills/src/<module>.rs` themselves: the pattern in a toy context of their own, plus a `#[cfg(test)]` test that checks the behaviour the pattern exists for. They add `pub mod <module>;` to `drills/src/lib.rs`. Don't write it for them. When they're done, run `cargo test -p drills <module>`, compare against the canonical form, and point out differences directly.
6. **Record it.** Add or update the row in `drills/README.md`: today as "first learned" and rebuild dates at +1, +3, +7 and +14 days (`MM-DD`).
7. Then send them back to the milestone. Applying the pattern to their project is theirs to work out; the tutor rules in `CLAUDE.md` apply from here.
