@history_item n=1 marker=amd-lrjd5uwd
**amd-lrjd5uwd** — `SNAPSHOT_TEXT_BUDGET` (10,000 bytes) set when the snapshot's text form landed; the snapshot-text-budget row's origin. This chunk measures the canvas dump's size at both levels against that budget.

@history_item n=2 marker=amd-pgaqvksr
**amd-pgaqvksr** — Large screen in parts: the `part` argument (`ArgKind::Part`, 1 to `MAX_PART` = 1000), `Outcome::Part`, `--part` on the CLI, and the counts that sweep moved (unit tests, `cli_commands`, `stand_act_spans`, `host_log` and `mcp_log` part reads). This is the precedent for paging a dump that runs over the budget.

@history_item n=3 marker=amd-halqoyxj
**amd-halqoyxj** — The Cells graph answer was measured at 293,869 B against a 200 KB forecast, which proved wrong. «An unpaged graph» became a stated limit (residual `res-7cc5bi4k`), and `stand_graph` holds the answer within the answer bound. Trap for this chunk: measure the dump's size, do not forecast it.

@history_item n=4 marker=amd-jjah2vzy
**amd-jjah2vzy** — The Cells `actions` answer was measured at 106,088 bytes, and `ACTIONABLE_AT_BOOT` was recorded as measured, not assumed. Precedent for this chunk's size measured in bytes.

@history_item n=5 marker=amd-bc5fqyab
**amd-bc5fqyab** — Driver-debts: `graph --part` on the CLI, with the whole and part answer shapes, `--ctrl`, and the usage line. These are the picked command-form and usage-line rows a new verb or argument extends.

@history_item n=6 marker=amd-wudwalw5
**amd-wudwalw5** — `Outcome::GraphPart` and the graph answer's `part` and `parts` counts on the session wire. Precedent for an answer cut into parts at the snapshot budget.

@history_item n=7 marker=amd-lfwo5vi7
**amd-lfwo5vi7** — `graph-command/argument` (the picked row) gained `part`, and the graph answer's closed list of what it carries gained part counts. Bounds how a new argument and its answer contents are stated.

@history_item n=8 marker=amd-3q4iwwwi
**amd-3q4iwwwi** — The `lint` verb added (twelve verbs, eight instance-level): describe's count row, the MCP listing, `cli_commands`, `mcp_server`, `stand_mcp_tools`, `stand_act_spans`, `host_log`/`mcp_log` driving one call each, the cold-agent pipe's tool list, and the recording keeping instance verbs. This is the template sweep for adding a verb.

@history_item n=9 marker=amd-flbkdilp
**amd-flbkdilp** — The visual lint first shipped in process only (`Session::lint`), PROVISIONAL, and its carriage came later as a verb. Precedent for the founder's carrier fork that this chunk faces.

@history_item n=10 marker=amd-rrbdvino
**amd-rrbdvino** — The `screenshot` verb added. The in-band PNG on the wire was the founder's fork, and the harness gained `screenshot`/`screenshot_rect`, `ScreenshotError::FontsUnavailable` and `session_common` `screenshot()`/`shot()`. This chunk's canvas read sits beside `screenshot` in the harness.

@history_item n=11 marker=amd-afxaahjb
**amd-afxaahjb** — `graph` took the verb table to thirteen: schema_tests' verb, field-kind and node-key pins, `VERBS`, describe's count row, mcp_tests' listing, json.rs pins, and cli_tests' rows. These are the count sites a sixteenth verb moves.

@history_item n=12 marker=amd-7efxs2hi
**amd-7efxs2hi** — The session contract took `graph` as its ninth instance verb: a reading verb runs no step, `Outcome::Graphed`, the recording keeps every instance verb, plus session-binary and session-proof. A canvas read is another reading verb of this kind.

@history_item n=13 marker=amd-5zqg4udh
**amd-5zqg4udh** — A graph answer crossing the session socket was a boundary widening ratified by the founder (provisional delegate confirm). Precedent for this chunk's carrier fork on what a canvas dump may carry.

@history_item n=14 marker=amd-o5bpvnvg
**amd-o5bpvnvg** — The command line and MCP answer `graph`: `cli_commands` +1, `mcp_server` +1, `tools/list` order. These are the carrier checks a new verb adds.

@history_item n=15 marker=amd-tvsd3b6b
**amd-tvsd3b6b** — A graph call logs `verb` alone with no span field added, and `host_log`/`mcp_log` read 0 graph needles at `trace` (`graph_needles()`). Holds the picked command-span verb and tool_call rows for a new verb.

@history_item n=16 marker=amd-poiocitr
**amd-poiocitr** — `stand_graph`: a read changes nothing and holds no typed text, the whole Cells answer stays within the answer bound, the verb equals the in-process reader, and an argument is refused `malformed`. This is the check pattern for a new reading verb.

@history_item n=17 marker=amd-zsfczxyi
**amd-zsfczxyi** — The cold-agent pipe was taken to thirteen verbs (`VERBS`, `valid_step`, `isolated`, the fake's answer). The pipe's verb list moves with every verb added.

@history_item n=18 marker=amd-dkb4oe4y
**amd-dkb4oe4y** — `actions` took the verb table to fourteen: field kinds, describe's row, cli_tests' rows, the renamed listing test, and recording of the tenth instance verb. A further precedent for the count-site sweep.

@history_item n=19 marker=amd-ldelf34f
**amd-ldelf34f** — `actions` on the CLI and MCP: `cli_commands`, `mcp_server`, `stand_mcp_tools`, and the shared read-verb helper `a_read_tool_call_answers_its_reader_and_parents_a_command_recording_its_verb_alone`. A new read verb reuses this helper.

@history_item n=20 marker=amd-epp6lr5n
**amd-epp6lr5n** — The `actions` call was added to `host_log`/`mcp_log` (`actions_needles()`) and to `stand_act_spans` (the tenth instance verb, 38 calls per mode). These span and log counts move with a sixteenth verb.

@history_item n=21 marker=amd-q5lvjmtv
**amd-q5lvjmtv** — The cold-agent pipe at fourteen tools, its fake speaking `actions`. The same sweep applies when this chunk adds a verb.

@history_item n=22 marker=amd-n3udqhi7
**amd-n3udqhi7** — `stand_actions`: a read changes nothing and holds no typed text or mask, a refused argument, the Cells answer within the bound, and `fonts-unavailable` read before painting. This is the stand check pattern for a new read verb.

@history_item n=23 marker=amd-i4iccah5
**amd-i4iccah5** — No test exercised the replay of a kept `actions` step, so the replay test was owed as a CARRY. Trap: a new verb's kept-check replay must be measured, not assumed by construction.

@history_item n=24 marker=amd-gh4iy36p
**amd-gh4iy36p** — The kept `actions` step's replay was measured by `stand_replay` (a changed answer stops naming its key), which discharged the CARRY. This is the replay test a new recorded verb is owed.

@history_item n=25 marker=amd-abrsrea5
**amd-abrsrea5** — `mcp-server/tools-list` (picked row) moved to fifteen verbs, with `layout` after `actions` and `help` last. A sixteenth verb moves it again.

@history_item n=26 marker=amd-smsbvscr
**amd-smsbvscr** — The `command-form`, `usage-line` and `driver-session/reading-verbs` rows (all picked) moved for `layout`. They name the verb count and the reading-verb set that this chunk extends.

@history_item n=27 marker=amd-ewa3qy3e
**amd-ewa3qy3e** — `socket-carries-commands` (picked row) gained «layout answers» out on the founder's ratified fork. Whatever the canvas dump carries out must be added to this list.

@history_item n=28 marker=amd-h4mn5gu4
**amd-h4mn5gu4** — `driver-verbs/verb-table` (picked row) went from fourteen to fifteen verbs.

@history_item n=29 marker=amd-cdohja6t
**amd-cdohja6t** — `session-layout/changes-nothing` (picked row) now covers the verb as well as the in-process read, and `session-run/layout-answer` was added. Precedent for an in-process read and its verb sharing one no-change promise.

@history_item n=30 marker=amd-qfxe5qxs
**amd-qfxe5qxs** — `mcp-tools-list/order` (picked row) moved to fifteen with its stated order.

@history_item n=31 marker=amd-yfp5ovdp
**amd-yfp5ovdp** — Added `count-and-rename-sweeps` (picked row) after a missed numeric pin and a missed renamed witness when the verb count changed.

@history_item n=32 marker=amd-n2omhba3
**amd-n2omhba3** — `count-and-rename-sweeps` widened to CI leg file counts and CI-script tests.

@history_item n=33 marker=amd-mmdtifvl
**amd-mmdtifvl** — Served-error-page: `cross-os-readings-by-answer-digest` (picked row) moved from six to seven served tasks, and `count-and-rename-sweeps` gained its exception for count-worded witness names. Relevant if a Circle Drawer task is served.

@history_item n=34 marker=amd-jdiryrjq
**amd-jdiryrjq** — Trap: the driver crate's count is stated in two word orders («13 unit tests», «unit tests 13»), so a count sweep must search both.

@history_item n=35 marker=amd-3quv32td
**amd-3quv32td** — Cold-agent step and isolated rows moved to fifteen verbs. Also added `harness-construction/from-component-alone` and `vdom-alone` (picked rows), the mount a canvas fixture may use.

@history_item n=36 marker=amd-jadf4shh
**amd-jadf4shh** — Retired `cold-agent-verbs-without-layout`: by the founder's word, a new verb joins the cold-agent pipe at once.

@history_item n=37 marker=amd-q3tlwlgs
**amd-q3tlwlgs** — Self-description: `help`/describe count row, remedies, and `help` the same bytes on CLI and MCP. A new verb changes describe's output and its count row.

@history_item n=38 marker=amd-ybvlv3ad
**amd-ybvlv3ad** — Eight refusal causes in order with `fonts-unavailable` eighth (picked rows), seven served over MCP, and the harness virtual clock (`screenshot` never moves it). Fixes the cause set a canvas read may refuse with.

@history_item n=39 marker=amd-s53lzeiq
**amd-s53lzeiq** — The driver command span line was added. Trap: a sink capture's lines are counted by target, never in total, and a green per-package run is not the workspace leg's green.

@history_item n=40 marker=amd-t5mve4e6
**amd-t5mve4e6** — Rules for driver checks: a failure message names a layout mode, a task and an index, never an id or screen content. Binds this chunk's new checks.

@history_item n=41 marker=amd-xliprnr6
**amd-xliprnr6** — The session recording: `Session::recording_into`, records over the instance verbs, the 400-record / 1 MiB bounds, wire `v3`. A new instance verb is recorded through it.

@history_item n=42 marker=amd-7qex4fkt
**amd-7qex4fkt** — The kept `answer` (a shot's without `png`) and `Check::replay` stopping at the first differing answer: the `session-recording/record-fields` and `check-replay/stop` rows (picked). A canvas dump's kept form must follow them.

@history_item n=43 marker=amd-siqngma2
**amd-siqngma2** — A shot's PNG bytes differ between linux and macOS, so a check compared byte for byte across OSes holds no `screenshot` step. Origin of the no-shot rows that the canvas exceptions later narrowed.

@history_item n=44 marker=amd-5e244nch
**amd-5e244nch** — Added `app-widget-shows-in-shots` (picked row): a Rust-painted widget shows in harness and driver shots, whole and cropped, both modes, per package and in the workspace. This rests on the widget paint path, which this chunk must not change.

@history_item n=45 marker=amd-5xumfbc5
**amd-5xumfbc5** — `cross-os-checks-hold-no-shot` (picked row) gained its canvas-alone exception: a canvas-only shot is compared across OSes by answer digest (the founder's F1, byte equality).

@history_item n=46 marker=amd-ipy6bzhk
**amd-ipy6bzhk** — `no-shot-in-cross-os-byte-checks` gained the canvas exception, and `canvas-shots-cross-os-by-digest` (picked row) was added with `stand_canvas_cross_os`. This is the cross-host determinism precedent the dump's cross-host claim extends.

@history_item n=47 marker=amd-d26vi7ce
**amd-d26vi7ce** — `no-screenshot-in-cross-os-checks` (picked row, session-lifecycle) gained the canvas-alone exception.

@history_item n=48 marker=amd-wsteqwoi
**amd-wsteqwoi** — The reason in `no-screenshot-step-in-committed-checks` (picked row) narrowed to shots holding document text.

@history_item n=49 marker=amd-gplax6l7
**amd-gplax6l7** — The reason in `cross-os-checks-hold-no-shot` was restated to the measured cause: document text differs by OS, a canvas alone does not.

@history_item n=50 marker=amd-i2ckej2w
**amd-i2ckej2w** — The same reason restated on `no-shot-in-cross-os-byte-checks`.

@history_item n=51 marker=amd-tqpvxvpt
**amd-tqpvxvpt** — The same reason restated on `no-screenshot-in-cross-os-checks`.

@history_item n=52 marker=amd-cvy4bwz6
**amd-cvy4bwz6** — The reason in `checks-hold-no-screenshot-step` (picked row) was scoped to shots holding document text.

@history_item n=53 marker=amd-lbkskdc6
**amd-lbkskdc6** — The reason in `no-screenshot-in-recorded-check-scripts` (picked row) was scoped the same way.

@history_item n=54 marker=amd-6at54l43
**amd-6at54l43** — Added `canvas-text-on-whole-device-pixels` (picked row): Circle Drawer's canvas text is byte-equal across hosts only through the whole-pixel glyph snap. Glyph-run coordinates in a dump hang on this, the scope's premise and its WATCH.

@history_item n=55 marker=amd-ccaqhf5e
**amd-ccaqhf5e** — `canvas-shots-cross-os-by-digest` now names Circle Drawer's canvas element shot (`SAMPLE`) beside the three fixtures, read equal on three hosts by CI#45305097481.

@history_item n=56 marker=amd-zbmtsabl
**amd-zbmtsabl** — `canvas-text-on-whole-device-pixels` ratified by the founder. The engine-side remedy stays residual `res-ieayznir`.

@history_item n=57 marker=amd-nasxjktn
**amd-nasxjktn** — Added `stand-boot/boot-circle-drawer` (picked row): Circle Drawer is booted in process alone and served by no escher-session role, under the founder's «not now». This is why CLI, MCP and kept-check carriers reach no stand canvas.

@history_item n=58 marker=amd-sam3o45d
**amd-sam3o45d** — Cells and the load page were served through `stand::boot_cells` and `boot_load`, with their checks and fixture rows. Precedent if a served canvas page or fixture is needed to reach the carriers.

@history_item n=59 marker=amd-o3j7n4hl
**amd-o3j7n4hl** — The six served slugs through the binary (`cli_commands`, `host_binary`, the lifecycle). Serving a new task moves these.

@history_item n=60 marker=amd-knnsalwo
**amd-knnsalwo** — Serving the seventh task (order) moved the cold-agent pass, the prompt, and the stand task rows from six to seven, and added `stand::boot_order`. This is the cost of serving one more app.

@history_item n=61 marker=amd-arfsbrh6
**amd-arfsbrh6** — `session-run/layout-answer` moved to equality over the seven served screens. A new served screen moves per-verb equality claims.

@history_item n=62 marker=amd-7w6mfsac
**amd-7w6mfsac** — `escher-session-command-line/start` (picked row) moved to «the seven served tasks».

@history_item n=63 marker=amd-hf3zl3jw
**amd-hf3zl3jw** — Handler coverage's stated limit: «a handler inside a canvas is not counted», deferred to 0.4.0, and the capability statement's claim map gained the limits. Canvas limits are stated in `docs/capabilities.md` with their checks.

@history_item n=64 marker=amd-aj6rkfgc
**amd-aj6rkfgc** — `CLICKABLE` table: Circle Drawer reads 1 clickable element without a role (its canvas), and `NODE_FIELDS` is fourteen. These stand pins sit beside the canvas.

@history_item n=65 marker=amd-de7olhxu
**amd-de7olhxu** — `typed-password-never-returned` (picked row) covers every answer carrier (graph, actions, layout, kept and explored checks). A new verb's answer is covered by the same rule.

@history_item n=66 marker=amd-iriujbey
**amd-iriujbey** — That row's guard gained `stand_masks_on_new_answers`. A new answer carrier joins that check.

@history_item n=67 marker=amd-n3bdcrdl
**amd-n3bdcrdl** — `socket-carries-commands` gained «a step's caused with its error text». This is how the row's carried-out list grows by founder resolution.

@history_item n=68 marker=amd-wyiufcyk
**amd-wyiufcyk** — The session-lifecycle key file respelled the founder's word on the socket's crossing («Сокет несёт команды»), the provenance of `socket-carries-commands`.

@history_item n=69 marker=amd-472linzq
**amd-472linzq** — The capability statement pin (`test_capability_statement.py`): verbs and causes as code spans, schema counts in words, README's «the {word} driver commands as tools». A new verb reddens ci-scripts until the page restates it.

@history_item n=70 marker=amd-m7fw5fjq
**amd-m7fw5fjq** — `capability-statement-pinned` (picked row) now also pins the a11y and stand-repeat leg counts.

@history_item n=71 marker=amd-zn44tekk
**amd-zn44tekk** — `capabilities-doc-pinned-to-schema` (picked row) holds the schema counts to the schema and the leg counts to the CI leg script.

@history_item n=72 marker=amd-aiakktz6
**amd-aiakktz6** — `capability-check-names-held-to-schema` (picked row): the claim section's names and counts are pinned, so a renamed or moved check changes the section by hand.

@history_item n=73 marker=amd-755msr7u
**amd-755msr7u** — `capability-statement-from-schema` (picked row) restated to «the schema's counts».

@history_item n=74 marker=amd-4oiy4oeg
**amd-4oiy4oeg** — Added `requirement-witness-inside-stand-repeat` (picked row): each requirement names its witness inside stand-repeat, pinned by function name.

@history_item n=75 marker=amd-kube42bu
**amd-kube42bu** — stand-repeat's named `--test` words became seventeen, its file glob 51, pinned by `test_stand_repeat_leg_runs_three_rounds`. A new check without the `stand_` prefix must be named there, which moves the capability statement's count.

@history_item n=76 marker=amd-fcxxug5t
**amd-fcxxug5t** — Added `no-change-read-control-needs-pending-work` (picked row): a control of a read that must change nothing needs pending work. Binds this chunk's «changes nothing» control.

@history_item n=77 marker=amd-447m7y2r
**amd-447m7y2r** — Added `session-layout/changes-nothing` (picked row) with the stated-tolerance and reason-word rules for the layout reader. Precedent for a no-change read asserted in both modes.

@history_item n=78 marker=amd-mnnwqipq
**amd-mnnwqipq** — `overview/session-lifecycle` (picked) restated: the run session selection runs cli_flow flows.

@history_item n=79 marker=amd-h47hxvhq
**amd-h47hxvhq** — Added `doc-leg-gates-public-doc-edits` (picked row). Editing public doc comments (the schema's help, the harness's API) needs the doc leg in the local gate block.

@history_item n=80 marker=amd-6eogcl6u
**amd-6eogcl6u** — Added `oversize-test-targets-kept` (picked row): integration targets over 800 lines are kept whole, with no split.

@history_item n=81 marker=amd-ke4vohyd
**amd-ke4vohyd** — `shared-stand-tables-in-one-module` (picked row) gained its except: a helper whose body or message differs stays in its check. Governs the canvas tables added to `tests/blitz-tests/tests/common/mod.rs`.

@history_item n=82 marker=amd-nlavi4cc
**amd-nlavi4cc** — Shared fixtures module `tests/fixtures/mod.rs` (picked rows `shared-fixture-pages`, `shared-fixtures-in-one-module`): a page more than one check reads is stated once, and the module holds no print, env read or net.

@history_item n=83 marker=amd-umaactff
**amd-umaactff** — `stand-boots-offline` (picked row) gained the error-text page's `FailingNet` exception. The list of stand boots that keep a provider is closed.

@history_item n=84 marker=amd-z23dex63
**amd-z23dex63** — `GRAPH_DRIVES` in `tests/common/mod.rs` drives every conditional write once (picked row `graph-drives-confirm-every-write`). Shared stand tables that a new reader may lean on.

@history_item n=85 marker=amd-efljpsvp
**amd-efljpsvp** — `harness-construction/from-vdom` (picked row) now provides `SeededRandom::from_seed(DEFAULT_SEED)`. This is the determinism the harness gives a canvas fixture.

@history_item n=86 marker=amd-nx5bnxd7
**amd-nx5bnxd7** — Added `harness-inspect/focused` (picked row): with nothing focused, the focused reading answers the root element.

@history_item n=87 marker=amd-n5xqwtgz
**amd-n5xqwtgz** — Coverage floors: escher-driver 97 %, dioxus-native-dom 83 %, blitz-test-harness 90 %, and the zero-failure flakiness budget on stand-repeat. New driver and harness code for the canvas read must keep the floors.
