# 02 · Your Own Data Structures

> Level 2 of 5 · about 24 nights · crate: `projects/structures` (library)

## Why this project

Linked structures are easy in garbage-collected languages and famously awkward in Rust, which makes them the best place to really learn ownership, generics, lifetimes and the smart pointers. In real code you would use `Vec`, `VecDeque`, `HashMap`, `BTreeMap` and `BinaryHeap`. This is the gym.

It is also preparation for the longer Rust interview rounds: "build an LRU cache", "implement a linked list with iterators", "walk me through your ownership model". Those rounds are less about speed than about making sensible ownership choices and explaining them. The rebuild drills at the end train exactly that. Timed DSA practice lives in a separate repo, in Python.

The project has three parts. Do them in order: each part uses what the earlier parts built.

| Part | Covers | Milestones | Nights |
|------|--------|------------|--------|
| 1 · Linear | stack, arrays, matrix, linked lists (singly, doubly, circular), queues (linear, circular, double-ended) | M1–M10 | ~12 |
| 2 · Hashing and trees | hash map and set, LRU cache, binary tree, BST, AVL, heap and priority queue | M11–M17 | ~9 |
| 3 · Graphs and friends | trie, union-find, graphs (directed/undirected, weighted/unweighted, adjacency list/matrix) | M18–M20 | ~3 |
| Wrap-up | random tests and benchmarks against std | M21 | ~1 |

## Rust concepts

- Generics, trait bounds, `where` clauses
- `Option<Box<Node<T>>>` and its toolkit: `take`, `as_ref`, `as_mut`, `as_deref`, `map`
- `std::mem::replace` / `take` / `swap`
- Lifetimes in structs: writing `Iter<'a, T>` and `IterMut<'a, T>`
- Implementing standard traits: `Iterator`, `IntoIterator` (three ways), `FromIterator`, `Extend`, `Default`, `Debug`, `Drop`, `Index`, `IndexMut`, `PartialEq`
- Const generics and arrays: `[T; N]`, `std::array::from_fn`
- `Rc`, `Weak`, `RefCell`: shared ownership and interior mutability
- Reference cycles and leaks
- Using indices into a `Vec` instead of pointers (the arena pattern)
- `Hash`, `Hasher`, `BuildHasher`; `Eq` and `Ord` and what they promise
- Associated types, `impl Trait` in return position, marker types with `PhantomData`
- Doc comments, doctests, `cargo doc`
- Property testing and benchmarking

## Dev skills

Classic data structures, API design that mirrors the standard library, testing against an oracle, invariant checkers, measuring instead of guessing, and writing all of it fast from memory.

## Allowed crates

None in the library. Dev: `proptest`, `criterion`. No `unsafe` (not part of this ladder). Where a milestone builds on an earlier one, use your own structure, not the std one, unless the milestone says otherwise.

## Patterns for this project

To be written up in the primer format (canonical example in a toy context, why it's written that way, the common mistake) when you start this project. Learn each one, rebuild it in `drills/`, then use it in the milestones.

- [ ] The `Option<Box<Node<T>>>` toolkit: `take`, `as_ref`, `as_mut`, `as_deref`, `map`
- [ ] `mem::replace`, `mem::take` and `mem::swap`
- [ ] An iterative `Drop` for a linked structure
- [ ] Writing `Iter<'a, T>` and `IterMut<'a, T>` for your own type
- [ ] The three `IntoIterator` impls: `T`, `&T`, `&mut T`
- [ ] `FromIterator` and `Extend`
- [ ] `Index` and `IndexMut`
- [ ] Const generic arrays and `std::array::from_fn`
- [ ] `where` clauses and `impl` blocks with extra bounds
- [ ] `Rc<RefCell<T>>` with `Weak` back links: `borrow`, `borrow_mut`, `upgrade`
- [ ] Nodes in a `Vec` addressed by index (the arena pattern)
- [ ] Implementing `Hash`, `Eq` and `Ord` by hand
- [ ] Doc comments with runnable examples (doctests)
- [ ] A proptest against a std oracle

## Spec

Everything is generic, documented, and has iterators where it makes sense. Each structure goes in its own module of `projects/structures`.

### Part 1 · Linear

1. **`Stack<T>`**: singly linked. `push`, `pop`, `peek`, `peek_mut`, `len`, `is_empty`, `reverse`. `iter`, `iter_mut`, `into_iter`. Dropping a million-element stack must not overflow the call stack.
2. **`FixedArray<T, const N: usize>`**: capacity fixed at compile time, no heap allocation. `push`, `pop`, `get`, `insert`, `remove` (shifting), `len`, `capacity`, `iter`. Operations that would exceed capacity return an error instead of panicking. `Index` / `IndexMut`.
3. **`Matrix<T>`**: rows × cols in one flat `Vec`. Construct filled or from nested rows (rejecting ragged input). `Index<(usize, usize)>`, checked `get`, `row`, `col`, `transpose`, iterator over 4- and 8-direction in-bounds neighbors of a cell, and multiplication for numeric `T`.
4. **`Deque<T>`**: doubly linked with `Rc<RefCell<...>>` forward links and `Weak` back links. `push_front`, `push_back`, `pop_front`, `pop_back`, `peek_front`, `peek_back`, `len`.
5. **`CircularList<T>`**: singly linked, last node points back to the first. `push_front`, `push_back`, `pop_front`, `rotate` (move the head forward by k), `len`, and an `iter` that visits each element once and stops. Pick the representation yourself.
6. **`Queue<T>`**: unbounded FIFO. `enqueue`, `dequeue`, `peek`, `len`. Amortised O(1).
7. **`RingQueue<T>`**: circular queue on fixed capacity. `enqueue` fails when full, `dequeue`, `peek`, `is_full`. Then a growable version that doubles when full and keeps FIFO order.

### Part 2 · Hashing and trees

8. **`HashMap<K, V>`** with `K: Hash + Eq`: `insert` (returns the old value), `get`, `get_mut`, `remove`, `contains_key`, `len`, `iter`. Grows when the load factor passes a threshold you choose. Pick separate chaining or open addressing. **`HashSet<T>`** on top of it.
9. **`LruCache<K, V>`**: fixed capacity; `get` and `put` in O(1); evicts the least recently used entry. Your `HashMap` plus a linked list stored as indices in a `Vec`.
10. **`BinaryTree<T>`** (unordered): build from LeetCode-style level order (`Vec<Option<T>>`), pre-, in- and post-order traversal both recursive and iterative, level-order traversal (grouped by level), `height`, `mirror`, lowest common ancestor.
11. **`Bst<K, V>`**: unbalanced binary search tree map. `insert`, `get`, `get_mut`, `contains_key`, `remove`, `len`, in-order `iter`, `min`, `max`, `floor`, `ceiling`, and a `range` iterator.
12. **`AvlTree<K, V>`**: same API as `Bst`, height-balanced after every insert and remove.
13. **`BinaryHeap<T: Ord>`** on a `Vec`: `push`, `pop`, `peek`, `len`, `from(Vec<T>)` in O(n), `into_sorted_vec`. Usable as a min-heap too. **`PriorityQueue`** on top: items with separate priorities, plus `change_priority`.

### Part 3 · Graphs and friends

14. **`Trie`**: `insert`, `contains`, `starts_with`, `remove`, and an iterator over all words with a given prefix, in sorted order.
15. **`UnionFind`**: over `0..n`. `find` with path compression, `union` by rank or size, `connected`, `set_count`, `set_size`. Then a version keyed by any hashable type.
16. **Graphs**: one trait that algorithms are written against, with two implementations, adjacency list and adjacency matrix. Both support directed and undirected, weighted and unweighted graphs, chosen at the type level rather than by a runtime flag. `add_vertex`, `add_edge`, `neighbors`, `has_edge`, `vertex_count`, `edge_count`.

## Milestones

### Part 1 · Linear

- [ ] **M1 · Stack basics.** `push` and `pop` for a concrete `i32` stack. You will fight the borrow checker until you find `Option::take`.
- [ ] **M2 · Generic and droppable.** Make it `Stack<T>`. Add `peek`, `peek_mut`, `Default`. Write an iterative `Drop` and a test that drops a huge stack.
- [ ] **M3 · Iterators.** `IntoIter`, `Iter<'a, T>`, `IterMut<'a, T>`, plus `IntoIterator` for `Stack<T>`, `&Stack<T>` and `&mut Stack<T>`. This is the lifetime night; expect it to take two.
- [ ] **M4 · Standard traits and docs.** `FromIterator`, `Extend`, `Debug`. A doc comment with a runnable example on every public item. `cargo doc --open`.
- [ ] **M5 · Fixed array.** `FixedArray<T, N>` to spec. No `T: Default` or `T: Copy` bound allowed. Compare its `size_of` with a `Vec<T>` and explain the difference.
- [ ] **M6 · Matrix.** `Matrix<T>` to spec. Multiplication only exists when `T` supports it; write the bounds yourself.
- [ ] **M7 · Deque with `Rc<RefCell>`.** Push and pop at both ends. Watch `Rc::strong_count`. Cause a `BorrowMutError` panic on purpose to see what moved from compile time to run time.
- [ ] **M8 · Cycles.** Write a test that would leak if the back links were `Rc` instead of `Weak`, and convince yourself why.
- [ ] **M9 · Circular list.** `CircularList<T>` to spec, no leaks (prove it with a test like M8's). Write two sentences in `notes/` on why you chose your representation.
- [ ] **M10 · Queues.** `Queue<T>`, then `RingQueue<T>` fixed and growable. Proptest all three queues and the deque against `VecDeque`.

### Part 2 · Hashing and trees

- [ ] **M11 · Hash map.** `HashMap<K, V>` and `HashSet<T>` to spec, using std's hasher machinery but none of std's maps. Test with a key type whose hash always collides and make sure it still works.
- [ ] **M12 · LRU cache.** Entries in a `Vec`, `prev`/`next` as indices, your `HashMap` from key to index, a free list for reuse.
- [ ] **M13 · Binary tree.** `BinaryTree<T>` to spec. Write every traversal twice, recursive and iterative (use your own `Stack` and `Queue`).
- [ ] **M14 · BST insert and lookup.** `K: Ord`. Insert, get, contains, len, min, max.
- [ ] **M15 · BST iteration and removal.** In-order iterator using an explicit stack. `remove`, handling all three cases. `floor`, `ceiling`, `range`.
- [ ] **M16 · AVL tree.** Insert and remove with rotations. Write an invariant checker (ordering, balance factor, stored heights) and run it after every operation in a proptest. Measure the height after inserting 1..=100_000 in order into both `Bst` and `AvlTree`.
- [ ] **M17 · Heap and priority queue.** `BinaryHeap<T>` and `PriorityQueue` to spec. Show the O(n) build is really faster than n pushes with a benchmark.

### Part 3 · Graphs and friends

- [ ] **M18 · Trie.** To spec. Build it with two child representations (one sparse, one fixed-size), benchmark both on a real word list, and keep the one you would defend.
- [ ] **M19 · Union-find.** Both versions to spec. Count how many parent hops `find` takes with and without path compression on a worst-case input.
- [ ] **M20 · Graphs.** The trait and both representations, covering all four directed/weighted combinations. Make an invalid combination (a weighted edge on an unweighted graph) a compile error, not a panic.

### Wrap-up

- [ ] **M21 · Prove it.** Proptests running random operation sequences against `Vec`, `VecDeque`, `HashMap`, `BTreeMap` and `BinaryHeap` as oracles for every structure that has a std equivalent. A `criterion` benchmark of each against its std equivalent.

## Rebuild drills

A Rust round will ask you to build one of these live and talk while you do it. Once a structure's milestone is done, add it to the rotation: once a week, in a scratch file outside the workspace, rebuild one from an empty file with no docs open, until its tests pass. Log the time and anything you fumbled in `PROGRESS.md`.

Narrate as you go, out loud or in a comment block at the top of the file. For every pointer or ownership choice (`Box`, `Option<Box>`, `Rc`, `Weak`, `RefCell`, an index into a `Vec`), say why it's that one and what the alternative would cost. For every `clone()`, say why it's needed. That explanation is what the interviewer is grading; the times are a rough guide, not the point.

| Drill | Rough target |
|-------|--------------|
| Stack with `Iter` and an iterative `Drop` | 15 min |
| Ring queue (fixed capacity) | 10 min |
| Deque with `Rc`/`Weak` | 20 min |
| Hash map with chaining (insert/get/remove/resize) | 20 min |
| LRU cache on indices | 20 min |
| BST insert/get/remove/in-order `iter` | 20 min |
| Binary heap (push/pop/heapify) | 12 min |
| Trie (insert/contains/starts_with) | 10 min |
| Union-find with path compression and rank | 8 min |
| Graph as adjacency list from an edge list | 8 min |

## Traps when coming from other languages

- A node cannot be pointed at mutably from two places. Most "obvious" linked-list code from other languages assumes it can.
- You cannot move a value out from behind a `&mut` and leave nothing there. `take` and `mem::replace` leave something valid behind.
- `Rc<RefCell<T>>` compiles easily and then panics at run time if you borrow twice. It moves the rules; it does not remove them.
- Two `Rc`s pointing at each other never get freed.
- The default recursive `Drop` for a long list is a deep recursion.
- `IterMut` is hard for a real reason: handing out several `&mut` into one structure must be provably disjoint.
- Indices are "pointers" the borrow checker does not track. They can dangle logically; you have to keep them valid.

## Done when

- [ ] Every structure meets the spec, with doctests on every public method.
- [ ] Property tests pass against the std oracles.
- [ ] No `unsafe` anywhere.
- [ ] You can explain each of `as_ref`, `as_mut`, `as_deref` and `take` on an `Option<Box<T>>`.
- [ ] You can explain what `'a` ties together in `Iter<'a, T>`.
- [ ] You can say when `Rc<RefCell<T>>` is the right tool and when indices are better.
- [ ] You can state the time and space complexity of every operation you wrote, and point to the line that makes it so.
- [ ] You have benchmark numbers and can explain why `Vec` beats your linked list.
- [ ] You can explain why an AVL tree stays O(log n) and what a rotation preserves.
- [ ] For every structure, you can explain its ownership model in two sentences: who owns each node, and why that pointer type.
- [ ] You have done every rebuild drill at least once, narrating each ownership choice, within the rough targets.

## Stretch

- A red-black tree (classic or left-leaning), with an invariant checker.
- An N-ary tree that serializes to a string and parses back.
- Robin Hood hashing, or the open-addressing/chaining variant you didn't pick in M11, benchmarked against the first.
- An `entry` API for your `HashMap`.
- A persistent (immutable, structure-sharing) stack with `Rc`.
- A skip list.
- A segment tree or Fenwick tree for range-sum queries.
- A B-tree.

## Reading

- Learn Rust With Entirely Too Many Linked Lists, the safe chapters only. Stop when it reaches `unsafe`: <https://rust-unofficial.github.io/too-many-lists/>
- The Rust Book: ch. 10 (Generics, Traits, Lifetimes), ch. 15 (Smart Pointers)
- The `std::option::Option` docs page, top to bottom
- The `std::collections` module docs, the "when should you use which collection" section: <https://doc.rust-lang.org/std/collections/index.html>
- The `std::hash` module docs, before M11
- Sedgewick and Wayne, *Algorithms* (4th ed.), ch. 3.3 for balanced trees and ch. 4.1–4.2 for graph representations. Read after you've attempted the milestone.
- After finishing a structure, not before: read the std source for its equivalent (`BinaryHeap`, `VecDeque`; `HashMap` is a wrapper over the `hashbrown` crate) and compare.
