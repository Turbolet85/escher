@leaf_group group=architecture-4
@leaf_read leaf=.claude/docs/services/seven_guis.md lines=44
last: - `.claude/rules/a11y.md` (the stand is in its paths) · `.cla
@leaf_read leaf=.claude/docs/stack.md lines=59
last: 4. Commit as `chore: bump {crate} to {version}`.
@leaf_read leaf=.claude/docs/workflow.md lines=47
last: A reference-based CLAUDE.md: a thin index, with depth in `.cl
@leaf_edit n=1 leaf=.claude/docs/workflow.md line=15 class=stale
leaf-text: git Stylo 0.23-pre, the six Taffy-main commits and Dioxus 0.8.0-alpha.3 wait for escher's next version
source: dec-vu7h54n4 «decision/deferred-upstream-dependency-bumps · Deferred Stylo, Taffy and Dioxus bumps — Stylo stays "0.22.0", Taffy the registry "0.15" and Dioxus "0.7.3": git Stylo 0.23-pre and Dioxus 0.8.0-alpha.3 wait»
replace-with: git Stylo 0.23-pre and Dioxus 0.8.0-alpha.3 wait for escher's next version, and the Taffy-main content comes through registry Taffy 0.15.x save #1303, which waits with the later git Taffy chain for its own chunk
@leaf_edit n=2 leaf=.claude/docs/stack.md line=39 class=omission
leaf-text: each unit's mutant test command read green on the unmutated tree first
source: dec-la3dsw2u «decision/mutation-tier-excludes-demo-shells · Demo shells outside the mutation tier — The mutation tier does not run the rdme and blitz packages and owes them no test for it; churn, sizes and the other collectors»
replace-with: each unit's mutant test command read green on the unmutated tree first, the rdme and blitz packages (upstream's demo shells, holding no tests) not run by it and owed no test for it
