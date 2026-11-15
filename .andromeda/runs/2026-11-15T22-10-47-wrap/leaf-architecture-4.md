@leaf_group group=architecture-4
@leaf_read leaf=.claude/docs/services/seven_guis.md lines=44
last: - `.claude/rules/a11y.md` (the stand is in its paths) · `.cl
@leaf_read leaf=.claude/docs/stack.md lines=59
last: 4. Commit as `chore: bump {crate} to {version}`.
@leaf_read leaf=.claude/docs/workflow.md lines=47
last: A reference-based CLAUDE.md: a thin index, with depth in `.c
@leaf_edit n=1 leaf=.claude/docs/stack.md line=17 class=stale
leaf-text: Taffy 0.15 (crates.io registry, resolved 0.15.1; a git dependency at a pinned rev until 2026-10-13-upstream-sync-polish-and-ship;
source: dec-qxbkesvi «Taffy for layout — Taffy 0.15 from upstream's git source, pinned to one revision, does block, flexbox, grid, leaf, cached and out-of-flow layout through the stylo_taffy bridge»
replace-with: Taffy 0.15 (upstream's git source, pinned to one revision before #1372 — the one crate allowed a git source;
@leaf_edit n=2 leaf=.claude/docs/workflow.md line=15 class=stale
leaf-text: the Taffy-main content comes through registry Taffy 0.15.x save #1303, which waits with the later git Taffy chain for its own chunk, so Stylo stays "0.22.0", Taffy the registry "0.15" and Dioxus "0.7.3".
source: dec-vu7h54n4 «Stylo stays "0.22.0", Taffy upstream's git source pinned before #1372 and Dioxus "0.7.3": git Stylo 0.23-pre and Dioxus 0.8.0-alpha.3 wait for escher's next version; Taffy from #1372 on, which leaves»
replace-with: Taffy from #1372 on, which leaves blitz-dom uncompiled, waits for its own take, where its reversed-scroll-origin change is asked, so Stylo stays "0.22.0", Taffy upstream's git source pinned before #1372 and Dioxus "0.7.3".
