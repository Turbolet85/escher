
## 2026-10-06-compact-snapshot-serialization — the text form's tests and the re-counted totals
**Section:** §1 Coverage scope (dioxus-native-dom · dioxus-native and stylo_taffy · tests/blitz-tests) · §3 Crate-local test helpers · §3 Agent-run contract → Proof · §9 Pipeline facts → Local baseline
**Change:**
- dioxus-native-dom unit tests: was 33 in five files, thirteen on the snapshot builder; now 41 in six — fourteen on the snapshot builder (a file input reading the mask and never its path, an upper-case `FILE` included) and a new group of seven on `Snapshot::to_text` (a line's fields, the state tokens and their order, pre-order indent, `Debug`-form quoting with an apostrophe written as it is, shortest-decimal bounds, the empty snapshot, a real document), compiled only under `accessibility`. The list of the crate's files holding tests gains `snapshot_text.rs`.
- tests/blitz-tests coverage names a new stand file, `stand_snapshot_text.rs`: each lean task's screen as the snapshot's text in both layout modes — one line per node, every id on exactly one line, each screen under its in-file ceiling and under `SNAPSHOT_TEXT_BUDGET` (10,000 bytes; the largest reads 2034), the same text across two calls, the two layout modes and a second boot, `focused` on no line at boot and on `back-btn`'s after a Tab press, and a typed password and a file input's path masked on two in-file fixtures.
- Agent-run Proof: a dated link appended — `run stand` 48 → 54 `ok` stand events.
- Local baseline: a dated link appended — 129 result lines, 504 · 0 · 5 → 130 result lines, 518 · 0 · 5.
- 3 line citations re-pointed (`snapshot.rs`) by the chunk's measured line map.
**Why:** the chunk's own measured counts; earlier links of both chains stay as written. Trap for later chunks: a search for `snapshot.rs:` also matches `stand_snapshot.rs:`, a different file with its own line ranges — anchor a citation search on the path.
**Ref:** .andromeda/runs/2026-10-06T23-58-14-wrap/
