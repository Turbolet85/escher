@history_item n=1 marker=2026-10-06-telemetry-bootstrap
**2026-10-06-telemetry-bootstrap** — (the sink, the scrub, the panic hook) put escher-telemetry's allowlist scrub in place, where engine prefixes such as `dioxus_native` print only the seven safe fields. It also added the chaining native panic hook, after which std still prints the raw message, and set the rule that only the engine and upstream crates gate tracing behind the `tracing` feature. This chunk rests on all three. The RefCell panic it removes used to reach that hook. The mutation_writer trace sites that replay re-fires are judged by that scrub. The new buffer module must stay feature-gated, and escher-telemetry must stay unchanged.

@history_item n=2 marker=2026-10-06-snapshot-state-fidelity
**2026-10-06-snapshot-state-fidelity** — (the bridge's log-site citations) re-pointed the mutation_writer.rs citations in §6 Logged events and §8 Values logged as-is, and recorded the `trace!` count at 13 with no site added. This is the count and the citation set the buffered replay must leave unchanged with mutation_writer.rs kept byte-identical. It is also the precedent for re-pointing citations only, claims unchanged, if any line the plan cites moves.

@history_item n=3 marker=2026-10-07-audit-corrections
**2026-10-07-audit-corrections** — (Kept clause) recorded that dioxus-native-dom's unit tests appear in no obs-plan sentence, so their silence census was kept in the chunk's report only. This chunk puts the ported witness in dioxus_document.rs `mod tests`, and this is the precedent for where its 0 · 0 · 0 reading belongs.

@history_item n=4 marker=2026-10-07-command-and-refusal-schema
**2026-10-07-command-and-refusal-schema** — recorded that `Command`, `Call` and `ArgValue` derive `Debug`, which would print ids and typed text, and that nothing prints, logs or fields them. It set the rule: never field a type with `Debug`. That rule is the precedent for the chunk's anti-pattern: the buffer of ids, tags, attribute names and text must never be logged, fielded or `Debug`-dumped.

@history_item n=5 marker=2026-10-07-driver-session
**2026-10-07-driver-session** — (the second sink installer) made `escher-session` the second binary that installs the sink, beside `seven_guis_native`, and recorded a trap: a zero read from a host that installs no sink says nothing about one that does. The in-process witness and probe pins therefore cannot stand in for `host_log` / `mcp_log`. Those two read the sink-installing host that the buffered render now runs in.

@history_item n=6 marker=2026-10-07-sink-target-allowlist
**2026-10-07-sink-target-allowlist** — the sink drops every target outside the two prefix sets whole, `dioxus_core::*` and `dioxus_signals::*` included, and it set the rule that a target is admitted only together with a scrub rule for it. The cascade's binding relies on this: dioxus-core records stay dropped, and any site the port moves can reach the sink only as an engine-target event.

@history_item n=7 marker=2026-10-07-driver-command-spans
**2026-10-07-driver-command-spans** — (`command` span as built; `tracing` ungated in escher-driver) built the `command` span with its eight fields. It also recorded the two-build distinction: a workspace-wide build turns `blitz-dom/tracing` on, and a package-alone build leaves it off. That distinction is the basis for this chunk's "under both builds" acceptance and for the eight fields `stand_act_spans` re-asserts. The same chunk's PROVISIONAL entry added the closed-span record class and the rule that engine-target span pairs are judged by the safe-field rule.

@history_item n=8 marker=2026-10-09-founder-rulings
**2026-10-09-founder-rulings** — the founder ratified the closed span's line as the sink's second record class (2026-10-09). `host_log` and `mcp_log` count lines in this class, and the chunk re-asserts those counts unchanged.

@history_item n=9 marker=2026-10-10-driver-cli
**2026-10-10-driver-cli** — (the session host writes the command span; typed text measured in a host's log) made each hosted call write one closed `command` span. It measured the host by level under both builds, the workspace-built host writing one extra `blitz_dom::document` INFO line, with 0 id, name and typed-text needles. It also set the standing rule that a later surface adds no span or log field. These are the per-build readings the render change must leave as they are.

@history_item n=10 marker=2026-10-10-mcp-surface
**2026-10-10-mcp-surface** — added the `tool_call` span (`tool`, `ending`), with `command` nested as its child, and recorded the mcp role's stderr as 9 lines built alone and 10 workspace-built. The chunk keeps the 2 `info_span!` sites and these nesting and line counts.

@history_item n=11 marker=2026-10-13-upstream-sync-polish-and-ship
**2026-10-13-upstream-sync-polish-and-ship** — recorded the `434e0661` upstream merge as silent: the fixed-sha census read 0, and `host_log` / `mcp_log` passed under both builds after the merge. `434e0661` is the baseline against which this chunk keeps mutation_writer.rs byte-identical and its diff empty.

@history_item n=12 marker=2026-10-14-stand-requirement-sweep
**2026-10-14-stand-requirement-sweep** — (the workspace host's engine line printed twice) re-measured the workspace-built host at 6 lines with the `focussed node` INFO line printing twice, and the mcp role at 11 lines, with needles 0. Why the line prints twice was recorded, not established. The witness and pins move focus through `set_focus`, so the chunk must keep these counts and the focus line's form.

@history_item n=13 marker=2026-10-14-quality-gates
**2026-10-14-quality-gates** — drove `busy="render"` on a real line in `stand_act_spans`, using a fixture that re-keys an element every render, and read the workspace-built windowed `seven_guis_native` and clients by build and level, 28 runs, 0 needles. The buffered render changes how each re-render reaches the document, so the `busy` reading and these windowed readings are among what it must leave as they are.

@history_item n=14 marker=2026-10-19-recorded-check-replay
**2026-10-19-recorded-check-replay** — added the "Committed checks" row: `counter-{cli,mcp}.json` are committed in git and read by `stand_replay` and seven_guis' freshness check. Each replayed step writes its own `command` span. The cascade's binding requires the buffered render to replay these checks byte-unchanged, with neither re-recorded.

@history_item n=15 marker=2026-10-19-upstream-sync-visual-lint-and-real-app-readiness
**2026-10-19-upstream-sync-visual-lint-and-real-app-readiness** — recorded upstream's focus checks as silent, and kept the `Focussed node` INFO line standing after the new focus gate, with `host_log` and `mcp_log` passing under both builds. This is the focus line the chunk's focus moves fire under the workspace build.

@history_item n=16 marker=2026-10-19-test-splits
**2026-10-19-test-splits** — moved `host_log` and `mcp_log`'s needle helpers into seven_guis' shared `tests/common/mod.rs`, with assertion messages naming a kind and a count only, and recorded escher-driver's 2 `info_span!` sites. The chunk re-asserts these sink witnesses and the site count.

@history_item n=17 marker=2026-10-25-cold-agent-bar
**2026-10-25-cold-agent-bar** — (the host-sink part read measured) added `host_log`'s fourth test, unix only: `serve cells` at `RUST_LOG=trace`, a sentinel in a part-2 cell, then `snapshot --part 2`, with 0 id and 0 name needles under both builds. The cascade carries this test as the sink witness the render change reaches.

@history_item n=18 marker=2026-10-27-upstream-sync-engine-debts
**2026-10-27-upstream-sync-engine-debts** — (the take read at `ebacd68bd`; the test crate slice's absences) is the engine-change silence precedent: no log site, span, field or env read added, `packages/escher-telemetry/` and `packages/escher-driver/` both named unchanged, and dioxus-native-dom taking a single test. It also extended the 61-file test-crate slice's span and telemetry absences to a new target. The chunk's edit to a blitz-tests file must keep that slice reading true.

@history_item n=19 marker=2026-10-31-engine-debts
**2026-10-31-engine-debts** — (no log site added; the census's carve-out; `focussed node` re-read) added the added-lines census over `packages/` as the engine-change witness, with an inline `#[cfg(test)]` module held out so that its 0 does not speak for that module. That bears directly on the ported test in dioxus_document.rs `mod tests`. It also re-read the focussed-node row: once per focus move under the workspace build, a node number and no id or name.

@history_item n=20 marker=2026-10-31-select-popup
**2026-10-31-select-popup** — (an open list read through the host and the role; no log site or span; the Focussed node count) added `host_log` and `mcp_log`'s popup tests at `trace`, with 0 option needles and `host_log` counting its closed `command` span lines. It also recorded the focussed-node line's sink count on the workspace build as 0, with no base reading, so it settles nothing either way. Both are carried as bindings this chunk re-asserts.

@history_item n=21 marker=2026-11-01-upstream-sync-ui-graph
**2026-11-01-upstream-sync-ui-graph** — (the take read at `e42ee1f25` is silent) recorded that the `flush_queued_mounted_events` guard opens no span and adds no env read, print or `tracing` site. Its callers are the two that this chunk reshapes. It also kept 2 `info_span!`, the eight fields, and 0 needles in `host_log` / `mcp_log` under both builds. Its trap applies as well: a trait default can hide a fix, so sweep the implementers of any trait you extend. That means the buffer's `WriteMutations` implementation and its replay into `MutationWriter`.

@history_item n=22 marker=2026-11-04-dioxus-runtime-probe
**2026-11-04-dioxus-runtime-probe** — (the silence row; the read values stay in the test) added the `dioxus_runtime_probe` observed-absent row: 10 tests, both layout modes, the silence census reading 0. It also added the §8 item: failure messages name a layout mode, a task or page and a link or index, never a node id, and the reading quotes no panic payload or stderr. Its Kept clause holds that §3 takes no row for this target. This chunk inverts two of the file's tests, and both records must stay true.
