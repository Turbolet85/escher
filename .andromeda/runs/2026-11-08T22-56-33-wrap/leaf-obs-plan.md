@leaf_group group=obs-plan
@leaf_read leaf=.claude/docs/obs-summary.md lines=59
last: **Full plan:** `.andromeda/obs-plan.md`. Path-scoped rules: `
@leaf_read leaf=.claude/rules/observability.md lines=41
last: - 2026-10-06: With the workspace's tracing 0.1.44, `tracing::
@leaf_edit n=1 leaf=.claude/docs/obs-summary.md line=43 class=stale
leaf-text: is read by `host_log`'s fourth test (unix only)
source: fct-hlttx6z4 «a paged `part` read through the `escher-session` host's own sink is read by its fifth test (unix only), which runs `serve cells` at `RUST_LOG=trace` in `hl-cells`»
replace-with: is read by `host_log`'s fifth test (unix only)
@leaf_edit n=2 leaf=.claude/docs/obs-summary.md line=43 class=stale
leaf-text: (`host_log` 4; measured at 2026-10-25-cold-agent-bar)
source: fct-hlttx6z4 «its failures naming a kind and a count only (`host_log` 5)»
replace-with: (`host_log` 5; measured at 2026-10-25-cold-agent-bar)
@leaf_edit n=3 leaf=.claude/docs/obs-summary.md line=43 class=omission
leaf-text: a command's answer, kept whole in a recorded check, never logged or fielded (2026-11-08-what-can-i-do-here).
source: fct-hlttx6z4 «`host_log` makes one `coverage crud <check>` run over a CRUD session kept through the binary, with a typed sentinel and a row click, and reads 0 id, name, coverage (`coverage_needles`, the CRUD screen's UI-graph ids), sentinel, path and recorded needles at `trace`»
replace-with: a command's answer, kept whole in a recorded check, never logged or fielded (2026-11-08-what-can-i-do-here). The `escher-session` `coverage` role's line — ids, owner ids, event names and two counts — is written on stdout as an answer, never a log line, a `malformed` refusal holding nothing of the path; `host_log` makes one `coverage crud <check>` run over a CRUD session kept through the binary, with a typed sentinel and a row click, and reads 0 id, name, coverage (`coverage_needles`, the CRUD screen's UI-graph ids), sentinel, path and recorded needles at `trace` under both builds (2026-11-08-handler-coverage).
@leaf_edit n=4 leaf=.claude/docs/obs-summary.md line=49 class=omission
leaf-text: escher-driver keeping its 2 `info_span!` sites (2026-11-08-what-can-i-do-here).
source: fct-d7dwo6bi «whose `handler_coverage` and dioxus-native-dom's reached record install no subscriber, read no environment variable or clock and add no log site, span or field, each replayed step of a kept check leaving its own `command` span»
replace-with: escher-driver keeping its 2 `info_span!` sites (2026-11-08-what-can-i-do-here); the `escher-session` `coverage` role (`coverage <app> <check>...`), its `handler_coverage` and dioxus-native-dom's reached record install no subscriber, read no environment variable or clock and add no log site, span or field, each replayed step of a kept check leaving its own `command` span through `Session::run`, escher-driver keeping its 2 `info_span!` sites (2026-11-08-handler-coverage).
@leaf_edit n=5 leaf=.claude/rules/observability.md line=18 class=omission
leaf-text: (its command line, host role and, since 2026-10-10-mcp-surface, `mcp` role)
source: fct-d7dwo6bi «the reader takes the first word `coverage` as a further role, `coverage <app> <check>...` with no `--session` and no `--record`»
replace-with: (its command line, host role, since 2026-10-10-mcp-surface, `mcp` role and, since 2026-11-08-handler-coverage, `coverage` role, whose `handler_coverage` and reached record install no subscriber, read no env var or clock and add no log site, span or field)
@leaf_edit n=6 leaf=.claude/rules/observability.md line=32 class=omission
leaf-text: an `actions` answer's ids, owner ids, the table's verb words, closed cause words and booleans, never a value, typed text or mask (a boundary widening, ratified) — never a log line
source: fct-d7dwo6bi «its coverage line — ids, owner ids, event names and two counts — is written on stdout by the crate's hand writer as an answer, never a log line, and a `malformed` refusal holds nothing of the path»
replace-with: an `actions` answer's ids, owner ids, the table's verb words, closed cause words and booleans, never a value, typed text or mask (a boundary widening, ratified), and the `coverage` role's line — ids, owner ids, event names and two counts, a `malformed` refusal holding nothing of the path (`host_log` reads 0 id, name, coverage, sentinel, path and recorded needles over one `coverage crud <check>` run at `trace`) — never a log line
@leaf_edit n=7 leaf=.claude/rules/observability.md line=36 class=stale
leaf-text: by `host_log`'s fourth test (unix only)
source: fct-hlttx6z4 «a paged `part` read through the `escher-session` host's own sink is read by its fifth test (unix only), which runs `serve cells` at `RUST_LOG=trace` in `hl-cells`»
replace-with: by `host_log`'s fifth test (unix only)
