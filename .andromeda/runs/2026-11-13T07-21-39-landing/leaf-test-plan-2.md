@leaf_group group=test-plan-2
@leaf_read leaf=.claude/rules/testing.md lines=66
last: - 2026-11-09: When a test joins a check file whose tests the
@leaf_read leaf=.claude/rules/verification-harness.md lines=42
last: - 2026-10-31: State what a CI job did from that job's own lo
@leaf_edit n=1 leaf=.claude/rules/testing.md line=30 class=stale
leaf-text: two recorded exceptions whose subject is wall time
source: dec-6he65a5m «No sleep or wall-clock read in checks — Session checks take no sleep and read no wall clock; time moves by `advance` alone.»
replace-with: four recorded exceptions whose subject is wall time (`stand_session_lifecycle`'s wait on the host's idle expiry and `stand_contrast`'s bound on a load's answer beside the two below)
@leaf_edit n=2 leaf=.claude/rules/testing.md line=48 class=omission
leaf-text: by `.github/scripts/coverage_floor.py` (
source: dec-apon7s4t «keyboard_compat.rs left out of coverage — Both coverage runs exclude dioxus-native-dom's crate-private keyboard_compat.rs until escher takes upstream's retirement of it; the floors are unchanged by the exclusion.»
replace-with: by `.github/scripts/coverage_floor.py`, both coverage runs excluding dioxus-native-dom's crate-private `keyboard_compat.rs` (upstream's temporary keyboard-types bridge) until escher takes upstream's retirement of it, the floors unchanged by the exclusion (
@leaf_edit n=3 leaf=.claude/rules/testing.md line=23 class=omission
leaf-text: Assert first that the fixture produces the condition under test, then the behaviour.
source: dec-7jkoc3jb «Tests controlled red-before-green — A new check is controlled: a named mutation of what it guards turns it red, and the restored tree reads green again.»
replace-with: Assert first that the fixture produces the condition under test, then the behaviour; a new check is controlled — a named mutation of what it guards turns it red, and the restored tree reads green again.
@leaf_edit n=4 leaf=.claude/rules/verification-harness.md line=25 class=omission
leaf-text: (`--model claude-opus-5-5 --max-budget-usd 3`)
source: dec-d6nq2ipk «Cold-agent client run bounded — The client runs under a 600 s timeout and a 3 USD budget; a timeout or non-zero client exit is a `client-exit` failure, never a propagated 124.»
replace-with: (`--model claude-opus-5-5 --max-budget-usd 3`, the client under a 600 s timeout, a timeout or non-zero client exit a `client-exit` failure, never a propagated 124)
