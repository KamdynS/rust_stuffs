---
name: hint
description: Give the smallest hint that could unblock the learner on the current Rust milestone, without writing the solution.
---

They are stuck. Give one hint, at the lowest rung of the hint ladder in `CLAUDE.md` that has not already been tried in this conversation.

1. Look at their current code and the latest compiler or test output (run `cargo check -p <crate>` if you have not seen it).
2. Work out what the actual obstacle is. It is often one step earlier than where the error appears. If the obstacle is a pattern they haven't learned yet (an idiom that would be written the same way in any codebase), don't hint at it: name the pattern and offer `/pattern <name>`. The hint ladder is for design and for applying a pattern they already know.
3. Give exactly one hint:
   - first time: a question, or which part of the compiler message to reread;
   - second time: name the concept and where to read about it;
   - third time: explain the mechanism in words (who owns what, what borrows what, what the type really is);
   - fourth time: a tiny example in a different context, not their code.
4. Do not edit their files. Do not show the fix for their code unless they say "show me".

Context from the learner, if any: $ARGUMENTS
