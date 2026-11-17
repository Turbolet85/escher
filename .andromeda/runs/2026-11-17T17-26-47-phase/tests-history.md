@history_item n=1 marker=amd-dvdqody5
**amd-dvdqody5** — 2026-10-16-file-splits: this entry moved escher-driver's schema and mcp unit tests out of line into `schema_tests.rs` and `mcp_tests.rs`, each under `#[cfg(test)]` with `#[path]` at its parent and its test paths unchanged. It recounted the crate's tests by their new files (73 in total, so many inline and so many in `#[path]` files). This is the precedent for splitting `json.rs` (the `json_tests.rs` path in the scope) and for rows inline-cfg-test-modules, test-module-split-keeps-paths and unit-tests-in-source-file. The per-file count sites in §1/§4 and in session-lifecycle's session-proof must follow the moved tests.

@history_item n=2 marker=amd-h4ip3qlm
**amd-h4ip3qlm** — 2026-10-19-test-splits: moved `cli_tests.rs` and `recording_tests.rs` out of line, which made five out-of-line modules eight, with test paths unchanged and line citations re-pointed. Trap: a `*_tests.rs` `#[path]` file is no record of the per-crate coverage reading, so moving tests out of line moved escher-driver's coverage from 98.33 % to 98.07 %. The entry says a later split should forecast a reading change, not none. This bears directly on the 97 % floor if the json tests move to a file of their own.

@history_item n=3 marker=amd-ur7tfp4h
**amd-ur7tfp4h** — 2026-10-07-audit-corrections: added the first `cfg(test)` module held in a file of its own through `#[path]` (`bridge_tests`) and set the layout convention: unit tests sit inline, bar a path-held module. This is the origin of the except clause on decision/unit-tests-in-source-file and decision/inline-unit-test-modules. Trap: a sed or grep address ending at a function's name also selects every item whose name starts with it, which matters when moving items by name.

@history_item n=4 marker=amd-lfmouigx
**amd-lfmouigx** — 2026-11-14-start-in-a-state: `~ decision/inline-cfg-test-modules` dropped the stated count of path-held cfg(test) modules ("Eight …") from the except clause. Another `json_tests.rs` therefore needs no row edit, but any count stated elsewhere (§2 and §4 bodies) still does.

@history_item n=5 marker=amd-ieh35jjn
**amd-ieh35jjn** — 2026-10-10-driver-cli: introduced `json.rs` (with `cli.rs` and `client.rs`) as a new unit-tested module. The schema is driven over the socket and through the binary by the seven_guis targets, and eight mutation controls were recorded. These are the origins of the answer writer and its CLI/MCP witnesses (`cli_commands`, `cli_flow`) that must stay byte-identical after the split.

@history_item n=6 marker=amd-itmrnscn
**amd-itmrnscn** — 2026-10-10-driver-cli: counted escher-driver at 50 unit tests in ten files, json 7 among them. This introduced the seven_guis `cli_commands`, `cli_flow` and `host_timer` targets (unix only) and the shared runner. It is the first per-file count site naming json.rs; per-file counts move if json's tests are split by family.

@history_item n=7 marker=amd-q3tlwlgs
**amd-q3tlwlgs** — 2026-10-12-self-description: json.rs (still 7 tests) now pins the remedy and `differs`. This added `describe.rs` (2) with its count row and the `help` self-description shared by both surfaces (`cli_commands`, `mcp_server`). `describe` and `help` bytes are scope witnesses that must stay unchanged.

@history_item n=8 marker=amd-3q4iwwwi
**amd-3q4iwwwi** — 2026-10-19-visual-lint (the gated part): json.rs's tests pin a finding's written form. It added the `lint` verb across carriers (`cli_commands`, `mcp_server`, `stand_mcp_tools`, `stand_lint`). The lint form (`finding`, `bounds`, `ratio`) is one of the unnamed families whose landing P4 must settle; its pins are witnesses.

@history_item n=9 marker=amd-4g5qmthy
**amd-4g5qmthy** — 2026-10-31-select-popup: `schema_tests`, `refusal` and `json` pin the help of `click`, `type` and `press` and the `covered` closed-option clause as fixed `'static` text, along with describe's count row. Rows contract/refusal-cause/covered and printed-form: the json pins of the refusal form (`impl Refusal`) must survive the move.

@history_item n=10 marker=amd-afxaahjb
**amd-afxaahjb** — 2026-11-07-ui-graph: json.rs pins the graph's `nodes` and `states` among the outcomes, and a node's `clickable` beside `focused`. The verb table went to 13 and cli_tests to 101 rows. The graph form (`graph_node`, `graph_state`, `write_readings`) and the snapshot node form (`node`, with `clickable`) are families of this split, and these pins witness them.

@history_item n=11 marker=amd-eiwetouu
**amd-eiwetouu** — 2026-11-08-handler-coverage: json.rs now holds 8 tests, including a coverage line keyed per handler with an absent `owner` left out. The coverage line form is `{"unexercised":[…],"checks":n,"held":m}`, and escher-driver reads 104 unit tests (67 inline / 37 `#[path]`) in the session-proof per-file list. This is the source of contract/coverage-line/shape and owner (picked rows) and of `impl Coverage`, an unnamed family of the split.

@history_item n=12 marker=amd-7otlq3ax
**amd-7otlq3ax** — 2026-11-08-what-can-i-do-here: escher-driver went to 102 unit tests in seventeen files, 66 inline, with actions.rs 2. This is the latest per-file and inline/`#[path]` count series before eiwetouu. A json split that moves tests to a new file changes the file count and the inline/path split at every count site.

@history_item n=13 marker=amd-dkb4oe4y
**amd-dkb4oe4y** — 2026-11-08-what-can-i-do-here: the verb table went to 14, describe's count row changed, and `the_listing_is_the_twelve_verbs_of_the_table` was renamed to fourteen while the complexity ceiling kept the old name's reading. Precedent: a rename leaves stale names at other sites. The `action_node` family's written form (`ActionNodes`) is pinned here.

@history_item n=14 marker=amd-yfp5ovdp
**amd-yfp5ovdp** — 2026-11-17-layout-on-the-carriers: `+ decision/count-and-rename-sweeps` (picked row). When the verb count changes, sweep for numeric pins; when a test is renamed, sweep for its old name held as a string literal (for example a requirement's witness). If the split renames or re-paths any json test, sweep `WITNESSES` and the capability statement for the old name.

@history_item n=15 marker=amd-h4mn5gu4
**amd-h4mn5gu4** — 2026-11-17-layout-on-the-carriers: `~ contract/driver-verbs/verb-table` now reads fifteen verbs. The layout family (`placer_word` … `layout_answer`) is the newest addition to the writer, and the verb table and `VERBS` must stay unchanged by the split.

@history_item n=16 marker=amd-cdohja6t
**amd-cdohja6t** — 2026-11-17-layout-on-the-carriers: `+ contract/session-run/layout-answer` (picked): the `layout` answer line holds ids, kebab-case words and numbers only, equal to the in-process read on every node of six screens in both modes. `~ contract/session-layout/changes-nothing` was amended alongside. These are the guard contracts of the `layout` family this chunk moves.

@history_item n=17 marker=amd-ewa3qy3e
**amd-ewa3qy3e** — 2026-11-17-layout-on-the-carriers: `~ decision/socket-carries-commands` (picked) adds layout answers to what the socket carries out. The socket's carried set is a boundary the split must not move.

@history_item n=18 marker=amd-abrsrea5
**amd-abrsrea5** — 2026-11-17-layout-on-the-carriers: `~ contract/mcp-server/tools-list` (picked) now reads fifteen verbs, `layout` after `actions` and `help` last. The tools listing is a byte-level MCP witness that must stay unchanged.

@history_item n=19 marker=amd-qfxe5qxs
**amd-qfxe5qxs** — 2026-11-17-layout-on-the-carriers: `~ contract/mcp-tools-list/order` (picked) now reads the fifteen-verb order. Same MCP witness as n=18, seen from the E2E side.

@history_item n=20 marker=amd-447m7y2r
**amd-447m7y2r** — 2026-11-16-layout-explained: added `+ decision/layout-reason-words-each-read` (picked: every placing word and size cause read on a fixture in both modes, guard `layout_reason`), the layout-session contract and stated-tolerance-read-inclusively. These are the witnesses of the layout writer's closed words.

@history_item n=21 marker=amd-fcxxug5t
**amd-fcxxug5t** — 2026-11-16-layout-explained: `+ decision/no-change-read-control-needs-pending-work`. A control of a no-change read must be built on pending work. This applies if the chunk adds any control around the moved reading verbs.

@history_item n=22 marker=amd-uqva53q6
**amd-uqva53q6** — 2026-11-16-errors-with-the-action: `~ contract/driver-session/acting-verbs` (picked) adds `caused`, last among the acting verbs' fields. The `caused` family (`with_caused`, `caused_item`, `caused_past_item`) writes it, and field order is part of the unchanged bytes.

@history_item n=23 marker=amd-n3bdcrdl
**amd-n3bdcrdl** — 2026-11-16-error-text-in-the-answer: `~ decision/socket-carries-commands` (picked) adds a step's `caused` with its error text. `+ contract/driver-session/payload-unread` (picked) adds the fixed `<payload unread>` panic item. Both are written by the `caused` family this chunk moves.

@history_item n=24 marker=amd-2ftiicau
**amd-2ftiicau** — 2026-11-16-error-text-in-the-answer: `~ decision/root-boundary-err-on-own-session` (picked: caused carries the root Err's text and chain on its own session). `+ decision/mcp-result-decoded-whole` (picked): an MCP result compared to the in-process answer is decoded whole through the one shared helper, a Windows red (CI#43940101161). Both are witnesses (`stand_act_errors`) of the caused written form.

@history_item n=25 marker=amd-rlvpc7s2
**amd-rlvpc7s2** — 2026-11-16-error-text-in-the-answer: `+ decision/engine-items-asserted-on-tracing-build` (picked). Engine-sourced caused items are asserted only on a tracing build (`stand_act_error_text`), so the caused form's witness reads differently per runner. Pair it with per-package vs workspace green (n=27).

@history_item n=26 marker=amd-z6xuttzw
**amd-z6xuttzw** — 2026-11-16-error-text-in-the-answer: `+ decision/expected-value-pinned-as-literal`. The cut control (the flood check on `cut_text`) held only once its measured value was pinned as a literal. `cut_text` belongs to the `caused` family this chunk moves, and its pin must move with it unchanged.

@history_item n=27 marker=amd-s53lzeiq
**amd-s53lzeiq** — 2026-10-07-driver-command-spans: the origin of decision/per-package-green-not-workspace-green (picked). The same check file was green per package and red in the workspace leg, because engine `tracing` features differ by runner. Gate the split on the workspace `fast` leg, not on `cargo test -p escher-driver` alone.

@history_item n=28 marker=amd-h47hxvhq
**amd-h47hxvhq** — 2026-11-16-error-field-in-the-answer: `+ decision/doc-leg-gates-public-doc-edits` (picked). Public doc comments linking a crate-private item were green locally and red only in CI's Documentation job. The scope's gates already name `ci-leg.sh doc`; a moved item whose path an intra-doc link names (for example json.rs's module doc) can break rustdoc.

@history_item n=29 marker=amd-uimtzpsg
**amd-uimtzpsg** — 2026-11-14-bundled-fonts: `+ decision/cross-os-readings-by-answer-digest` (picked). `stand_cross_os_readings` compares the answer digests of snapshot, lint and actions on the six served tasks against one committed table. This is one of the scope's named byte witnesses: any changed byte in those families turns it red on every OS.

@history_item n=30 marker=amd-xliprnr6
**amd-xliprnr6** — 2026-10-19-session-recording: the origin of contract/session-recording/record-fields (picked). Each record keeps `tool`, `arguments`, `masked`, `accepted` and an FNV-1a `digest` over the whole written line, and the wire went to `v3`. A kept check's digest makes any byte moved by the writer visible.

@history_item n=31 marker=amd-7qex4fkt
**amd-7qex4fkt** — 2026-10-19-recorded-check-replay: each record keeps its `answer` as an object, with a shot's `png` left out. The committed checks under `tests/blitz-tests/checks/` are replayed by `stand_replay`, red at the first differing kept answer (the check-replay/stop contract, picked). `counter-{cli,mcp}.json` are the scope's committed-check witnesses.

@history_item n=32 marker=amd-siqngma2
**amd-siqngma2** — 2026-10-19-recorded-check-replay: seven_guis `recording`'s `the_committed_checks_are_what_the_binary_records` reads the committed checks byte-equal to what the binary records (rows checks-byte-equal-to-recording, recorded-checks-kept-fresh). A shot's PNG differs between linux and macOS, so no cross-OS byte check holds a screenshot step. The witness that a writer byte moved without re-recording.

@history_item n=33 marker=amd-d7nbibtj
**amd-d7nbibtj** — 2026-11-07-ui-graph: both committed checks were re-recorded through `record_checks.sh` when the written form changed, and their byte sizes are no longer stated. The precedent shows that a change to an answer's bytes forces re-recording. A behaviour-free split must leave `counter-{cli,mcp}.json` untouched, so a needed re-record would itself prove a moved byte.

@history_item n=34 marker=amd-rrbdvino
**amd-rrbdvino** — 2026-10-12-headless-screenshot: the origin of decision/screenshot-in-band (picked). A shot's PNG crosses as padded base64 in `{"width","height","png"}`, and a shot's command span holds no image byte. This is the shot family (`shot`, `shot_without_image`, used by recording) whose landing the scope leaves to P4.

@history_item n=35 marker=amd-pgaqvksr
**amd-pgaqvksr** — 2026-10-20-large-screen-in-parts: added `Outcome::Part` and the paged `{"text","part","parts"}` answer line, the `--part` bound in `cli_commands`, and the `stand_snapshot_parts` check. The snapshot family's paged form, guarded by picked rows cli-snapshot/part, snapshot-text-budget and driver-session/snapshot.

@history_item n=36 marker=amd-lrjd5uwd
**amd-lrjd5uwd** — 2026-10-06-compact-snapshot-serialization: added the snapshot text form and `SNAPSHOT_TEXT_BUDGET` (10,000 B) checks (`stand_snapshot_text`), the source of decision/snapshot-text-budget (picked). Trap: a search for `snapshot.rs:` also matches `stand_snapshot.rs:`, and in the same way a `json.rs:` citation search can match `*_json.rs` or `json_tests.rs`, so anchor citation searches on the path.

@history_item n=37 marker=amd-vjv7kjzl
**amd-vjv7kjzl** — 2026-10-31-select-popup: citations re-pointed, including escher-driver `json.rs`, after edits moved its lines. Test-plan cites `json.rs` line ranges, so splitting the file retires or re-points every such citation, as wrap leaf work.

@history_item n=38 marker=amd-tssmtd5i
**amd-tssmtd5i** — 2026-11-08-what-can-i-do-here: re-pointed the cited ranges in json.rs, cli_tests.rs, mcp_tests.rs, schema_tests.rs and others to where added lines moved them. This is the latest re-point of json.rs citations: the sites a split must sweep.

@history_item n=39 marker=amd-nakcvsae
**amd-nakcvsae** — 2026-10-26-codebase-health: re-pointed line spans into edited files. Spans ending above a file's first hunk stay, and a `lint.rs:` pattern also matches `stand_lint.rs`, which was left alone. This is the rule for the citation sweep after `json.rs` is split.

@history_item n=40 marker=amd-n5xqwtgz
**amd-n5xqwtgz** — 2026-10-14-quality-gates: set coverage floors (escher-driver 97 %) and the zero-failure stand flakiness budget (picked rows stand-checks-zero-failure-budget and zero-failure-stand-repeat). Moving json's ~1060 test lines out of line moves escher-driver's per-crate reading (see n=2) under this floor.

@history_item n=41 marker=amd-le75xddb
**amd-le75xddb** — 2026-10-16-fast-failing-mutants: mcp_tests.rs gained `framing::read_request`'s test, the `mcp/{framing,…}` child-module precedent the scope's module shape follows. It also records the mutation tier recipe (cargo-mutants under nextest's `mutants` profile, `--baseline=skip`); the next audit of the split writer must run under that profile or tests read `timeout`.

@history_item n=42 marker=amd-yptdayxo
**amd-yptdayxo** — 2026-10-26-codebase-health: recorded the dev-host cognitive-complexity ceiling of 15 (rust-code-analysis). `cognitive.sum` adds nested spaces into their parent, so only splitting into module-level sibling items lowers a reading. Relevant if the family split is also read against the code audit's ceiling.

@history_item n=43 marker=amd-4mesoims
**amd-4mesoims** — 2026-10-26-codebase-health: factored escher-driver tests with private `#[track_caller]` helpers alone, no public symbol changed, every assertion and message kept, and controls red then green. This is the precedent for a behaviour-free structural refactor of driver tests, the same discipline the json test move needs.

@history_item n=44 marker=amd-jdiryrjq
**amd-jdiryrjq** — 2026-10-07-command-and-refusal-schema: refusal.rs pins the closed cause set and a refusal's printed form (rows cause-set and printed-form, picked). Trap: escher-driver's count is stated in two word orders ("13 unit tests" and "unit tests 13"), so a count site must be found by both when the json tests are recounted.

@history_item n=45 marker=amd-ybvlv3ad
**amd-ybvlv3ad** — 2026-10-18-one-virtual-clock: set eight causes in order with `fonts-unavailable` last (`the_eight_causes_are_named_in_order`), seven of them served over MCP, and `timer_step` carrying its remainder. These are the sources of picked rows eight-refusal-causes-in-order, mcp-causes-served-with-remedy and timer-step-carries-remainder; the `impl Refusal` written form serves them.

@history_item n=46 marker=amd-472linzq
**amd-472linzq** — 2026-10-25-capability-statement: `CapabilityStatementTest` pins `docs/capabilities.md`'s verbs, causes and counts to the schema, and its claim section names its checks by hand (picked rows capability-statement-pinned, capabilities-doc-pinned-to-schema, capability-claims-name-their-checks, capability-check-names-held-to-schema). If a json test named there as a claim's check moves or is renamed, the statement changes by hand.

@history_item n=47 marker=amd-poiocitr
**amd-poiocitr** — 2026-11-07-ui-graph: `stand_graph` and `stand_mcp_tools` hold the graph verb, whose answer carries no value (decision/graph-answer-holds-no-value, picked; guards `stand_graph`, `cli_commands`, `mcp_server`, `stand_mcp_tools`). These are byte and content witnesses of the graph writer family.

@history_item n=48 marker=amd-5zqg4udh
**amd-5zqg4udh** — 2026-11-07-ui-graph: the graph answer crosses the session socket carrying ids, names, flag words and booleans, never a value or `NodeId` (contract/session-wire/graph-answer, picked). This is the wire boundary the graph writer serves, unchanged by the split.

@history_item n=49 marker=amd-ckioxq2y
**amd-ckioxq2y** — 2026-11-07-ui-graph: the `unconfirmed` flag's stated limit, the source of decision/unrun-declared-write-reads-unconfirmed (picked). The `write_readings` flag words are part of the graph writer's output.

@history_item n=50 marker=amd-aj6rkfgc
**amd-aj6rkfgc** — 2026-11-07-ui-graph: `NodeState.clickable`, with `NODE_FIELDS` going to fourteen (contract/snapshot-node/clickable, picked). The scope requires `NODE_FIELDS` unchanged, and the `node` writer writes `clickable` after `focused`, only when true.

@history_item n=51 marker=amd-n3udqhi7
**amd-n3udqhi7** — 2026-11-08-what-can-i-do-here: `stand_actions` (8) holds the actions answer. It carries ids, owner ids, verb and cause words and booleans, with no value, mask or `NodeId`, and an argument is refused `malformed` (decision/actions-answer-holds-no-value and contract/driver-verbs/actions, picked). These are witnesses of the `action_node` writer.

@history_item n=52 marker=amd-nhvwrpl5
**amd-nhvwrpl5** — 2026-11-08-handler-coverage: `cli_commands` (18) runs the binary's `coverage` role and reads the one coverage line on stdout, status 0 (contract/cli-coverage-role/stdout and escher-session-command-line/coverage, picked). This is an end-to-end witness of `impl Coverage`'s written line.

@history_item n=53 marker=amd-i7b3rmrc
**amd-i7b3rmrc** — 2026-11-08-handler-coverage: `stand_handler_coverage` holds `handler_coverage`, and a typed sentinel never reaches the coverage line (decision/handler-coverage-holds-no-typed-text, picked). This is a witness of the coverage writer's content.

@history_item n=54 marker=amd-ij32pdcj
**amd-ij32pdcj** — 2026-11-15T05-21-42-wrap: the session binary's `coverage` role was ratified (no longer PROVISIONAL). Its refusals hold nothing of the path or file (contract/escher-session-command-line/coverage-refusal, picked). This is a ratified boundary the split must not move.

@history_item n=55 marker=amd-flbkdilp
**amd-flbkdilp** — 2026-10-19-visual-lint: lint.rs's tests pin `Defect::ALL`'s six defects in order and findings in document order, then by defect (picked contract/lint-defects/* and lint-findings/order). These are the reading the lint writer (`finding`) serialises.

@history_item n=56 marker=amd-u7jdserc
**amd-u7jdserc** — 2026-10-10-mcp-surface: introduced the MCP checks (`mcp_server`, `mcp_log`, `stand_mcp_tools`) and the `tool_call` span. A socket check is `cfg(unix)`-gated, while the MCP stdio-child checks through `Speaker` are not, bar `mcp_server`'s CLI comparison, and they run on windows (picked socket-checks-unix-gated, socket-session-checks-unix-only). The MCP witnesses run on all three OSes.

@history_item n=57 marker=amd-keodlnj3
**amd-keodlnj3** — 2026-10-10-driver-cli: a new test level of command-line checks and shell flows. The binary runs once per command, and two `sh` flows are assets of the `cli_flow` cargo target with no framework added. This is the origin of picked rows cli-checks-run-the-binary, cli-checks-run-own-binary, shell-flows-are-cargo-test-assets, shell-flows-under-cargo-test, started-host-held-behind-guard and cli-and-flow-targets-unix-gated, which cover the `cli_{commands,flow}` witnesses the scope names.

@history_item n=58 marker=amd-mnnwqipq
**amd-mnnwqipq** — 2026-11-15-session-binary-smoke: `~ overview/session-lifecycle` (picked): `run session` runs the session binary's `cli_flow` flows as tests through the agent-run contract. A cheap local way to read the CLI carrier's bytes after the split.

@history_item n=59 marker=amd-2gzx73ga
**amd-2gzx73ga** — 2026-10-06-headless-stand: stand checks assert unconditionally under the bundled DejaVu Sans with system fonts off and no skip path, and `stand::options(incremental)` pins harness options. These are the sources of picked rows stand-checks-bundled-font, stand-checks-bundled-font-no-skip, stand-pinned-real-font and contract/stand-boot/options. Every stand witness of the split reads under them.

@history_item n=60 marker=amd-gkbc5lsj
**amd-gkbc5lsj** — 2026-11-13-upstream-sync-determinism-and-exploration: `~ decision/layout-tests-in-both-layout-modes` (picked) gained the except clause for upstream targets that run one mode. The scope's gates require the stand checks in both layout modes.

@history_item n=61 marker=amd-3quv32td
**amd-3quv32td** — 2026-11-17-a-component-in-isolation: `+ contract/stand-boot/boot-alone` (picked), the harness's component-alone constructors, and the cold-agent step and isolated contracts at fifteen verbs. This is the chunk immediately before this one; its pipe reads `layout`. The split leaves the verb lists unchanged.

@history_item n=62 marker=amd-b4vvhst7
**amd-b4vvhst7** — 2026-11-17-a-component-in-isolation: `+ decision/harness-graph-tests-enable-accessibility` (picked). A per-package test build compiles the UI graph out unless the dev-dependency turns `accessibility` on. Relevant if any moved json graph test is gated by that feature: a per-package `-p escher-driver` run may compile it differently from the workspace run.

@history_item n=63 marker=amd-dhibncze
**amd-dhibncze** — 2026-10-05-fork-ci-reached: CI runs on PRs and on pushes to main, v0.* and build/**. Linux legs run through `ci-leg.sh`, the same command the host runs. The workspace tests are the fast `test` leg (`--locked`), and the matrix tests windows and macos. These are the sources of picked rows ci-trigger-branches, ci-legs-host-reproducible, workspace-tests-in-fast-ci-leg and native-platform-ci-matrix; the chunk's "fork CI green" gate rests on them.

@history_item n=64 marker=amd-yx45xaju
**amd-yx45xaju** — 2026-11-01-upstream-sync-ui-graph: `fast` now runs fmt → clippy → test → ci-scripts → embolden, with embolden a local-only leg (picked rows local-pre-push-gate and embolden-leg-local-only). This is the local gate block of `ci-leg.sh fast` named in the scope's gates.

@history_item n=65 marker=amd-tcnuzkmh
**amd-tcnuzkmh** — 2026-10-31-engine-debts: every cargo leg runs `--locked`, except `fmt` and `examples/wasm_hello` (picked decision/cargo-legs-locked). The scope keeps `Cargo.lock` unchanged.

@history_item n=66 marker=amd-d4acxs6q
**amd-d4acxs6q** — 2026-10-24-a11y-re-read: a per-job CI tally strips ANSI codes written both as ESC and as the literal `^[` (picked decision/ci-log-ansi-strip). Applies when re-reading CI tallies for this chunk's verdict.

@history_item n=67 marker=amd-jy34nxjf
**amd-jy34nxjf** — 2026-11-07-mounteddata-buffered-render: a tally move is attributed by grepping the test's own name, never by the preceding `Running` line (picked decision/tally-move-attributed-by-test-name). Applies to proving the split moved no test count in the CI job logs.

@history_item n=68 marker=amd-xy6zexlx
**amd-xy6zexlx** — 2026-11-17-upstream-sync-ahead-of-polish-and-ship: the free-disk step now stands on the default-features test job too. This is the previous chunk (`d875c3d22`), whose CI verdict the scope reads as not yet available and re-reads at P3 and before push.
