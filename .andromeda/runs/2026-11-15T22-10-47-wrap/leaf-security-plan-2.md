@leaf_group group=security-plan-2
@leaf_read leaf=.claude/rules/security.md lines=32
last: - 2026-10-15: A listener or env read that only a test makes
@leaf_edit n=1 leaf=.claude/rules/security.md line=28 class=stale
leaf-text: the root manifest included (taffy, the one there was, a crates.io registry version since 2026-10-13-upstream-sync-polish-and-ship), save taffy alone, which may take a git source — a git Taffy still passing the lock, audit and MSRV legs, every other crate a crates.io registry version
source: dec-jm3wczan «Taffy alone takes a git source: upstream pinned at a full commit rev, no patch section, integrity resting on the rev as its lock entry has no checksum; it still passes the lock, audit and MSRV legs.»
replace-with: the root manifest included, save Taffy alone, which takes a git source — upstream DioxusLabs/taffy pinned at a full commit rev (e90751fd), no patch section, integrity resting on the rev as its lock entry has no checksum, still passing the lock, audit and MSRV legs; every other crate a crates.io registry version with its checksum in the lockfile
