# 11 · Your Own Vec, an Arena Allocator, and a C Binding

> Level 11 of 12 · about 14 nights · crates: `projects/myvec`, `projects/arena`, `projects/ffi-demo`

## Why this project

Everything so far relied on safe abstractions someone else wrote with `unsafe`. Now you write them. The goal is not to use `unsafe` more; it is to understand exactly what the compiler normally guarantees, so you can uphold those guarantees by hand when you must, and wrap the result in an API that cannot be misused.

Run Miri from the first night. It catches undefined behaviour that tests pass straight through.

## Rust concepts

- The five things `unsafe` allows, and everything it does not switch off
- Raw pointers, `NonNull`, pointer arithmetic
- `std::alloc`: `alloc`, `realloc`, `dealloc`, `Layout`, alignment
- `ptr::read`, `ptr::write`, `ptr::copy`, `ptr::drop_in_place`
- `MaybeUninit`
- Zero-sized types
- `PhantomData`, variance, the drop checker
- Manual `unsafe impl Send` / `Sync`
- Safety invariants, `# Safety` docs and `// SAFETY:` comments
- Undefined behaviour: aliasing, dangling pointers, uninitialised memory, invalid values
- Panic safety inside unsafe code
- `Deref` / `DerefMut` to a slice
- FFI: `extern "C"`, `#[repr(C)]`, `CStr` / `CString`, `build.rs`, linking
- The `-sys` crate convention and safe wrappers with `Drop`
- Tools: Miri, sanitizers, `bindgen`, `cbindgen`

## Dev skills

Memory layout, allocator behaviour, reading and binding C APIs, build scripts, designing a safe boundary around unsafe internals, auditing.

## Allowed crates

`myvec` and `arena`: none. FFI: `cc`, `bindgen`, `libc`, `cbindgen`. Dev: `proptest`. Tool: `cargo +nightly miri`.

## Spec

**Part A · `MyVec<T>`**

- `new`, `with_capacity`, `push`, `pop`, `insert`, `remove`, `truncate`, `clear`, `len`, `capacity`.
- Amortised growth by doubling. Capacity overflow is handled, not ignored.
- `Deref` and `DerefMut` to `[T]`, so every slice method works.
- `Drop` drops the elements and frees the buffer.
- `IntoIterator` by value with a double-ended iterator; `drain`.
- `Clone`, `FromIterator`, `Extend`, `Debug`.
- Works for zero-sized types.
- Correct `Send` / `Sync` and variance.
- If a `T`'s `Clone` or `Drop` panics part-way through an operation, nothing is dropped twice and nothing is read uninitialised.

**Part B · `Arena`**

- A bump allocator: `arena.alloc(value)` returns a `&mut T` that lives as long as the arena borrow.
- Grows by adding chunks; previously returned references stay valid.
- Respects each type's alignment.
- Document, and enforce in the API, what happens to destructors.
- Used to rebuild either the tree from project 05 or the AST from project 10, with a before/after benchmark.

**Part C · FFI in both directions**

- A small C library of your own (a few functions, one struct, one callback), compiled by `build.rs` with `cc`, declared by hand in Rust.
- A real C library (zlib or SQLite) bound with `bindgen` in a `-sys` crate, plus a safe wrapper crate: handles own their resource and free it in `Drop`, error codes become `Result`, strings cross the boundary correctly.
- The reverse: expose a Rust function set to C with a generated header, and call it from a C program.

## Milestones

- [ ] **M1 · Allocate.** Read the opening of the Rustonomicon and its Vec chapter's layout section. A `RawVec<T>` that allocates, grows and frees. Set up Miri and run it.
- [ ] **M2 · Push and pop.** With `ptr::write` and `ptr::read`. Miri clean.
- [ ] **M3 · Drop and Deref.** After this, indexing, iteration by reference and sorting all work for free.
- [ ] **M4 · Insert and remove.** Shifting with `ptr::copy`. Bounds checks.
- [ ] **M5 · IntoIter.** By-value iteration from both ends, with its own `Drop`.
- [ ] **M6 · Drain and panic safety.** Write a test type whose `Drop` panics and prove no double drop.
- [ ] **M7 · Zero-sized types, variance, Send/Sync.** Every `unsafe` block gets a `// SAFETY:` comment that states the invariant relied on. Property tests against `std::vec::Vec`, run under Miri.
- [ ] **M8 · Arena core.** One chunk, bump pointer, alignment.
- [ ] **M9 · Arena growth and lifetimes.** Multiple chunks. Get the API's lifetimes right so use-after-free does not compile; add a `trybuild` or doc `compile_fail` test proving it.
- [ ] **M10 · Arena in anger.** Port an earlier structure to it and benchmark.
- [ ] **M11 · Hand-written FFI.** Your C library through `cc` and manual `extern` declarations. Pass integers, a struct, a string each way, and a callback.
- [ ] **M12 · bindgen.** A `-sys` crate for the real library.
- [ ] **M13 · Safe wrapper.** An idiomatic Rust API over the `-sys` crate with no `unsafe` needed by its users.
- [ ] **M14 · Rust called from C.** Exported functions, a generated header, a C `main` that links against your Rust library. Make sure a panic cannot unwind into C.

## Traps when coming from other languages (especially C and C++)

- `unsafe` does not turn off the borrow checker or any other check on safe code. It unlocks a few extra operations and makes you responsible for their rules.
- Undefined behaviour does not mean "crashes". The program may appear to work for months. Passing tests prove nothing; Miri proves more.
- Creating a reference to uninitialised memory, or a `&mut` that aliases another live reference, is undefined behaviour even if you never use it. Stay in raw pointers until the data is valid and unaliased.
- A `&mut T` is closer to a C `restrict` pointer than to a plain one: the compiler assumes nothing else can reach that memory while it is live.
- Allocating zero bytes through the global allocator is not allowed. Zero-sized types and empty vectors need a dangling-but-aligned pointer.
- Assigning through a raw pointer with `*p = value` drops the old value first. On uninitialised memory that is a bug; use `ptr::write`.
- Memory must be freed by the allocator that produced it. A C `malloc` pointer is never freed with Rust's allocator, or the reverse.
- Miri cannot execute calls into real C code. Test the FFI parts with sanitizers and ordinary tests.
- In the 2024 edition, `extern` blocks are written `unsafe extern`, and attributes such as `no_mangle` are written `#[unsafe(no_mangle)]`. Older material shows the earlier syntax.

## Done when

- [ ] `MyVec` passes its property tests under Miri.
- [ ] Every `unsafe` block has a `// SAFETY:` comment a reviewer could check.
- [ ] The arena's misuse cases fail to compile.
- [ ] The safe wrapper's public API contains no raw pointers and no `unsafe fn`.
- [ ] A C program calls your Rust code and a Rust program calls your C code.
- [ ] You can list the five unsafe superpowers and three kinds of undefined behaviour from memory.
- [ ] You can explain why `PhantomData<T>` is in your `MyVec` and what would go wrong without it.
- [ ] You can explain what "sound" means for a safe API built on unsafe code.

## Stretch

- An unsafe doubly linked list with a cursor API (the second half of Too Many Lists).
- A fixed-capacity lock-free single-producer single-consumer queue with atomics, checked with `loom`.
- Implement the `GlobalAlloc` trait with a simple allocator and run a program on it.
- Compile one of your earlier projects to WebAssembly and call it from JavaScript.

## Reading

- The Rustonomicon, especially "Implementing Vec": <https://doc.rust-lang.org/nomicon/>
- The Rust Book, ch. 20.1 (Unsafe Rust)
- Learn Rust With Entirely Too Many Linked Lists, the unsafe chapters: <https://rust-unofficial.github.io/too-many-lists/>
- The Miri README
- The bindgen User Guide: <https://rust-lang.github.io/rust-bindgen/>
- The Rust Reference, "Behavior considered undefined"
