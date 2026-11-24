@leaf_group group=architecture-2
@leaf_read leaf=.claude/docs/services/blitz-dom.md lines=45
last: - `doc:architecture` (the records store) · `.claude/docs/sta
@leaf_read leaf=.claude/docs/services/blitz-paint.md lines=36
last: - `doc:architecture` (the records store) · `.claude/docs/ser
@leaf_read leaf=.claude/docs/services/blitz-shell.md lines=35
last: - `doc:architecture` (the records store) · `.claude/docs/ser
@leaf_read leaf=.claude/docs/services/blitz-test-harness.md lines=38
last: - `.claude/rules/verification-harness.md` · `.claude/docs/te
@leaf_edit n=1 leaf=.claude/docs/services/blitz-dom.md line=12 class=stale
leaf-text: Taffy 0.15 (upstream's git source pinned to one revision, from #1372 on at a3997d55 since 2026-11-16-taffy-past-1372, the crates.io registry until then; taffy the one crate allowed a git source)
source: dec-qxbkesvi «Taffy 0.16 from crates.io does block, flexbox, grid, leaf, cached and out-of-flow layout through the stylo_taffy bridge, and is part of blitz-dom's public API through its signatures, with no re-export.»
replace-with: Taffy 0.16 (the crates.io registry; every dependency a registry version, no git source)
