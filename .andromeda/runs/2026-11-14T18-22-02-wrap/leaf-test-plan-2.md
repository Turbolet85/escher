@leaf_group group=test-plan-2
@leaf_read leaf=.claude/rules/testing.md lines=71
last: - 2026-11-14: To plant a panicking handler in a Dioxus fixtu
@leaf_read leaf=.claude/rules/verification-harness.md lines=42
last: - 2026-10-31: State what a CI job did from that job's own lo
@leaf_edit n=1 leaf=.claude/rules/testing.md line=18 class=omission
leaf-text: `is_noop` not overridden; `RecordingShell`
source: dec-c7viqc7i «A blitz-tests check may read `file:` loads through blitz-net's real provider inside a hand-written stand-in, blocking on a current-thread tokio runtime the stand-in owns»
replace-with: `is_noop` not overridden; a blitz-tests check's hand-written stand-in that reads `file:` loads through blitz-net's real provider, blocking on a current-thread tokio runtime it owns, with blitz-net and tokio taken as blitz-tests dev-dependencies for it (`a_remote_page_reads_no_local_file_and_a_file_page_reads_them_all`); `RecordingShell`
@leaf_edit n=2 leaf=.claude/rules/testing.md line=48 class=omission
leaf-text: by `.github/scripts/coverage_floor.py` (a
source: dec-apon7s4t «Both coverage runs exclude dioxus-native-dom's crate-private keyboard_compat.rs until escher takes upstream's retirement of it; the floors are unchanged by the exclusion.»
replace-with: by `.github/scripts/coverage_floor.py`, both coverage runs excluding dioxus-native-dom's crate-private `keyboard_compat.rs` (upstream's temporary keyboard-types bridge) until escher takes upstream's retirement of it, the floors unchanged by the exclusion (a
