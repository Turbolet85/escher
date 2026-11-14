@leaf_group group=obs-plan
@leaf_read leaf=.claude/docs/obs-summary.md lines=59
last: **Full plan:** `.andromeda/obs-plan.md`. Path-scoped rules: `
@leaf_read leaf=.claude/rules/observability.md lines=41
last: - 2026-10-06: With the workspace's tracing 0.1.44, `tracing::
@leaf_edit n=1 leaf=.claude/docs/obs-summary.md line=19 class=omission
leaf-text: and an engine sub-resource fetch fails with fixed text naming no URL or path, adding no log site.
source: dec-5qmnwhpl «A file: load asked by a non-file: document is refused, adding no log site: a script fetch rejects with a TypeError, a module import fails, a script source records a script error, an engine fetch fails with fixed text naming no URL; the browser app's favicon and link navigation drop it silently.»
replace-with: and an engine sub-resource fetch fails with fixed text naming no URL or path, the browser app's favicon and link navigation dropping it silently, adding no log site.
@leaf_edit n=2 leaf=.claude/docs/obs-summary.md line=43 class=omission
leaf-text: a `malformed` refusal holding nothing of the path; `host_log` makes one
source: ctr-624ckxgp «A file that cannot be read, is over 1,048,576 bytes, is no check, is a check of another app or carries a format other than 1 is refused `malformed`, status 1, holding nothing of the path or the file.»
replace-with: a `malformed` refusal holding nothing of the path or the file; `host_log` makes one
@leaf_edit n=3 leaf=.claude/rules/observability.md line=32 class=omission
leaf-text: a `malformed` refusal holding nothing of the path (`host_log` reads
source: ctr-624ckxgp «A file that cannot be read, is over 1,048,576 bytes, is no check, is a check of another app or carries a format other than 1 is refused `malformed`, status 1, holding nothing of the path or the file.»
replace-with: a `malformed` refusal holding nothing of the path or the file (`host_log` reads
