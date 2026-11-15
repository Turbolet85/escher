@leaf_group group=architecture-1
@leaf_read leaf=.claude/docs/commands.md lines=82
last: - Fonts missing in headless tests → `system-fonts` feature / f
@leaf_read leaf=.claude/docs/conventions.md lines=61
last: - Architecture: `doc:architecture` (the records store) · dire
@leaf_read leaf=.claude/docs/gotchas.md lines=68
last: - Decisions and rationale: `doc:architecture` (the records st
@leaf_edit n=1 leaf=.claude/docs/gotchas.md line=7 class=omission
leaf-text: so no git dependency is left in the manifest —
source: dec-ofaljlii «decision/no-git-dependencies · No git dependencies — No git dependency is left in the manifest; taffy is a registry version. · except: The taffy crate alone may take a git source; every other crate»
replace-with: so no git dependency is left in the manifest, though taffy alone may take a git source (every other crate stays a registry version; a git Taffy still passes the lock, audit and MSRV legs) —
@leaf_edit n=2 leaf=.claude/docs/commands.md line=30 class=omission
leaf-text: cargo-mutants 27.1.0 with `--test-tool nextest`
source: dec-la3dsw2u «decision/mutation-tier-excludes-demo-shells · Demo shells outside the mutation tier — The mutation tier does not run the rdme and blitz packages and owes them no test for it;»
replace-with: cargo-mutants 27.1.0 (never over the rdme and blitz packages, upstream's test-less demo shells, which it owes no test) with `--test-tool nextest`
