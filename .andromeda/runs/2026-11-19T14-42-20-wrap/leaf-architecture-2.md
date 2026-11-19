@leaf_group group=architecture-2
@leaf_read leaf=.claude/docs/services/dioxus-native-dom.md lines=44
last: - `doc:architecture` · `.claude/docs/services/blitz-test-harn
@leaf_read leaf=.claude/docs/stack.md lines=59
last: 4. Commit as `chore: bump {crate} to {version}`.
@leaf_read leaf=.claude/docs/workflow.md lines=47
last: A reference-based CLAUDE.md: a thin index, with depth in `.cl
@leaf_edit n=1 leaf=.claude/docs/workflow.md line=9 class=omission
leaf-text: then `embolden`, which has no CI job;
source: dec-qcfn3mho «The fast leg run — fmt, clippy, test, ci-scripts, embolden, stopping at the first red — is the local pre-push gate; on a dev host, after a green verdict and skipped under CI, it ends by running the target sweep»
replace-with: then `embolden`, which has no CI job, stopping at the first red; on the dev host, after a green verdict and skipped under CI, it ends by running the target sweep, whose refusal or failure never reddens it;
@leaf_edit n=2 leaf=.claude/docs/stack.md line=39 class=omission
leaf-text: `ci-leg.sh fast` is the local pre-push gate
source: dec-qcfn3mho «The fast leg run — fmt, clippy, test, ci-scripts, embolden, stopping at the first red — is the local pre-push gate; on a dev host, after a green verdict and skipped under CI, it ends by running the target sweep»
replace-with: `ci-leg.sh fast` is the local pre-push gate, stopping at the first red and, on the dev host after a green verdict and skipped under CI, ending with the target sweep, whose refusal or failure never reddens it
