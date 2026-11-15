@row_set n=1 row=decision/taffy-layout cell=rule seen=1 detector=D-arch-decisions
value: Taffy 0.15 from upstream's git source, pinned to one revision, does block, flexbox, grid, leaf, cached and out-of-flow layout through the stylo_taffy bridge, and is part of blitz-dom's public API through its signatures, with no re-export.
basis: fct-xfjzk5xx
because: Taffy moved from the crates.io registry 0.15.1 to upstream's git source pinned at a revision, so the row's registry source is now false.
@row_set n=2 row=decision/no-git-dependencies cell=rule seen=2 detector=D-arch-decisions
value: Every dependency in the manifest is a crates.io registry version, with no git source.
basis: fct-xfjzk5xx
because: Taffy is now the lock's one git source under the Taffy-only exception, so the rule's half naming taffy a registry version is false while the except stands as written.
@row_set n=3 row=decision/deferred-upstream-dependency-bumps cell=rule seen=2 fact=F1 detector=D-arch-decisions
value: Stylo stays "0.22.0", Taffy upstream's git source pinned before #1372 and Dioxus "0.7.3": git Stylo 0.23-pre and Dioxus 0.8.0-alpha.3 wait for escher's next version; Taffy from #1372 on, which leaves blitz-dom uncompiled, waits for its own take, where its reversed-scroll-origin change is asked.
basis: fct-tjzrz6wj
because: Taffy #1303 and its chain are taken through a git pin while any pin from #1372 on is deferred to its own take, so the row's registry Taffy and its waiting #1303 are false.
@row_set n=4 row=decision/upstream-take-engine-changes cell=rule seen=2 detector=none
value: The twenty-nine commits of the upstream take read at `563d3db73` and the eight upstream commits taken with git Taffy are engine behaviour changes for every document.
basis: fct-yad3la6t
because: The eight commits taken with git Taffy land as layout changes for every document ratified for their take, so the row naming the ratified upstream takes is incomplete without them.
