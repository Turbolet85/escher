@history_item n=1 marker=amd-jdiryrjq
**amd-jdiryrjq** — (2026-10-07-command-and-refusal-schema) gave `packages/escher-driver/src/command.rs` its first unit tests (validate over admitted calls, unknown verbs, one malformed witness per rule, the first-failure order, table and `Command` agreeing), beside refusal.rs and schema.rs, each table's row count asserted. These are the unit pins that must stay green and keep their paths when command.rs is split. Trap: the crate's count is written in two word orders ("13 unit tests", "unit tests 13"), so a count sweep has to search both.

@history_item n=2 marker=amd-itmrnscn
**amd-itmrnscn** — (2026-10-10-driver-cli) command.rs reached 7 tests within escher-driver's per-file census. The per-row figures of command.rs's tests were taken out of the body because they went unmeasured. Splitting command.rs moves this per-file census, so it has to be re-measured and must not be assumed.

@history_item n=3 marker=amd-ieh35jjn
**amd-ieh35jjn** — (2026-10-10-driver-cli) §4 records that command.rs pins "a verb is checked at its own level only". It also records the schema driven over the socket (`stand_session_lifecycle`), through the binary (seven_guis targets) and in process (`stand_act_*`). Together these are the existing checks that prove command.rs's moved bodies behave the same. The level check is row contract/driver-verbs/level-check.

@history_item n=4 marker=amd-dvdqody5
**amd-dvdqody5** — (2026-10-16-file-splits) Precedent for a behaviour-preserving split. schema.rs's and mcp.rs's tests moved whole into `#[path]` sibling files (`schema_tests.rs`, `mcp_tests.rs`), each declared `#[cfg(test)]` at its parent with test paths unchanged, and the per-file counts followed the moved tests. This is row decision/test-module-split-keeps-paths, the form a command.rs split should follow.

@history_item n=5 marker=amd-h4ip3qlm
**amd-h4ip3qlm** — (2026-10-19-test-splits) cli.rs's and recording.rs's tests moved out of line into `*_tests.rs` files. A `*_tests.rs` file is no record of the per-crate coverage reading, so moving a test module out of line changed escher-driver's coverage (98.33 → 98.07 %). Its rule: "a later test split should forecast a reading change, not none". The command.rs split must forecast the effect on escher-driver's 97 % floor (row decision/per-crate-coverage-floors).

@history_item n=6 marker=amd-krpdwno6
**amd-krpdwno6** — (2026-11-20-codebase-health) ~ decision/inline-cfg-test-modules: the exception now allows unit tests out of line in a `_tests` sibling declared `cfg(test)` by path, with fixture builders in a child module. This is the allowed form for any of command.rs's tests the split moves.

@history_item n=7 marker=amd-6oxgyfcu
**amd-6oxgyfcu** — (2026-11-20-codebase-health) ~ decision/inline-unit-test-modules, the twin of the row above, given the same `_tests` sibling and fixtures-child exception. Both twin rows bind how a split places its tests.

@history_item n=8 marker=amd-lfmouigx
**amd-lfmouigx** — (2026-11-14-start-in-a-state) ~ decision/inline-cfg-test-modules: the stated count of `#[path]` test modules was dropped from the exception because a new driver module made it false. A split that adds modules does not have to re-count that row.

@history_item n=9 marker=amd-ur7tfp4h
**amd-ur7tfp4h** — (2026-10-07-audit-corrections) The bridge tests sit as a `cfg(test)` child of `dioxus_document` because the handler's fields are private (row decision/unit-tests-in-source-file except). Trap for this chunk's moves: a sed or grep address that ends at a function's name also selects every item whose name starts with it, and a control built on the same address cannot catch that.

@history_item n=10 marker=amd-yptdayxo
**amd-yptdayxo** — (2026-10-26-codebase-health) Set the dev-host cognitive-complexity ceiling of 15, read with rust-code-analysis-cli, test functions counted. Its key analyser fact: `cognitive.sum` adds nested closures and nested `fn`s into their parent, and only splitting into module-level sibling items lowers a reading. This decides whether escher's additions, moved behind one call, give back upstream's 19 · 46 · 167, so P3 has to check it.

@history_item n=11 marker=amd-4mesoims
**amd-4mesoims** — (2026-10-26-codebase-health) Precedent: two functions were brought under the ceiling with private `#[track_caller]` helpers alone. No public symbol changed, every assertion and message was kept, and the controls read red then green. This is the form of a behaviour-preserving extraction.

@history_item n=12 marker=amd-nsqxmuql
**amd-nsqxmuql** — (2026-11-13-cognitive-load-corrective) ~ decision/complexity-ceiling-host-only except: the engine's click handler is held below the ceiling with upstream's arms included, at the cost of a hand re-apply when an upstream sync edits one of its arms. This is the nearest precedent for restructuring an upstream engine function. This chunk instead leaves upstream's text untouched plus one call, so the sync cost should be weighed against it.

@history_item n=13 marker=amd-ihzt4hfv
**amd-ihzt4hfv** — (2026-11-20-codebase-health) ~ decision/complexity-ceiling-host-only why: upstream-authored functions over the ceiling are kept as upstream wrote them (founder F2). This is the ruling behind the boundary that leaves the other ten fork-grown functions as they are and moves only escher's additions.

@history_item n=14 marker=amd-htmd7cg7
**amd-htmd7cg7** — (2026-10-31-select-popup) `clear_interaction_state_for_removed_node`'s 17.0 was re-labelled "as read before" an edit, with no later reading claimed. Precedent: once an edit touches a function, its complexity reading is dated, so this chunk re-measures at its own HEAD.

@history_item n=15 marker=amd-6eogcl6u
**amd-6eogcl6u** — (2026-11-20-codebase-health) + decision/upstream-oversize-files-kept and + decision/oversize-test-targets-kept: upstream's files over 800 code lines are not split, and escher's integration targets over 800 are kept whole. command.rs is escher-authored source, so the split stays clear of both rows' kept sets.

@history_item n=16 marker=amd-rx3se5ar
**amd-rx3se5ar** — (2026-11-20-codebase-health) + decision/upstream-test-clone-pairs-kept: upstream's duplicated test pairs are kept, and the duplication watch is discharged. This bears on this chunk's re-measure of stand-check duplication (`stand_reading.py`, bar under 47 / 550).

@history_item n=17 marker=amd-3mgq245q
**amd-3mgq245q** — (2026-11-20-browser-s-surviving-mutants) + decision/mutation-filter-anchored-on-verb: a scoped cargo-mutants run anchors its filter on the replace verb and the function name, and its listing is read first. A bare function name matched other functions. This applies if moved bodies get a scoped mutation control.

@history_item n=18 marker=amd-le75xddb
**amd-le75xddb** — (2026-10-16-fast-failing-mutants) The mutation tier's recipe: cargo-mutants under nextest's `mutants` profile, `--baseline=skip` with each unit's command read green first, on the dev host only. This bears on rows decision/mutation-baseline-* and mutation-runs-*, which bind any mutation control of the moved bodies. Note that this chunk's cargo runs wait on the operator's quiet window.

@history_item n=19 marker=amd-n5xqwtgz
**amd-n5xqwtgz** — (2026-10-14-quality-gates) Set the per-crate coverage floors (escher-driver 97 %, dioxus-native-dom 83 %, blitz-test-harness 90 %) and the zero-failure stand-repeat budget. These are rows decision/per-crate-coverage-floors, coverage-leg-exits-with-floor and stand-checks-zero-failure-budget, the gates the command.rs split must still pass.

@history_item n=20 marker=amd-xpqvmmzd
**amd-xpqvmmzd** — (2026-10-17-upstream-sync-engine-debts) The coverage leg excludes dioxus-native-dom's `keyboard_compat.rs` by filename (row decision/keyboard-compat-coverage-exclusion). It is not blitz-dom's `events/keyboard.rs`, so the exclusion does not reach this chunk's keyboard function, and a file-name regex must not be widened by mistake.

@history_item n=21 marker=amd-vrpzudrd
**amd-vrpzudrd** — (2026-11-01-upstream-sync-ui-graph) The keyboard_compat exclusion's owner moved, and the take left `286874cc1` out, so the exclusion stands. This is the latest state of the exclusion row that this chunk carries untouched.

@history_item n=22 marker=amd-s53lzeiq
**amd-s53lzeiq** — (2026-10-07-driver-command-spans) Engine features by runner: `cargo test --workspace` builds blitz-dom with `tracing` compiled in, and a per-package build does not, so the same check read green per package and red in the workspace leg. This is row decision/per-package-green-not-workspace-green. Behaviour-unchanged proof for the blitz-dom bodies must be read under the workspace leg.

@history_item n=23 marker=amd-cqdq6a7d
**amd-cqdq6a7d** — (2026-10-10-upstream-sync-agent-surfaces) The workspace build resolves blitz-dom with 16 features and the per-package build with 10, `writing-mode` among the difference, and this selects which bodies compile. Feature-gated escher additions moved out of upstream bodies (e.g. accessibility-gated) must keep their cfg gates, and must be proved under both builds.

@history_item n=24 marker=amd-pzdy3myb
**amd-pzdy3myb** — (2026-10-10-scrolling-box-bounds-and-hit) The reader's two feature-selected bodies each got a check, with mutation controls red under its own build only. Trap: a change in one body alone reads green under the other build. This applies when escher's additions sit inside cfg-gated arms.

@history_item n=25 marker=amd-yx45xaju
**amd-yx45xaju** — (2026-11-01-upstream-sync-ui-graph) ~ decision/local-pre-push-gate: `fast` runs fmt → clippy → test → ci-scripts → embolden. The embolden step is the local witness of a macOS-only feature, after a red only `Test (macos)` showed (amd-23w5gd5l). This is the local gate this chunk's restructure runs.

@history_item n=26 marker=amd-3j6rcoyw
**amd-3j6rcoyw** — (2026-11-19-build-directory-held) ~ decision/local-pre-push-gate: a green fast leg on a dev host ends with the target sweep, which never reddens the leg. This is the current form of the pre-push gate row this chunk is held to.

@history_item n=27 marker=amd-h47hxvhq
**amd-h47hxvhq** — (2026-11-16-error-field-in-the-answer) + decision/doc-leg-gates-public-doc-edits: a chunk that edits a public doc comment lists the doc leg in its gate block. A public doc comment linking a crate-private item read green locally and red only in CI's Documentation job. Moving command.rs's items between modules can break intra-doc links.

@history_item n=28 marker=amd-lde2vbr3
**amd-lde2vbr3** — (2026-11-13-local-files-for-local-documents) + decision/blitz-dom-requests-through-stamping-constructor: blitz-dom carries a crate-level clippy disallowed-methods config with one expect, so the clippy leg fails on any other request construction. Code moved inside blitz-dom stays under that crate config.

@history_item n=29 marker=amd-nakcvsae
**amd-nakcvsae** — (2026-10-26-codebase-health) Precedent: citations into edited files were re-pointed after private helpers moved lines. Trap: a `lint.rs:` pattern also matches `stand_lint.rs`. Splitting command.rs moves every `command.rs:` citation, and a sweep has to anchor on the path.

@history_item n=30 marker=amd-lrjd5uwd
**amd-lrjd5uwd** — (2026-10-06-compact-snapshot-serialization) Trap: a citation search for `snapshot.rs:` also matches `stand_snapshot.rs:`. A citation search needs a path anchor, which also covers `command.rs:` and the three blitz-dom files.

@history_item n=31 marker=amd-q3tlwlgs
**amd-q3tlwlgs** — (2026-10-12-self-description) Added describe.rs, `help` via `validate_session`, the remedies and exit statuses pinned in error.rs and schema.rs, `cli_commands` asserting each cause's `remedy`, and `help` the same bytes on both surfaces. These are rows contract/cli-help, cli-refusal/remedy, mcp-help/parity and the checks that hold the command module's answers byte-stable.

@history_item n=32 marker=amd-ybvlv3ad
**amd-ybvlv3ad** — (2026-10-18-one-virtual-clock) Eight refusal causes in order (`the_eight_causes_are_named_in_order`, `fonts-unavailable` the eighth, seven served over MCP), and `advance` accepted on every session, moving the clock by exactly `ms`. These are rows decision/eight-refusal-causes-in-order, mcp-causes-served-with-remedy and advance-*, whose guards must stay green after the split.

@history_item n=33 marker=amd-oodkokb2
**amd-oodkokb2** — (2026-10-18-engine-debts) The founder ratified the one-virtual-clock marks (eight causes, `advanced_ms` exactly `ms`, the stand's time step). Those advance and cause rows stand ratified, and the restructure must not move them.

@history_item n=34 marker=amd-afxaahjb
**amd-afxaahjb** — (2026-11-07-ui-graph) The verb-table pins: schema_tests' verb, field-kind and node-key counts, describe's count row, `cli_tests.rs`'s row table and mcp_tests' listing. These are the numeric pins over the table command.rs reads, and they must read unchanged after the split.

@history_item n=35 marker=amd-dkb4oe4y
**amd-dkb4oe4y** — (2026-11-08-what-can-i-do-here) The verb table grew to fourteen, `actions` the tenth instance verb, with the per-verb field-kind table. `the_listing_is_the_fourteen_verbs_of_the_table` was renamed, and its complexity reading of 3 under the earlier name was not re-read.

@history_item n=36 marker=amd-h4mn5gu4
**amd-h4mn5gu4** — (2026-11-17-layout-on-the-carriers) ~ contract/driver-verbs/verb-table: fifteen verbs at three levels, each with help, arguments and a result field. This is the current verb-table row the split's schema and command pins hold.

@history_item n=37 marker=amd-abrsrea5
**amd-abrsrea5** — (2026-11-17-layout-on-the-carriers) ~ contract/mcp-server/tools-list: fifteen verbs in order, `layout` after `actions`, `help` last. It is paired with amd-qfxe5qxs (~ contract/mcp-tools-list/order). Both MCP listing rows are picked, and their guards stay green under the split.

@history_item n=38 marker=amd-bc5fqyab
**amd-bc5fqyab** — (2026-11-20-driver-debts) ~ contract/escher-session-command-line/graph, command-form and usage-line: `graph --part`, press's bare `--ctrl`, and the usage line. These are the most recent additions to the command module, the 72 lines from 2026-11-20-driver-debts that pushed command.rs from 776 to 848. The split's seams have to keep these command-line contracts as they read.

@history_item n=39 marker=amd-lfwo5vi7
**amd-lfwo5vi7** — (2026-11-20-driver-debts) ~ contract/graph-command/argument: graph takes an optional `part` from 1 to 1000, and anything else is refused `malformed` with nothing run. ~ decision/graph-answer-holds-no-value: the graph answer gains the part counts. Both rows are picked, and both cover command parsing that may move in the split.

@history_item n=40 marker=amd-wudwalw5
**amd-wudwalw5** — (2026-11-20-driver-debts) ~ contract/driver-session/graph: `Outcome::Graphed`, or `Outcome::GraphPart` with part and parts. These are public outcome arms, so the split must keep their `escher_driver::…` paths and visibility.

@history_item n=41 marker=amd-pgaqvksr
**amd-pgaqvksr** — (2026-10-20-large-screen-in-parts) `ArgKind::Part` 1 to `MAX_PART` = 1000, `stand_snapshot_parts`, and `cli_commands`' `--part` refusals. This is row decision/snapshot-text-budget with its guards, and the part-argument handling lives in the command and schema path the split moves.

@history_item n=42 marker=amd-tem72y7f
**amd-tem72y7f** — (2026-10-20-load-wait) `LOAD_WAIT` and the acting verbs waiting on a load in flight, with `stand_load_wait` and session.rs's four unit tests. These are rows decision/load-wait-bound and contract/driver-session/load-wait, and their guards stay green under the split.

@history_item n=43 marker=amd-jvq3mywt
**amd-jvq3mywt** — (2026-11-01-upstream-sync-ui-graph) A failed load ends the settle wait through `CountedHandler`'s forward of `NetHandler::error`. Trap stated: a trait's do-nothing default compiled silently through a wrapper, so later work that extends a trait sweeps its implementers. This is row decision/failed-load-ends-settle-wait.

@history_item n=44 marker=amd-6ah7rxit
**amd-6ah7rxit** — (2026-10-17-upstream-sync-engine-debts) The harness re-exports keyboard-types 0.8's `Key`, `Modifiers` and `NamedKey` (row decision/harness-reexports-key-types). This is the key surface keyboard checks drive into `handle_key_or_input_event`.

@history_item n=45 marker=amd-myqerkgx
**amd-myqerkgx** — (2026-10-06-snapshot-state-fidelity) Trap: the editor's Backspace arm is compiled out on macOS, so a deleting-key check read green on Linux and red on the macOS CI leg. `handle_key_or_input_event`'s escher additions may sit beside cfg'd platform arms, so moving them needs macOS's CI witness, not Linux alone.

@history_item n=46 marker=amd-lzvb3u3d
**amd-lzvb3u3d** — (2026-10-13-stand-keyboard-harness) `stand_keyboard`: Tab walk, Enter/Space as click, `disabled` and `inert` skipped, the Apple delete binding on every OS, and `stand_act_keys` press enter/space. These are existing checks over the keyboard handler's escher additions that the plan can name to prove the moved keyboard body unchanged.

@history_item n=47 marker=amd-jyrjp4tc
**amd-jyrjp4tc** — (2026-10-31-engine-debts) The engine's `change` after `input` on a checkbox and radio (a Space included), with no change on text and range keys. `checkable_change_event` and `text_range_no_change_event`, both named `stand-repeat` targets, witness escher's key and input additions.

@history_item n=48 marker=amd-efvgllgo
**amd-efvgllgo** — (2026-10-31-select-popup) Select keys: Enter/Space open the list, `select_interaction`'s `KEYS` table, `stand_keyboard` +2, and `stand_act_select`'s click then key. These are among the keyboard-handler witnesses a moved keyboard body must keep green. Related: amd-gtfexkqk (`select_popup` key rows) and amd-dlzb6vu6 (range input by key).

@history_item n=49 marker=amd-2ljhblkm
**amd-2ljhblkm** — (2026-11-01-upstream-sync-ui-graph) The key-press panic past a dropped anonymous block was retired (`key_press_after_container_lost_its_box`, 3 tests). Later amd-ffczzr5t (2026-11-21-engine-debts) retired decision/key-after-dropped-anonymous-block: the post-layout fixup gives focus up. This is the current reading of the key path a moved keyboard body has to keep.

@history_item n=50 marker=amd-2672diu7
**amd-2672diu7** — (2026-10-10-snapshot-states) `accessibility_states`: aria-selected, aria-pressed and aria-invalid read on supporting and unsupporting roles, the row counts asserted. The adapter-crossing gap is unmeasured. These are existing witnesses of escher's additions in `build_accessibility_node`.

@history_item n=51 marker=amd-3kuwk3el
**amd-3kuwk3el** — (2026-10-18-engine-debts) `accessibility_states` 5, `accessibility_merged_states`, `accessibility_active_descendant_unread` and `inert_subtree` were added. Later amd-nmeae3wm added a native control's `toggled` and amd-kkp6rzzv an inert node losing focus. Together these are the accessibility-node checks a moved `build_accessibility_node` addition must keep green. amd-f6jmv7bb's `stand_accessibility_ids` (`author_id` on every element) is another.

@history_item n=52 marker=amd-d4acxs6q
**amd-d4acxs6q** — (2026-10-24-a11y-re-read) The a11y leg names fourteen files: tier-0 witnesses and tree readings. Each joining file keeps its legs (row decision/joining-file-keeps-its-legs). The a11y leg is the CI leg that reads the accessibility checks over the moved body.

@history_item n=53 marker=amd-j45pb4zi
**amd-j45pb4zi** — (2026-11-18-a11y-re-read) ~ decision/a11y-leg-membership: the rule keeps its criterion (SC 2.1.1 or 2.4.3 witness, tree reading, recorded gap) and drops the list. This is the current form of the picked row.

@history_item n=54 marker=amd-e42t37s6
**amd-e42t37s6** — (2026-10-31-select-popup) A `multiple` select's `size` hints a height of `size × 1.2em` through `size_rows`, capped, read by `size_makes_a_multiple_select_that_many_rows_tall_…` and `size_reads_a_whole_number_above_zero_capped`. This is a likely escher addition in `compute_presentational_hints`, together with its existing witnesses.

@history_item n=55 marker=amd-stldjcwi
**amd-stldjcwi** — (2026-11-21-single-select-sized-as-a-list-box) ~ decision/mutation-controlled-tests: a control's case is widened until it reads red, because other arms routed around the guard. Siblings amd-5rdehljq (box presence read with computed display) and amd-lofuqks6 (moved fixture select controlled) come from the same sized-select work, the latest presentational-hint addition. This is how a plan proves a check would fail when a moved body moved.

@history_item n=56 marker=amd-hw6eolsp
**amd-hw6eolsp** — (2026-10-31-engine-debts) The `:open` gap closed with `open_state`, read from the attribute at construction and through mutator writes (row decision/open-pseudo-class-follows-attribute). The row is picked, and it is a guard to keep green if attribute-driven element code near the moved bodies shifts.

@history_item n=57 marker=amd-x5hh3g7d
**amd-x5hh3g7d** — (2026-11-08-what-can-i-do-here) One actionable rule for both readers, read from current listeners, never from the `data-dioxus-id` mark (row decision/one-actionable-rule). amd-2ho3wh7t gives the vdom-walk listener reading (row decision/listeners-read-from-vdom-walk). Both are picked rows whose guards stay green under the split.

@history_item n=58 marker=amd-b4vvhst7
**amd-b4vvhst7** — (2026-11-17-a-component-in-isolation) + decision/harness-graph-tests-enable-accessibility: a per-package test build leaves the `accessibility` feature off, so harness graph unit tests turn it on through the dev-dependency. This is the feature-gating trap for proving `accessibility`-gated bodies per package.

@history_item n=59 marker=amd-rlvpc7s2
**amd-rlvpc7s2** — (2026-11-16-error-text-in-the-answer) + decision/engine-items-asserted-on-tracing-build: engine caused items are asserted only on a tracing build, and the check runs on both builds. Behaviour-unchanged proof that reads engine warnings depends on the build.

@history_item n=60 marker=amd-2ftiicau
**amd-2ftiicau** — (2026-11-16-error-text-in-the-answer) ~ decision/root-boundary-err-on-own-session: caused carries the error text and chain. + decision/mcp-result-decoded-whole: an MCP comparison decodes the result text whole, after a Windows red. Both are picked rows, and their checks witness the command and answer path.

@history_item n=61 marker=amd-arfsbrh6
**amd-arfsbrh6** — (2026-11-18-served-error-page) + decision/caused-owner-pinned-as-graph-reads: a check pins the graph's owner reading, never an asserted owner. ~ contract/session-run/layout-answer: over seven served screens. Both are picked rows.

@history_item n=62 marker=amd-mmdtifvl
**amd-mmdtifvl** — (2026-11-18-served-error-page) ~ decision/cross-os-readings-by-answer-digest: seven served tasks read on every CI OS by answer digest against one committed table. Its guard `stand_cross_os_readings` is a byte-level witness that the command module's answers did not move. Also ~ decision/count-and-rename-sweeps: a count-worded test name pinned as a witness string is kept.

@history_item n=63 marker=amd-n2omhba3
**amd-n2omhba3** — (2026-11-21-engine-debts) ~ decision/count-and-rename-sweeps: a verb-count or CI-leg-file-count change sweeps the CI-script tests too, and a test rename sweeps the old name held as a string literal. This applies if the split renames or moves any witness test.

@history_item n=64 marker=amd-4oiy4oeg
**amd-4oiy4oeg** — (2026-11-17-stand-requirement-sweep) + decision/requirement-witness-inside-stand-repeat: each requirement names its witness by test function name in a pinned table, so a renamed witness reads red. A split that renames a test breaks it.

@history_item n=65 marker=amd-cixe5kae
**amd-cixe5kae** — (2026-10-24-stand-requirement-sweep) `stand-repeat` runs the `stand_*.rs` glob plus named `--test` targets, pinned by `test_stand_repeat_leg_runs_three_rounds`. Its current count is seventeen (amd-kube42bu). This is rows decision/stand-repeat-three-rounds and zero-failure-stand-repeat, the leg that witnesses the moved bodies three times with no retry.

@history_item n=66 marker=amd-472linzq
**amd-472linzq** — (2026-10-25-capability-statement) `test_capability_statement.py` pins `docs/capabilities.md`'s verb and cause code spans and its counts read from the schema's tables. This is row decision/capability-statement-pinned and its twins. The split moves no name, so these pins must stay green.

@history_item n=67 marker=amd-m7fw5fjq
**amd-m7fw5fjq** — (2026-11-18-capability-statement) ~ decision/capability-statement-pinned, ~ capabilities-doc-pinned-to-schema (amd-zn44tekk), ~ capability-check-names-held-to-schema (amd-aiakktz6) and ~ capability-statement-from-schema (amd-755msr7u): the pin also holds the a11y-leg file count and the stand-repeat named-target count to the CI leg script. These are the current forms of all four picked capability rows.

@history_item n=68 marker=amd-siqngma2
**amd-siqngma2** — (2026-10-19-recorded-check-replay) seven_guis `recording`'s `the_committed_checks_are_what_the_binary_records` reads both committed checks byte-equal to what the binary records. This is row decision/recorded-checks-kept-fresh, a byte-for-byte witness that the command path's answers did not move. amd-7qex4fkt gives replay through `Session::run` (rows recorded-checks-replayed-as-tests, replay-through-the-call-path).

@history_item n=69 marker=amd-i7b3rmrc
**amd-i7b3rmrc** — (2026-11-08-handler-coverage) `handler_coverage` and `Check::replay_each`, with `stand_handler_coverage`: a refused step exercises no handler, and a keyboard activation exercises its click. These are rows contract/handler-coverage/refused-step and handler-coverage-holds-no-typed-text.

@history_item n=70 marker=amd-de7olhxu
**amd-de7olhxu** — (2026-11-17-stand-requirement-sweep) ~ decision/typed-password-never-returned: masks are held on every answer carrier and on kept and explored checks, and amd-iriujbey adds the guard `stand_masks_on_new_answers`. This is the picked row whose guards witness the command path's masking.

@history_item n=71 marker=amd-447m7y2r
**amd-447m7y2r** — (2026-11-16-layout-explained) + contract/session-layout/changes-nothing and + decision/layout-reason-words-each-read, both picked rows. amd-cdohja6t adds the `layout` verb through the call path. The `layout` verb's command handling is among the bodies the split moves.

@history_item n=72 marker=amd-n3udqhi7
**amd-n3udqhi7** — (2026-11-08-what-can-i-do-here) `stand_actions`: a fetch is put in flight with the harness's `click`, which waits on nothing, against a driver `click` that waits out `LOAD_WAIT` (row decision/harness-click-waits-on-nothing). The actions answer holds no value (row decision/actions-answer-holds-no-value), and an argument to `actions` is refused on every carrier.
