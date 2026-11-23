# 04 · Monkey Interpreter

> Level 4 of 5 · about 14 nights · crate: `projects/monkey`

## Why this project

A programming language is the classic deep project. It tests how you model data with large enums and recursive trees, how you own graph-shaped runtime data (environments and closures), and how you test something with a huge input space.

You implement Monkey, following Thorsten Ball's *Writing An Interpreter In Go*: lexer, Pratt parser, tree-walking evaluator. The book's code is Go. Go has a garbage collector, interfaces, `nil`, and pointers you can share freely, and Rust has none of those in the same form. Turning the book's design into idiomatic Rust, rather than transliterating it, is the exercise. Read a chapter section, close the book, write the Rust.

## Rust concepts

- Large enums for tokens, AST nodes and runtime values; `Box` for recursion; `size_of` to keep them small
- Go interfaces (`Node`, `Expression`, `Statement`, `Object`) vs Rust enums vs trait objects, and when each fits
- A lexer over `&str`: bytes vs `char`, `Peekable`, implementing `Iterator`
- Pratt parsing in Rust: how you represent the book's prefix/infix function tables
- `Display` for the AST, used to test the parser
- Collecting several errors instead of stopping at the first one
- Returning control flow as values: `return` and runtime errors without exceptions or sentinel objects
- Shared, mutable, nested scopes: `Rc<RefCell<Environment>>`, what it costs, and what it leaks
- Function pointers vs closures vs `dyn Fn` for built-in functions
- `Hash` and `Eq` for a hash key type built from runtime values
- File-driven (golden) tests

## Dev skills

Language implementation (lexer, parser, evaluator), operator precedence, error reporting, a REPL, test suites driven by files, defending a program against arbitrary input.

## Allowed crates

None in the interpreter. `rustyline` for the REPL. Dev: `insta` for snapshot tests if you want them.
Not allowed: parser generators or combinator crates (`nom`, `pest`, `lalrpop`, `chumsky`, ...).

## Patterns for this project

To be written up in the primer format (canonical example in a toy context, why it's written that way, the common mistake) when you start this project. Learn each one, rebuild it in `drills/`, then use it in the milestones.

- [ ] Recursive enums with `Box`
- [ ] Implementing `Iterator` for a lexer
- [ ] `Peekable`, `next_if` and `peek` for lookahead
- [ ] `Display` for a recursive tree
- [ ] Collecting several errors into a `Vec` instead of stopping at the first
- [ ] `?` through a recursive tree walk
- [ ] Nested scopes with `Rc<RefCell<...>>` parent links
- [ ] Function pointers, closures and `dyn Fn` side by side
- [ ] Deriving `Hash` and `Eq` for a key enum
- [ ] File-driven (golden) tests
- [ ] Checking type sizes with `size_of`

## Spec

Everything in the book's chapters 1–4:

- `monkey` starts a REPL; `monkey run file.mk` runs a file.
- Integers, booleans, strings, arrays, hash maps, `null`.
- `let` bindings, `return`, `if`/`else` as expressions.
- Prefix `!` and `-`; infix `+ - * / < > == !=` with correct precedence; string concatenation; grouping with parentheses.
- First-class functions, closures, higher-order functions, recursion.
- Index expressions on arrays and hashes.
- Built-ins: `len`, `first`, `last`, `rest`, `push`, `puts`.
- Parse errors report what was expected and where (line and column, which the book does not do), and the parser reports more than one error per run.
- Runtime errors (type mismatch, unknown identifier, wrong arity, unusable hash key) stop cleanly with a message.
- No input can make the interpreter panic or overflow the stack.

Macros from the "Lost Chapter" are out of scope.

## Milestones

- [ ] **M1 · Lexer.** Token type and a lexer, with the book's tests ported. Add line and column to tokens now; it is painful to retrofit.
- [ ] **M2 · AST and statements.** The AST types, parsing `let` and `return`, and `Display` for the tree. Decide how you are mapping the book's interfaces.
- [ ] **M3 · Pratt parser.** Identifiers, literals, prefix and infix operators, precedence. Port the book's precedence tests and add your own.
- [ ] **M4 · Rest of the parser.** Booleans, grouping, `if`, function literals, calls. Errors collected and reported together.
- [ ] **M5 · Evaluator core.** A value type, integers, booleans, prefix and infix, `if`, `return`, runtime errors. Model `return` and errors as values the type system tracks, not as the book's wrapper objects checked at every step.
- [ ] **M6 · Environments and closures.** Bindings, functions, calls, closures. Write the book's counter/adder closure tests. Then write a recursive function and work out whether your environments leak. Write your conclusion in `notes/`.
- [ ] **M7 · Strings and built-ins.** String values and concatenation, and the built-in function mechanism with `len`.
- [ ] **M8 · Arrays and hashes.** Array literals, indexing, `first`/`last`/`rest`/`push`, hash literals, hashable keys. Run the book's `map` and `reduce` written in Monkey.
- [ ] **M9 · REPL, runner, golden tests.** The REPL and `monkey run`. A test harness that runs every `.mk` file in a directory and compares its output with an expected-output file beside it.
- [ ] **M10 · Hardening and write-up.** Recursion depth limits in the parser and evaluator. Throw garbage at it (empty input, `((((((`, huge integers, emoji, unterminated strings) until nothing panics. Check `size_of` for your AST and value types and shrink anything oversized. A short README: architecture, and what did not translate from Go.

## Traps when coming from Go

- Go's `interface` plus type switches looks like a trait object. A closed set of node kinds is usually an enum. Ask which one you actually need.
- The book returns `nil` from a parse function when it fails. In Rust that is `Option` or `Result`, and the choice changes how errors flow.
- The book shares `*Environment` pointers freely and the GC cleans up after it. With `Rc` you get shared ownership, but cycles are never freed.
- A function value that captures the environment it is stored in is a cycle.
- Integer overflow panics in debug builds. User input can contain `9223372036854775807 + 1`.
- Go's `map[HashKey]Object` relies on a hand-computed hash. Rust's `Hash` derive can do this for you, if your key type can implement `Eq` honestly.
- Deep recursion in a tree walker uses the native stack. The default main thread stack is small; decide on a limit.
- The book's parser keeps `curToken` and `peekToken` and copies tokens around freely. Watch for `clone()` creeping in, and ask whether each one is needed.

## Done when

- [ ] Every example program in the book's chapters 1–4 runs with the expected output, as golden tests.
- [ ] No input you can think of makes it panic.
- [ ] The parser has at least 30 tests, including error cases.
- [ ] There is no wildcard `_ =>` arm in the evaluator hiding unhandled variants.
- [ ] You can explain where every `Box` and every `Rc` in your design is, and why.
- [ ] You can explain how a closure keeps its captured variables alive, and whether your implementation leaks.
- [ ] You can name two places where the Go design did not fit Rust and what you did instead.

## Stretch

- The book's "Lost Chapter" macro system.
- Break closure cycles: an arena of environments indexed by ID, or a simple mark-and-sweep over it.
- Floats, `while` loops, or assignment to existing bindings.
- Tokens that borrow from the source (`Token<'src>`), and measuring whether it was worth it.
- Benchmark `fib(25)`, profile, and make one change that measurably helps.
- *Writing A Compiler In Go*: a bytecode compiler and VM, using the same test suite.

## Reading

- *Writing An Interpreter In Go* by Thorsten Ball, chapters 1–4
- The Rust Book: ch. 15 (Smart Pointers), ch. 18.2 (trait objects)
- "Simple but Powerful Pratt Parsing" by Alex Kladov (matklad). Read it **after** M3, as a comparison.
- Crafting Interpreters, chapter 8 ("Statements and State") and the resolver chapter, for another view of environments
