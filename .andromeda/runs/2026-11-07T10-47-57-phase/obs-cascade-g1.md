@carried n=1 label=obs-plan/telemetry-strategy fact=fct-weudw6wt
why: the panic path that the removed RefCell panic used to reach is escher-telemetry's, compiled into both seven_guis binaries, so the port must add no panic path or failure path in the render sites, and its sink checks must hold under both the package-alone and the workspace build.
quote: escher-telemetry depends on `tracing` with no feature and always compiles its startup and panic events into the two native binaries of the seven_guis package, `seven_guis_native` and `escher-session`, whose engine `tracing` features stay off in a package-alone build (`-p seven_guis`; a workspace-wide build turns blitz-dom's on — Feature wiring, below);
@carried n=1 run=2 label=obs-plan fact=fct-6oskq4op
quote: dioxus-native's `tracing` feature turns on tracing across dioxus-native-dom and the blitz crates (packages/dioxus-native/Cargo.toml:69)
@carried n=2 label=obs-plan/telemetry-strategy fact=fct-46rgllvo
why: the ported witness and the two inverted pins must run silent under both the workspace build and a `-p blitz-tests` build, and host_log and mcp_log must pass under both.
quote: By invocation: a build that resolves the whole workspace (`cargo test --workspace`, `ci-leg.sh fast`, CI's test leg) turns `blitz-dom/tracing` on — blitz's default `tracing` reaches blitz-shell and blitz-html, and each of those forwards to blitz-dom — while `cargo test -p blitz-tests` and a `-p seven_guis` build leave it off;
@carried n=2 run=2 label=obs-plan fact=fct-upbrcclf
quote: so under the workspace build a test child that installs escher's sink at `info` also receives engine events (test-plan §9 → Engine features by runner).
@carried n=3 label=obs-plan/span-trace-coverage fact=fct-j47hr7dn
why: the chunk edits a blitz-tests file and must add no span there, so this absence over the test crate must still hold after it.
quote: observed absent — spans or traces · searched: `tracing|log::|env_logger|sentry|opentelemetry|metrics` over the 61 slice files (test crate slice)
@carried n=4 label=obs-plan/span-trace-coverage fact=fct-zuu6oxcq
why: the render change reaches the escher-session host, so the command span must keep its eight fields and each hosted call must still write one closed span, as stand_act_spans and host_log re-assert.
quote: the span keeps its eight fields (tests/blitz-tests/tests/stand_act_spans.rs:555-567). Since 2026-10-10-driver-cli the span is also written by a hosted call: the `escher-session` host role decodes a wire `call` and runs it through `Session::run` and nothing else, so each hosted call is one closed `command` span under that binary's sink — measured, 3 lines for 3 calls at `RUST_LOG=trace` (`host_log`);
@carried n=4 run=2 label=obs-plan fact=fct-yqpbkmod
quote: every call handed to `Session::run` opens one INFO span `command` at target `escher_driver`, its eight fields (`verb` · `settled` · `busy` · `passes` · `added` · `removed` · `changed` · `cause`) closed words, bools and counts recorded only where they apply;
@carried n=5 label=obs-plan/span-trace-coverage fact=fct-jyfscbc7
why: host_log, which the chunk re-asserts, must still count the host's closed command span lines and read 0 id and 0 name needles under both builds, with no span, field or log line added.
quote: seven_guis `host_log`'s fourth test counts the host's closed `command` span lines and reads 0 id and 0 name needles under both builds (§6 Log coverage, §8 PII); no span, field or log line added
@carried n=5 run=2 label=obs-plan/span-trace-coverage fact=fct-gf4hwplk
quote: seven_guis `host_log`'s `a_select_list_opened_and_chosen_through_the_host_logs_no_option_at_trace` (under `#[cfg(unix)]`) and `mcp_log`'s `a_select_list_opened_and_chosen_through_the_role_logs_no_option_at_trace` drive an open list at `trace`, reading 0 needles (§6 Log coverage, §8 PII), `host_log` counting the host's closed `command` span lines
@carried n=5 run=3 label=obs-plan/log-coverage fact=fct-h6shwcuf
quote: searched: `host_log`'s `a_select_list_opened_and_chosen_through_the_host_logs_no_option_at_trace`, under `#[cfg(unix)]` and the state directory `hl-popup`, reading the host at `trace`: 0 option needles, its `command` span lines counted
@carried n=5 run=4 label=obs-plan/log-coverage fact=fct-k3vr6esa
quote: searched: `mcp_log`'s `a_select_list_opened_and_chosen_through_the_role_logs_no_option_at_trace`, reading the role at `trace`: 0 option needles; it records nothing, so no `ml-popup` directory exists
@carried n=6 label=obs-plan/span-trace-coverage fact=fct-xswdmupu
why: the chunk reshapes the two callers of flush_queued_mounted_events, which must keep opening no span and adding no tracing site, and host_log and mcp_log must still read 0 needles at trace under both builds.
quote: the mounted-event guard in `flush_queued_mounted_events` and `Harness::text_contrast`'s blend read open no span; `host_log` and `mcp_log` read 0 needles at `trace` under both builds
@carried n=6 run=2 label=obs-plan/log-coverage fact=fct-7dblu2wz
quote: dioxus-native-dom's mounted-event guard and the `deallocate_anonymous_block` fix add no env read, print, `tracing` site or wall-clock read
@carried n=6 run=3 label=obs-plan/log-coverage fact=fct-jok55ozp
quote: escher-driver holds 2 `info_span!`, `stand_act_spans` eight fields, and `host_log` and `mcp_log` read 0 needles at `trace` under both builds
@carried n=7 label=obs-plan/log-coverage fact=fct-7dblu2wz
why: the witness and the inverted pins move focus through `set_focus`, which fires this engine line under the workspace build, so it must stay the one focus line, holding a node number and no id or name, and the tests must install no subscriber that would print it.
quote: info: focussed node, once per focus move under the workspace build, where engine `tracing` is compiled in, holding a node number and no id or name;
@carried n=8 label=obs-plan/span-trace-coverage fact=fct-mp6exhtv
why: the render change reaches the mcp role, where each instance verb's command span must still nest under its tool_call, as mcp_log counts.
quote: an instance verb run through `Session::run` makes its `command` span the child by nesting — the executor unchanged —
@carried n=8 run=2 label=obs-plan fact=fct-wa4xq2qh
quote: opened once per MCP `tools/call` request, before its params are read, and held until its reply is written; two fields and no other, each a closed word — `tool`, the verb table's word, and `ending`, `accepted` · `refused` · `error`;
@carried n=9 label=obs-plan/log-coverage fact=fct-uz2p5hye
why: host_log and mcp_log count closed span lines in this record class, so the class must hold unchanged for the counts the chunk re-asserts.
quote: A closed span's line — a second record class since 2026-10-07-driver-command-spans, a boundary widening ratified by the founder (2026-10-09) as built — is written for each closed span whose target the allowlist admits, and no line when a span is created, recorded to, entered or exited:
@carried n=9 run=2 label=obs-plan fact=fct-dsb5vef5
quote: A closed span's line, the sink's second record class, is written for each closed span whose target the allowlist admits, and no line when a span is created, recorded to, entered or exited:
@carried n=10 label=obs-plan/log-coverage fact=fct-gwvelgmp
why: the mcp role's stderr line counts by build, with 0 needles, are what the render change must leave as they are, since the mutation-replay path adds no INFO line.
quote: over one CRUD flow at `info` the role's stderr reads 9 lines built alone and 10 built with the workspace — the tenth `INFO blitz_dom::document … message=[redacted]` — with 5 `tool_call` and 3 `command` lines in each (packages/escher-driver/src/mcp.rs:118-121; as measured at escher-0.1.0/chunks/2026-10-10-mcp-surface/report.md, read on the hand-run role, 2026-10-11); re-measured at 2026-10-14-stand-requirement-sweep, 9 built alone and 11 built with the workspace — the `blitz_dom::document` INFO line now printing twice — the 5 `tool_call` and 3 `command` lines unchanged, needles 0
@carried n=10 run=2 label=obs-plan fact=fct-pzih6tk6
quote: one closed-span line per call to `Session::run`, `span="command"` with its recorded fields, and one per MCP tool call, `span="tool_call"` with `tool` and `ending`, each INFO at target `escher_driver`, closed words, bools and counts only
@carried n=11 label=obs-plan/log-coverage fact=fct-gxkjmvep
why: the buffer replays through the same MutationWriter, so these per-mutation debug sites must fire on the replay exactly as on the direct write, none added, moved or widened, and the buffer itself must get no log site of its own.
quote: Every DOM mutation (assign_node_id, create_placeholder, create_text_node, append/insert/replace, remove_node, push_root, set_node_text, load_template, set_attribute) is logged at debug through `trace!` (packages/dioxus-native-dom/src/mutation_writer.rs:141-227; packages/dioxus-native-dom/src/mutation_writer.rs:327; packages/dioxus-native-dom/src/mutation_writer.rs:410)
@carried n=11 run=2 label=obs-plan fact=fct-2ebvsswy
quote: **Engine, test and tool emitters** — blitz-dom, blitz-net, blitz-html, blitz-shell, blitz-paint, blitz, blitz-vibey-script, dioxus-native, dioxus-native-dom, tests/blitz-tests, wpt/runner and the CI publish builds: each one's log sites by level, message and field
@carried n=12 label=obs-plan fact=fct-rvanvmh2
why: dioxus-native-dom is the crate the port edits, and its logging must stay optional behind its tracing feature, with any site the port moves still behind it.
quote: **dioxus-native, dioxus-native-dom** — optional `tracing` logging plus `log-times` features (log-phase-times, log-frame-times) (packages/dioxus-native/Cargo.toml:60-69; packages/dioxus-native-dom/Cargo.toml:19)
@carried n=13 label=obs-plan/log-coverage fact=fct-pynegysb
why: the chunk inverts two of this file's tests, so the record must stay true after it: 10 tests, both layout modes, silent, its failure messages naming a layout mode and never captured page text.
quote: observed absent — a print, an env read, a subscriber or a listener in the test target `dioxus_runtime_probe` (blitz-tests, 10 tests, both layout modes, no `stand_` prefix), which drives `blitz_test_harness::Harness` over Counter, CRUD, Cells and its own pages `toggled_listener`, `shared_state` and `hazard`, its failure messages naming a layout mode, a task or page and a link or index only
@carried n=14 label=obs-plan fact=fct-icom2cq7
why: host_log is the check that reads the sink-installing host, which the render change reaches, at trace, and it must still pass with 0 id and name needles under both builds.
quote: seven_guis' `host_log` spawns the same binary on `crud` and, in its fourth test, on `cells` at `RUST_LOG=trace` (a paged part read through the host's own sink: a sentinel committed into a part-2 cell, then `snapshot --part 2`, 0 id and 0 name needles under both builds), drains both streams from the spawn on and reads them into its assertions only
@binding n=1
**log-coverage ↔ telemetry-strategy**: the per-mutation debug sites in mutation_writer.rs must fire on the buffer's replay exactly as on the direct write, and must compile only through the tracing feature that dioxus-native turns on for dioxus-native-dom. The buffer must add no site of its own (obs-plan/log-coverage fct-gxkjmvep) (obs-plan fct-6oskq4op) (obs-plan fct-rvanvmh2)
@binding n=2
**telemetry-strategy ↔ span-trace-coverage / log-coverage**: host_log and mcp_log must pass under both the workspace build and the package-alone build, with the 2 command and tool_call span sites and stand_act_spans' eight fields unchanged and 0 needles at trace (obs-plan/telemetry-strategy fct-46rgllvo) (obs-plan/span-trace-coverage fct-jyfscbc7) (obs-plan/log-coverage fct-jok55ozp) (obs-plan/log-coverage fct-gwvelgmp)
@binding n=3
**log-coverage (blitz-dom focus line) ↔ log-coverage (dioxus_runtime_probe record)**: the inverted pins and the witness move focus, which fires the focussed-node INFO line under the workspace build. They must install no subscriber, so the probe file's silence record stays true and 10 tests stand in it (obs-plan/log-coverage fct-7dblu2wz) (obs-plan/log-coverage fct-pynegysb) (obs-plan/telemetry-strategy fct-46rgllvo)
@binding n=4
**span-trace-coverage ↔ the scope's flush callers**: the reshaped callers of flush_queued_mounted_events must open no span and add no tracing site, keeping the mounted-event guard's record (obs-plan/span-trace-coverage fct-xswdmupu) (obs-plan/log-coverage fct-7dblu2wz)
@binding n=5
**telemetry-strategy (panic events) ↔ the scope's removed hazard**: the RefCell panic the port removes used to reach escher-telemetry's chaining panic event in both seven_guis binaries, and the new render sites must add no panic or failure path in its place (obs-plan/telemetry-strategy fct-weudw6wt)
