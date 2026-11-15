@leaf_group group=architecture+security-plan-2
@leaf_read leaf=CLAUDE.md lines=164
last: <!-- USER:session-learnings end -->
@leaf_edit n=1 leaf=CLAUDE.md line=59 class=stale
leaf-text: the taffy crate alone excepted: it may take a git source, and a git Taffy still passes the lock, audit and MSRV legs
source: dec-jm3wczan «Taffy alone takes a git source: upstream pinned at a full commit rev, no patch section, integrity resting on the rev as its lock entry has no checksum; it still passes the lock, audit and MSRV legs.»
replace-with: the taffy crate alone excepted: it takes upstream's git source pinned at a full commit rev, no patch section, its integrity resting on the rev (its lock entry has no checksum), and still passes the lock, audit and MSRV legs
@leaf_edit n=2 leaf=CLAUDE.md line=9 class=omission
leaf-text: · Taffy 0.15 ·
source: dec-qxbkesvi «Taffy 0.15 from upstream's git source, pinned to one revision, does block, flexbox, grid, leaf, cached and out-of-flow layout through the stylo_taffy bridge, and is part of blitz-dom's public API»
replace-with: · Taffy 0.15 (upstream's git source, pinned to one revision) ·
