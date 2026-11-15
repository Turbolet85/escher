@row_set n=1 row=decision/engine-dependency-pins cell=rule seen=1 detector=D-security-deps
value: The pins stay: Stylo 0.22.0, Taffy the git pin of upstream DioxusLabs/taffy at commit e90751fd, Dioxus 0.7.3 and winit =0.31.0-beta.3.
basis: fct-xfjzk5xx
because: Taffy moved from the crates.io registry 0.15 to upstream's git repository at rev e90751fd, so the row's Taffy pin no longer holds.
@row_set n=2 row=decision/no-git-dependencies cell=rule seen=2 detector=D-security-deps
value: The root manifest holds no git dependency; parley and every other crate are crates.io registry versions with checksums in the lockfile.
basis: fct-xfjzk5xx
because: Taffy is now the lock's one git source under the row's own exception, so the rule's naming of taffy as a registry version with a checksum is false while parley's half stands.
@row_set n=3 row=decision/no-git-dependency cell=except seen=2 fact=F1 detector=D-security-deps
value: Taffy alone takes a git source: upstream pinned at a full commit rev, no patch section, integrity resting on the rev as its lock entry has no checksum; it still passes the lock, audit and MSRV legs.
basis: fct-xfjzk5xx
because: The Taffy exception is now taken, pinned at a full commit rev with no patch section and no lock checksum, so the exception must say what its integrity rests on.
@row_set n=4 row=decision/upstream-sync-dependency-set cell=rule seen=2 detector=D-security-deps
value: Each upstream sync takes upstream's dependency set whole or, by cherry-pick, enters no package and takes git-pinned content only by a lock-only, semver-compatible update of a pinned registry crate or Taffy's own git pin; its lock is read name by name, the audit leg green after it.
basis: fct-xfjzk5xx
because: This cherry-pick sync moved the taffy package alone from a registry version to a git pin under the Taffy exception, which the rule's lock-only registry update did not allow.
