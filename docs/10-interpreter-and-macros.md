# 10 · Interpreter plus a Derive Macro

> Level 10 of 12 · about 22 nights · crates: `projects/lox`, `projects/lox-derive`

## Why this project

A programming language is the classic deep project: it stresses data modelling, ownership of graph-shaped runtime data (closures and scopes), performance, and testing. You build it twice, first as a tree-walking interpreter and then as a bytecode virtual machine, and in between you learn both kinds of Rust macros by using them to remove real boilerplate from your own code.

The language is Lox from Crafting Interpreters. The book's code is Java, then C. Translating its ideas into idiomatic Rust, where the book's designs often do not fit, is the exercise.

## Rust concepts

- Large enums; keeping them small (`size_of`, boxing big variants)
- Tokens that borrow from the source text (`Token<'src>`)
- Trait objects for callables (`Rc<dyn Callable>`)
- Shared, mutable, nested scopes: `Rc<RefCell<Environment>>`, and its costs
- Control flow as values: implementing `return` with `Result` or a dedicated enum
- `macro_rules!`: fragment specifiers, repetition, hygiene
- Procedural macros: `proc_macro`, `syn`, `quote`, spans, compile errors from macros
- `#[repr(u8)]`, converting between integers and enums
- A dispatch loop written for speed
- Snapshot and golden-file testing

## Dev skills

Language implementation (scanner, parser, resolver, evaluator, compiler, VM), error recovery, test harnesses driven by files, code generation, comparing designs by benchmark.

## Allowed crates

Interpreter: none (`rustyline` allowed for the REPL). Macro crate: `syn`, `quote`, `proc-macro2`. Dev: `trybuild`, `insta`, `criterion`. Tool: `cargo expand`.

## Spec

**Part A · Tree-walking interpreter** (`lox run file.lox`, and a REPL)

- Values: numbers, strings, booleans, nil.
- Expressions with the usual precedence; string concatenation; comparison and equality; `and` / `or` with short-circuiting.
- `var` declarations, assignment, block scoping.
- `if`, `while`, `for`.
- Functions with parameters, `return`, recursion, and closures that capture their environment correctly.
- Native functions: `clock()`, `len()`.
- `print` statement.
- Syntax errors report line and position; the parser recovers and reports more than one error per run.
- Runtime errors (type mismatch, undefined variable, wrong arity) report the line and stop cleanly.

**Part B · Macros**

- Two `macro_rules!` macros that each remove real repetition from Part A.
- A derive macro in its own crate, `#[derive(Opcode)]`, for a fieldless `#[repr(u8)]` enum. It generates `TryFrom<u8>`, a `name()` method for disassembly, and a constant with the number of variants. Using it on anything else is a compile error with a clear message pointing at the right span.

**Part C · Bytecode VM** (`lox run --vm file.lox`)

- A compiler from source to bytecode chunks and a stack-based VM that runs them.
- Supports everything in Part A up to and including functions. Closures are a stretch goal.
- A disassembler (`lox dis file.lox`) built on your derive macro.
- Same test suite, same expected output, as Part A.

## Milestones

Week 1: front end and expressions

- [ ] **M1 · Scanner.** Tokens whose lexemes borrow from the source string.
- [ ] **M2 · AST.** `Expr` and `Stmt` enums and a pretty printer.
- [ ] **M3 · Expression parser.** Reuse what you learned in project 04. Add error recovery.
- [ ] **M4 · Evaluator.** A `Value` enum, expression evaluation, runtime errors with line numbers.

Week 2: statements, functions, closures

- [ ] **M5 · Statements and scopes.** Variables, blocks, a chain of environments.
- [ ] **M6 · Control flow.** `if`, `while`, `for`, logical operators.
- [ ] **M7 · Functions.** A `Callable` trait, user functions and native functions, `return`.
- [ ] **M8 · Closures.** Functions capture their defining environment. This is where shared ownership becomes unavoidable.
- [ ] **M9 · Resolver.** A static pass that fixes the closure scoping bug described in the book.
- [ ] **M10 · Test harness.** Run every `.lox` file in a directory and compare output with the expected output stored beside it.

Week 3: macros

- [ ] **M11 · Declarative macros.** Write the two `macro_rules!` macros. Read their expansion with `cargo expand`.
- [ ] **M12 · Proc macro skeleton.** The `lox-derive` crate: parse the input with `syn`, emit a trivial impl with `quote`.
- [ ] **M13 · The real derive.** Generate `TryFrom<u8>`, `name()` and the count.
- [ ] **M14 · Errors and tests.** Reject structs and enums with fields via `syn::Error` with good spans. `trybuild` tests for both passing and failing cases.

Week 4 and beyond: virtual machine

- [ ] **M15 · Chunks.** Opcodes, a constant pool, line information, and the disassembler.
- [ ] **M16 · VM core.** The value stack and the dispatch loop for arithmetic.
- [ ] **M17 · Compiling expressions.** A single-pass compiler straight from tokens to bytecode.
- [ ] **M18 · Variables.** Globals, then locals as stack slots.
- [ ] **M19 · Jumps.** `if`, `while`, `for`, `and`, `or` with back-patching.
- [ ] **M20 · Functions.** Call frames, arguments, returns, natives.
- [ ] **M21 · Race.** Benchmark recursive `fib(30)` on both implementations. Profile the VM and improve it; check `size_of::<Value>()`.
- [ ] **M22 · Write-up.** A README describing the architecture and what was hard to express in Rust.

## Traps when coming from other languages

- The book leans on a garbage collector. Closures and environments form graphs, sometimes cycles. With `Rc` you will leak cycles; decide whether to accept that, break them, or use an arena.
- A struct cannot hold both a `String` and slices borrowing from that same `String`. Keep the source alive outside and borrow from it.
- The visitor pattern from the book is mostly unnecessary in Rust; `match` on an enum does the job.
- Exceptions for `return` do not exist here. Model non-local exits as values.
- Deeply nested input recurses deeply. Decide on a depth limit.
- Proc macros must live in a separate crate with `proc-macro = true`, and they can only be tested from outside that crate.
- Code generated by a macro runs in the user's namespace. Use full paths such as `::core::convert::TryFrom`.
- `f64` cannot be a `HashMap` key without a wrapper.

## Done when

- [ ] Both implementations pass the same test suite.
- [ ] The VM is measurably faster than the tree walker, and you have the numbers.
- [ ] The derive macro has passing and failing `trybuild` tests.
- [ ] You can explain when to use `macro_rules!`, when a proc macro, and when a plain function or generic is better than either.
- [ ] You can explain how a closure keeps its captured variables alive in your implementation.
- [ ] You can explain what `'src` buys you and what it costs.

## Stretch

- Closures and upvalues in the VM.
- Classes, methods and inheritance.
- A mark-and-sweep garbage collector for the VM (needs `unsafe`; come back after project 11).
- String interning.
- An attribute macro, for example `#[native_fn]`, that registers native functions.

## Reading

- Crafting Interpreters by Robert Nystrom, free online: <https://craftinginterpreters.com>
- The Little Book of Rust Macros: <https://veykril.github.io/tlborm/>
- David Tolnay's `proc-macro-workshop` repository, particularly the `Builder` exercise
- The Rust Book: ch. 18 (trait objects), ch. 20.5 (Macros)
- The `syn` and `quote` docs on docs.rs
