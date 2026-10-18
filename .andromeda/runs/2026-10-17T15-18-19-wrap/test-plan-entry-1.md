
## 2026-10-17-upstream-sync-engine-debts — the two merged ARIA checks; 127 files and 126 targets; bump leaves the inventory (§1, §2)
**Section:** §1 Test Scope Summary (tests/blitz-tests · apps) · §2 Test Strategy (Directory pattern)
**Change:**
- tests/blitz-tests: gains upstream's `accessibility_label` 10 and `accessibility_description` 3, asserted through test-that's `verify_that!`, each its own target, run by the workspace `test` leg and per package, not by the `a11y` leg's six; and records observed absent a test naming the merged `aria-expanded`, `aria-disabled` or `aria-level` reads.
- apps: `bump` leaves the no-tests inventory.
- Directory pattern: was «124 `.rs` files, 123 test targets»; now 127 and 126, `all.rs` gaining the two `mod` lines and staying unbuilt — the tree before the merge measured 125 and 124, so the earlier figures were one short; the run-time read gains 137 executables with no `all` among them.
**Why:** upstream's #859 · #1013 files, merged by the sync, and the measured count correcting a figure one short; the 127 / 126 on the operator's resume directive. The report credited the three closed-value reads to upstream tests, and the wrap's search found none.
**Ref:** .andromeda/runs/2026-10-17T15-18-19-wrap/
