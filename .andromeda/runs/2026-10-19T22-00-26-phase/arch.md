# arch extract

## Relevance
partial — test-code placement, the provider-trait contract a held network stand-in implements, the stand's boot options and the agent-run contract; no engine, driver or app surface.

## Constraints
- Shared test code lives in a module in a subdirectory of blitz-tests' integration-test directory, read by `mod common;` / `mod session_common;`, so it is no test target and each test-target file still covers one behaviour; any third shared module beside them takes the same form. Whether the chunk's moved fixtures fit `common` (stand checks) or `session_common` (checks that hold a `Session`) by what each reader declares is research's question (per architecture §Conventions).
- blitz-tests keeps one test target per file directly under `tests/`, `[package]` sets no `autotests`, and the manifest's one `[[test]]` table is upstream's `tests/all.rs` at `test = false`, pinned by `BlitzTestsTargetsTest`; the chunk adds no `[[test]]` table and leaves `tests/all.rs` untouched (per architecture §Conventions).
- Every scenario moved runs in both layout modes, `for incremental in [false, true]` (per architecture §Conventions), the incremental and non-incremental pipelines staying one code path (per architecture §Design Philosophy).
- A fixture is booted with the stand's own `seven_guis::stand::options(incremental)` — pinned 800×600 viewport, bundled font, `net_provider: None` (offline through `DummyNetProvider`), `base_url: None`; whether every file-private boot helper the scope lists already boots that way is research's question (per architecture §Standard Contracts → Headless stand).
- The held network stand-in implements `NetProvider::fetch(doc_id, Request, Box<dyn NetHandler>)` and keeps `is_noop()` false, since callers register no pending-critical resource for a no-op provider; a supplied non-no-op `net_provider` in `HarnessOptions` is wrapped by the harness's crate-private counting provider, which is what a settle check's load-in-flight reading rests on (per architecture §Standard Contracts → Provider traits; per architecture §Standard Contracts → harness settle and load counting).
- No new listener, port, env var or workspace crate: a test-only bind or env read is registered in Occupied Resources before it lands; the scope's in-process stand-in should need none, and whether the file-private stand-ins read env, files or sockets today is research's question (per architecture §Occupied Resources → Network ports and listeners; per architecture §Occupied Resources → Environment variables; per architecture §Occupied Resources → Names).
- The agent-run contract `bash scripts/agent-run.sh {boot | run {stand|all|name} | status | cleanup | logs}` keeps its exit grammar `0`·`1`·`2`·`3`, its JSON-line events and its state in `target/agent-run/` only, pinned by `test_agent_run.py` in the `ci-scripts` leg; a new selection, if the P4 fork lands one, extends that grammar rather than adding a second entry (per architecture §Standard Contracts → agent-run test contract; per architecture §Occupied Resources → Filesystem).

## Patterns to follow
- Tables and helpers several checks share are stated once in `tests/blitz-tests/tests/common/mod.rs`, and what the `Session`-holding checks share in `tests/blitz-tests/tests/session_common/mod.rs` (per architecture §Conventions).
- A check first asserts its fixture produces the condition under test; assertion helpers carry `#[track_caller]`; each integration file opens with a `//!` doc naming its behaviour (per architecture §Conventions).
- Embedder services reach a document only through injected provider traits — the stand-in is handed in through `HarnessOptions`' `net_provider`, never reached by a global (per architecture §Design Philosophy → Embedder services are injected, with no-op defaults).
- The seven_guis test crate's own shared `tests/common/mod.rs` is the precedent for one helper set stated once for named readers alone (per architecture §Occupied Resources → Process-wide state and threads detail).

## Anti-patterns to avoid
- Rewriting upstream-authored test bodies (`render_blocking_stylesheet::ManualNetProvider`, `link_rel_attribute::RecordingNetProvider`) to use the shared stand-in — they are upstream's text, taken at each sync against a recorded merge base (per architecture §Project Intent → Upstream sync).
- A shared module made a test target, a new `[[test]]` table, or `autotests` set — it breaks the pinned blitz-tests target contract (per architecture §Standard Contracts → blitz-tests target contract).
- A stand-in that names `blitz_net`, fetches, opens a socket or file, or reads an env var — the stand's boot names no `blitz_net` and is offline (per architecture §Standard Contracts → Headless stand).

## Contract bindings
- arch §Standard Contracts (provider traits · harness load counting) ↔ tests: `stand_settle`'s load-in-flight reading and `font_face_sniffed_source`'s registration both rest on the shared stand-in being a non-no-op `NetProvider` the harness counts.
- arch §Standard Contracts (agent-run test contract) ↔ test-plan §3 (full contract and its «NOT YET MEASURED» bootstrap line) and `.github/scripts/test_agent_run.py`.
- arch §Conventions (shared test modules) ↔ test-plan §1/§7/§8 (shared modules, fixtures, network stand-in rows) and `.claude/rules/testing.md`.
- arch §Occupied Resources ↔ security-plan environment values: a test-only listener or env read is registered in both before it lands.

## Acceptance criteria contributions
- `BlitzTestsTargetsTest` passes unchanged: no `[[test]]` table added, `tests/all.rs` untouched, the shared module(s) under a subdirectory of `tests/blitz-tests/tests/` (per architecture §Conventions).
- The shared held stand-in's `is_noop()` reads false, and `stand_settle`'s load-in-flight assertion and `font_face_sniffed_source`'s assertions hold in both layout modes after the move (per architecture §Standard Contracts → Provider traits).
- The diff adds no workspace crate, listener, port or env read: the shared module names no `std::env`, `TcpListener`, `UnixListener`, `std::fs` or `blitz_net` (per architecture §Occupied Resources → Environment variables).
- `bash scripts/agent-run.sh run stand` (and any new selection) exits `0` with its state under `target/agent-run/` alone, and `test_agent_run.py` stays green in the `ci-scripts` leg (per architecture §Standard Contracts → agent-run test contract).
