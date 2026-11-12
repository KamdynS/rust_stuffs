# 04 · Calculator with a Tokenizer and Parser

> Level 4 of 12 · about 7 nights · crate: `projects/calc`

## Why this project

Enums and pattern matching are where Rust is most pleasant, and a parser uses them harder than anything else. You also meet heap allocation with `Box`, recursive data, and error values that carry position information. Project 10 builds directly on this.

## Rust concepts

- Recursive enums and why they need `Box`
- Exhaustive `match`: nested patterns, bindings, guards, `matches!`
- Matching through references and boxes
- `char` methods, `Peekable` iterators, byte offsets vs characters
- Implementing `Iterator` for your own type
- `FromStr`, `Display`, `Debug`
- Propagating `Result` through recursion
- `HashMap` as an environment; `&mut` threading through an evaluator
- Why `f64` is not `Eq` or `Hash`

## Dev skills

Lexing, parsing, operator precedence, abstract syntax trees, tree-walking evaluation, error messages that point at the problem, building a REPL.

## Allowed crates

None for the core. `rustyline` for line editing in the REPL (stretch). Dev: `proptest` (stretch).

## Spec

- `calc "1 + 2 * 3"` prints `7`. With no argument it starts a REPL.
- Operators: `+ - * / % ^`, unary minus, parentheses.
- Correct precedence and associativity. `2 ^ 3 ^ 2` is 512; `-2 ^ 2` is -4.
- Floating-point numbers, including `1.5e3`.
- Variables: `x = 3` assigns; later lines can use `x`. Constants `pi` and `e`.
- Functions: `sqrt`, `sin`, `cos`, `abs`, `min(a, b)`, `max(a, b)`.
- Errors never panic. They show the input with a caret under the offending position:

  ```
  1 + * 2
      ^ expected a number or '(', found '*'
  ```

- Division by zero and unknown names are errors, not `inf` or a crash.

## Milestones

- [ ] **M1 · Tokens.** A `Token` enum and a function from `&str` to a list of tokens or a lex error. Tests for numbers, operators, identifiers, whitespace and bad characters.
- [ ] **M2 · AST and evaluator.** An `Expr` enum. Build a tree by hand in a test and write `eval`. Find out why the compiler rejects the recursive enum and what `Box` changes.
- [ ] **M3 · Parser.** Recursive descent or precedence climbing, from tokens to `Expr`. Test precedence and associativity thoroughly.
- [ ] **M4 · Good errors.** Every token carries a span. Lex, parse and eval errors are one enum that implements `Display` and renders the caret diagnostic.
- [ ] **M5 · Variables and functions.** An environment passed to `eval`. Assignment. Built-in functions with arity checking.
- [ ] **M6 · REPL and printing.** The REPL loop. Implement `Display` for `Expr` so that printing and re-parsing gives back the same tree; test that round trip.
- [ ] **M7 · Lazy lexer.** Turn the tokenizer into a struct that implements `Iterator`, and have the parser pull from a `Peekable` instead of a `Vec`.

## Traps when coming from other languages

- A type that contains itself has infinite size. `Box` gives it a fixed size by putting the child on the heap.
- Matching on a `Box<Expr>` needs a deref; matching on `&Expr` binds references. Read the error, then read about match ergonomics.
- Moving a child out of a `Box` inside a `match` consumes the parent. Decide whether `eval` takes `&Expr` or `Expr` and understand the difference.
- String slices use byte offsets. Slicing in the middle of a multi-byte character panics.
- Deep recursion can overflow the stack: try 100,000 nested parentheses.
- Deriving `PartialEq` on an AST holding `f64` works; deriving `Eq` or `Hash` does not.

## Done when

- [ ] Everything in the spec works, including the caret diagnostics.
- [ ] No input can make it panic. Try empty input, `((((`, `1e999`, emoji.
- [ ] The parser has at least 20 tests, including error cases.
- [ ] There is no wildcard `_ =>` arm hiding unhandled variants in `eval`.
- [ ] You can explain where every `Box` in your AST is and why it is there.
- [ ] You can draw the ownership of the tree for `1 + 2 * 3`.

## Stretch

- Rewrite the parser as a Pratt parser.
- User-defined functions: `f(x) = x ^ 2 + 1`.
- Property test: for random trees, `parse(print(tree)) == tree`.
- Compile to bytecode for a small stack machine (a preview of project 10).
- Fuzz the parser with `cargo fuzz`.

## Reading

- The Rust Book: ch. 6 (Enums and Pattern Matching), ch. 15.1 (`Box`), ch. 19 (Patterns and Matching)
- Crafting Interpreters, the chapters on scanning, representing code, and parsing expressions: <https://craftinginterpreters.com>
- "Simple but Powerful Pratt Parsing" by Alex Kladov (matklad), for the stretch goal
