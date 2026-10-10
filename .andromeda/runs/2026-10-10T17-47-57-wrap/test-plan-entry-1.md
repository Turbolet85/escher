
## 2026-10-10-snapshot-states — the driver-action census at twelve; counts re-read
**Section:** §1 → Coverage scope (escher-driver · dioxus-native-dom · tests/blitz-tests) · §2 → Directory pattern · §3 → Crate-local test helpers · §3 → Agent-run contract → Proof · §3 → Session lifecycle (`session-proof`) · §4 → What unit tests cover (escher-driver) · §5 → Session ↔ held instance · §6 → Command line · §9 → Local baseline
**Change:**
- Was "the eleven `stand_act_*` files" at seven sites (§1 escher-driver, §2, §3 twice, §4, §5 and the key file); now twelve, `stand_act_states` the twelfth with 3 tests. The key file's total was 37 over eleven files, now 40 over twelve, and its CI witnesses gain run 38072258834 on `2fc32829` (16 of 16 checks green).
- `mod session_common;` was read by 16 checks, now 17; `mod common;` stays at 17, six driver-action checks declaring both.
- §2: was 106 `.rs` files and 105 test targets; now 108 and 107 — `accessibility_states.rs` and `stand_act_states.rs`, auto-discovered with no manifest line. "No `all` executable among 107" is left as it reads, not re-measured.
- §1 dioxus-native-dom: was 55 unit tests — fourteen on the snapshot builder, seven on the text form, eight on the diff; now 60 — seventeen, eight and nine. The two limits of the boolean model are pinned there and at the engine only; no stand screen shows either.
- §1: the largest boot text, Crud, was 2034 bytes, now 2049. §6: the hosted `snapshot` line was 832 · 1,375 · 1,368 · 2,187 bytes, now 832 · 1,393 · 1,368 · 2,202.
- §3 Proof: `run stand` was 119 passed · 6 ignored over 31 files, now 125 · 6 over 32; `run all` not re-read. §9: the workspace leg was 165 result lines and 776 passed, now 167 and 791, 0 failed and 11 ignored both.
- "The file's eighth test" of `stand_snapshot_state` reads "the eighth test the file gained, of eleven".
**Why:** each figure is a measured reading at the chunk's gates or its operator pass.
**Ref:** .andromeda/runs/2026-10-10T17-47-57-wrap/
