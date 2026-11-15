@row_set n=1 row=decision/no-git-dependency cell=except seen=1 fact=F1 detector=D-security-deps
value: The taffy crate alone may take a git source; every other crate stays a crates.io registry version, and a git Taffy still passes the lock, audit and MSRV legs.
basis: fct-xv7xpnit
because: The no-git-dependency rule now carries one exception for the taffy crate alone, so the row that states the rule must name it.
@row_set n=2 row=decision/no-git-dependencies cell=except seen=1 fact=F1 detector=D-security-deps
value: The taffy crate alone may take a git source; every other crate stays a crates.io registry version, and a git Taffy still passes the lock, audit and MSRV legs.
basis: fct-xv7xpnit
because: The rule is amended at every site it stands for the Taffy-only exception, this row among them, so its exception cell must name it.
@row_set n=3 row=decision/upstream-sync-dependency-set cell=rule seen=1 fact=F2 detector=D-security-deps
value: Each upstream sync takes upstream's dependency set whole or, by cherry-pick, enters no package and takes git-pinned content only by a lock-only, semver-compatible update of a pinned registry crate; its lock is read name by name against upstream's or the prior tree, the audit leg green after it.
basis: fct-hqhbkefo
because: The cherry-pick take carried upstream's git-pinned Taffy content by a lock-only semver-compatible registry move, a shape the row's rule did not name.
