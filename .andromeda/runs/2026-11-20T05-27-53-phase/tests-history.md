@history_item n=1 marker=amd-yptdayxo
**amd-yptdayxo** (2026-10-26-codebase-health, the dev-host complexity ceiling) — recorded the audit's cognitive-complexity ceiling of 15, read on the dev host alone with rust-code-analysis-cli 0.0.25, test functions counted, no CI leg running it, engine functions over it left as they are (`decision/complexity-ceiling-host-only`). Trap this chunk rests on: `cognitive.sum` adds nested spaces into their parent, so a closure or nested `fn` keeps its branches counted in the enclosing function; only splitting into module-level sibling items lowers a reading. The prior codebase-health chunk's form for `read`, `run` and the five test functions.

@history_item n=2 marker=amd-4mesoims
**amd-4mesoims** (2026-10-26-codebase-health, the two table tests' private helpers) — precedent for bringing test functions under the ceiling: private `#[track_caller]` helpers (`assert_0_2_0_row`, `assert_described`, `assert_schema`, `assert_property`), every assertion and message kept, each helper's message naming the row and index only, no public symbol changed, controls red then green. The shape to follow for `visit`, the two `stand_graph` tests, `stand_actions`' and `stand_accessibility_ids`' over-ceiling tests.

@history_item n=3 marker=amd-nakcvsae
**amd-nakcvsae** (2026-10-26-codebase-health, line spans into the three edited files) — the prior health chunk's helpers moved cited line ranges, re-pointed by the wrap's citation sweep; a `lint.rs:` pattern also matches `stand_lint.rs`. Expect the same re-pointing for every file this chunk splits or extracts in.

@history_item n=4 marker=amd-ubmaiuyk
**amd-ubmaiuyk** (2026-10-26-codebase-health, the coverage leg read at this chunk) — the prior health chunk read `ci-leg.sh coverage` on the dev host with the floor verdict `3 of 3`, escher-driver's reading moving with its added helpers under unchanged floors (`decision/per-crate-coverage-floors`, `decision/coverage-leg-exits-with-floor`). Precedent for reading the floors after a refactor.

@history_item n=5 marker=amd-h57xnyqw
**amd-h57xnyqw** (2026-10-26-codebase-health, the per-OS tallies) — the prior health chunk added no test and its CI tallies read equal to the base run's; the expectation for a no-behaviour-change chunk's tallies.

@history_item n=6 marker=amd-6adlxvzj
**amd-6adlxvzj** (2026-10-26-codebase-health, the 0.2.0 sweep test's reach) — the sweep test reads `ci-leg.sh` whole by `contains`, so stand-repeat membership is held jointly with `test_stand_repeat_leg_runs_three_rounds`; bears on the test-file splits this chunk leaves for a later entry (target layout, stand-repeat named targets).

@history_item n=7 marker=amd-nsqxmuql
**amd-nsqxmuql** (2026-11-13-cognitive-load-corrective) — `~ decision/complexity-ceiling-host-only` gained its except: the engine's click handler held below the ceiling, upstream's arms included, at the price of a hand re-apply on a sync. The one engine exception; this chunk's scope leaves every upstream-authored function as it is.

@history_item n=8 marker=amd-htmd7cg7
**amd-htmd7cg7** (2026-10-31-select-popup, the complexity reading) — the ceiling's population count (85) and `clear_interaction_state_for_removed_node`'s 17.0 dated to before an edit, no later reading claimed; a stale-reading precedent for this chunk's re-measured 101 over the ceiling.

@history_item n=9 marker=amd-dvdqody5
**amd-dvdqody5** (2026-10-16-file-splits) — the file-split precedent: unit tests moved out whole into `#[path]` sibling files declared `#[cfg(test)]` at their parent (`schema_tests.rs`, `mcp_tests.rs`, `document::changed_set_tests`, …), test paths unchanged, counts followed by file (`decision/unit-tests-in-source-file`, `decision/inline-cfg-test-modules`). The shape for `json_tests.rs` → one child per `json/{answer}.rs` and for `snapshot.rs` / `command.rs`.

@history_item n=10 marker=amd-h4ip3qlm
**amd-h4ip3qlm** (2026-10-19-test-splits) — `cli_tests.rs` and `recording_tests.rs` moved out of line, test paths unchanged. Trap: a `*_tests.rs` `#[path]` file is no record of the per-crate coverage reading, so moving a test module out of line moved escher-driver's reading (98.33 % → 98.07 %); a later test split should forecast a reading change, not none.

@history_item n=11 marker=amd-lfmouigx
**amd-lfmouigx** (2026-11-14-start-in-a-state) — `~ decision/inline-cfg-test-modules` except dropped its count of out-of-line `cfg(test)` modules, so a new `#[path]` module (this chunk's split children) needs no row edit for a count.

@history_item n=12 marker=amd-ur7tfp4h
**amd-ur7tfp4h** (2026-10-07-audit-corrections) — introduced the bridge tests as a `#[path]` child of `dioxus_document` (fields private, no constructor) and the stand checks' shared module (`decision/bridge-tests-child-module`, `decision/shared-stand-tables-in-one-module`). Trap: a sed or grep address ending at a function's name also selects every item whose name opens with it.

@history_item n=13 marker=amd-xep3xm3z
**amd-xep3xm3z** (2026-11-07-ui-graph, dioxus-native-dom's one test-only shim) — `decision/bridge-tests-without-widening`: no function under test gains a constructor or `pub`, the one `cfg(test)` shim `DioxusDocument::dispatch_to_vdom`. A `snapshot.rs` split must widen nothing for its tests.

@history_item n=14 marker=amd-lrjd5uwd
**amd-lrjd5uwd** (2026-10-06-compact-snapshot-serialization) — `snapshot_text.rs` and `SNAPSHOT_TEXT_BUDGET` (`decision/snapshot-text-budget`). Trap: a search for `snapshot.rs:` also matches `stand_snapshot.rs:`; anchor citation searches on the path when splitting `snapshot.rs`.

@history_item n=15 marker=amd-pgaqvksr
**amd-pgaqvksr** (2026-10-20-large-screen-in-parts) — `Snapshot::text_parts` unit tests, `ArgKind::Part` and `MAX_PART` 1000 (`contract/driver-argument-kinds/part`, `decision/snapshot-text-budget`, `contract/cli-snapshot/part`); behaviour a snapshot or command split must keep unchanged.

@history_item n=16 marker=amd-myqerkgx
**amd-myqerkgx** (2026-10-06-snapshot-state-fidelity) — the snapshot unit tests' `build` helper and its no-op HTML parser, the `disabled` pin written through `DocumentMutator`; the password mask (`decision/typed-password-never-leaves`). Trap: a headless key press is not the same on every platform.

@history_item n=17 marker=amd-x5hh3g7d
**amd-x5hh3g7d** (2026-11-08-what-can-i-do-here) — `decision/one-actionable-rule`: both actionable readers read one rule from current listeners, never the `data-dioxus-id` mark, guarded by `a_listener_removed_on_a_re_render_is_read_by_neither_reader`; the snapshot's `clickable` reads the same rule.

@history_item n=18 marker=amd-a2uwkadt
**amd-a2uwkadt** (2026-11-07-ui-graph, dioxus-native-dom's graph unit tests) — `an_element_with_an_app_click_listener_reads_clickable` pins `contract/snapshot-node/clickable`, and the reserved `{owner}///{key}` form's tests (`contract/element-id/virtual-form`); unit tests in and beside `snapshot.rs` that a split must carry with unchanged paths.

@history_item n=19 marker=amd-afxaahjb
**amd-afxaahjb** (2026-11-07-ui-graph, the driver's verb-table pins) — json.rs pins the graph's `nodes`/`states` and `clickable`; `cli_tests.rs` reads a line by its rules in 101 rows; schema pins of verbs, field kinds and node keys. Bears on `json_tests.rs`, `command.rs` and `cli.rs`'s `read`.

@history_item n=20 marker=amd-dkb4oe4y
**amd-dkb4oe4y** (2026-11-08-what-can-i-do-here, the verb table at fourteen) — `VERBS` 14, describe's count row, `cli_tests` rows 102, and the complexity ceiling's record naming the renamed `the_listing_is_the_fourteen_verbs_of_the_table` with its reading not re-read; a ceiling record follows a rename.

@history_item n=21 marker=amd-eiwetouu
**amd-eiwetouu** (2026-11-08-handler-coverage, escher-driver's unit tests) — escher-driver 104 unit tests, 37 in four `#[path]` files, json.rs 8 (the coverage line, `contract/coverage-line/shape`, `contract/coverage-line/owner`), `cli_tests.rs` 6, the command line's rule table 109 rows; the counts and pins a `json_tests.rs` split and a `read` extraction must leave whole.

@history_item n=22 marker=amd-7hxia3by
**amd-7hxia3by** (2026-11-08-handler-coverage, the session binary's coverage role) — `coverage <app> <check>...`, its bounded read and endings 0 · 1 · 2 · 3 (`contract/escher-session-command-line/coverage`, `contract/escher-session-commands/coverage`); ratified by amd-ij32pdcj. A path `cli.rs`'s `read` handles.

@history_item n=23 marker=amd-nhvwrpl5
**amd-nhvwrpl5** (2026-11-08-handler-coverage, cli_commands and the coverage role) — the usage line and the usage-error rows (`coverage nowhere <dir>`, a misplaced `--record`), status `2` (`contract/escher-session-exit-codes/2`, `contract/cli-exit-codes/2`); `cli_commands` guards what `read` must keep.

@history_item n=24 marker=amd-smsbvscr
**amd-smsbvscr** (2026-11-17-layout-on-the-carriers) — `~ contract/escher-session-command-line/command-form` (fifteen verbs), `~ …/usage-line`, `~ contract/driver-session/reading-verbs`; the command line `cli.rs`'s `read` parses and its usage text, held unchanged.

@history_item n=25 marker=amd-h4mn5gu4
**amd-h4mn5gu4** (2026-11-17-layout-on-the-carriers) — `~ contract/driver-verbs/verb-table`: fifteen verbs at three levels; the table `command.rs` validates against (`contract/driver-verbs/level-check`, `contract/driver-verbs/lookup`).

@history_item n=26 marker=amd-qfxe5qxs
**amd-qfxe5qxs** (2026-11-17-layout-on-the-carriers) — `~ contract/mcp-tools-list/order`: fifteen tools, `help` last; served by `mcp.rs`, whose `run` this chunk brings under the ceiling.

@history_item n=27 marker=amd-cdohja6t
**amd-cdohja6t** (2026-11-17-layout-on-the-carriers) — `~ contract/session-layout/changes-nothing`, `+ contract/session-run/layout-answer`; the layout answer's JSON form, held by `json_tests`.

@history_item n=28 marker=amd-u7jdserc
**amd-u7jdserc** (2026-10-10-mcp-surface) — the `mcp` role writes one `tool_call` line per call, parent of `command` (`contract/stand-log/tool-call-span`, `contract/escher-session-stderr/tool-call-line`, `decision/mcp-capture-counts-by-span`). Trap: `tracing`'s callsite interest is process-global — a `with_default` check needs its own recorder per test. An extraction from `mcp.rs`'s `run` must keep every span and log site (scope: no new log site or span).

@history_item n=29 marker=amd-tvsd3b6b
**amd-tvsd3b6b** (2026-11-07-ui-graph, a graph call logs its verb alone) — `contract/command-span/graph`, `contract/command-span/verb`; `host_log`/`mcp_log` count command-span lines per call and read 0 needles at `trace` — the guards that a `run` refactor leaves the spans unchanged.

@history_item n=30 marker=amd-s53lzeiq
**amd-s53lzeiq** (2026-10-07-driver-command-spans) — a check reading a sink's capture counts lines by target, never in total; a green per-package run is not the workspace leg's green (`decision/sink-capture-counted-by-target`, `decision/per-package-green-not-workspace-green`). Gate both builds after the splits.

@history_item n=31 marker=amd-n3bdcrdl
**amd-n3bdcrdl** (2026-11-16-error-text-in-the-answer) — `~ decision/socket-carries-commands` (caused with its error text), `+ contract/driver-session/payload-unread`; answer-carrier content the JSON writer emits and `json_tests` pins.

@history_item n=32 marker=amd-ewa3qy3e
**amd-ewa3qy3e** (2026-11-17-layout-on-the-carriers) — `~ decision/socket-carries-commands`: layout answers among what crosses out; the newest form of that row.

@history_item n=33 marker=amd-de7olhxu
**amd-de7olhxu** (2026-11-17-stand-requirement-sweep) — `~ decision/typed-password-never-returned`: masks held on graph, actions and layout answers and kept checks; with amd-iriujbey guard `stand_masks_on_new_answers`. The masked-value writing (`contract/driver-json/masked-value`) a JSON split must keep.

@history_item n=34 marker=amd-4oiy4oeg
**amd-4oiy4oeg** (2026-11-17-stand-requirement-sweep) — `+ decision/requirement-witness-inside-stand-repeat`: witnesses pinned by function name, so a renamed witness reads red — test functions this chunk refactors (`stand_graph`, `stand_actions`, `stand_accessibility_ids`) must keep their names.

@history_item n=35 marker=amd-yfp5ovdp
**amd-yfp5ovdp** (2026-11-17-layout-on-the-carriers) — `+ decision/count-and-rename-sweeps`: a missed numeric pin and a missed renamed witness set the sweep convention for count changes and renames.

@history_item n=36 marker=amd-mmdtifvl
**amd-mmdtifvl** (2026-11-18-served-error-page) — `~ decision/count-and-rename-sweeps` except: a test name holding a count word pinned as a witness string is kept, the name searched before any rename; `~ decision/cross-os-readings-by-answer-digest` to seven served tasks.

@history_item n=37 marker=amd-poiocitr
**amd-poiocitr** (2026-11-07-ui-graph, the graph verb held in session) — what `stand_graph`'s tests assert, among them `every_graph_node_is_a_snapshot_node_and_no_served_id_holds_the_reserved_form` and the declarations-agree test (`decision/graph-answer-holds-no-value`, `decision/reserved-graph-id-form`, `contract/session-run/graph`); the assertions an extraction must keep whole.

@history_item n=38 marker=amd-z23dex63
**amd-z23dex63** (2026-11-07-ui-graph, the shared modules gain the graph's drives) — `GRAPH_DRIVES` in `tests/common/mod.rs`, `graph()`/`graphed()` in `session_common` (`decision/graph-drives-confirm-every-write`); the shared helpers `stand_graph`'s refactored tests read.

@history_item n=39 marker=amd-ckioxq2y
**amd-ckioxq2y** (2026-11-07-ui-graph, the unconfirmed flag's stated limit) — a conditional write a run skipped reads `unconfirmed`, and `GRAPH_DRIVES` drives each once (`decision/unrun-declared-write-reads-unconfirmed`); what `every_stand_tasks_declarations_agree_with_its_code_once_its_handlers_have_run` relies on.

@history_item n=40 marker=amd-jiyzcs5d
**amd-jiyzcs5d** (2026-11-07-ui-graph, the stand's state reaches the graph) — `decision/graph-signal-registration`: task signals registered through `use_graph_signal`, guarded by `graph::tests` and `stand_graph`.

@history_item n=41 marker=amd-n3udqhi7
**amd-n3udqhi7** (2026-11-08-what-can-i-do-here, stand_actions) — what `stand_actions`' 8 tests assert, among them `each_served_screen_lists_its_actionable_nodes_with_the_tables_verbs` (`contract/driver-verbs/actions`, `decision/actions-answer-holds-no-value`); assertions the extraction keeps.

@history_item n=42 marker=amd-jjah2vzy
**amd-jjah2vzy** (2026-11-08-what-can-i-do-here, the measured actionable counts) — `ACTIONABLE_AT_BOOT` in the stand checks' shared module and the Cells answer size; the table `stand_actions`' listing test reads.

@history_item n=43 marker=amd-54i6rjfu
**amd-54i6rjfu** (2026-10-14-stand-a11y-assertions) — `stand_accessibility_ids` gained interactive-role naming over nine roles (the test `every_interactive_role_element_reads_a_name`) and the a11y leg grew; that file runs in the `a11y` leg, so its refactor is read by that leg too.

@history_item n=44 marker=amd-2ho3wh7t
**amd-2ho3wh7t** (2026-11-04-dioxus-runtime-probe, listeners and readers) — the vdom walk's dynamic attributes read current listeners exactly, pinned by `link_2_a_removed_listeners_attribute_stays` (`decision/listeners-read-from-vdom-walk`); the walk `visit` in `dioxus_runtime_probe.rs` serves.

@history_item n=45 marker=amd-kmiudnmn
**amd-kmiudnmn** (2026-11-07-mounteddata-buffered-render, the probe's two pins) — the probe's `…_spawned_acts_*` pins through `#[track_caller]` `spawned_focus_acts` are the buffered render's discriminating witness on Dioxus 0.7, upstream's test kept as written (`decision/buffered-render-on-dioxus-07-bridge`, `decision/upstream-buffered-render-test-kept-as-written`); a `visit` refactor must keep them discriminating.

@history_item n=46 marker=amd-hqsdue54
**amd-hqsdue54** (2026-11-04-dioxus-runtime-probe, its legs) — `dioxus_runtime_probe` runs in the workspace `test` leg and `run all`, none of stand-repeat's named words nor the a11y leg; the legs that read its refactor.

@history_item n=47 marker=amd-ke4vohyd
**amd-ke4vohyd** (2026-11-20-stand-test-setup-shared) — `~ decision/shared-stand-tables-in-one-module` except: a helper whose body or failure message differs stays in its check. The previous chunk's move of the stand checks' setup into `session_common` and `common`; this chunk's test extractions follow the same rule.

@history_item n=48 marker=amd-u7vpyh4t
**amd-u7vpyh4t** (2026-10-10-upstream-sync-agent-surfaces) — `BlitzTestsTargetsTest` pins one auto-discovered target per file with upstream's `all` at `test = false` (`decision/one-test-target-per-file`, `decision/blitz-tests-single-test-target`). Why the seven oversize test files are left: a split there touches the target layout.

@history_item n=49 marker=amd-cixe5kae
**amd-cixe5kae** (2026-10-24-stand-requirement-sweep) — `stand-repeat` runs the `stand_*.rs` glob plus named `--test` words pinned by `test_stand_repeat_leg_runs_three_rounds` (`decision/stand-repeat-three-rounds`, `decision/zero-failure-stand-repeat`); the reason a split of a named target is a CI-pin change.

@history_item n=50 marker=amd-kube42bu
**amd-kube42bu** (2026-10-31-select-popup) — stand-repeat named words seventeen, `select_popup` among them (one of the files this chunk leaves), and the sweep test's reach over them.

@history_item n=51 marker=amd-n5xqwtgz
**amd-n5xqwtgz** (2026-10-14-quality-gates) — the per-crate floors escher-driver 97 % · dioxus-native-dom 83 % · blitz-test-harness 90 % and the zero-failure stand budget (`decision/per-crate-coverage-floors`, `decision/stand-checks-zero-failure-budget`, `decision/coverage-floor-per-crate`); the floors this chunk's splits move readings against.

@history_item n=52 marker=amd-xpqvmmzd
**amd-xpqvmmzd** (2026-10-17-upstream-sync-engine-debts) — `keyboard_compat.rs` excluded from coverage by filename (`decision/keyboard-compat-excluded-from-coverage`, `decision/keyboard-compat-coverage-exclusion`); a dioxus-native-dom coverage reading after the `snapshot.rs` split stands under it.

@history_item n=53 marker=amd-le75xddb
**amd-le75xddb** (2026-10-16-fast-failing-mutants) — the mutation tier's recipe: cargo-mutants under the nextest `mutants` profile, baseline skipped, dev host only; an audit re-reading this chunk's moved code must run under it.

@history_item n=54 marker=amd-knbkpfwu
**amd-knbkpfwu** (2026-11-14-auto-exploration) — `~ decision/mutation-controlled-tests`: a new check is controlled by a named mutation red then green; the controls this chunk's refactors use to show assertions kept.

@history_item n=55 marker=amd-jdiryrjq
**amd-jdiryrjq** (2026-10-07-command-and-refusal-schema) — `command.rs`'s tests pin `validate` (first-failure order, table and `Command` agreeing), every table stated with its row count asserted. Trap: the crate's count is stated in two word orders («13 unit tests», «unit tests 13») — find both when a split moves a count.

@history_item n=56 marker=amd-ieh35jjn
**amd-ieh35jjn** (2026-10-10-driver-cli) — json.rs, cli.rs and client.rs's unit tests introduced; `command.rs` checks a verb at its own level only (`contract/driver-verbs/level-check`, `contract/driver-cli/refused-line`); the origin of the modules this chunk splits.

@history_item n=57 marker=amd-pzdy3myb
**amd-pzdy3myb** (2026-10-10-scrolling-box-bounds-and-hit) — each of the bounds reader's two bodies (writing-mode on in the workspace build only) holds a check of its own (`decision/bounds-reader-bodies-each-checked`); snapshot bounds differ by runner, so gate the snapshot split under both builds.

@history_item n=58 marker=amd-gkbc5lsj
**amd-gkbc5lsj** (2026-11-13-upstream-sync-determinism-and-exploration) — `~ decision/layout-tests-in-both-layout-modes` except for upstream targets taken in a sync; the both-modes rule the refactored stand tests keep (`decision/stand-checks-in-both-layout-modes`).

@history_item n=59 marker=amd-d4acxs6q
**amd-d4acxs6q** (2026-10-24-a11y-re-read) — the a11y leg's fourteen files and the ANSI strip for per-job tallies (`decision/ci-log-ansi-strip`); the leg reading `stand_accessibility_ids`.

@history_item n=60 marker=amd-jy34nxjf
**amd-jy34nxjf** (2026-11-07-mounteddata-buffered-render, attributing a tally move) — a tally move in a job log is attributed by grepping the test's own name, never by the preceding `Running` line (`decision/tally-move-attributed-by-test-name`).

@history_item n=61 marker=amd-yx45xaju
**amd-yx45xaju** (2026-11-01-upstream-sync-ui-graph) — `fast` ends on the `embolden` step (`decision/local-pre-push-gate`), the local gate this chunk's gate block runs.

@history_item n=62 marker=amd-3j6rcoyw
**amd-3j6rcoyw** (2026-11-19-build-directory-held) — `~ decision/local-pre-push-gate`: a green fast leg on the dev host ends with the target sweep, never reddening it.

@history_item n=63 marker=amd-h47hxvhq
**amd-h47hxvhq** (2026-11-16-error-field-in-the-answer) — `+ decision/doc-leg-gates-public-doc-edits`: a chunk editing a public doc comment lists the doc leg in its gate block — moving items between files (`json/*.rs`, `snapshot.rs`, `command.rs`) can break intra-doc links read only in CI's Documentation job.

@history_item n=64 marker=amd-lde2vbr3
**amd-lde2vbr3** (2026-11-13-local-files-for-local-documents) — `+ decision/blitz-dom-requests-through-stamping-constructor`: the clippy leg enforces blitz-dom's request construction; the upstream-authored file class this chunk leaves unedited.

@history_item n=65 marker=amd-b4vvhst7
**amd-b4vvhst7** (2026-11-17-a-component-in-isolation) — `+ decision/harness-graph-tests-enable-accessibility`: a per-package build leaves `accessibility` off, gating the UI graph; snapshot and graph unit tests compile only under that feature, which a split's `cfg` must keep.

@history_item n=66 marker=amd-755msr7u
**amd-755msr7u** (2026-11-18-capability-statement) — `~ decision/capability-statement-from-schema`: the capability statement's counts read from the schema's tables, pinned by `CapabilityStatementTest`; a `command.rs` split leaves the schema's tables unmoved.

@history_item n=67 marker=amd-mnnwqipq
**amd-mnnwqipq** (2026-11-15-session-binary-smoke) — `~ overview/session-lifecycle`: the session checks and the `run session` selection running `cli_flow`; the session surface whose binary `cli.rs` and `mcp.rs` serve.
