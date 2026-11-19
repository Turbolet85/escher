@leaf_group group=security-plan-1
@leaf_read leaf=.claude/docs/security-summary.md lines=57
last: **Full plan:** `.andromeda/security-plan.md`.
@leaf_edit n=1 leaf=.claude/docs/security-summary.md line=18 class=omission
leaf-text: naming no element id and no app slug.
source: dec-wj53ssit «decision/target-sweep-argv-validation · Build-directory sweep takes plan or run alone — The build-directory sweep takes only plan or run as its one argument; anything else prints one usage line on stderr and exits 2»
replace-with: naming no element id and no app slug; the build-directory sweep takes only `plan` or `run` as its one argument (anything else one usage line on stderr, exit 2, nothing run) and reads no environment value of its own.
