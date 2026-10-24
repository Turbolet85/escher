
## 2026-10-24-stand-requirement-sweep — `stand-repeat`'s named list, the pinned limits and two state directories
**Section:** §Standard Contracts (standard-contracts.md: Scrolling, selection, tree → the text extent; Driver session → the visual lint; CI contracts → `stand-repeat`) · §Occupied Resources (occupied-resources.md: Filesystem; Process-wide state) · §Existing Scopes (existing-scopes.md: blitz-tests)
**Change:**
- `stand-repeat`: after the `stand_*.rs` glob, on the same line, ten named blitz-tests targets as literal `--test` words, each file keeping its name (test-plan §9 names them), pinned by `test_stand_repeat_leg_runs_three_rounds` and read from the script's text by `stand_requirement_sweep`'s leg check.
- The text extent's limits: each pinned by `text_extent` in both layout modes (was stated with no check).
- The visual lint: was «also reads `truncated`, asserted by no check»; now pinned by `stand_lint`, the `lint` verb answering the same.
- Filesystem: the seven_guis state directories gain `hl-client` and `hl-client-rec`; Process-wide state: `host_log`'s recording-clients check runs the binary as a client, `start crud --record` through `stop` at `RUST_LOG=trace`.
- blitz-tests: `stand_requirement_sweep` 3 tests (was 2) with `WITNESSES_020` and its `Unit` witness; `accessibility_states` also in `stand-repeat`; `stand_act_spans`' capture reads a paged part.
**Why:** the selection is the founder's own word «Список в скрипте (Рекомендую)» (2026-10-24, relayed verbatim by the overseer, chunk inputs#I1); the rest measured at this chunk. The two directories are test-only, under `target/tmp/`, beside the registered set — no listener, env var or crate added (`RUST_LOG` the registered read).
**Kept:** `hl-rec` and `hl-rec-kept` still unlisted in the Filesystem row, a gap older than this chunk.
**Ref:** .andromeda/runs/2026-10-24T17-32-19-wrap/
