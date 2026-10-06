
## 2026-10-06-snapshot-state-fidelity — state checks counted; the unit-level route for a `disabled` pin recorded
**Section:** §1 Coverage scope → dioxus-native-dom · → tests/blitz-tests · §3 Crate-local test helpers → dioxus-native-dom · §3 Agent-run contract → Proof · §9 Local baseline
**Change:**
- dioxus-native-dom unit tests: was 29 in five files, nine on the snapshot builder; now 33, thirteen on the snapshot builder (a password mask, a present-`disabled` reader pin, a radio, a range input).
- tests/blitz-tests coverage names two new files: `stand_snapshot_state.rs` (per-control state through real input; checked and the password mask on an in-file fixture) and `dioxus_falsy_boolean_attrs.rs` (the 27-name falsy clear on both attribute paths).
- Crate-local helpers: the snapshot unit tests' `build` helper carries the no-op HTML parser, so `dangerous_inner_html` renders nothing at unit level; the `disabled` pin there writes its attributes through `DocumentMutator`, and parsed markup is pinned in blitz-tests.
- Agent-run Proof: a dated link appended — `run stand` 41 → 48 `ok` stand events.
- Local baseline: a dated link appended — 127 result lines, 490 · 0 · 5 → 129 result lines, 504 · 0 · 5.
- 2 line citations re-pointed (`snapshot.rs`).
**Why:** the chunk's own measured counts; earlier links of both chains stay as written. Trap for later chunks: a headless key press is not the same on every platform — the editor's Backspace arm is compiled out on macOS, so a deleting key in a stand check read green on Linux and red on the macOS CI leg; prove a state change with clicks and typed characters.
**Ref:** .andromeda/runs/2026-10-06T22-39-16-wrap/
