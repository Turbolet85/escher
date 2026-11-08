@carried n=1 label=test-plan/test-strategy fact=fct-slbj25yc
why: the read's command on the command line must be exercised by seven_guis' command-line checks, an acting-free read answering exit status 0 and its usage errors 2, through the bounded shared runner.
quote: seven_guis' checks run the package's own `escher-session` binary once per command as a client process and read its stdout, its stderr and its exit status — `0` accepted · `1` refused · `2` usage · `3` session error — through the shared runner of `tests/common/mod.rs`, which bounds the run and drains both streams
@carried n=1 run=2 label=test-plan fact=fct-nsfd2ghz
quote: the POSIX `sh` flow scripts are assets of a `cargo test` target, not a runner of their own, and no test framework is added; the targets are `cfg(unix)`-gated
@carried n=2 label=test-plan/test-strategy fact=fct-5d4ttzku
why: the chunk edits recording.rs and replay.rs so the read becomes a step of a kept check, which must replay through Session::run with its answer compared as a JSON value and stay byte-equal to what the binary records.
quote: a check a session recorded, committed as data under `tests/blitz-tests/checks/` (`counter-cli.json`, `counter-mcp.json`, git mode `100644`), read by the strict `Check::from_bytes` and run as an ordinary `cargo test` target (`stand_replay`) through `Check::replay` — every step re-validated through `Session::run`, red at the first step whose acceptance or kept answer differs, the answers compared as JSON values — and kept fresh by a byte-for-byte comparison with what the binary records (seven_guis `recording`)
@carried n=2 run=2 label=test-plan fact=fct-obwdwo5i
quote: a check a session recorded is committed as data under `tests/blitz-tests/checks/` and run as an ordinary `cargo test` target through `Check::replay`, red at the first step whose acceptance or kept answer differs, and kept fresh by a byte-for-byte comparison with what the binary records
@carried n=3 label=test-plan/test-strategy fact=fct-krjmr356
why: the chunk edits docs/capabilities.md, whose closing section must name the check behind the read's claim and its limits by hand, only names and counts being pinned.
quote: `docs/capabilities.md`, a document with no runtime surface, ends on `## Where each claim is proven`, naming the check behind each claim it makes — the stand checks the `stand-repeat` leg runs among them — and states each limit as pinned by its check or as asserted by no check, the `select` model's limits in its four bullets (docs/capabilities.md:227-235; docs/capabilities.md:256-259), its keys, rows and groups pinned by `select_interaction` and `stand_act_select`, the list a click, Enter and Space open read by `select_popup`, `select_popup_painted` and `stand_act_popup`; only its names and counts are held to the schema's tables, by the CI-script pin `test_capability_statement.py` (§4), so a check renamed or moved changes that section by hand
@carried n=4 label=test-plan/unit-test-strategy fact=fct-rg6yibnz
why: the re-read capability-statement pin must stay green over the verb and cause names and counts in words the chunk edits, README.md's driver-command count moving with a verb on the MCP carrier.
quote: every verb of `VERBS` (`packages/escher-driver/src/schema.rs`, lower-cased) is a code span of `docs/capabilities.md`; every cause of `Cause::name`'s match arms (`packages/escher-driver/src/refusal.rs`) is one; the statement states five counts in words read from the schema's tables — `{word} verbs` · `{word} refusal causes` · `{word} session errors` (`SESSION_ERRORS`, `error.rs`) · `{word} node keys` (`NODE_FIELDS`) · `{word} exit statuses` (`EXIT_STATUSES`), each word from `NUMBER_WORDS` (one … twenty), none hand-copied; and README.md holds «the {word} driver commands as tools»
@carried n=4 run=2 label=test-plan fact=fct-pf335kvt
quote: test_capability_statement.py pins `docs/capabilities.md` to the driver schema's tables — every verb and refusal cause named as a code span, the verb, cause, session-error, node-key and exit-status counts stated in words — and README.md's count of the driver commands an MCP client gets as tools
@carried n=5 label=test-plan/test-strategy fact=fct-7m7lxsmy
why: the chunk edits the schema, MCP, command-line and recording unit tests held in these #[path] files, whose list must stay true.
quote: `mcp::tests` (packages/escher-driver/src/mcp.rs:291-293), `schema::tests` (packages/escher-driver/src/schema.rs:603-605) and, since 2026-10-19-test-splits, `cli::tests`, `recording::off_unix_tests` and `recording::tests` (packages/escher-driver/src/cli.rs:478-480; packages/escher-driver/src/recording.rs:453-455; packages/escher-driver/src/recording.rs:457-459)
@carried n=5 run=2 label=test-plan fact=fct-by5wg4wl
quote: inline `#[cfg(test)]` modules in source files, bar the modules held in files of their own through `#[path]`
@carried n=6 label=test-plan/test-strategy fact=fct-7m7lxsmy
why: a new stand check for the read that declares mod session_common must be counted in this reader count.
quote: `tests/session_common/mod.rs`, read through `mod session_common;` by 33 checks, as measured at escher-0.3.0/chunks/2026-11-07-ui-graph/report.md
@carried n=7 label=test-plan/test-strategy fact=fct-7m7lxsmy
why: a covered, off-screen or owner-served fixture the read's check reads through mod fixtures must be counted in this declaring count.
quote: `tests/fixtures/mod.rs`, read through `mod fixtures;` by 31 checks, counted anchored at line start, as measured at escher-0.3.0/chunks/2026-10-31-select-popup/report.md
@carried n=8 label=test-plan/test-strategy fact=fct-7m7lxsmy
why: a new blitz-tests target for the read moves the file and target count, each its own auto-discovered target with no manifest line.
quote: 159 `.rs` files, 158 test targets and the unbuilt `all.rs`, as measured at escher-0.3.0/chunks/2026-11-07-ui-graph/report.md
@carried n=9 label=test-plan/unit-test-strategy fact=fct-klxgwgcy
why: the verb table the chunk edits is pinned in order and shape by schema_tests, so a new verb or a widened VerbSpec must extend these pins.
quote: schema_tests.rs (`schema::tests`) pins the verb table — thirteen verbs in a stated order and three levels (`Level::Instance` · `Level::Session` · `Level::Tool`, `Level::name` reading `instance` · `session` · `tool`: the nine instance-level verbs, `screenshot` after `scroll` since 2026-10-12-headless-screenshot, `lint` after `screenshot`, `graph` after `lint` (no argument; fields `nodes` and `states`), then `start`, `status`, `stop`, then, since 2026-10-12-self-description, `help`), each with help, arguments and at least one result field, the lookup by exact name only (a name differing in case or padding finds nothing), each verb's argument and result shapes
@carried n=9 run=2 label=test-plan fact=fct-mnyw2bw5
quote: its unit tests pin the wire grammar, the session's label rule and record of ids, the error and refusal texts, the verb schema, command validation, the executor's key table and span, the JSON forms, the command line, the client, MCP, recording, replay, image and lint
@carried n=10 label=test-plan/unit-test-strategy fact=fct-klxgwgcy
why: a new result field for the read's answer must join the pinned field kinds, as graph's two did.
quote: the field kinds, twelve (the seventh, `FieldKind::Count`; then `Words` and `Entries`, since 2026-10-12-self-description, and `Png`; then `GraphNodes` (`graph-nodes`) and `GraphStates` (`graph-states`))
@carried n=11 label=test-plan/unit-test-strategy fact=fct-klxgwgcy
why: the read's per-verb refusal causes must be drawn from this closed, ordered cause set, a cause added moving its pinned table.
quote: refusal.rs pins the closed cause set — eight causes in a stated order (`fonts-unavailable` the eighth, ratified by the founder, 2026-10-18), each name, meaning and remedy a fixed text against a stated table, the remedies pairwise distinct and no `/` in any — and a refusal's printed form, which opens with its cause's name and ends with its remedy
@carried n=12 label=test-plan/unit-test-strategy fact=fct-klxgwgcy
why: the chunk edits execute.rs, whose command span's eight field names are pinned, so the read records verb and, refused, cause with no new field.
quote: execute.rs pins the executor's key table — each of the schema's twelve keys stands for one keyboard key, pairwise distinct, the row count asserted — and the command span: its eight field names against the table the span's own macro emits, none an argument's name, and each busy class reading as its schema word
@carried n=13 label=test-plan/unit-test-strategy fact=fct-klxgwgcy
why: the read's written answer in json.rs must be pinned with its verb's fields in order, as the graph's are.
quote: json.rs pins the written forms — a string with its escapes, a number as the shortest decimal and `null` when not finite, a node with the schema fields it has (a masked value as `MASKED_VALUE`), each outcome with its verb's fields in order, the graph's `nodes` and `states` among them, each refusal from its cause's fixed strings
@carried n=14 label=test-plan/unit-test-strategy fact=fct-klxgwgcy
why: the read's command-line command is pinned by cli_tests, its usage line built from the verb table.
quote: cli_tests.rs (`cli::tests`, in `cli.rs` until 2026-10-19-test-splits) pins the command line — a line read by its rules in order (61 rows), an argument that is not text as a usage error, each ending's streams and status, the usage line built from the verb table and the app names, and a line that is no command or is refused running nothing
@carried n=15 label=test-plan/unit-test-strategy fact=fct-klxgwgcy
why: the read's verb must be validated at its own level, its admitted calls and malformed witnesses pinned in command.rs.
quote: command.rs pins that a verb is checked at its own level only — `validate` refuses a session-level verb as `unknown-verb` before its arguments are read, `validate_session` admits the three session commands and, since 2026-10-12-self-description, `help`, refusing an argument to it as `malformed`, and refuses an instance-level verb
@carried n=16 label=test-plan/unit-test-strategy fact=fct-klxgwgcy
why: the chunk edits describe.rs, whose self-description must hold only its tables' texts.
quote: describe.rs that the document holds `help`'s seven fields in order and every text in it is its tables'
@carried n=17 label=test-plan/unit-test-strategy fact=fct-klxgwgcy
why: the read's recorded step must keep the recording pins, its answer digested and the strict check grammar extended to its step.
quote: recording_tests.rs (`recording::tests`, in `recording.rs` until 2026-10-19-test-splits) pins the FNV-1a 64 digest of the written answer, a recording's read-back, the masked `type` stand-in, the calls no record can spell left out, owner-only modes with no file replaced and no foreign temporary removed, a recording ending at its bound or on a failed write with its check removed, and `Check::from_bytes`'s strict grammar (40 refusal rows and 8 read rows, the record keeping its answer, a shot's without `png`, edge rows at both bounds)
@carried n=18 label=test-plan/unit-test-strategy fact=fct-klxgwgcy
why: the read's replayed step must hold on a fresh session and stop the replay naming its key when its answer changes.
quote: replay.rs that a check replayed on a fresh session holds, that an acceptance flip and a changed answer each stop the replay naming their keys, that a shot holds at its size whatever its pixels and that a check of another app runs nothing (packages/escher-driver/src/replay.rs:141-318)
@carried n=19 label=test-plan/unit-test-strategy fact=fct-klxgwgcy
why: the read's widest call must fit the request bound and its answer must stay inside the answer bound on the socket.
quote: the widest call the schema admits fits the request bound, and an answer at its 1,048,576-byte bound is carried while one byte over is not
@carried n=20 label=test-plan/unit-test-strategy fact=fct-klxgwgcy
why: the counts this chunk moves — verbs, field kinds, node keys — must be re-measured at its report.
quote: the verb table's thirteen verbs, twelve field kinds and fourteen node keys at escher-0.3.0/chunks/2026-11-07-ui-graph/report.md
@carried n=21 label=test-plan/unit-test-strategy fact=fct-fghufsse
why: VERBS at 13, causes at 8 and describe's count row are pinned values a new verb, field kind or cause must move together.
quote: `schema_tests`, `refusal` and `json` pin the help of `click`, `type` and `press` and `covered`'s clause, an option of a closed select lying under it, each fixed `'static` text, with `VERBS` at 13, causes at 8 and `describe`'s count row `[13, 14, 12, 3, 8, 4, 15]`
@carried n=22 label=test-plan/unit-test-strategy fact=fct-zot44d7e
why: a verb on the MCP carrier must be asserted by the listing test in VERBS order through its row-and-index-only helpers.
quote: mcp_tests.rs's `the_listing_is_the_twelve_verbs_of_the_table` asserts the thirteen verbs of the table in `VERBS`' order, `graph` after `lint`, keeps its outer assertions and its row walk, each row's name asserted inline, and asserts each listed tool through three private `#[track_caller]` helpers — `assert_described` (the tool's description and its result fields), `assert_schema` (the input schema's properties and their order) and `assert_property` (one argument's kind, enum and maximum) — whose messages name the row and the index only; the file holds 16 `#[test]`, 15 on unix
@carried n=23 label=test-plan/unit-test-strategy fact=fct-5zhbhdnd
why: a widened VerbSpec stating which elements type applies to must keep type's pinned help on a select.
quote: schema_tests.rs pins `type`'s help on any select, which `type` focuses with no click, inside a standing test, with no test of its own
@carried n=24 label=test-plan/unit-test-strategy fact=fct-3ferhyj7
why: a verb on the MCP carrier moves the cold-agent pipe's fake server tool set and its isolation reading of the tools.
quote: a fake `target/debug/escher-session` (`FAKE`, which speaks the thirteen tools, answering `lint` with `{"findings":[]}` and `graph` with `{"nodes":[],"states":[]}`, no verdict requiring `graph` of a cold agent (.github/scripts/test_cold_agent.py:140-141)
@carried n=25 label=test-plan/unit-test-strategy fact=fct-hskbfmoc
why: the read's node model and owner-served nodes come from the UI graph reader, whose listener and select-owned option pins must hold.
quote: `graph::tests` pins the UI graph reader — each node's component; its current listeners, `mounted` included and a removed one gone; a declared write reading declared before a dispatch and confirmed after; two reader components reading a state the writer does not; an option owned by its select
@carried n=26 label=test-plan/unit-test-strategy fact=fct-5kl4r7k5
why: the read's actionable set draws on the snapshot's clickable reading, whose pinned rule it must agree with.
quote: `an_element_with_an_app_click_listener_reads_clickable` pins `NodeState.clickable` true where the element's current listeners hold `click` and it is no disabled form control (packages/dioxus-native-dom/src/snapshot.rs:1038-1059)
@carried n=27 label=test-plan/unit-test-strategy fact=fct-3cpa7acl
why: the read lists stable ids and must produce no reserved virtual id, virtual-node production staying in 0.4.0.
quote: `element_id`'s tests `no_id_the_walk_writes_holds_the_virtual_separator` and `a_virtual_node_id_spells_the_reserved_form_and_names_no_element` pin the reserved `{owner}///{key}` form over an adversarial page; `virtual_node_id` returns `None` for an empty or `/`-bearing key, and no caller produces one, the walk not calling it
@binding n=1
**unit-test-strategy ↔ test-strategy**: a new verb, field kind or cause moves the unit pins of the verb table and describe's count row (test-plan/unit-test-strategy fct-fghufsse) and the counts in words of docs/capabilities.md pinned by the capability statement (test-plan/unit-test-strategy fct-rg6yibnz) in one change, the proven-claims section naming the read's check by hand (test-plan/test-strategy fct-krjmr356).
@binding n=2
**unit-test-strategy ↔ test-strategy**: the carrier the founder picks decides which pins move together — on MCP the listing test (test-plan/unit-test-strategy fct-zot44d7e) and the cold-agent fake's tool set (test-plan/unit-test-strategy fct-3ferhyj7), on the command line cli_tests (test-plan/unit-test-strategy fct-klxgwgcy) and seven_guis' command-line checks (test-plan/test-strategy fct-slbj25yc).
@binding n=3
**unit-test-strategy ↔ test-strategy**: the read's step in a kept check is pinned at unit level by recording and replay (test-plan/unit-test-strategy fct-klxgwgcy) and at integration level by stand_replay and the byte-equal committed checks (test-plan/test-strategy fct-5d4ttzku).
@binding n=4
**unit-test-strategy ↔ the scope**: the actionable set and owner-served nodes the read lists must agree with the graph reader's and the snapshot's clickable pins (test-plan/unit-test-strategy fct-hskbfmoc) (test-plan/unit-test-strategy fct-5kl4r7k5), with no reserved virtual id produced (test-plan/unit-test-strategy fct-3cpa7acl).
@binding n=5
**test-strategy ↔ the plan's new checks**: a new stand check or fixture for the read moves the session_common, fixtures and target counts (test-plan/test-strategy fct-7m7lxsmy), to be re-measured at the chunk's report.
