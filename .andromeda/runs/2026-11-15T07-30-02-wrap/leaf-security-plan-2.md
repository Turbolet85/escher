@leaf_group group=security-plan-2
@leaf_read leaf=.claude/rules/security.md lines=32
last: - 2026-10-15: A listener or env read that only a test makes
@leaf_edit n=1 leaf=.claude/rules/security.md line=28 class=omission
leaf-text: the workspace takes no git dependency, the root manifest included (taffy, the one there was, a crates.io registry version since 2026-10-13-upstream-sync-polish-and-ship)
source: dec-jm3wczan «decision/no-git-dependency · No git dependency — The workspace takes no git dependency. · except: The taffy crate alone may take a git source; every other crate stays a crates.io registry version, and a git Taffy»
replace-with: the workspace takes no git dependency, the root manifest included, save taffy alone, which may take a git source (a git Taffy still passing the lock, audit and MSRV legs; every other crate a crates.io registry version)
