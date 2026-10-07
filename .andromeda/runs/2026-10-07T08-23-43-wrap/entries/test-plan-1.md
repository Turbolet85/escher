## 2026-10-07-sink-target-allowlist — the sink-installing host is covered: `host_log`, `telemetry_drop`, re-counts
**Section:** §1 Test Scope Summary → Coverage scope (apps · escher-telemetry · tests/blitz-tests) · §2 Test Strategy → Process-lifecycle checks, Directory pattern · §3 Test Harness Contract → Stand log format · §3 → Session lifecycle · §5 Integration Test Strategy → Session host ↔ lifecycle socket, Session host ↔ what it writes · §9 CI Integration → Local baseline
**Change:**
- Session host ↔ what it writes: was "It does NOT cover a host that installs escher's sink", the check "owed by the route entry"; now seven_guis' `host_log` covers it — the real `escher-session` binary on `crud` at `RUST_LOG=trace`, needles read from the stand booted in the test's own process (at least 15 ids asserted, 16 read; row and label texts), both streams drained by a thread each, `attach` under a 60 s bound and never `start`, exit 0, empty stdout, no state directory, every stderr line stamped, no id and no name on stderr, a failure printing counts only; seen red on the unfixed sink at 16 of 16 ids and 6 of 6 names in 1501 lines. `telemetry_drop` proves the drop for a third-party target under a naming `RUST_LOG` directive.
- Stand log format: the scrub clause restated as the three-outcome target allowlist; both binaries read no id and no name at the default level, `info`, `debug` and `trace` (0 lines, then the one 124-byte install line); "one line per printed event".
- escher-telemetry: unit tests 5 → 10 (9 in `format.rs`, five of them on the drop); its `telemetry_*` integration files four → five.
- seven_guis: integration-test targets one → two, with a shared module `tests/common/mod.rs` that is no target (the apps row and Directory pattern).
- Process-lifecycle checks: the 1.1 MB-at-`trace` figure is dated to before the drop (now one 124-byte line); a capturing check still drains while the host runs.
- Local baseline: 140 · 572 · 0 · 7 → 142 result lines · 579 passed · 0 failed · 8 ignored.
- Session lifecycle → session-proof: `host_log` 1 added, the fork's CI run on the chunk's pre-CI commit its witness on the other platforms.
- Three citations into `host_binary.rs` and two into the sink's source re-pointed.
**Why:** the chunk wrote the check the previous entry recorded as owed, and the drop it proves. Traps: a check that must fail red on a leaking sink cannot use `start` on a piped host — the unfixed host filled the pipe before it answered; a needle list read from the stand can outgrow a hand-kept one (16 against the instrument's 15).
**Kept:** §3 Agent-run Proof gains no link — `run stand` reads 72 · 0 · 3, unmoved.
**Ref:** .andromeda/runs/2026-10-07T08-23-43-wrap/
