@leaf_group group=test-plan-2
@leaf_read leaf=.claude/rules/testing.md lines=77
last: - 2026-11-22: Any cargo run on the shared `target/` without
@leaf_read leaf=.claude/rules/verification-harness.md lines=42
last: - 2026-10-31: State what a CI job did from that job's own lo
@leaf_edit n=1 leaf=.claude/rules/testing.md line=15 class=omission
leaf-text: guards the host with kill-and-reap and,
source: dec-6xc6dozl «holds the host behind a guard that kills and reaps it, and sends the child's streams to null unless it reads them.»
replace-with: guards the host with kill-and-reap, sends the child's streams to null unless it reads them (so the child's libtest lines never reach agent-run's count) and,
@leaf_edit n=2 leaf=.claude/rules/testing.md line=15 class=omission
leaf-text: and read its stdout, stderr and exit status (`0` · `1` · `2` · `3`) through `tests/common/mod.rs`,
source: dec-7z5dsl7e «A check that captures a host installing escher's sink drains its pipes on threads from the spawn on, or captures to a file, and never waits for exit before reading.»
replace-with: and read its stdout, stderr and exit status (`0` · `1` · `2` · `3`) through `tests/common/mod.rs` (a check capturing a host that installs escher's sink drains its pipes on threads from the spawn on, or captures to a file, and never waits for exit before reading),
@leaf_edit n=3 leaf=.claude/rules/testing.md line=48 class=omission
leaf-text: Upstream's cdp-server and vibey-script duplicated test pairs are kept as upstream wrote them, the duplication reading not held against them.
source: dec-nh5z5mxh «Escher's integration-test targets over 800 code lines are kept whole, each one file and one target, with no route entry to split them.»
replace-with: Upstream's cdp-server and vibey-script duplicated test pairs are kept as upstream wrote them, the duplication reading not held against them, and escher's integration-test targets over 800 code lines are kept whole, each one file and one target, with no route entry to split them (a split moves the pinned target layout the CI legs read).
