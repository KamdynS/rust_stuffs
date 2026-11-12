# 12 · Capstone

> Level 12 of 12 · two months or more · crate: your choice of name

## Why this project

One large system, designed by you, that draws on everything before it. Pick **one** track. All three are things people are hired to build, and each one is big enough to be a centrepiece of a portfolio.

| Track | Pick it if you want to go towards | Builds on |
|-------|-----------------------------------|-----------|
| A · Raft-backed distributed key-value store | backend and distributed systems | 08, 09 |
| B · SQLite clone | databases and storage engines | 04, 05, 09 |
| C · Your own async runtime | systems programming and Rust internals | 06, 08, 11 |

## Rules for the capstone

These apply to every track, and they are the dev skills this project teaches.

- [ ] **Design doc first.** One or two pages in the crate's `DESIGN.md` before any code: goals, non-goals, main types, how data flows, what you will test and how. Update it when you change your mind.
- [ ] **Testing strategy before features.** Each track below names the testing approach that makes it tractable. Build that harness early.
- [ ] **Devlog.** Keep using `PROGRESS.md`. Record decisions and dead ends, not just progress.
- [ ] **CI from day one.** fmt, clippy with `-D warnings`, tests, and Miri if there is any `unsafe`.
- [ ] **Finish in public.** A README with an architecture diagram, how to run it, benchmarks, known limitations.

Allowed crates are up to you now. Write the list in the design doc and justify anything that does a core part of the project for you.

---

## Track A · Raft-backed distributed key-value store

A cluster of three or five nodes that agree on a replicated log using Raft and apply it to a key-value store. It stays correct when nodes crash and restart and when the network drops, delays or partitions messages.

**New concepts:** consensus, replicated state machines, sans-I/O design (the algorithm as a pure state machine), deterministic simulation testing, linearizability, RPC, durable state.

**Testing approach:** write the Raft node as a type with no I/O at all: inputs (message received, timer fired, client request) go in, outputs (messages to send, entries to persist, entries to apply) come out. Then drive many nodes inside one test with a simulated network and a seeded random number generator, so any failure can be replayed exactly.

**Milestones**

- [ ] **A1 · Read.** The extended Raft paper, twice. Figure 2 is the specification.
- [ ] **A2 · Sans-I/O skeleton.** Node state, the input and output enums, and a `step` function. No sockets, no clocks.
- [ ] **A3 · Simulator.** An in-process cluster with a fake network that can delay, drop, reorder and partition, driven by a seed.
- [ ] **A4 · Leader election.** Including split votes and term changes. Invariant checked after every step: at most one leader per term.
- [ ] **A5 · Log replication.** Append, commit, apply. Invariants from the paper checked continuously.
- [ ] **A6 · Persistence.** Term, vote and log survive restarts. Crash nodes at random points in the simulator.
- [ ] **A7 · Real transport.** Run the same node type over tokio and TCP with real timers.
- [ ] **A8 · Key-value service.** Plug in a store (reuse project 09) as the state machine. Client API with leader redirection and de-duplication of retried requests.
- [ ] **A9 · Snapshots.** Log compaction and installing snapshots on lagging followers.
- [ ] **A10 · Chaos.** Long randomised runs with crashes and partitions; record client histories and check them for linearizability.
- [ ] Stretch: cluster membership changes; read-only queries without a log write.

**Reading:** "In Search of an Understandable Consensus Algorithm (Extended Version)" and the visualisations at <https://raft.github.io>; the MIT 6.5840 distributed systems labs; PingCAP's Talent Plan; Designing Data-Intensive Applications by Martin Kleppmann, the chapters on replication and consistency.

---

## Track B · SQLite clone

A single-file relational database with a SQL subset, a B+tree storage engine, and crash-safe transactions.

**New concepts:** page-based storage, buffer pools, B+trees on disk, record formats, query execution as composable iterators, write-ahead logging and recovery.

**Testing approach:** use real SQLite as an oracle. Generate random statements, run them against both, and compare results. For durability, inject crashes at every write and verify recovery.

**Milestones**

- [ ] **B1 · Front end.** A REPL, a SQL tokenizer and parser for `CREATE TABLE`, `INSERT`, `SELECT ... WHERE ... ORDER BY ... LIMIT`, `UPDATE`, `DELETE`.
- [ ] **B2 · Pager.** A file of fixed-size pages and a buffer pool with eviction (reuse the LRU cache from project 05) and pinning.
- [ ] **B3 · Records.** Serialising typed rows into bytes within a page; a slotted page layout.
- [ ] **B4 · B+tree, insert and search.** Leaf nodes, internal nodes, splits, a cursor for range scans.
- [ ] **B5 · B+tree, delete.** Merging and rebalancing.
- [ ] **B6 · Catalog.** Table schemas stored in the database itself.
- [ ] **B7 · Execution.** Operators as iterators (scan, filter, project, sort, limit) composed into plans.
- [ ] **B8 · Indexes and planning.** Secondary indexes and a planner that chooses an index scan when one applies.
- [ ] **B9 · Transactions.** `BEGIN`, `COMMIT`, `ROLLBACK` with a write-ahead log; recovery at startup.
- [ ] **B10 · Hardening.** The oracle fuzzer and the crash-injection tests running in CI.
- [ ] Stretch: joins; concurrent readers with a single writer; reading a real SQLite file.

**Reading:** "Let's Build a Simple Database" by cstack; the CMU 15-445 Database Systems lectures; the SQLite file format documentation; Database Internals by Alex Petrov, part one.

---

## Track C · Your own async runtime

An executor, a reactor and timers, sufficient to run a real network server, built from `std` plus an OS polling crate. By the end `async`/`.await` holds no mysteries.

**New concepts:** the `Future` trait in full, `Pin`, `Context` and `Waker`, task scheduling, readiness-based I/O (epoll/kqueue), timer data structures, work stealing, concurrency testing with model checkers.

**Testing approach:** unit-test futures by polling them by hand with a counting waker. Test the scheduler's concurrent data structures with `loom`. Then run real workloads: your HTTP server from project 06 ported onto your runtime.

**Milestones**

- [ ] **C1 · Futures by hand.** Implement `Future` for a countdown and a simple combinator. Poll them manually with a no-op waker.
- [ ] **C2 · `block_on`.** Run one future to completion on the current thread, parking the thread between wake-ups.
- [ ] **C3 · Executor.** A task queue, `spawn`, and a `JoinHandle` that is itself a future. Wakers re-queue their task.
- [ ] **C4 · Reactor.** An event loop over `mio` (or `polling`) that maps OS readiness events to wakers.
- [ ] **C5 · Async TCP.** `TcpListener` and `TcpStream` with async accept, read and write built on the reactor.
- [ ] **C6 · Timers.** `sleep` and `timeout` backed by a heap or a timer wheel.
- [ ] **C7 · A real program.** Port project 06's HTTP server to your runtime. Load test it.
- [ ] **C8 · Channels and combinators.** `oneshot`, `mpsc`, `join` and a two-way `select`.
- [ ] **C9 · Multi-threaded scheduler.** Worker threads with per-worker queues and work stealing. `loom` tests.
- [ ] **C10 · Raw wakers.** Replace the `Wake` trait convenience with a hand-built `RawWakerVTable`, with `// SAFETY:` comments and Miri.
- [ ] **C11 · Compare.** Benchmark against tokio on the same server and explain the gap.
- [ ] Stretch: an `io_uring` backend; task-local storage; cooperative budgeting.

**Reading:** Asynchronous Programming in Rust (the official async book), the parts on futures, pinning and implementing a runtime: <https://rust-lang.github.io/async-book/>; the Tokio blog post "Making the Tokio scheduler 10x faster"; the source of the `smol` family of crates, which is small enough to read; Rust Atomics and Locks by Mara Bos; the Rust Book ch. 17.5.

---

## Done when

- [ ] The design doc matches what you built.
- [ ] The track's testing approach is implemented and runs in CI.
- [ ] Someone else could clone the repo, follow the README and run it.
- [ ] The README states the limitations plainly.
- [ ] You have written up, in a page, the three hardest problems and how you solved them.
- [ ] You can whiteboard the whole architecture from memory.

## After the ladder

Contribute a fix to a crate you used along the way, publish one of your own crates, or pick a second capstone track. You are past the point where projects need to be assigned.
