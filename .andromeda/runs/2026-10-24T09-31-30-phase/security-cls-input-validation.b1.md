# Classified — security · input-validation.b1

## Answers
input-validation#1.1 · no · the URL argument rules of the screenshot and paint_bench examples are Not needed, since their CLI-argument rules are not in the delta
input-validation#1.2 · no · the numeric example arguments of paint_bench and screenshot are Not needed, since their CLI-argument rules are not in the delta
input-validation#1.3 · no · the screenshot output filename rule is Not needed, since the screenshot example's CLI-argument rules are not in the delta
input-validation#1.4 · no · the preact_script path rule is Not needed, since the preact_script example's CLI-argument rules are not in the delta
input-validation#1.5 · no · the agent-run.sh verb and selection rules are Not needed, since the script is not changed
input-validation#1.6 · no · the cold-agent.sh verb and task rules are Not needed, since the script is not changed and the cold-agent pipe is not run by a sync
input-validation#1.7 · no · the apt-install.sh argument rules are Not needed, since the script is not changed, and a package the take adds to ci.yml is checked by it as it stands
input-validation#1.8 · no · the coverage_floor.py path and floor arguments are Not needed, since the script is not changed and only ci-leg.sh's coverage filename exclusion can move
input-validation#1.9 · no · the cold-agent check's reply parsing is Not needed, since the cold-agent pipe and its model-provider crossing are not run by a sync
input-validation#1.10 · no · the driver socket's request grammar, wire, state and answer bounds are Not needed, since the chunk adds no verb, request or carrier, and its masked answer readings come from dioxus-native-dom, whose rules the Needs carry
input-validation#1.11 · no · the driver's state directory is Not needed, since the chunk adds no verb, request or carrier and its 0700 directory and 0600 socket rules are unchanged
input-validation#1.12 · yes · the two committed checks may be re-recorded through record_checks.sh into a launcher-named directory, and that run must meet the owner-only rule from which the committed checks are the one exception
input-validation#1.13 · yes · the strict reader parses the two committed checks that stand_replay replays, and a Taffy or Stylo move may re-record them, so their bytes must pass its closed keys, eight-verb set and required answer
input-validation#1.14 · no · the driver session label is Not needed, since labels are not changed and the chunk adds no verb, request or carrier
input-validation#1.15 · no · validate, the verb table, the argument kinds and the refusal causes are Not needed, since there is no schema change, and the executor's refusals read the snapshot and hit test, whose own rules the Needs carry
input-validation#1.16 · no · the escher-session argv reader, its usage line, its endings and its stdout carrier are unchanged, since the take adds no verb, argument or stdout carrier
input-validation#1.17 · no · the MCP server's framing, bounds, protocol errors and echoed ids are Not needed, since serve_mcp is not changed
input-validation#1.18 · no · the browser urlbar is Not needed, since mimalloc changes the browser's allocator and not its inputs
input-validation#1.19 · yes · dioxus-native's link_handler.rs gate (GET with http, https or mailto) lies in the crate the Dioxus 0.8 resync takes, so it must survive that take, while the browser's nav.rs is Not needed
input-validation#1.20 · yes · events/pointer.rs is a file the take reaches, and its href resolution at lines 884-902 sits in it, so an unparseable href must stay un-navigated
input-validation#1.21 · no · url.rs is not among the files the take names, and the remote-content rule binds only mutator.rs, layout/construct.rs and the hit testing in node/node.rs
input-validation#1.22 · yes · scrolling.rs is a predicted conflict the take resolves, and the fragment's lossy percent-decode before matching at lines 805-808 sits in that file and must survive the resolution
input-validation#1.23 · no · blitz-net is not among the engine files the take carries, so its file: path read is outside what the chunk changes
input-validation#1.24 · no · blitz-net's data: decoding is not among the engine files the take carries, so the chunk does not reach its data_url errors
input-validation#1.25 · no · blitz-shell's data: provider is not among the take's named source files, and the chunk moves only that crate's manifest
