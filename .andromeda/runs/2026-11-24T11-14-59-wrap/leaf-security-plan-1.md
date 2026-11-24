@leaf_group group=security-plan-1
@leaf_read leaf=.claude/docs/security-summary.md lines=56
last: **Full plan:** `.andromeda/security-plan.md`.
@leaf_stands n=1 leaf=.claude/docs/security-summary.md line=42
because: The line states the dependency audit's reach, its ignored advisories and their residual owners, and says nothing of Taffy's source or a git pin, so the taffy-layout row's move from upstream's git pin to Taffy 0.16 from crates.io leaves it true.
