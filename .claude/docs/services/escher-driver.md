# escher-driver

_Crate notes. Primary source: `.andromeda/architecture.md` §Standard Contracts Driver session, §Occupied Resources and [Driver session]; the checks' contract is test-plan §3 → Session lifecycle (`.andromeda/registries/contracts/test-plan/session-lifecycle.md`)._

## Responsibility
The driver's first piece (unpublished, no `[features]`, no binary): a **session** — one headless app instance held by one process while commands arrive. It is app-agnostic: the caller supplies the boot, and the crate names no app and no stand type. No verb set, settle, act by id, CLI JSON or MCP tool is built yet — those are later route entries.

## Key integrations

### Consumes from
- `blitz-test-harness` (`Harness`) and `dioxus-native-dom` (`DioxusDocument`), with no feature named on either — it reads no snapshot, so it enables no accessibility code. The entry that first reads the snapshot through a session decides the feature.
- `std` only for the socket (`std::os::unix::net`), the spawn and the file modes. No `tracing`, no env read, no default state location.

### Publishes to
- `Session::start(label, boot)` — the label is 1 to 32 bytes of `a-z`, `0-9`, `-`, checked BEFORE the boot runs (`InvalidLabel`, nothing booted); the boot runs once on the calling thread; `label()`, `harness()`, `harness_mut()`.
- `serve(state_dir, session)` — hosts one session until a `stop` request, on the calling thread (the instance is not `Send`): creates the state directory `0700` (a present one open to group or others is `StateDirNotPrivate`), binds `session.sock` and sets it `0600`, replaces a socket nothing answers on, reads `AlreadyRunning` when one answers, and on `stop` drops the session and removes the socket file and the directory — non-recursively, nothing else.
- `start(state_dir, host, ready)` → `Started { child, hello }` · `attach(state_dir)` → `Hello { pid, label, served }` · `stop(state_dir)` (waits up to 5 s for the directory to go).
- `SessionError`: `AlreadyRunning` · `NoSession` · `Dead` · `HostExited(code)` · `Timeout` · `Protocol` · `InvalidLabel` · `StateDirTooLong` · `StateDirNotPrivate` · `Unsupported` · `Io(kind)` — fixed messages, never a path, label, id or value.
- Consumed by seven_guis (the `escher-session` host binary, native target only) and by blitz-tests (dev-dependency).

## Internal conventions
- The wire (`wire.rs`, crate-private, versioned `v1`) is one request line and one reply line per connection: `hello v1` → `ok v1 pid= label= served=`, `stop v1` → `ok v1 stopping`, another version of a known word → `refused version`, anything else → `refused malformed`. Requests are bounded at 64 bytes, replies at 128, each read and write at 2 s. **Nothing of the screen crosses it** — the first request that carries an id, a name, a value, snapshot text or a diff is a new crossing question for the operator.
- `served` counts every request answered, refusals included; a refusal or a broken connection changes nothing; a `stop` takes effect once its reply is written.
- Unix only: on other targets `serve`, `start`, `attach` and `stop` return `Unsupported`; a `Session` held in process works everywhere.

## Crate-specific gotchas
- A Unix socket address leaves about a hundred bytes for the whole path (108 on Linux, 104 on macOS): keep a state directory's name short; an over-long one is `StateDirTooLong`.
- There is no idle expiry: a host whose client died without `stop` keeps running (owed by the "Driver CLI" entry).
- `start` does not check that the answering pid is the child it spawned.
- The `Io` message is spelled "input or output" — "I/O" holds a `/`, and the messages are asserted to hold none.
- A test's working directory is its package: build a check's state directory from `env!("CARGO_TARGET_TMPDIR")`, never from a relative path.

## Tests
- 13 unit tests (`wire.rs` 8 · `session.rs` 2 · `error.rs` 3) — they need no app.
- Over the stand, in blitz-tests: `stand_session_state` · `_ids` · `_fresh` (a session held in process) and `stand_session_lifecycle` · `_quiet` (a host process: the test binary re-run on an `#[ignore]` child); seven_guis' `host_binary` drives the real `escher-session` binary, and `host_log` reads its stderr at `RUST_LOG=trace`. `stand_session_quiet`'s host installs no log sink, so it proves nothing about a host that does — `host_log` is the check that does.
