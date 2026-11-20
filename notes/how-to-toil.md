# How to toil

Being frozen usually means trying to design the whole thing in your head. Toiling means making small moves that each give you information.

## Moves when you don't know how to think about it

1. **Do it by hand.** Take a tiny input and solve it on paper or in a comment. Write down exactly what you keep at each step. The design questions show up while you're doing the work.
2. **Write the code that calls it first.** Write the line that calls the function and the line that uses its result. Once you see how it gets used, the type often becomes obvious.
3. **Write something wrong on purpose, quickly.** Guess, compile, read the error. A wrong guess plus an error message beats an empty file.
4. **Make it smaller.** Solve a smaller version (only the first match, only one line, a hard-coded input), then generalise.
5. **Go to the docs before asking anyone.** Read the std docs for the types and methods you're touching. Their signatures contain answers.
6. **Write down what you currently believe.** A two-line comment: "I think X because Y." Then test it.

Before every `cargo check` / `cargo test`: **predict what will happen.** Being wrong is the useful part.

## Am I toiling or stuck?

- **Toiling:** every 5–10 minutes you learn something new, even if it's only that an idea doesn't work. Keep going.
- **Stuck:** 20 minutes on the same wall with nothing new. Now ask, and say what you tried, so the answer can take the *next* small step instead of the first one.

It feels worse than asking. That uncomfortable feeling is what learning feels like from the inside, not a sign you're doing it wrong.
