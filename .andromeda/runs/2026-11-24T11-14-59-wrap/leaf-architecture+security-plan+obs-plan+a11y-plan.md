@leaf_group group=architecture+security-plan+obs-plan+a11y-plan
@leaf_read leaf=.claude/docs/gotchas.md lines=68
last: - Decisions and rationale: `doc:architecture` (the records st
@leaf_read leaf=CLAUDE.md lines=169
last: <!-- USER:session-learnings end -->
@leaf_edit n=1 leaf=.claude/docs/gotchas.md line=7 class=stale
leaf-text: taffy is Taffy 0.15 from upstream's git source, pinned to upstream DioxusLabs/taffy commit a3997d55 (Taffy from #1372 on, taken), the one git source the manifest admits (every other crate stays a crates.io registry version; a git Taffy still passes the lock, audit and MSRV legs) — and parley a registry version (0.13)
source: dec-jlrj566g «The pins stay: Stylo 0.24, Taffy 0.16.0 from crates.io, Dioxus 0.7.3 and winit =0.31.0-beta.4.»
replace-with: taffy is Taffy 0.16.0 from crates.io and parley a registry version (0.13), the manifest admitting no git source
@leaf_edit n=2 leaf=CLAUDE.md line=9 class=stale
leaf-text: Taffy 0.15 (upstream's git source, pinned to one revision)
source: dec-qxbkesvi «Taffy 0.16 from crates.io does block, flexbox, grid, leaf, cached and out-of-flow layout through the stylo_taffy bridge, and is part of blitz-dom's public API through its signatures, with no re-export.»
replace-with: Taffy 0.16 (crates.io)
@leaf_edit n=3 leaf=CLAUDE.md line=59 class=retired
leaf-text: every crate a crates.io registry version, the taffy crate alone excepted: it takes upstream's git source pinned at a full commit rev, no patch section, its integrity resting on the rev (its lock entry has no checksum), and still passes the lock, audit and MSRV legs
source: dec-kxrtstbq «The root manifest holds no git dependency; parley and every other crate are crates.io registry versions with checksums in the lockfile.»
replace-with: every crate a crates.io registry version with its checksum in the lockfile
