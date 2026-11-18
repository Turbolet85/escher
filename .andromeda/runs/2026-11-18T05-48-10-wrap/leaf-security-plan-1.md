@leaf_group group=security-plan-1
@leaf_read leaf=.claude/docs/security-summary.md lines=56
last: **Full plan:** `.andromeda/security-plan.md`.
@leaf_edit n=1 leaf=.claude/docs/security-summary.md line=42 class=omission
leaf-text: paste (RUSTSEC-2024-0436, unmaintained) reaches it and is ignored by ID,
source: dec-joj2g7z3 «An advisory is ignored only by its ID, each with a written reason that names its owner as a residual row by its id, with no blanket allow and unmaintained and unsound at their defaults; no product file names the residuals file.»
replace-with: an advisory is ignored only by its ID, each with a written reason that names its owner as a residual row by its id, no product file naming the residuals file; paste (RUSTSEC-2024-0436, unmaintained) reaches it and is ignored by ID,
