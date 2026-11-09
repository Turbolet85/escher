@leaf_group group=architecture+obs-plan
@leaf_read leaf=.claude/docs/services/escher-telemetry.md lines=46
last: - `.claude/rules/observability.md` · `.claude/docs/obs-summar
@leaf_edit n=1 leaf=.claude/docs/services/escher-telemetry.md line=3 class=tree
leaf-text: `.andromeda/obs-plan.md` §3 / §6 / §8
source: fct-dwg4fa67 «**Telemetry bootstrap (escher-telemetry):** `init` installs one stderr sink once, `ForeignSubscriber` (never a panic) when another is global»
replace-with: `doc:obs-plan` §3 / §6 / §8
@leaf_edit n=2 leaf=.claude/docs/services/escher-telemetry.md line=3 class=tree
leaf-text: `.andromeda/architecture.md` §Standard Contracts Telemetry bootstrap
source: fct-dwg4fa67 «**Telemetry bootstrap (escher-telemetry):** `init` installs one stderr sink once, `ForeignSubscriber` (never a panic) when another is global»
replace-with: `doc:architecture` §Standard Contracts Telemetry bootstrap
@leaf_edit n=3 leaf=.claude/docs/services/escher-telemetry.md line=32 class=omission
leaf-text: one made by the `escher-session` binary — its host role or its `mcp` role — runs under that binary's sink.
source: fct-gwvelgmp «also the `escher-session coverage` role, each replayed step of a kept check run through `Session::run` writing its own `command` line, the coverage line itself never logged or fielded»
replace-with: one made by the `escher-session` binary — its host role, its `mcp` role or its `coverage` role, each replayed step of a kept check writing its own `command` line, the coverage line itself never logged or fielded — runs under that binary's sink.
@leaf_edit n=4 leaf=.claude/docs/services/escher-telemetry.md line=33 class=stale
leaf-text: is read by `host_log`'s fourth test (unix only)
source: fct-hlttx6z4 «a paged `part` read through the `escher-session` host's own sink is read by its fifth test (unix only), which runs `serve cells` at `RUST_LOG=trace` in `hl-cells`»
replace-with: is read by `host_log`'s fifth test (unix only)
@leaf_edit n=5 leaf=.claude/docs/services/escher-telemetry.md line=42 class=stale
leaf-text: `host_log`'s fourth test (unix only) reads a paged part of a served Cells screen
source: fct-gwvelgmp «and `host_log`'s fifth test reads a paged part of a served Cells screen through the host's own sink (`serve cells`, `snapshot --part 2`), 0 id and 0 name needles under both builds»
replace-with: `host_log`'s fifth test (unix only) reads a paged part of a served Cells screen
@leaf_edit n=6 leaf=.claude/docs/services/escher-telemetry.md line=42 class=omission
leaf-text: the actions call's `actions_needles` (the CRUD screen's listed author-key ids);
source: fct-hlttx6z4 «`host_log` makes one `coverage crud <check>` run over a CRUD session kept through the binary, with a typed sentinel and a row click, and reads 0 id, name, coverage (`coverage_needles`»
replace-with: the actions call's `actions_needles` (the CRUD screen's listed author-key ids); since 2026-11-08-handler-coverage `host_log` also makes one `coverage crud <check>` run over a CRUD session kept through the binary, with a typed sentinel and a row click, reading 0 id, name, coverage (`coverage_needles`, the CRUD screen's UI-graph ids), sentinel, path and recorded needles at `trace` under both builds;
