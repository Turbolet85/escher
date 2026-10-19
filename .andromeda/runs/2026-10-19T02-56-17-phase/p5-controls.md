# P5 controls — 2026-10-19-session-recording

Entry 8 (the silence probe), fired as the entry through `gate.py run --plan escher-0.2.0/chunks/2026-10-19-session-recording/plan.md --entry 8 --project-root $TMPDIR/{scratchpad}/p5-clone`, over a `git clone --shared` of the project at HEAD `93336e886aee198b345af2a85d0756e0d0124149`.

| clone role | input | reading |
|---|---|---|
| unmodified | neither file exists (as on the real tree) | `red · exit 0 ✗ (exit 2)` — the awk cannot open the two absent files |
| must-pass | `packages/escher-driver/src/recording.rs` (a stand-in, an FNV-1a 64 and a `fs::write`, no print, env, subscriber, listener or `tracing` site) and `tests/blitz-tests/tests/stand_session_recording.rs` (a both-modes test over `session_common::hold`) | `green · exit 0` — last line `0` |
| must-fail | the must-pass input plus one `tracing::info!(target: "escher_driver", …)` line in `recording.rs` | `red · last line '0' ✗ (last: '1')` |

The inputs lived in the scratch clone only; only these verdicts are recorded here.
