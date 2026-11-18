@row_set n=1 row=decision/stand-readings-same-on-every-ci-os cell=rule seen=1 detector=none
value: The seven served stand tasks read the same driver answer digests of snapshot, lint and actions on linux x86_64, windows x86_64 and macOS aarch64, in both layout modes, at boot and after one step, compared with no screenshot.
basis: fct-uduwzs7i, fct-wj6hglte
because: The served-task set moved from six to seven with the order page, whose readings stand in the cross-OS table and read identical on every CI OS, so the row's count must follow.
@row_set n=2 row=contract/cold-agent-verdict/passed cell=promise seen=1 detector=none
value: The verdict is `passed` only on positive evidence: client exit 0, isolation, matching tool-call counts, all seven apps met, and the agent's final check replaying passing and failing on an ablated replay.
basis: fct-wnmdhla7, fct-uduwzs7i
because: The cold-agent pipe now names seven apps with a done condition for the order page, so passing requires all seven met, not six.
@row_set n=3 row=contract/cold-agent-verdict-reasons/capability-unused cell=promise seen=2 detector=none
value: The verdict fails with `capability-unused` unless `lint`, `screenshot`, `advance`, `graph`, `actions` and `layout` are each used, a part is read, a list is opened and `caused_by_kind` reads at least 1 for each of panic, error and log.
basis: fct-wnmdhla7, fct-vkogrj6o, fct-tosqicaq
because: The capability-unused reason now also fires when any of the three caused kinds reads 0, so its promise must name that condition.
@row_set n=4 row=contract/escher-session-cli/usage cell=promise seen=1 detector=none
value: escher-session <verb> [arguments] --session <dir> drives seven served stand tasks: the four lean ones, Cells, the load page and the order page.
basis: fct-wnmdhla7, fct-uduwzs7i
because: The served-task table gains `order` after `load`, which the usage line names, so the row's six now reads seven.
@row_set n=5 row=contract/escher-session/invocation cell=promise seen=1 fact=F2 detector=none
value: escher-session <verb> [arguments] --session <dir> drives the stand's seven served tasks: counter, flight-booker, timer, crud, cells, load, order.
basis: fct-wnmdhla7, fct-uduwzs7i
because: escher-session now serves seven tasks, `order` the seventh after `load`, so the invocation's task list must name it.
@row_set n=6 row=contract/escher-session/mcp cell=promise seen=1 detector=none
value: An MCP server on the binary's standard streams whose start tool boots the same seven tasks through the same boot.
basis: fct-wnmdhla7, fct-uduwzs7i
because: The MCP start enum is written from the served-task table, which now holds seven tasks.
@row_set n=7 row=contract/headless-stand/boot cell=promise seen=2 detector=D-arch-resources
value: seven_guis::stand, native only, boots the four lean tasks, Cells, a load page, the order page and one component alone in a pinned 800x600 Light viewport with one bundled font.
basis: fct-wnmdhla7, fct-wjnblmax
because: The stand gains the public offline boot of the order page, so the stand's boot list must name it.
@row_set n=8 row=decision/cold-agent-run-carries-stand-screen cell=rule seen=2 detector=none
value: The live cold-agent run sends the stand's served screens to the model provider from the driver's own MCP server: element ids, accessible names, control values, snapshot texts, diffs, graph and actions answers, and acting answers' caused text, the order page's authored messages among it.
basis: fct-x6bscvow, fct-rvcgucdu, fct-wnmdhla7
because: The chunk's boundary widening has the order page's authored caused text cross to the model provider on the live run, which the row's list of what is sent does not name.
@row_retire n=1 row=decision/cold-agent-six-apps seen=1 heir=decision/cold-agent-prompt-apps detector=none
why: The cold-agent prompt now names seven apps, so a row named and keyed by six apps no longer stands.
basis: fct-wnmdhla7, fct-uduwzs7i
because: The prompt names seven apps and goal 7, so the six-app row is replaced by one not keyed by a count.
@row_add n=1 row=decision/cold-agent-prompt-apps area=standard-contracts detector=none
name: Cold-agent prompt names the served apps
rule: The cold-agent prompt names seven apps (counter, flight booker, timer, CRUD, cells, the load page and the order page) and their goals, naming no stand id.
basis: fct-wnmdhla7, fct-uduwzs7i
because: The prompt gains the order page as its seventh app and goal 7, so the heir of the six-app row states seven.
@row_add n=2 row=contract/cold-agent-verdict/caused-by-kind area=standard-contracts fact=F1 detector=D-arch-resources
boundary: cold-agent verdict.json
item: caused_by_kind
promise: The cold-agent verdict and its run.end event carry `caused_by_kind`, an object of the closed words panic, error and log to counts of accepted click, type, press, advance and scroll answers whose caused list holds an item of that kind, each kind counted once per answer; a non-JSON answer is skipped.
basis: fct-wnmdhla7, fct-vkogrj6o
because: The verdict gains a new field on verdict.json and the run.end event, an unregistered contract item at the verdict boundary.
@row_add n=3 row=decision/app-crate-takes-own-tracing area=conventions fact=F4 detector=D-arch-decisions
name: App crates that log take tracing themselves
rule: An app crate that logs takes tracing itself as a workspace dependency.
why: dioxus-native's prelude re-exports no tracing.
basis: fct-mqjftslj, fct-r5pxjumu, fct-vtqo6tjc
because: seven_guis now logs and took tracing as a workspace dependency since no re-export reaches it, a convention every logging app crate follows and no row states.
@row_none n=1 fact=F3 type=4
relied: The stand check of the order page pins that a caused item carries the UI graph's owner of the control that caused it, none for a component keyed by HTML ids under no keyed element.
