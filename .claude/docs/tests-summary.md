# Tests Summary — escher

_Distilled from `.andromeda/test-plan.md` (adopted reading). wrap-session's cascade re-derives it when the plan changes._

## Test tier

**Tier:** 0
**Justification:** NOT YET MEASURED — the reading recorded no tier justification or critical-path list.

## Harness contract (§3)

- **Test runner:** Rust built-in harness (`#[test]`, `#[tokio::test]`); `test-that` matchers in some tests; WPT runner for conformance; `unittest` for CI scripts.
- **In-process harness (measured):** `blitz-test-harness::Harness` — construct (`from_html`, `from_component`, `from_vdom`; `HarnessOptions` also takes `font_ctx` and `incremental`), `pump`/`tick`, input helpers that pump, inspection (`query`, `layout_rect`, `text_content`, `focused`, `hit`, `dom_string`). See `.claude/rules/verification-harness.md`.
- **Headless stand (measured):** `seven_guis::stand::boot(LeanTask, options)` / `boot_timer(options)` mount counter, flight booker, timer or CRUD in TaskShell — fresh `VirtualDom` per boot, pinned 800×600 · scale 1 · Light, bundled DejaVu Sans with system fonts off, offline (`net_provider: None`); the timer advances only through its `TimerTicks` handle (`deliver(n)` then `pump`).
- **5-command discipline (measured):** `bash scripts/agent-run.sh {boot | run {stand|all|name} | status | cleanup | logs}` from the repository root (`scripts/agent-run.ps1` passes through to it on Windows). Exit `0` success · `1` failed (an empty run is never a pass) · `2` usage (checked first) · `3` precondition unmet. `boot` builds every blitz-tests binary; `run` drives `cargo test -p blitz-tests --locked`; `status` reads `status.json`; `cleanup` is idempotent; `logs` prints `events.jsonl`. Stdout is JSON lines only (`boot`, `run.start`, `test`, `run.end`, `status`, `cleanup`), never captured test output; state in `target/agent-run/{status.json, events.jsonl, run.log}`. The script starts no daemon and has no PID file, status endpoint, port or env var; the stand checks boot in process, bar the two session process checks, which start and stop their own host and socket (the "Session lifecycle" key, `.andromeda/registries/contracts/test-plan/session-lifecycle.md`). See `.claude/rules/verification-harness.md`.
- **Cold-agent run pipe (measured):** `bash scripts/cold-agent.sh {run counter | status | cleanup | logs}` — same exit grammar; `run` starts one isolated `claude -p` session (a per-run `mktemp -d` cwd, only the stdlib stdio MCP stub's `list`/`read`/`press`) and writes `target/cold-agent/verdict.json`: `passed` only on positive evidence (client exit 0, a successful `result`, `isolated`, `counts_agree`, ≥ 1 tool call, the stub's own count at 3); `wrong_calls` recorded, never deciding. Proof: `test_cold_agent.py` under a `claude` shim + one recorded live run (passed, 5 calls, 0 wrong). The live run is never in CI.
- **Stand log format (measured):** seven_guis' two binaries, `seven_guis_native` and `escher-session`, write stderr only, one `{RFC 3339 UTC} {LEVEL} {target} service.name=… service.version=… {field}={value}…` line per event, scrubbed (escher-telemetry, bound to obs-plan §3); exercised by the four `telemetry_*` integration files and a 10 s boot smoke (exit 124 + `service.name=seven_guis`).
- **Test-data bootstrap:** NOT YET MEASURED.

## Coverage as built
- `tests/blitz-tests` — integration tests across accessibility, layout, text, paint, hit testing, input, scrolling, invalidation, leaks, stylesheet loading, Dioxus integration (incl. falsy-`disabled` clearing and the clearing of the bridge's 27 boolean attributes when falsy, a `hidden: false` element staying displayed), the headless stand (`stand_*`: TaskShell mount, bundled-font text, two-boot and incremental `dom_string` identity, zero net requests, one driven interaction per lean task, timer ticks, one stable element id per element in both layout modes, the same id across a re-render, a remount and a fresh process, the id carried as AccessKit `author_id` with the 15 controls' roles and names and the Tab order unchanged, and each lean task's snapshot — id, role, name, state and bounds per node, coherent with the accessibility tree, deterministic and following a re-render, each control's state proven through real input (disabled until the app enables it, typed text read back, focus after a click, Tab and Shift+Tab; checked and the password mask on an in-file fixture), each lean task's screen as the snapshot's text — one line per node with its role, name, id, state and bounds, nested by indent, deterministic, every screen under its ceiling and the 10,000-byte `SNAPSHOT_TEXT_BUDGET`, a typed password and a file input's path masked on in-file fixtures —, the diff of the snapshots taken before and after a step — exactly the nodes whose reading changed, an empty diff and a false change flag for a step that changes nothing, equal across the two layout modes, with a hidden element, a masked password and a rewritten label on in-file fixtures —, the ids held across edits of the code around an element (before/after component pairs), and every actionable element of the lean tasks and Home reading an author key, with the three other tasks' unkeyed counts pinned — asserted unconditionally, no font skip; the tables and helpers the stand checks share are stated once in `tests/common/mod.rs`, which is no test target) and process telemetry (stdout silent, scrub, panic hook, idempotent init — one process per file).
- `escher-telemetry`: 5 inline unit tests (scrub decision branches, a bridged `log` record, the identity macro).
- `escher-driver`: 13 inline unit tests (the lifecycle wire's closed grammar, the label rule, the error messages); over the stand, `stand_session_state` · `_ids` · `_fresh` (one `Session` held across commands, both layout modes) and `stand_session_lifecycle` · `_quiet` (a host process over the socket, unix only); seven_guis' `host_binary` (2) drives the real `escher-session` binary. `stand_session_quiet`'s host installs no log sink — a sink-installing host at `debug` / `trace` is untested, owed by "Sink target allowlist".
- Differential oracle: incremental vs full layout compared node-by-node after every mutation step.
- Pixel tests on a CPU buffer (box-shadow expectations from Chromium, 1px AA tolerance).
- `blitz-vibey-script`: 26 DOM API tests + 2 headless Preact TodoMVC flows.
- WPT `css` + `svg` in upstream CI only (diffed against main, posted to PRs); on the fork WPT runs on the host.
- CI (§9): every linux leg runs through `.github/scripts/ci-leg.sh` — fast legs fmt · clippy · `test` (`cargo test --workspace --locked`) · CI scripts, then build/MSRV/counter/wasm/docs and the windows/macos/ios/android matrix; `test_ci_workflows.py` pins the workflow invariants, `test_agent_run.py` the agent-run contract under a `cargo` shim and `test_cold_agent.py` the cold-agent pipe and stub under a `claude` shim (64 CI-script tests).
- Local baseline (§9, dev host, dev profile `debug = "line-tables-only"`): `cargo test -p blitz-tests` 255 passed · 0 failed · 3 ignored (`paint_tree_bench`), 45 s cold (2026-10-05; +18 tests, +1 ignored since, not re-measured per crate); `cargo test --workspace` 572 · 0 · 7 over 140 result lines at 2026-10-07-driver-session (+13 `escher-driver` unit tests, +2 `host_binary`, +9 across the five `stand_session_*` files, +2 ignored host children; `run stand` 72 ok); 548 · 0 · 5 over 131 at 2026-10-07-audit-corrections (+6 dioxus-native-dom `bridge_tests` unit tests, no new result line; 542 · 0 · 5 over 131 at change-tracking-and-diff, +7 blitz-dom `changed_set_` and +8 `snapshot_diff` unit tests, +9 `stand_diff`; 518 · 0 · 5 over 130 at compact-snapshot-serialization, +7 `snapshot_text` and +1 `snapshot` unit tests, +6 `stand_snapshot_text`; 504 · 0 · 5 over 129 at snapshot-state-fidelity, +4 `snapshot` unit tests, +7 `stand_snapshot_state`, +3 `dioxus_falsy_boolean_attrs`; 490 · 0 · 5 over 127 at id-stability-across-code-edits, +7 `element_id` and +4 `actionable` unit tests, +5 `stand_id_edits`, +3 `stand_actionable_keys`; 471 · 0 · 5 over 125 at snapshot-model, +9 `snapshot` unit tests, +8 `stand_snapshot`; 454 · 0 · 5 at accessibility-tree-identity, +5 `accessibility_names`, +5 `stand_accessibility_ids`; 444 · 0 · 5 at id-persistence, +3 `stand_id_persistence`, +1 ignored re-exec child; 441 · 0 · 4 at stable-element-ids, +6 `element_id` unit tests, +4 `stand_element_ids`; 431 · 0 · 4 at upstream-sync-element-identity, +1 upstream wpt/runner unit test; 430 · 0 · 4 at headless-stand; 416 · 0 · 4 at telemetry-bootstrap; 407 · 0 · 3, 52 s cold at the 2026-10-05 baseline, was 1633 s under full debuginfo).
- Apps: only the browser crates test (about pages, history, favicon, suggestions, persistence).

## E2E coverage (§6)
- **Headless harness smoke** — HTML + Dioxus documents built, inspected and driven (`harness_smoke.rs`).
- **Preact TodoMVC** — add / toggle / filter / destroy / clear-completed through `ScriptDocument`.
- **Headless stand** — each lean task booted in TaskShell and driven through one interaction (`stand_counter` / `stand_flight_booker` / `stand_timer` / `stand_crud`).
- **Windowed E2E** — NOT YET MEASURED; the remaining stand flows are a route chunk ("Stand requirement sweep").

## Quality gates (§10)
| Gate | Threshold | Tool |
|---|---|---|
| Coverage | no threshold — first reading 53.23 % lines (dev host), 53.25 % (CI); a floor is "Quality gates" | `ci-leg.sh coverage` (cargo-llvm-cov; `coverage-report` artifact) |
| Flakiness budget | NOT YET MEASURED | "Quality gates" |
| Performance budget | NOT YET MEASURED | — |
| Format / lint / docs | must pass — fmt, clippy and the workspace rustdoc gate green | `ci-leg.sh fmt` · `ci-leg.sh clippy` (`cargo clippy --workspace --locked -- -D warnings`) · `ci-leg.sh doc` (`RUSTDOCFLAGS="-D warnings" cargo doc --workspace --no-deps --locked`) |

## Universal anti-patterns
> NO RECORDED INTENT.

## Critical decisions
> NO RECORDED INTENT — conventions as built are in `.claude/rules/testing.md`.

---

**Full plan:** `.andromeda/test-plan.md`. Path-scoped rules: `.claude/rules/testing.md`.
