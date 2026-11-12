# 06 · HTTP Server from Scratch

> Level 6 of 12 · about 11 nights · crate: `projects/httpd`

## Why this project

You learn HTTP by implementing it, and you learn Rust's concurrency model (threads, `Send`/`Sync`, `Arc`, `Mutex`, channels) by needing it. Using only the standard library means nothing is hidden. After this, frameworks like axum will make sense instead of looking like magic.

The Rust Book's final chapter builds a much smaller version. Use it as a hint when stuck, not as a script.

## Rust concepts

- `TcpListener`, `TcpStream`; the `Read`, `Write` and `BufRead` traits
- Bytes vs text: `&[u8]`, `Vec<u8>`, when a `String` is the wrong type
- `thread::spawn`, `JoinHandle`, `move` closures, the `'static` bound
- `Send` and `Sync`, and why `Rc` cannot cross threads
- `Arc`, `Mutex`, `RwLock`, atomics; lock poisoning
- `mpsc` channels
- Trait objects: `Box<dyn Fn(&Request) -> Response + Send + Sync>`
- Generics vs `dyn`: static and dynamic dispatch
- `Drop` for cleanup and graceful shutdown
- `TryFrom` / `FromStr` for parsing into types

## Dev skills

HTTP/1.1 on the wire, routing, static file serving, path traversal defence, timeouts and limits, load testing, debugging with `curl -v`.

## Allowed crates

**None.** `std` only. External tools for testing: `curl`, and a load tester such as `oha` or `wrk`.

## Spec

- Listens on an address given on the command line (default `127.0.0.1:8080`).
- Parses the request line, headers, and a body sized by `Content-Length`.
- Supports `GET`, `HEAD`, `POST`. Other methods get 405.
- A router maps (method, path) to a handler. Handlers are plain functions or closures.
- Built-in routes:
  - `GET /` and static files from a directory, with a correct `Content-Type` and 404 when missing
  - `POST /echo` returns the request body
  - `GET /stats` returns the number of requests served so far
- A request for `/../../etc/passwd` must not escape the static directory.
- Malformed requests get 400. Oversized headers or bodies get 431 or 413. Nothing a client sends can panic or hang the server.
- A fixed-size thread pool handles connections.
- A handler that panics takes down neither the server nor its worker thread permanently.
- Keep-alive: several requests on one connection.
- Shuts down cleanly: stops accepting, finishes in-flight requests, joins all workers.

## Milestones

- [ ] **M1 · Hello, socket.** Accept one connection, print the raw bytes received, send a hard-coded `200 OK`. Look at it with `curl -v`.
- [ ] **M2 · Request parser.** A `Request` struct and a parser that reads from any `impl BufRead`. Unit-test it against byte strings; no sockets in tests.
- [ ] **M3 · Response.** A `Response` type with a status enum, headers and body that writes itself to any `impl Write`.
- [ ] **M4 · Router.** Store handlers as boxed trait objects in a map. Register closures. 404 and 405.
- [ ] **M5 · Static files.** Content types by extension, `HEAD`, and the traversal defence with a test that proves it.
- [ ] **M6 · Thread per connection.** `thread::spawn` for each connection. Read the compiler errors about `Send` and `'static` carefully, then fix them with `Arc`.
- [ ] **M7 · Shared state.** `/stats` with `Arc<Mutex<u64>>`, then with `AtomicU64`. Know why both work.
- [ ] **M8 · Thread pool.** N workers pulling jobs from an `mpsc` channel. Decide what type a "job" is.
- [ ] **M9 · Robustness.** Read and write timeouts, header and body size limits, catching handler panics, respawning or surviving dead workers.
- [ ] **M10 · Keep-alive and shutdown.** Loop over requests per connection. Implement `Drop` for the pool so it drains and joins. Trigger shutdown from a line on stdin.
- [ ] **M11 · Measure and test.** Integration tests that bind port 0, start the server on a thread, and talk to it with a raw `TcpStream`. Load test single-threaded vs thread-per-connection vs pool, in `--release`, and write down the numbers.

## Traps when coming from other languages

- One `read` call does not return one request. TCP is a byte stream; you may get half a request or two.
- The body is bytes and may not be valid UTF-8. Headers are ASCII; do not treat the whole message as a `String`.
- Lines end in `\r\n`, and an empty line ends the headers. Off-by-one here causes hangs, not errors.
- A closure passed to `thread::spawn` must own what it uses (`move`) and that data must be `Send + 'static`.
- A `MutexGuard` holds the lock until it is dropped. Holding it while doing I/O serialises your whole server.
- With `Arc<Mutex<Receiver>>`, look closely at how long each worker holds the lock while waiting for a job.
- A thread that panics dies quietly, and poisons any mutex it was holding.

## Done when

- [ ] A browser can load an HTML page with CSS and an image from your server.
- [ ] `curl` checks pass for every route, for 400/404/405/413, and for the traversal attempt.
- [ ] A load test with 100 concurrent connections completes with no errors.
- [ ] Shutdown joins every thread; nothing is left running.
- [ ] Zero dependencies in `Cargo.toml`.
- [ ] You can explain `Send` and `Sync` in one sentence each, and why `Arc<Mutex<T>>` needs both layers.
- [ ] You can explain the difference between `impl Fn` and `Box<dyn Fn>` in your router.

## Stretch

- Chunked transfer encoding.
- Path parameters in the router (`/users/:id`).
- An access log in Common Log Format, written by a dedicated logger thread fed by a channel.
- Replace the thread pool with a non-blocking event loop using `mio` (a preview of project 12, track C).
- TLS with `rustls`.

## Reading

- The Rust Book: ch. 16 (Fearless Concurrency), ch. 18.2 (trait objects), ch. 21 (Final Project: Building a Multithreaded Web Server)
- MDN's "An overview of HTTP" and "HTTP messages"
- RFC 9112 (HTTP/1.1), sections on message format and message body
- Rust Atomics and Locks by Mara Bos, chapters 1–3 (free online): <https://marabos.nl/atomics/>
