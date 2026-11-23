# 05 · Redis Clone

> Level 5 of 5 · about 20 nights · crate: `projects/rudis`

## Why this project

The finale, and your first async Rust. It opens with two short milestones that teach tokio on a toy server, so you can compare tasks with the threads from project 03 before the real work starts. After that it is a network server where performance is the point. You parse a binary-safe protocol by hand, make real clients talk to your code, persist data so it survives a crash, and then profile and optimise with numbers. This is also where lifetimes stop being a compiler chore and become a tool: borrowing from the input buffer instead of copying.

## Rust concepts

- `async fn`, `.await`, and what a future is (a value that does nothing until polled)
- The tokio runtime, `#[tokio::main]`, `tokio::spawn`, and why spawned tasks need `Send + 'static`
- `std::sync::Mutex` vs `tokio::sync::Mutex`, and never holding a guard across `.await`
- `tokio::sync` channels (`mpsc`, `broadcast`) and `select!`
- Byte-level parsing over `&[u8]`; the `bytes` crate (`Bytes`, `BytesMut`, `Buf`, `BufMut`)
- Framing: parsing from a buffer that may hold a partial message
- Zero-copy parsing with borrowed frames (`Frame<'a>`), and `Cow`
- Generic I/O: code written over `AsyncRead + AsyncWrite` so it is testable without sockets
- Enums as a command set; `TryFrom<Frame>`; table-driven dispatch
- State machines for connection modes
- Concurrency designs: one big lock, sharded locks, or a single owner task
- `Instant`, timers and expiry
- File I/O, buffering and `fsync`
- Release profiles, inlining, allocation behaviour
- Fuzzing and property testing a parser

## Dev skills

Reading a protocol specification, wire compatibility with existing clients, durability trade-offs, crash recovery, benchmarking methodology, profiling with flame graphs, defending a parser against hostile input.

## Allowed crates

`tokio`, `bytes`, `tracing`, `tracing-subscriber`, `thiserror`, `anyhow`, `clap`. Dev: `criterion`, `proptest`, the `redis` client crate (for tests only). Tools: `redis-cli`, `redis-benchmark`, `cargo flamegraph` or `samply`, `cargo fuzz`.

Not allowed: parser-combinator crates, existing RESP or Redis implementation crates.

## Patterns for this project

To be written up in the primer format (canonical example in a toy context, why it's written that way, the common mistake) when you start this project. Learn each one, rebuild it in `drills/`, then use it in the milestones.

- [ ] `#[tokio::main]` and a `tokio::spawn` accept loop
- [ ] Shared state in async code: std `Mutex` vs tokio `Mutex`, guards dropped before `.await`
- [ ] A `BytesMut` read buffer filled with `read_buf`
- [ ] Incremental parsing over `&[u8]` with a cursor
- [ ] `Buf` and `BufMut`
- [ ] `TryFrom<Frame>` for a command enum
- [ ] `tokio::select!` in a loop
- [ ] `broadcast` channels
- [ ] A background task on `tokio::time::interval`
- [ ] Code generic over `AsyncRead + AsyncWrite + Unpin`, tested with `tokio::io::duplex`
- [ ] `Cow` and borrowed views into a buffer
- [ ] `BufWriter` and choosing when to flush
- [ ] `tracing` spans and events
- [ ] A `criterion` benchmark

## Spec

- Speaks RESP2 over TCP on port 6379 (configurable). Works with the real `redis-cli` and `redis-benchmark`.
- Keys and values are arbitrary bytes.
- Commands:
  - Connection: `PING`, `ECHO`
  - Strings: `GET`, `SET` (with `EX`, `PX`, `NX`, `XX`), `DEL`, `EXISTS`, `INCR`, `DECR`, `APPEND`
  - Expiry: `EXPIRE`, `TTL`, `PERSIST`
  - Lists: `LPUSH`, `RPUSH`, `LPOP`, `RPOP`, `LRANGE`, `LLEN`
  - Hashes: `HSET`, `HGET`, `HDEL`, `HGETALL`
  - Pub/sub: `SUBSCRIBE`, `UNSUBSCRIBE`, `PUBLISH`
- Using a command on the wrong type of value returns a `WRONGTYPE` error. Unknown commands and wrong argument counts return errors and keep the connection open.
- Pipelining works: many commands in one write, replies in order.
- Expired keys are invisible immediately and are eventually removed even if never read again.
- Append-only file persistence: every write command is logged and replayed at startup. Sync policy is configurable: `always`, `everysec`, `no`.
- After `kill -9`, a restart recovers everything the sync policy promised. A truncated final entry in the log is tolerated.
- A client cannot make the server allocate unbounded memory with a forged length prefix.

## Milestones

Async on-ramp (throwaway code, same crate):

- [ ] **M1 · Async echo.** A tokio TCP echo server, one task per connection. Compare it with project 03's thread-per-connection: what changed in the code, and what the compiler now demands of what you `spawn`.
- [ ] **M2 · Shared state across tasks.** A plain-text `GET key` / `SET key value` line protocol over `nc`, backed by a shared map. Try to hold a `std::sync::MutexGuard` across an `.await` and read the error. Write in `notes/` when a std mutex is right in async code and when tokio's is.

The real server:

- [ ] **M3 · Frames out.** Read the RESP spec. A `Frame` enum and a serializer, with tests.
- [ ] **M4 · Frames in.** A parser over `&[u8]` that distinguishes "complete frame", "need more bytes" and "invalid". Property test: split any valid input at any byte and feed it in two pieces.
- [ ] **M5 · Connection.** A type that owns a read buffer and a stream, with `read_frame` and `write_frame`, generic over the stream. Test with an in-memory duplex stream.
- [ ] **M6 · First commands.** Accept loop, `PING`, `ECHO`. `redis-cli` connects.
- [ ] **M7 · Strings.** A `Command` enum parsed from frames. `GET`, `SET`, `DEL`, `EXISTS` on a shared map.
- [ ] **M8 · Errors and numbers.** Arity checks, `INCR`/`DECR`/`APPEND`, proper error replies. Make dispatch table-driven.
- [ ] **M9 · Expiry.** `SET` options, `EXPIRE`, `TTL`, `PERSIST`. Lazy expiry on access plus a background sweeper.
- [ ] **M10 · More types.** A `Value` enum; lists and hashes; `WRONGTYPE`.
- [ ] **M11 · Pipelining.** Correct ordering and a sensible flush strategy. Test with the `redis` client crate's pipelines.
- [ ] **M12 · Pub/sub.** Channels (`broadcast`), `select!` over the socket and subscriptions, and the connection's subscriber mode as an explicit state machine.
- [ ] **M13 · Append-only file.** Log write commands; replay at startup. The three sync policies.
- [ ] **M14 · Crash testing.** A script that writes, kills the server with `-9`, restarts and verifies. Handle the truncated tail.
- [ ] **M15 · Baseline.** `redis-benchmark` against a `--release` build and against real Redis. Record the numbers in the README. Capture a flame graph and name the top three costs.
- [ ] **M16 · Fewer allocations.** Move to `Bytes` for keys and values; remove copies between parse, store and reply. Measure after each change.
- [ ] **M17 · Less contention.** Replace the single lock with shards, or with one task that owns the data. Measure both if you have the energy.
- [ ] **M18 · Borrowed frames.** A second parser that returns frames borrowing from the read buffer. Make it compile against a buffer you also need to advance. Measure.
- [ ] **M19 · Hostile input.** Limits on frame size, nesting and client count. Fuzz the parser for at least ten minutes.
- [ ] **M20 · Finish.** Graceful shutdown that flushes the log. Final benchmark table and a short write-up of what helped and what did not.

## Traps when coming from other languages

- TCP delivers a byte stream. Commands arrive split across reads and merged within a read; your parser must cope with both.
- A length prefix is a claim made by a stranger. Never allocate based on it before checking limits.
- Keys are bytes, not `String`. Using `String` will work until a binary key arrives.
- Benchmarks of debug builds mean nothing. Always `--release`.
- Optimising before profiling wastes evenings. Change one thing, measure, keep or revert.
- A blocking call (`std::fs`, `std::thread::sleep`, a long CPU loop) inside a task stalls every task on that worker thread.
- A lock held across a socket write means one slow client blocks every other.
- Borrowed frames cannot outlive the buffer they point into, and you cannot mutate that buffer while they exist. That tension is the lesson of M16.
- `write` returning does not mean the data is on disk. Know what `fsync` guarantees and what it costs.

## Done when

- [ ] `redis-cli` works for every command in the spec.
- [ ] `redis-benchmark -t set,get -c 50` completes without errors.
- [ ] The crash test passes under the `always` policy with no lost writes.
- [ ] The fuzzer finds no panics.
- [ ] The README has a benchmark table: baseline, after each optimisation, and real Redis.
- [ ] You can explain the difference between `Vec<u8>`, `Bytes` and `&[u8]` and where you use each.
- [ ] You can explain the lifetime in `Frame<'a>` and what it stops you from doing.

## Stretch

- Sets and sorted sets.
- Log rewriting to compact the append-only file.
- Point-in-time snapshots.
- Leader/follower replication.
- RESP3.

## Reading

- The Tokio tutorial, up to and including "Shared state" (for M1–M2): <https://tokio.rs/tokio/tutorial>. Its examples are themselves a tiny Redis; read them for the async mechanics and do not copy the design.
- The Redis serialization protocol specification on redis.io
- Tokio's `mini-redis` repository. Do not open it before M6; afterwards it is a good comparison.
- The Rust Performance Book by Nicholas Nethercote: <https://nnethercote.github.io/perf-book/>
- The `bytes` crate docs
- Build Your Own Redis (the book uses C/C++; read it for design, not code)
