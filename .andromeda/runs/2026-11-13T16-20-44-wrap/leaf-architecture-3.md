@leaf_group group=architecture-3
@leaf_read leaf=.claude/docs/services/seven_guis.md lines=44
last: - `.claude/rules/a11y.md` (the stand is in its paths) · `.cl
@leaf_read leaf=.claude/docs/stack.md lines=59
last: 4. Commit as `chore: bump {crate} to {version}`.
@leaf_read leaf=.claude/docs/workflow.md lines=47
last: A reference-based CLAUDE.md: a thin index, with depth in `.c
@leaf_edit n=1 leaf=.claude/docs/workflow.md line=15 class=stale
leaf-text: upstream `main` was last read at `e42ee1f25`, thirty-six of the fifty-seven commits past the merge base taken by no-commit cherry-picks with the merge base unmoved and twenty-one left out
source: dec-z45xhlo5 «decision/upstream-take-engine-changes · Upstream take as engine behaviour changes — The twenty-nine commits of the upstream take read at `563d3db73` are engine behaviour changes for every document.»
replace-with: upstream `main` was last read at `563d3db73`, the twenty-nine commits of that take taken by no-commit cherry-picks with the merge base unmoved, engine behaviour changes for every document
@leaf_edit n=2 leaf=.claude/docs/stack.md line=41 class=omission
leaf-text: with every `uses:` pinned to a commit SHA
source: dec-wholok3w «decision/pinned-actions · SHA-pinned actions — Every uses: of every workflow is pinned to a 40-hex commit SHA, its ref kept as a trailing comment, and every rust-toolchain step names its toolchain.»
replace-with: with every `uses:` pinned to a 40-hex commit SHA, its ref kept as a trailing comment, as in every workflow
