## 2026-10-07-settle-detection — settle neither reads nor drains the changed set
**Section:** §2 A11y Strategy → Accessibility tree lifecycle (the `changed_nodes` bullet)
**Change:** the bullet now states that a harness settle (`Harness::settle`, the driver session's `act`) neither reads nor drains the changed set, so a step settled through it leaves the tree refresh, and a later diff, exactly what the step marked; and that the set is non-empty on a freshly booted harness document — the boot's own in-document mutations mark it and nothing has drained it — so a check that reads `has_changes()` as "this step changed something" drains first. The ratified refresh-on-change and the changed-set contract are unchanged.
**Why:** a settle that drained the set would starve the shell's refresh and the later diff; the boot reading was measured when the chunk's idle check failed its plan's letter.
**Ref:** .andromeda/runs/2026-10-07T11-06-31-wrap/
