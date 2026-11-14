@leaf_group group=obs-plan
@leaf_read leaf=.claude/docs/obs-summary.md lines=59
last: **Full plan:** `.andromeda/obs-plan.md`. Path-scoped rules: `.
@leaf_read leaf=.claude/rules/observability.md lines=41
last: - 2026-10-06: With the workspace's tracing 0.1.44, `tracing::
@leaf_edit n=1 leaf=.claude/docs/obs-summary.md line=19 class=omission
leaf-text: a bad `href` starts no load, so it reaches no fetch or load line.
source: dec-5qmnwhpl «A file: load asked by a non-file: document is refused as a typed forbidden error: a script fetch rejects with a TypeError, a module import fails, a script element's source is recorded as a script error, and an engine sub-resource fetch fails with fixed text naming no URL or path, adding no log site.»
replace-with: a bad `href` starts no load, so it reaches no fetch or load line; a `file:` load asked by a non-`file:` document is refused as a typed forbidden error — a script fetch rejects with a TypeError, a module import fails, a script element's source is recorded as a script error, and an engine sub-resource fetch fails with fixed text naming no URL or path, adding no log site.
