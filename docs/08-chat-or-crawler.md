# 08 · WebSocket Chat Server (or Web Crawler)

> Level 8 of 12 · about 10 nights · crate: `projects/chat` (or `projects/crawler`)

## Why this project

Project 07 used async the easy way: one request in, one response out. Real concurrent systems have long-lived tasks that talk to each other, slow peers, timeouts and shutdown. This project is about structuring that: channels, `select!`, backpressure and cancellation.

The main track is the chat server. The crawler is an alternative with the same lessons; do it instead, or as well.

## Rust concepts

- Tasks: `tokio::spawn`, `JoinHandle`, `JoinSet`, what happens when a task panics
- Channels: `mpsc` (bounded), `broadcast`, `oneshot`, `watch`
- `tokio::select!` and cancel safety
- The actor pattern: a task that owns state and receives a message enum
- Backpressure: what should happen when a receiver is slow
- Cancellation: dropping a future, `CancellationToken`
- `Stream` and `Sink`, `StreamExt` / `SinkExt`, splitting a socket into halves
- Timeouts and intervals; `Semaphore` for limiting concurrency
- Serde enum representations for wire protocols (`#[serde(tag = "type")]`)
- First contact with `Pin`

## Dev skills

WebSockets, protocol design, real-time fan-out, heartbeats, graceful shutdown of a system with many tasks, testing concurrent code deterministically.

## Allowed crates

`tokio`, `axum` (with its WebSocket feature) or `tokio-tungstenite`, `futures`, `tokio-util`, `serde`, `serde_json`, `tracing`, `thiserror`, `anyhow`. Crawler track: also `reqwest`, `scraper`, `url`. Tools: `websocat` for manual testing, `tokio-console` for inspection.

## Spec: chat server

- Clients connect over WebSocket and exchange JSON messages. Define the protocol as two enums: client-to-server and server-to-client.
- A client sets a nickname, joins and leaves rooms, lists rooms, lists who is in a room, and sends messages to a room.
- A message sent to a room reaches every member of that room, in the same order for everyone.
- On joining, a client receives the last 50 messages of the room.
- **A slow client never slows down anyone else.** Per-client queues are bounded; choose and document a policy for when one fills (drop oldest, or disconnect).
- The server pings idle clients and disconnects those that do not answer within a timeout.
- Malformed messages get an error reply; they do not close the connection or crash a task.
- On shutdown, clients are told, queues are flushed, and every task ends. No task is leaked.

## Milestones: chat server

- [ ] **M1 · Echo.** A WebSocket endpoint that echoes text frames. Talk to it with `websocat`.
- [ ] **M2 · Two tasks per client.** Split the socket. One task reads, one writes, joined by an `mpsc` channel. When either ends, the other must end too.
- [ ] **M3 · One room.** Fan out with a `broadcast` channel. Handle the `Lagged` error deliberately.
- [ ] **M4 · Protocol.** The two message enums with serde. Nicknames. Error replies for bad input.
- [ ] **M5 · Rooms, version A: shared state.** `Arc<RwLock<HashMap<...>>>` of rooms. Get it working, then list everything that could deadlock or race.
- [ ] **M6 · Rooms, version B: actors.** A hub task owns all room state; client tasks send it commands and get answers over `oneshot`. Compare with version A in a short note in `notes/`.
- [ ] **M7 · Backpressure.** Bounded per-client queues and your slow-client policy. Write a deliberately slow test client and prove others are unaffected.
- [ ] **M8 · Heartbeats.** `select!` over incoming frames, outgoing messages and an interval timer. For each branch, check whether cancelling it mid-way loses data.
- [ ] **M9 · Shutdown.** A `CancellationToken` passed everywhere, a `JoinSet` holding the tasks, and a drain phase. Verify with a test that no task survives.
- [ ] **M10 · Tests.** Integration tests with several simulated clients asserting delivery and ordering. Use paused time for the timeout tests. Look at the running server in `tokio-console`.

## Spec: web crawler (alternative)

- Given a seed URL, crawls pages on the same host up to a maximum depth.
- At most N requests in flight at once; at least a configurable delay between requests to the same host.
- Honours `robots.txt`.
- Never fetches the same URL twice (after normalisation).
- Per-request timeout, limited retries with backoff, and a response size cap.
- Outputs JSON: every page with status, title, outgoing links, and a list of broken links.
- Ctrl-C stops new fetches, waits briefly for in-flight ones, and prints partial results.

Milestones for the crawler: (1) fetch and parse one page; (2) sequential crawl with a visited set; (3) concurrent workers with a `Semaphore` or a bounded channel; (4) politeness delay and `robots.txt`; (5) timeouts, retries, size cap; (6) cancellation and partial results; (7) tests against a local test server you spin up in the test.

## Traps when coming from other languages

- When one `select!` branch wins, the other futures are **dropped** where they stand. If one of them had read half a message, that half is gone unless the operation is cancel safe.
- Unbounded channels turn a slow consumer into unbounded memory growth. Reach for bounded first.
- Awaiting a send on a full channel while holding a lock is a deadlock waiting for load.
- A spawned task whose `JoinHandle` you drop keeps running, and its panic disappears silently.
- Tasks holding `Arc`s or senders to each other can keep the whole system alive after you thought it stopped. Shutdown is a design problem.
- `tokio::sync::broadcast` receivers that fall behind lose messages by design.

## Done when

- [ ] Three `websocat` sessions can chat in two rooms.
- [ ] The slow-client test passes and the policy is documented in the README.
- [ ] Killing a client mid-message leaves no stuck task (check with `tokio-console`).
- [ ] Shutdown completes within a bounded time with all tasks joined.
- [ ] You can explain cancel safety with an example from your own code.
- [ ] You can argue for shared state or for actors, with the trade-offs of each.

## Stretch

- Persist room history with the database layer from project 07.
- Authentication reusing project 07's tokens.
- A terminal client with `ratatui`.
- Scale out: two server processes sharing rooms through project 09's pub/sub.

## Reading

- The Tokio tutorial sections on channels, `select!` and streams; the Tokio "Graceful Shutdown" topic page: <https://tokio.rs/tokio/tutorial>
- "Actors with Tokio" by Alice Ryhl
- The Rust Book, ch. 17.3–17.5 (many futures, streams, the traits behind async)
- The `tokio::select!` macro docs, in particular the section on cancellation safety
