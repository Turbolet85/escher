
## 2026-10-07-refusal-detection — nine driver-action files, 27 tests; the record's unit test; the baselines re-counted
**Section:** §1 (escher-driver · tests/blitz-tests) · §2 (Process-lifecycle checks · Directory pattern) · §3 (Input helpers · blitz-tests stand checks · session checks · Agent-run Proof) · §3 → Session lifecycle (session-start · session-state · session-proof) · §4 escher-driver · §5 Session ↔ held instance · §9 Local baseline
**Change:**
- escher-driver: 27 unit tests (was 26) — `session.rs` 3, the third pinning the record of ids on a count of ids; six verbs (was five); the command tables read 28 admitted and 35 malformed rows (was 26, 34).
- Nine `stand_act_*` files with 27 tests (was six, 17): `stand_act_refused` 3, and new `stand_act_disabled` 2 · `stand_act_obstructed` 3 · `stand_act_scroll` 4; `stand_act_ids` reads 70 nodes accepted and 7 refused, and of the 15 controls 8 changing, 4 accepted with an empty diff, 3 refused (was accepted for all 77; 8 and 7). New engine check `scroll_into_view_nested` 7. Nine further mutation controls.
- `session_common`: `scroll`, `refused`, `refused_unchanged`, `Acted.in_view`; 14 readers (was 11); `mod common;` still 14. Input helper `scroll_into_view`, with no check of its own.
- Session lifecycle key: the lookup answers `stale` or `not-found`, the order `disabled` · `off-screen` · `covered`, `Outcome::Scrolled`; CI run 37673662374 on `983d8973` is the witness of the new files on macOS and windows.
- §5: any of the eight causes refused (was four named); `scroll` among the acting verbs. Re-counts appended: `run stand` 107 passed over 29 files; the workspace 647 passed over 153 result lines.
**Why:** the chunk's checks. Each figure is from the run's own listing; the reader counts are from the wrap's own grep.
**Kept:** the dated re-counts of earlier chunks stand as history. `stand_act_scroll` reads the list's box from its `bounds` before the scroll, since a scrolled box's own `bounds` read shifted.
**Ref:** .andromeda/runs/2026-10-07T20-27-47-wrap/
