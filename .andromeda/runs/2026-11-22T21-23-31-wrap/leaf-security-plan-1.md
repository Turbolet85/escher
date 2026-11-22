@leaf_group group=security-plan-1
@leaf_read leaf=.claude/docs/security-summary.md lines=57
last: **Full plan:** `.andromeda/security-plan.md`.
@leaf_edit n=1 leaf=.claude/docs/security-summary.md line=25 class=omission
leaf-text: `ci.yml` declares a workflow-level `permissions: contents: read` with no job grant, pins every action to a commit SHA and references no secret.
source: dec-gg74dumo «Every action a CI workflow uses is pinned to a 40-hex commit SHA, with its ref kept as a trailing comment.»
replace-with: `ci.yml` declares a workflow-level `permissions: contents: read` with no job grant and references no secret, and every action a CI workflow uses is pinned to a 40-hex commit SHA, its ref kept as a trailing comment.
