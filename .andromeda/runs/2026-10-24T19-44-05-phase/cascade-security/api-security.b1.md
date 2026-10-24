### api-security#1.0 · values · api · ## API Security · lines 8–8
No served network API exists — no TCP or UDP port; the one listener is local: the driver session's Unix-domain socket `{state_dir}/session.sock`, bound at one site since 2026-10-07-driver-session and serving the lifecycle (`hello`, `stop`) and, since 2026-10-10-driver-cli, driver calls and their answers (`call`, wire `v3` since 2026-10-19-session-recording, its replies including the fixed `ok v3 recording-failed`) (packages/escher-driver/src/host.rs:109-124; its row is below). Since 2026-10-10-mcp-surface the `escher-session mcp` role also serves the driver's verbs as MCP tools — twelve: the tenth, `help` (since 2026-10-12-self-description), answered by the server itself from the schema's fixed tables and running nothing on a session, the eleventh, `screenshot`, and the twelfth, `lint` — over its own stdin and stdout — no listener, port or socket, so the session socket stays the one listener — bounded by a 65,536-byte request line, a 1,048,576-byte tool result (one that holds an image measured by its whole written form, a text-only one by its text) and the idle expiry checked when a request arrives (the Input Validation MCP server row; the crossing ratified by the founder, 2026-10-10, «Ратифицировать как есть»). In the slices that searched, as adopted, server routes or listeners are observed absent — searched: `TcpListener|listen\(` over the 32 s10 files — and a served API is observed absent — searched: `TcpListener|TcpStream|bind\(|listen\(` over the 12 listed s13 files. The controls below govern outbound requests and in-document navigation.
### api-security#2.1 · values · api · ## API Security · lines 14–16
- **Control:** Rate limiting (outbound)
  - **Configuration:** Concurrent requests are limited to 6 per host (packages/blitz-net/src/lib.rs:24; packages/blitz-net/src/lib.rs:188-197)
  - **Stack reference:** blitz-net per-host semaphore
### api-security#2.2 · values · api · ## API Security · lines 17–19
- **Control:** CORS / same-origin
  - **Configuration:** Observed absent — searched: ` cors|content-security|same-origin ` over the 15 s05 files; and ` same.origin|same_origin|cross.origin|CORS ` over the 32 s10 files
  - **Stack reference:** —
### api-security#2.3 · values · api · ## API Security · lines 20–22
- **Control:** CSP
  - **Configuration:** Observed absent — searched: ` cors|content-security|same-origin ` over the 15 s05 files
  - **Stack reference:** —
### api-security#2.4 · values · api · ## API Security · lines 23–25
- **Control:** Request size limit
  - **Configuration:** Successful responses are read whole with `response.bytes()`, and the slice shows no size cap (packages/blitz-net/src/lib.rs:215-217); a response body size limit is observed absent — searched: ` body_limit|content_length|max_body ` over the 16 listed s08 files
  - **Stack reference:** reqwest
### api-security#2.5 · values · api · ## API Security · lines 26–28
- **Control:** Request timeout
  - **Configuration:** Observed absent — searched: `timeout` over the 16 listed s08 files
  - **Stack reference:** reqwest
### api-security#2.6 · values · api · ## API Security · lines 29–32
- **Control:** Settle and loads in flight
  - **Configuration:** A harness settle (`Harness::settle`) reports a load in flight at once — `NotSettled` naming the class `Loads` — and never waits on one: it waits on no clock and moves none — its `animating` reading reads the overlay scrollbar fade off the document's clock — so the absent request timeout above cannot hang it, and its pass loop is bounded at 64 passes; the driver's acting step over it (`Session::act` and the executor's settled step, so `click` · `type` · `press` · `advance` · `scroll` on every carrier) settles again every 10 ms while the answer is `Loads`, for at most `LOAD_WAIT`, 5 s of wall time under the 30 s answer read below, moving no time on the held instance, then returns the load still in flight as `settled: false` with its busy class — so the absent timeout cannot hang a command either, `LOAD_WAIT` ending every wait (since 2026-10-20-load-wait); the outcome is a returned value naming a class only — no URL, request, node, id, name or value — and the counter it reads stores counts only (packages/blitz-test-harness/src/settle.rs:15-28; packages/blitz-test-harness/src/settle.rs:60-127; packages/blitz-test-harness/src/settle.rs:151-186; as measured at escher-0.1.0/chunks/2026-10-07-settle-detection/report.md); the driver's command schema keeps this: none of its eight instance-level verbs has an argument or a result field of a wait — the seventh, `screenshot`, since 2026-10-12-headless-screenshot, taking one optional `id` and answering `width`, `height` and `png`, a shot resolving at the harness's own time with no tick and no pump, so it moves no time, and the eighth, `lint`, taking no argument and answering `findings`, running no step and settling nothing, so it moves none either; the overlay scrollbar fade reads the document's clock
 — on a harness-built document the harness's `VirtualClock`, which only `tick`, `advance_time` and the driver's `advance` move — so two shots of one state read the same pixels until an `advance` moves the fade, a document built with no clock still fading by wall time (ratified by the founder, 2026-10-18) (packages/blitz-test-harness/src/screenshot.rs:37-103) — nor do the three session-level verbs, whose one argument is `app` and whose result fields are `label`, `pid`, `served`, `idle_expiry_s` and `stopped`, nor, since 2026-10-12-self-description, the tool-level `help`, which takes no argument and whose seven result fields are fixed tables — and its validation reads no clock; the command line's own bounds, 30 s for a started host to answer and 30 s on each read of a call's answer, bound a transport and wait on no load; a step that ran and did not go quiet is a result (`settled: false` with its busy class) and not a refusal, and time on a held instance moves only when a caller asks for it with `advance`, 1 to 60000 ms a call — an acting driver command waits on a load in flight for at most `LOAD_WAIT`, the bound the `settled` field's help states, and `snapshot`, `screenshot` and `lint` wait on none — as measured at escher-0.1.0/chunks/2026-10-07-command-and-refusal-schema/report.md
  - **Stack reference:** blitz-test-harness · escher-driver
### api-security#2.7 · values · api · ## API Security · lines 33–35
- **Control:** Request headers
  - **Configuration:** Requests forward the caller-supplied headers and set `User-Agent` and an optional `Content-Type` (packages/blitz-net/src/lib.rs:199-206)
  - **Stack reference:** blitz-net
### api-security#2.8 · values · api · ## API Security · lines 36–38
- **Control:** HTTP status handling
  - **Configuration:** Non-success HTTP statuses become `ProviderError::HttpStatus` (packages/blitz-net/src/lib.rs:225-228)
  - **Stack reference:** blitz-net
### api-security#2.9 · values · api · ## API Security · lines 39–41
- **Control:** Cancellation
  - **Configuration:** Requests can carry an `AbortSignal` (packages/blitz-traits/src/net.rs:59; packages/blitz-traits/src/net.rs:74-77); aborting the configured `AbortSignal` cancels every in-flight fetch tied to the document (packages/blitz-dom/src/config.rs:62-65)
  - **Stack reference:** blitz-traits
### api-security#2.10 · values · api · ## API Security · lines 42–44
- **Control:** Sub-resource requests
  - **Configuration:** GETs built by `stamped_request` (packages/blitz-dom/src/net.rs:29-35)
  - **Stack reference:** blitz-dom
### api-security#2.11 · values · api · ## API Security · lines 45–47
- **Control:** JS-facing fetch
  - **Configuration:** GET/HEAD-only over the embedder's ScriptFetcher (packages/blitz-vibey-script/src/runtime.rs:155-180)
  - **Stack reference:** blitz-vibey-script
### api-security#2.12 · values · api · ## API Security · lines 48–50
- **Control:** Form submission
  - **Configuration:** GET for http/https/data appends the query; POST body only for http/https; mailto GET/POST build the URL; other scheme/method combinations are not implemented and return (packages/blitz-dom/src/form.rs:115-161); the `dialog` form method is rejected as not an HTTP method (packages/blitz-dom/src/form.rs:388-396)
  - **Stack reference:** blitz-dom
### api-security#2.13 · values · api · ## API Security · lines 51–53
- **Control:** Iframe navigation
  - **Configuration:** Sub-document navigations route back to the parent document instead of navigating the host (packages/blitz-dom/src/iframe.rs:29-46)
  - **Stack reference:** blitz-dom
### api-security#2.14 · values · api · ## API Security · lines 54–56
- **Control:** `dioxus` scheme
  - **Configuration:** Requests are answered from the local file system by `serve_asset` on the URL path; other schemes go to the inner net provider (packages/dioxus-native/src/assets.rs:42-61)
  - **Stack reference:** dioxus-native
### api-security#2.15 · values · api · ## API Security · lines 57–59
- **Control:** External navigation
  - **Configuration:** Allowed targets are handed to the system browser through `webbrowser::open` (packages/dioxus-native/src/link_handler.rs:12)
  - **Stack reference:** webbrowser
### api-security#2.16 · values · api · ## API Security · lines 60–62
- **Control:** JS evaluation (dioxus-native)
  - **Configuration:** Evaluation requested through the document context goes to `NoOpDocument` (packages/dioxus-native/src/contexts.rs:19-21)
  - **Stack reference:** dioxus-native
### api-security#2.17 · values · api · ## API Security · lines 63–65
- **Control:** Outbound HTTP (examples)
  - **Configuration:** A plain `reqwest` GET with only a `User-Agent` header (examples/screenshot.rs:45-52; examples/paint_bench.rs:74-83)
  - **Stack reference:** reqwest
### api-security#2.18 · values · api · ## API Security · lines 66–68
- **Control:** Outbound HTTP (apps)
  - **Configuration:** Outbound HTTP goes through blitz-net with http2 and, in rdme, reqwest (apps/browser/Cargo.toml:57; apps/readme/Cargo.toml:57)
  - **Stack reference:** blitz-net / reqwest
### api-security#2.19 · values · api · ## API Security · lines 69–71
- **Control:** CI API calls
  - **Configuration:** GitHub API calls go through the `gh api` CLI with `GH_TOKEN` from the workflow `token` secret (.github/scripts/wpt_diff_to_pr.py:263-270; .github/workflows/wpt-post-results.yml:29-49)
  - **Stack reference:** `gh` CLI
### api-security#2.20 · values · api · ## API Security · lines 72–74
- **Control:** Local session socket (escher-driver)
  - **Configuration:** Reachable by the owning user only (state directory `0700`, socket file `0600`, a directory open to its group or to others refused); no handshake secret, token or peer-credential check; one connection at a time, a 2 s read and write bound and 30 s on a client's read of a call's answer, a request line of at most 16,384 bytes, a lifecycle reply of at most 128 and an answer of at most 1,048,576 on its JSON line — over it the reply is `oversize`, read by the client as `AnswerTooLarge`; `stop` removes the socket and the directory; since 2026-10-10-driver-cli a host that has answered no request for its idle expiry — `IDLE_EXPIRY`, 1,800 s, `serve`'s third parameter — ends exactly as on `stop`, every answered request, a refused or malformed one and an `attach` or a `status` included, counting toward `served` and starting the count again, a connection that breaks before its request is whole doing neither, and an expiry, like a `stop`, writing no record (the founder's answer to the orphaned-host question, 2026-10-10, relayed by the overseer; until then a host whose client died without `stop` kept running) (packages/escher-driver/src/host.rs:36-214; packages/escher-driver/src/wire.rs:15-27)
  - **Stack reference:** `std::os::unix::net::UnixListener`
