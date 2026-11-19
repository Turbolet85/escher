@leaf_group group=obs-plan
@leaf_read leaf=.claude/docs/obs-summary.md lines=60
last: **Full plan:** `.andromeda/obs-plan.md`. Path-scoped rules: `.
@leaf_read leaf=.claude/rules/observability.md lines=42
last: - 2026-10-06: With the workspace's tracing 0.1.44, `tracing::
@leaf_edit n=1 leaf=.claude/docs/obs-summary.md line=32 class=omission
leaf-text: read by `stand_replay` and seven_guis' freshness check; no log or event line; uploaded by no CI leg.
source: ctr-qmpyuzzk «contract/target-sweep-line/line · The build-directory sweep's stdout line · line — Each ending of the dev-host build-directory sweep prints exactly one JSON line on stdout, a closed key set per outcome, holding no path, unit name or key.»
replace-with: read by `stand_replay` and seven_guis' freshness check; no log or event line; uploaded by no CI leg. The dev-host build-directory sweep prints exactly one JSON line on stdout per ending — a closed key set per outcome: `verb` (`plan` · `run`), `outcome` (`swept` · `planned` · `refused` · `failed`), a swept or planned line's counts `units_kept`, `units_removed` (run) or `units_to_remove` (plan), `bytes_before`, `bytes_after` and `seconds`, a refused or failed line's `cause` (`build-in-flight` · `no-build-directory` · `unsupported-host` · `keep-run`) — holding no path, unit name or key.
