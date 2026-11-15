@leaf_group group=architecture+security-plan-2
@leaf_read leaf=CLAUDE.md lines=164
last: <!-- USER:session-learnings end -->
@leaf_edit n=1 leaf=CLAUDE.md line=59 class=omission
leaf-text: The workspace takes no git dependency — every crate a crates.io registry version
source: dec-jm3wczan «decision/no-git-dependency · No git dependency — The workspace takes no git dependency. · except: The taffy crate alone may take a git source; every other crate stays a crates.io registry version, and a git Taffy»
replace-with: The workspace takes no git dependency — every crate a crates.io registry version, the taffy crate alone excepted: it may take a git source, and a git Taffy still passes the lock, audit and MSRV legs
