
## 2026-10-07-command-and-refusal-schema — escher-driver's unit tests 13 → 25: the schema's pins, the workspace re-count
**Section:** §1 Test Scope Summary → escher-driver · §4 Unit Test Strategy → What unit tests cover → escher-driver · §9 CI Integration → Local baseline · §3 → Session lifecycle (the `session-proof` label)
**Change:**
- §1: was "13 inline unit tests in three files"; now 25 in six — `refusal.rs` 3, `schema.rs` 4 and `command.rs` 5 beside `error.rs` 3, `session.rs` 2 and `wire.rs` 8, each new test named by what it pins.
- §4: the row adds the three pins: the closed cause set and a refusal's printed form; the verb table, its lookup, shapes, kinds, bounds and key names; `validate` over admitted calls, unknown verbs, one malformed witness per rule per kind, the first-failure order, and table and `Command` in agreement. Every table is stated in its test with its row count asserted; an assertion over a call carries a row index only; four mutation controls each turned its named test red. No test drives the schema over the socket or through a `Session`.
- §9: the re-count chain gains a link — 143 result lines, 610 passed · 0 failed · 8 ignored (+12 `escher-driver` unit tests in the crate's existing result line); was 598 at the previous link, which stays as history.
- §3 → Session lifecycle, `session-proof`: was "escher-driver unit tests 13"; now the lifecycle's 13 of the crate's 25, the other 12 named as no part of that contract.
**Why:** the chunk added twelve unit tests and no other check. Trap for later chunks: the crate's count is stated in two word orders ("13 unit tests", "unit tests 13") — a count site is found by both.
**Kept:** `run stand` is unmoved (no `stand_*` check added) and takes no link.
**Ref:** .andromeda/runs/2026-10-07T12-34-00-wrap/
