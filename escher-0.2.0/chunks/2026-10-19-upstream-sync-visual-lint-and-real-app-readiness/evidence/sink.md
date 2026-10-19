# Sink allowlist re-check — 2026-10-19-upstream-sync-visual-lint-and-real-app-readiness

Measured by /andromeda-implement, run `.andromeda/runs/2026-10-19T19-16-25-implement/`, 2026-10-19. Lines quoted from
that run's trail `gate-2026-10-19-upstream-sync-visual-lint-and-real-app-readiness.json`.

## Upstream's delta
- Research's sweep over upstream's Rust delta past `fb8d6c15e` (`git diff fb8d6c15e upstream/main -- '*.rs'`) found 0
  log, print, env-read, subscriber, listener or wall-clock lines in 379 added lines (research.md §Sweeps).
- The taken commit, read at its fixed sha (entry 5):
  ```
    5 probe       green · exit 1 · 0.01s · 2 B → 5.log · git show --format= 38ca4e777513607c7dcb659748380e7a650b018… (267 chars)
  ```
  Its log reads `0`. `38ca4e777` adds no `tracing::`/`debug!`/`info!`/`warn!`/`error!`/`log::`/print/`dbg!`/env
  read/listener/`.bind(`/`Instant::now`/`SystemTime::now` site. `17de049b1` changes no `.rs` file.

## The standing capture checks, both builds
- **Per-package build** (entry 27, `cargo test -p seven_guis --locked`): green, 12 result lines · 33 passed · 0 failed,
  no `FAILED` — as at the base.
  ```
   27 integration green · exit 0 · 10.98s · 5910 B → 27.log · cargo test -p seven_guis --locked 2>&1
  ```
  - `Running tests/host_log.rs`: `test result: ok. 2 passed; 0 failed`.
  - `Running tests/mcp_log.rs`: `test result: ok. 2 passed; 0 failed`.
- **Workspace build** (inside entry 14, `ci-leg.sh fast`, `target/ci-logs/test.log` written by this run, mtime
  2026-10-19T19:20:47Z):
  ```
   14 unit        green · exit 0 · 155.41s · 112200 B → 14.log · bash .github/scripts/ci-leg.sh fast
  ```
  - `Running tests/host_log.rs`: `the_host_logs_no_id_no_name_and_no_typed_text_at_trace ... ok`,
    `a_recording_host_logs_nothing_of_its_check_at_trace ... ok` — `2 passed; 0 failed`.
  - `Running tests/mcp_log.rs`: `the_mcp_role_logs_no_id_no_name_and_no_typed_text_at_trace ... ok`,
    `a_recording_mcp_role_logs_nothing_of_its_checks_at_trace ... ok` — `2 passed; 0 failed`.
  Each reads 0 id, name, typed-sentinel and shot-prefix needles at `trace`.

## escher-telemetry
`git diff --quiet 67f17b9d632ceb66f6c01b347d9f9ba036dd29b6 -- packages/escher-telemetry` exits 0: the sink, its target
allowlist and scrub sets are unchanged (and inside the preservation guard, entry 2, green).

## The silence census — upstream's two new files
| file | env | print | subscriber |
|---|---|---|---|
| `tests/blitz-tests/tests/focus_within.rs` | 0 | 0 | 0 |
| `tests/blitz-tests/tests/focus_redraw.rs` | 0 | 0 | 0 |

Read by research.md §Sweeps (the added lines against the pattern plus `env!`: 0 hits); the files are byte-identical to
upstream's at `17de049b1` (`evidence/take.md`).

## The `Focussed node` INFO line
Still inside `set_focus_to`, under `#[cfg(feature = "tracing")]`, at `packages/blitz-dom/src/document.rs:1805-1806`
(`tracing::info!("Focussed node {focus_node_id}");`), now after upstream's not-in-document gate
(`if !self.nodes[focus_node_id].flags.is_in_document() { return false; }`, lines 1800-1803). It carries a node id
number, an engine target, as before; the gate fires only for a node outside the document, so the line's count on the
host's flow is unmoved, and the standing capture checks above read green.
