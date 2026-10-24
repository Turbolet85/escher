# a11y-plan · picked accounting

## Picked
keyboard-navigation#2.4 · not touched · it cites `pointer.rs` and `stand_virtual_clock.rs`, neither reached, and names no runner, so `virtual_clock_engine` joining stand-repeat moves none of its claims (report: no engine change, behaviour unmoved)
keyboard-navigation#3.3 · not touched · the select key rules cite only `keyboard.rs` and `select.rs`, neither reached, and the report changes no engine file
keyboard-navigation#7.4 · not touched · its one runner claim, "neither file in the a11y leg's six", still holds (the a11y leg is unchanged at 6 files · 43 tests), and the report counts it among the ten-witness rows that name no leg the test runs in (no-change)
visual-design-verification#1.5 · proposed · `stand_lint.rs` gained lines 4-7, 110-142 and 226-275, so the row's two `stand_lint.rs` citations moved; its contrast prose and the 6 · 43 a11y tally still hold
visual-design-verification#8.1 · not touched · it cites `inert_subtree.rs`, which the change did not reach, and names no runner; the report lists it among the no-change ten-witness rows, and the behaviour is unmoved
screen-reader-support#3.1 · proposed · `stand_snapshot_parts.rs` lost `ITEMS`/`many_items` to `fixtures/mod.rs:64-86` and gained line 23, so both of its citations moved; the `fixtures/mod.rs:37-44` citation still holds, because the addition comes after it
ci-integration#1.0 · proposed · stand-repeat now also runs three a11y files by a named `--test` list (F1), with a check that reads the leg (F2); its `ci-leg.sh` stand-repeat range gained lines 54-58
