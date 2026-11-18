@history_item n=1 marker=2026-10-24-a11y-re-read
**2026-10-24-a11y-re-read** — the last a11y re-read: a new §6 observed-absent row recording the eight checks the `a11y` leg runs beside its six earlier files (`accessibility_states`, `accessibility_label`, `accessibility_description`, `accessibility_merged_states`, `accessibility_active_descendant_unread`, `inert_subtree`, `select_interaction`, `stand_act_select`) and the shared `tests/common/mod.rs` / `tests/fixtures/mod.rs` silent, measured by a silence-census gate read red with a planted `println!` and green on the tree. The direct precedent: any check this chunk adds to or extends in the `a11y` leg owes the same silence row and gate (decision/checks-silent).

@history_item n=2 marker=2026-10-13-stand-contrast-harness
**2026-10-13-stand-contrast-harness** — `stand_contrast` and the contrast reader (`Harness::text_contrast`, `contrast_ratio`, `TextContrast`, the private `contrast` module) recorded silent in §3 and spanless in §4: no subscriber, env read, print or listener, readings into assertions only, reading and changing nothing in the document. This chunk re-asserts SC 1.4.3 through that reader and check, and they must stay silent and spanless.

@history_item n=3 marker=2026-10-13-stand-keyboard-harness
**2026-10-13-stand-keyboard-harness** — `stand_keyboard` joined the silent census (four lean tasks and its own fixtures, both layout modes, no subscriber, env read, print or listener), the shared module's `tab_order` and `focused` probed silent too. This chunk's SC 2.1.1 / 2.4.3 re-assertion extends that check; a probe must cover the shared module as well as the check file (the report's probe once read the check alone).

@history_item n=4 marker=2026-10-14-stand-a11y-assertions
**2026-10-14-stand-a11y-assertions** — the silence census over the extended `stand_contrast`, `stand_keyboard` and `stand_accessibility_ids` (the last newly named): each still installs no subscriber, reads no env var, prints nothing, binds no listener. The prior SC assertions this chunk re-reads; extended tests re-take the same census.

@history_item n=5 marker=2026-10-18-engine-debts
**2026-10-18-engine-debts** (the silence census gains the engine-debts checks) — `inert_subtree`, `accessibility_merged_states`, `accessibility_active_descendant_unread` and the rewritten `stand_keyboard` and `accessibility_states` recorded silent; the engine changes (inert among them) add no log site, span, field or env read. Fixtures this chunk names in its surfaces; the `inert` behaviour its SC 2.4.3 re-read relies on.

@history_item n=6 marker=2026-10-31-engine-debts
**2026-10-31-engine-debts** (the chunk's surfaces in §6; the focussed-node re-read; the five new checks silent; the `multiple` select / `optgroup` / `change` crossings; `stand_keyboard`'s reach; the fixtures readers 24 → 28) — the inert focus fixup, the `multiple` select, `optgroup`, `size` and the engine's checkable `change` add no log site; `focussed node` INFO re-read beside the fixup with 0 needles; `checkable_change_event` and others silent; §8 rows for the chosen set, the `optgroup` label and `change` (rows decision/select-chosen-set-never-logged, contract/page-events/*, contract/multiple-select-form-entries/chosen-option, contract/snapshot/multiple-select-value); `stand_keyboard` now drives the lean tasks, the load page and Cells. These are the engine debts this chunk re-asserts after; a check reading them must keep the carried sets and the census as stated.

@history_item n=7 marker=2026-10-31-select-popup
**2026-10-31-select-popup** (the select list adds no log site or span; its checks silent; an open list read through the host and the role; labels as pixels and the `size` read ratified; an open row's bounds and `SelectListRow.label`) — `select_popup`, `select_popup_painted` and `stand_act_popup` silent; `node::select_list` / `render::select_list` open no span; `host_log` / `mcp_log` read 0 option needles at `trace`; ratified widenings: an open list's shot carries option and group labels as pixels, `size` capped at 65534, `optgroup` header painted (rows decision/open-list-shot-carries-labels, decision/optgroup-label-crossing, decision/select-size-attribute-read, decision/select-list-label-never-logged, contract/snapshot-diff/open-list-row-bounds). The select list's colours and Tab handling are what SC 1.4.3 / 2.4.3 re-read here; contrast and focus checks on it must add no log site and no new crossing.

@history_item n=8 marker=2026-10-31-select-popup
**2026-10-31-select-popup** (the Focussed node line's count, not settled) — the `focussed node` count through the sink on the workspace build read 0 with no base reading, neither moved nor unmoved. A trap for a focus-order re-read: a count with no base settles nothing.

@history_item n=9 marker=2026-10-20-select-by-id
**2026-10-20-select-by-id** — `select_interaction` and `stand_act_select` recorded silent and the select model adds no log site, span, field or env read. `select_interaction` is one of the a11y fixtures this chunk re-reads.

@history_item n=10 marker=2026-10-19-shared-fixtures
**2026-10-19-shared-fixtures** — the third shared module `tests/blitz-tests/tests/fixtures/mod.rs` recorded silent (no print, tracing call, subscriber, env read, file, socket, listener or `blitz_net`); the «their shared modules» clause of decision/checks-silent covers it. A fixture page added for this chunk's re-assertions lands there and stays silent.

@history_item n=11 marker=2026-10-07-audit-corrections
**2026-10-07-audit-corrections** — the headless-stand census names the checks' shared module `tests/blitz-tests/tests/common/mod.rs` (no init, env read or print). Origin of decision/checks-silent covering shared modules; a helper this chunk adds to `common` falls under it.

@history_item n=12 marker=2026-10-06-headless-stand
**2026-10-06-headless-stand** — the headless stand and its in-process checks install no subscriber, read no env var and print nothing; only the windowed binary installs telemetry. Origin of decision/headless-stand-no-sink, which the stand-facing a11y checks run under.

@history_item n=13 marker=2026-10-19-upstream-sync-visual-lint-and-real-app-readiness
**2026-10-19-upstream-sync-visual-lint-and-real-app-readiness** — upstream focus checks `focus_within` and `focus_redraw` silent; the focus engine changes (`:focus-within`, not-in-document focus refusal, focus-move redraw) add no log site; the `Focussed node` INFO line stands. Focus paths the SC 2.4.3 re-read walks.

@history_item n=14 marker=2026-10-19-visual-lint
**2026-10-19-visual-lint** — `Harness::text_contrast` is also the lint's contrast reader, recorded spanless with no new field (the no-new-field mark dropped by 2026-10-20-select-by-id on the founder's word). A change to the contrast reader for SC 1.4.3 reaches the lint too and must add no span or field.

@history_item n=15 marker=2026-11-01-upstream-sync-ui-graph
**2026-11-01-upstream-sync-ui-graph** (the upstream take read at `e42ee1f25` is silent) — `Harness::text_contrast`'s blend read opens no span; `key_press_after_container_lost_its_box` recorded silent. The contrast reader's blend and a keyboard check this chunk's re-read rests on.

@history_item n=16 marker=2026-10-25-cold-agent-bar
**2026-10-25-cold-agent-bar** (the host's six slugs, the served stand's silence) — the six served tasks (`counter` · `flight-booker` · `timer` · `crud` · `cells` · `load`) and their boots (`boot_cells`, `boot_load`, `LoadNet`) recorded silent, the census over `stand.rs`, `stand_cells_served`, `stand_load_page` and the shared modules reading 0. The served pages this chunk re-asserts on; reading them must move no driver schema, span field or log line.

@history_item n=17 marker=2026-10-12-headless-screenshot
**2026-10-12-headless-screenshot** — `fonts-unavailable` returned before anything is painted; a shot's image leaves only in its answer, 0 image bytes on stderr (rows contract/refusal-cause/fonts-unavailable, decision/shot-image-leaves-only-in-answer, contract/command-answer/shot). With bundled DejaVu fonts the contrast reader's painted text rests on this cause no longer firing; shots used as evidence stay answers, never logged.

@history_item n=18 marker=2026-10-18-one-virtual-clock
**2026-10-18-one-virtual-clock** — the command span's causes become eight, `fonts-unavailable` among them (ratified by 2026-10-18-engine-debts). The closed cause set a refused re-assertion call would record.

@history_item n=19 marker=2026-11-07-ui-graph
**2026-11-07-ui-graph** (the snapshot's clickable reading carries no page text) — `clickable` a closed bool after `focused` (rows contract/snapshot-node/clickable, contract/snapshot-text/clickable), false on a disabled form control. Read by any check asserting which controls are operable by keyboard.

@history_item n=20 marker=2026-11-08-what-can-i-do-here
**2026-11-08-what-can-i-do-here** (the actions read opens no span and adds no site) — `Session::actions` reads without acting, the read changing nothing in the document (row decision/actions-read-changes-nothing). If the SC 2.1.1 re-read enumerates actionable controls through `actions`, it must not act.

@history_item n=21 marker=2026-11-14-seeded-randomness
**2026-11-14-seeded-randomness** — decision/checks-silent's exception widened so a re-run child prints ids or digests its parent reads. The current named exceptions a new check must fit or be named in.

@history_item n=22 marker=2026-11-16-layout-explained
**2026-11-16-layout-explained** — decision/checks-silent and decision/headless-stand-no-sink exceptions gain the layout stand check's own recording subscriber. Precedent: a check that installs any subscriber needs the row's exception amended, else it reads as drift.

@history_item n=23 marker=2026-11-16-errors-with-the-action
**2026-11-16-errors-with-the-action** — decision/headless-stand-no-sink, decision/driver-library-installs-no-subscriber and decision/driver-emits-no-event reworded for escher-telemetry's scoped capture around an acting step (prints nothing); contract/telemetry-sink/rust-log and contract/escher-sink-env/rust-log scoped to the sink layer alone. Keyboard and contrast checks driving acting steps run under that capture; it is not a sink.

@history_item n=24 marker=2026-11-16-error-text-in-the-answer
**2026-11-16-error-text-in-the-answer** — decision/headless-stand-no-sink and decision/checks-silent exceptions gain the error-text founder check; decision/answers-echo-nothing-typed gains the caused-item exception; decision/driver-library-installs-no-subscriber adds the chaining panic hook; decision/driver-emits-no-event says the capture keeps records and panics in process. The current text of five picked rows.

@history_item n=25 marker=2026-11-16-error-field-in-the-answer
**2026-11-16-error-field-in-the-answer** — decision/answers-echo-nothing-typed's exception: a field of the answer's own hidden set reads [redacted], an `error` field shown as written, cut at the text bound (ratified by 2026-11-17-a-component-in-isolation). The present wording of the picked row.

@history_item n=26 marker=2026-11-17-layout-on-the-carriers
**2026-11-17-layout-on-the-carriers** (3. Observability Harness Contract) — decision/sink-host-read-at-trace's needle set gains layout-answer keys (guard `host_log`, `mcp_log`). If the chunk drives served pages through a sink-installing host, these trace reads must stay green.

@history_item n=27 marker=2026-10-07-sink-target-allowlist
**2026-10-07-sink-target-allowlist** — outside targets dropped whole at every level; rule for later chunks: a target is admitted only together with a scrub rule for it (row decision/sink-target-allowlist, guard `telemetry_drop`).

@history_item n=28 marker=2026-10-07-driver-session
**2026-10-07-driver-session** — a second sink installer (`escher-session`); the scrub's reach scoped to its measurement (row decision/scrub-sink-reach). Trap: a zero read from a host that installs no sink says nothing about one that does.

@history_item n=29 marker=2026-10-06-telemetry-bootstrap
**2026-10-06-telemetry-bootstrap** — escher-telemetry's sink: `EnvFilter` from `RUST_LOG`, default `warn`, non-ANSI, stderr only (rows contract/telemetry-sink/stream, contract/telemetry-sink/rust-log); the allowlist scrub's origin.

@history_item n=30 marker=2026-10-07-command-and-refusal-schema
**2026-10-07-command-and-refusal-schema** — `Command`, `Call`, `ArgValue` derive `Debug` and print ids and text; rule: never field them with `Debug` nor an argument by its schema name; the cause field takes a `Cause` name (rows decision/debug-types-never-logged, decision/command-span-no-call-content).

@history_item n=31 marker=2026-10-07-driver-command-spans
**2026-10-07-driver-command-spans** (the driver's command span as built) — the `command` span's eight fields and its never-fielded list; `stand_act_spans`' re-run children the census's exception (rows contract/command-span/line, decision/driver-spans-carry-no-content, decision/command-span-no-call-content). Scope boundary: no span or log field is added.

@history_item n=32 marker=2026-10-10-mcp-surface
**2026-10-10-mcp-surface** — the `tool_call` span (`tool`, `ending`), ratified; `stand_mcp_tools`' in-process recorder an exception to «installs no subscriber» (rows contract/tool-call-span/line, decision/headless-stand-no-sink).

@history_item n=33 marker=2026-10-10-driver-cli
**2026-10-10-driver-cli** (the session host writes the command span; the command line writes answers, not logs) — each command's answer is one JSON line on stdout, never a log line; standing rule: the next surface adds no span or log field holding what a call supplied or a screen reads (rows contract/escher-session-stdout/command-answer, decision/driver-library-installs-no-subscriber).

@history_item n=34 marker=2026-10-20-large-screen-in-parts
**2026-10-20-large-screen-in-parts** — a part read records `verb` alone; a part's number, count and text never fielded or logged (row decision/part-answer-never-logged). Applies if the re-read pages a large served screen (Cells).

@history_item n=35 marker=2026-10-19-test-splits
**2026-10-19-test-splits** — seven_guis' shared log-check module states `Needles`, `found`, `occurrences`; its assertion messages a kind and a count, never a needle or a captured line (row decision/log-check-failures-name-kind-count).

@history_item n=36 marker=2026-10-05-fork-ci-reached
**2026-10-05-fork-ci-reached** (per-leg CI failure logs) — each `ci-leg.sh` leg's merged output written to `target/ci-logs/{leg}.log`, uploaded only on failure as `ci-log-{job id}`, kept 7 days (row decision/ci-log-artifact-on-failure). This chunk edits the `a11y` leg's `--test` list in `.github/scripts/ci-leg.sh`; the leg's log and upload path must stay as stated.

@history_item n=37 marker=2026-11-12T03-36-23-rows
**2026-11-12T03-36-23-rows** — the master's text became rows, each verified against the code; later amendments name rows by handle. The picked rows stand in this form, and an obs change this chunk owes is a row amendment (`~`/`+`/`−`), not prose.
