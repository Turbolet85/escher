@leaf_group group=test-plan-2
@leaf_read leaf=.claude/rules/testing.md lines=64
last: - 2026-11-01: A probe of a panic can read green because its 
@leaf_read leaf=.claude/rules/verification-harness.md lines=42
last: - 2026-10-31: State what a CI job did from that job's own lo
@leaf_edit n=1 leaf=.claude/rules/testing.md line=48 class=stale
leaf-text: (dioxus-native-dom 2791/3241 = 86.12 %, blitz-test-harness 999/1040 = 96.06 %, those two inferred from counts, not from the lcov)
source: fct-fx4qekph «escher-driver 4959/5058 = 98.04 %, dioxus-native-dom 2937/3387 = 86.71 % (floor 83 %), blitz-test-harness 999/1040 = 96.06 % (as measured at CI#42371051720's `Coverage report` job»
replace-with: (dioxus-native-dom 2937/3387 = 86.71 %, blitz-test-harness 999/1040 = 96.06 %, as read at CI#42371051720's `Coverage report` job)
