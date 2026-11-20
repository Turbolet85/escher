@leaf_group group=test-plan-4
@leaf_read leaf=.claude/rules/testing.md lines=74
last: - 2026-11-19: A check written over a path just measured pins
@leaf_read leaf=.claude/rules/verification-harness.md lines=42
last: - 2026-10-31: State what a CI job did from that job's own l
@leaf_edit n=1 leaf=.claude/rules/testing.md line=48 class=omission
leaf-text: which runs the hung blitz-tests integration binaries before the library tests and reads their mutants `timeout`.
source: dec-vbmban3p «A cargo-mutants run scoped to one function anchors its filter on the mutant name's verb, the replace verb then the function's name and a space, and its mutant listing is read first and must show exactly»
replace-with: which runs the hung blitz-tests integration binaries before the library tests and reads their mutants `timeout`; a run scoped to one function anchors its filter on the mutant name's verb — the replace verb, then the function's name and a space (the filter is an unanchored regex over the whole mutant name, so a bare function name matches others holding it) — and its mutant listing is read first and must show exactly the mutants the run is meant to test.
