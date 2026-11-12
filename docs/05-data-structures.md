# 05 · Your Own Data Structures

> Level 5 of 12 · about 10 nights · crate: `projects/structures` (library)

## Why this project

Linked structures are easy in garbage-collected languages and famously awkward in Rust, which makes them the best place to really learn ownership, generics, lifetimes and the smart pointers. In real code you would use `Vec`, `VecDeque` or `BTreeMap`. This is the gym.

## Rust concepts

- Generics, trait bounds, `where` clauses
- `Option<Box<Node<T>>>` and its toolkit: `take`, `as_ref`, `as_mut`, `as_deref`, `map`
- `std::mem::replace` / `take`
- Lifetimes in structs: writing `Iter<'a, T>` and `IterMut<'a, T>`
- Implementing standard traits: `Iterator`, `IntoIterator` (three ways), `FromIterator`, `Extend`, `Default`, `Debug`, `Drop`
- `Rc`, `Weak`, `RefCell`: shared ownership and interior mutability
- Reference cycles and leaks
- Using indices into a `Vec` instead of pointers (the arena pattern)
- Doc comments, doctests, `cargo doc`
- Property testing and benchmarking

## Dev skills

Classic data structures, API design that mirrors the standard library, testing against an oracle, measuring instead of guessing.

## Allowed crates

None in the library. Dev: `proptest`, `criterion`. No `unsafe` (that is project 11).

## Spec

Four types, all generic, all documented, all with iterators where it makes sense.

1. **`Stack<T>`**: singly linked. `push`, `pop`, `peek`, `peek_mut`, `len`, `is_empty`, `reverse`. `iter`, `iter_mut`, `into_iter`. Dropping a million-element stack must not overflow the call stack.
2. **`Bst<K, V>`**: unbalanced binary search tree map. `insert`, `get`, `get_mut`, `contains_key`, `remove`, `len`, in-order `iter`.
3. **`Deque<T>`**: doubly linked with `Rc<RefCell<...>>` forward links and `Weak` back links. `push_front`, `push_back`, `pop_front`, `pop_back`, `len`.
4. **`LruCache<K, V>`**: fixed capacity; `get` and `put` in O(1); evicts the least recently used entry. Built from a `HashMap` plus a linked list stored as indices in a `Vec`.

## Milestones

- [ ] **M1 · Stack basics.** `push` and `pop` for a concrete `i32` stack. You will fight the borrow checker until you find `Option::take`.
- [ ] **M2 · Generic and droppable.** Make it `Stack<T>`. Add `peek`, `peek_mut`, `Default`. Write an iterative `Drop` and a test that drops a huge stack.
- [ ] **M3 · Iterators.** `IntoIter`, `Iter<'a, T>`, `IterMut<'a, T>`, plus `IntoIterator` for `Stack<T>`, `&Stack<T>` and `&mut Stack<T>`. This is the lifetime night; expect it to take two.
- [ ] **M4 · Standard traits and docs.** `FromIterator`, `Extend`, `Debug`. A doc comment with a runnable example on every public item. `cargo doc --open`.
- [ ] **M5 · BST insert and lookup.** `K: Ord`. Insert, get, contains, len.
- [ ] **M6 · BST iteration and removal.** In-order iterator using an explicit stack. `remove`, handling all three cases.
- [ ] **M7 · Deque with `Rc<RefCell>`.** Push and pop at both ends. Watch `Rc::strong_count`. Cause a `BorrowMutError` panic on purpose to see what moved from compile time to run time.
- [ ] **M8 · Cycles.** Write a test that would leak if the back links were `Rc` instead of `Weak`, and convince yourself why.
- [ ] **M9 · LRU cache.** Entries in a `Vec`, `prev`/`next` as indices, a `HashMap` from key to index, a free list for reuse.
- [ ] **M10 · Prove it.** Property tests that run random operation sequences against `Vec`, `BTreeMap` and `VecDeque` as oracles. A `criterion` benchmark of each structure against its std equivalent.

## Traps when coming from other languages

- A node cannot be pointed at mutably from two places. Most "obvious" linked-list code from other languages assumes it can.
- You cannot move a value out from behind a `&mut` and leave nothing there. `take` and `mem::replace` leave something valid behind.
- `Rc<RefCell<T>>` compiles easily and then panics at run time if you borrow twice. It moves the rules; it does not remove them.
- Two `Rc`s pointing at each other never get freed.
- The default recursive `Drop` for a long list is a deep recursion.
- `IterMut` is hard for a real reason: handing out several `&mut` into one structure must be provably disjoint.
- Indices are "pointers" the borrow checker does not track. They can dangle logically; you have to keep them valid.

## Done when

- [ ] All four types meet the spec, with doctests on every public method.
- [ ] Property tests pass against the std oracles.
- [ ] No `unsafe` anywhere.
- [ ] You can explain each of `as_ref`, `as_mut`, `as_deref` and `take` on an `Option<Box<T>>`.
- [ ] You can explain what `'a` ties together in `Iter<'a, T>`.
- [ ] You can say when `Rc<RefCell<T>>` is the right tool and when indices are better.
- [ ] You have benchmark numbers and can explain why `Vec` beats your linked list.

## Stretch

- Balance the tree (AVL or red-black).
- A trie with prefix iteration.
- A binary heap and a priority queue on top of it.
- A persistent (immutable, structure-sharing) stack with `Rc`.

## Reading

- Learn Rust With Entirely Too Many Linked Lists, the safe chapters only. Stop when it reaches `unsafe` and come back in project 11: <https://rust-unofficial.github.io/too-many-lists/>
- The Rust Book: ch. 10 (Generics, Traits, Lifetimes), ch. 15 (Smart Pointers)
- The `std::option::Option` docs page, top to bottom
