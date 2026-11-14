@leaf_group group=architecture-3
@leaf_read leaf=.claude/docs/stack.md lines=59
last: 4. Commit as `chore: bump {crate} to {version}`.
@leaf_read leaf=.claude/docs/workflow.md lines=47
last: A reference-based CLAUDE.md: a thin index, with depth in `.c
@leaf_edit n=1 leaf=.claude/docs/stack.md line=27 class=omission
leaf-text: the browser app (by default) and the WPT runner |
source: dec-7xt56qlk «decision/no-prng-crate · No PRNG crate for seeded randomness — Seeded randomness takes no PRNG crate: splitmix64 is written by hand and the host fallback seed comes from std RandomState. · why: no package enters the lockfile»
replace-with: the browser app (by default) and the WPT runner; no PRNG crate (since 2026-11-14-seeded-randomness): seeded randomness's splitmix64 is written by hand and its host fallback seed comes from std `RandomState`, no package entering `Cargo.lock` |
