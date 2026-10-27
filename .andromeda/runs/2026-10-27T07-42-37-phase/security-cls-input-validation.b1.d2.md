# Classified — security · input-validation.b1

## Answers
input-validation#1.1 · no · the examples' URL argument (examples/screenshot.rs, examples/paint_bench.rs) is not reached, and the task holds the examples' CLI handling untouched
input-validation#1.2 · no · the examples' numeric arguments and MULTI_DOC_N (examples/multi_doc.rs) are not reached, and the task holds the examples' CLI handling untouched
input-validation#1.3 · no · the screenshot example's output filename (examples/screenshot.rs) is not reached
input-validation#1.4 · no · the preact_script path (examples/preact_script.rs) is not reached, and the task holds its CLI handling untouched
input-validation#1.5 · no · agent-run.sh is not reached and the task holds its CLI handling untouched; the new blitz-tests files already meet its existing-stem rule
input-validation#1.6 · no · cold-agent.sh is not reached and the cold-agent pipe is Not needed; the chunk runs no live agent session
input-validation#1.7 · no · apt-install.sh is not reached and the task holds its CLI handling untouched; ci.yml's package calls are not in the reached files
input-validation#1.8 · no · the take edits only the coverage leg's two keyboard_compat ignore lines; the leg's three fixed floors are another list that must only keep holding, and coverage_floor.py is not reached
input-validation#1.9 · no · the cold-agent pipe's model reply, check extraction and replay rules are Not needed, and the chunk runs no live agent session
input-validation#1.10 · yes · the socket wire is Not needed, but the answer carries a file input's value as the fixed mask and element ids, which must hold after pointer.rs and dioxus-native-dom's snapshot move
input-validation#1.11 · no · the driver's state directory, socket file and idle expiry are Not needed; the chunk changes no escher-driver code or carrier
input-validation#1.12 · no · the recording directory's modes and check-file creation are the recorder's, unchanged and reached by no edited file; the committed checks the take re-records are bound through #1.13
input-validation#1.13 · yes · it names the two committed checks under tests/blitz-tests/checks/ that stand_replay reads; the take may move a kept answer and re-record them, so they must still pass the strict reader (a JSON object answer, closed keys)
input-validation#1.14 · no · the session label is read in session.rs, which the chunk does not reach
input-validation#1.15 · yes · the executor's disabled, off-screen and covered refusals read the hit test and bounds the take changes, `type` on a select uses the select rules in the event files, and Outcome's Debug prints a file input's value as the mask, which must hold
input-validation#1.16 · no · the escher-session argv reader, usage line, exit statuses and stdout answer are not reached; the values they carry cross by the socket row (#1.10), which binds
input-validation#1.17 · yes · MCP results carry a file input's value as the mask and element ids, which must hold after pointer.rs and dioxus-native-dom's snapshot move; the framing, bounds and protocol errors are Not needed
input-validation#1.18 · no · the browser urlbar parse sits in apps/browser/src/nav.rs, which the take does not reach
input-validation#1.19 · no · the external-navigation scheme and method rules sit in apps/browser/src/nav.rs and dioxus-native's link_handler.rs, neither reached; only dioxus-native's manifest is
input-validation#1.20 · yes · the href resolution and its unparseable-href refusal sit in events/pointer.rs, which is reached whole for the file-input path the take moves
input-validation#1.21 · no · DocumentUrl parsing sits in blitz-dom's url.rs, which the take does not reach
input-validation#1.22 · yes · the fragment's lossy percent-decoding sits in scrolling.rs, a conflict surface the take merges, with its scroll-target clamp and missing-node no-op
input-validation#1.23 · no · blitz-net's file: read is untouched (blitz-net is Not needed); the script fetcher's file: narrowing is a separate rule in blitz-vibey-script
input-validation#1.24 · no · blitz-net's data: decoding is untouched (blitz-net is Not needed)
input-validation#1.25 · no · the DataUriNetProvider sits in blitz-shell's net.rs, which the take does not reach; lib.rs and window.rs are reached, but not this rule
