# How to toil

## What to toil on

The ladder runs learn → drill → apply. Not everything is worth working out from first principles:

- **Patterns: learn them, don't toil.** If a fluent developer would write it the same way in any codebase (reading lines, writing to `impl Write`, collecting `Result`s), it's vocabulary. Read it in the project doc's primer or ask for `/pattern`, rebuild it from memory in `drills/`, and move on. Rediscovering it through compiler errors costs an evening and doesn't stick.
- **Domain facts: ask.** Exit-code conventions, what grep does, protocol details. Not Rust, not worth the time.
- **Design and application: toil.** Which types, who owns what, which error model, which of two patterns fits, and how to bend a pattern to this milestone. That's the problem the project is setting you; the moves below are for this.

If you're stuck and suspect the missing piece is a pattern, ask that first: "is there a standard way to do X?"

## Moves

Being frozen usually means trying to design the whole thing in your head. Toiling means making small moves that each give you information.

### When you don't know how to think about it

1. **Do it by hand.** Take a tiny input and solve it on paper or in a comment. Write down exactly what you keep at each step. The design questions show up while you're doing the work.
2. **Write the code that calls it first.** Write the line that calls the function and the line that uses its result. Once you see how it gets used, the type often becomes obvious.
3. **Write something wrong on purpose, quickly.** Guess, compile, read the error. A wrong guess plus an error message beats an empty file.
4. **Make it smaller.** Solve a smaller version (only the first match, only one line, a hard-coded input), then generalise.
5. **Go to the docs before asking anyone.** Read the std docs for the types and methods you're touching. Their signatures contain answers.
6. **Write down what you currently believe.** A two-line comment: "I think X because Y." Then test it.

Before every `cargo check` / `cargo test`: **predict what will happen.** Being wrong is the useful part.

### Am I toiling or stuck?

- **Toiling:** every 5–10 minutes you learn something new, even if it's only that an idea doesn't work. Keep going.
- **Stuck:** 20 minutes on the same wall with nothing new. Now ask, and say what you tried, so the answer can take the *next* small step instead of the first one.

It feels worse than asking. That uncomfortable feeling is what learning feels like from the inside, not a sign you're doing it wrong.
