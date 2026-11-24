@leaf_group group=architecture-4
@leaf_read leaf=.claude/docs/services/seven_guis.md lines=45
last: - `.claude/rules/a11y.md` (the stand is in its paths) · `.cla
@leaf_read leaf=.claude/docs/stack.md lines=60
last: 4. Commit as `chore: bump {crate} to {version}`.
@leaf_read leaf=.claude/docs/workflow.md lines=48
last: A reference-based CLAUDE.md: a thin index, with depth in `.c
@leaf_edit n=1 leaf=.claude/docs/stack.md line=17 class=stale
leaf-text: Taffy 0.15 (upstream's git source, pinned to one revision — the one crate allowed a git source; std,
source: dec-qxbkesvi «Taffy 0.16 from crates.io does block, flexbox, grid, leaf, cached and out-of-flow layout through the stylo_taffy bridge»
replace-with: Taffy 0.16 (crates.io registry, no git source; std,
@leaf_edit n=2 leaf=.claude/docs/workflow.md line=15 class=stale
leaf-text: (Taffy now from upstream's git source, pinned to one revision)
source: dec-qxbkesvi «Taffy 0.16 from crates.io does block, flexbox, grid, leaf, cached and out-of-flow layout through the stylo_taffy bridge»
replace-with: (Taffy now 0.16 from crates.io, no git source)
@leaf_stands n=1 leaf=.claude/docs/stack.md line=19
because: The line's 0.15 is anyrender 0.15.0, not Taffy, and no changed row bears on the rendering crates it lists.
@leaf_stands n=2 leaf=.claude/docs/stack.md line=26
because: The line names no Taffy version or git source (its 0.15 only inside serde_json 1.0.151), and no changed row bears on its media crates.
