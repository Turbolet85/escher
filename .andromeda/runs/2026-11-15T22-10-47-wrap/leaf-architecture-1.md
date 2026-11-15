@leaf_group group=architecture-1
@leaf_read leaf=.claude/docs/commands.md lines=82
last: - Fonts missing in headless tests → `system-fonts` feature / f
@leaf_read leaf=.claude/docs/conventions.md lines=61
last: - Architecture: `doc:architecture` (the records store) · dire
@leaf_read leaf=.claude/docs/gotchas.md lines=68
last: - Decisions and rationale: `doc:architecture` (the records st
@leaf_edit n=1 leaf=.claude/docs/gotchas.md line=7 class=stale
leaf-text: taffy is a registry version (0.15) since 2026-10-13-upstream-sync-polish-and-ship — a git dep pinned by `rev` until then, so no git dependency is left in the manifest, though taffy alone may take a git source (every other crate stays a registry version; a git Taffy still passes the lock, audit and MSRV legs)
source: dec-qxbkesvi «decision/taffy-layout · Taffy for layout — Taffy 0.15 from upstream's git source, pinned to one revision, does block, flexbox, grid, leaf, cached and out-of-flow layout through the stylo_taffy bridge»
replace-with: taffy is Taffy 0.15 from upstream's git source, pinned to one revision before #1372 (Taffy from #1372 on leaves blitz-dom uncompiled and waits for its own take), the one git source the manifest admits (every other crate stays a crates.io registry version; a git Taffy still passes the lock, audit and MSRV legs)
