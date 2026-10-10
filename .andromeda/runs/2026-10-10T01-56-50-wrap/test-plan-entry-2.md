
## 2026-10-10-upstream-sync-agent-surfaces — the test inventory gains upstream's tests: Selection, text-transform, autofocus, WPT variants; two arrivals with no test
**Section:** §1 Test Scope Summary → tests/blitz-tests (the coverage list), blitz-vibey-script, blitz-dom (layout), wpt/runner, blitz-net · §4 Unit Test Strategy → blitz-dom (layout), wpt/runner · §5 Integration Test Strategy → Script ↔ DOM
**Change:**
- tests/blitz-tests coverage gains four upstream checks: `autofocus_attribute` 5, `text_transform` 3, `anonymous_block_percentage_height` 1 and one test of `inline_fragment_rects` over three writing modes; the package's blitz-dom dev-dependency gains the `autofocus` and `text-transform-icu` features.
- blitz-vibey-script: tests/dom.rs was 26 `#[test]` functions, now 27; a third file, tests/selection.rs, holds 15 tests of the script Selection API; §5 names it as the crossing's coverage. Observed absent: a test of the `innerText` / `outerText` getters in the crate (whether a WPT case covers them was not read).
- blitz-dom (layout): was "one inline unit-test module, in list.rs"; now two — list.rs and upstream's `layout/text_transform.rs`, 22 tests; `layout/writing_mode.rs` holds none. §4 states what the 22 check.
- wpt/runner: the unit-test list (8, 3, 2, 1, 1) gains 10 in test_variants.rs, 2 in ref_test.rs and 1 in main.rs; §4 states what they check.
- blitz-net: the "no test" row adds that its `file:` read through `Url::to_file_path` has no test named for it either.
**Why:** every one of these arrived by the merge; none is a cargo test of ours added, removed or edited. The plan is tier 0 with no critical-path list, so the two untested arrivals are recorded as absences, not as failed gates.
**Kept:** upstream's default link rule on `a[href]` has no test named and no test-plan row states it; it is design-system's.
**Ref:** .andromeda/runs/2026-10-10T01-56-50-wrap/
