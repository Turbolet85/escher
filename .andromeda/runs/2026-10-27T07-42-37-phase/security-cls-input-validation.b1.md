# Classified — security · input-validation.b1

## Answers
input-validation#1.1 · no · the examples' URL argument handling is Not needed, as screenshot and paint_bench are untouched and examples/ is not reached
input-validation#1.2 · no · the numeric example arguments (paint_bench, screenshot, multi_doc) are Not needed and examples/ is not reached
input-validation#1.3 · no · the screenshot example's output filename rule is Not needed and examples/ is not reached
input-validation#1.4 · no · the preact_script path rule is Not needed, as preact_script and the vendored Preact fixture are untouched
input-validation#1.5 · no · scripts/agent-run.sh's verb and selection parsing is Not needed, and a new blitz-tests file only adds a stem the rule already accepts
input-validation#1.6 · no · scripts/cold-agent.sh's verb and task parsing is Not needed, and the chunk runs no live agent session
input-validation#1.7 · no · .github/scripts/apt-install.sh's package and option rule is Not needed, and no ci.yml run step is reached
input-validation#1.8 · no · .github/scripts/coverage_floor.py is not reached, and the leg's floors are another unit of ci-leg.sh beside the two ignore lines the take edits, a re-check every change owes
input-validation#1.9 · no · the cold-agent's agent-written check and its model reply are Not needed, and the chunk runs no live agent session
input-validation#1.10 · yes · the socket's answers carry stable ids and a file input's mask, which the take's id derivation must keep collision-free with no NodeId crossing as the Needs require, while its wire, state directory and connection bounds are Not needed
input-validation#1.11 · no · the driver's state directory (mode, path bounds, socket file) is Not needed and escher-driver's code is unchanged
input-validation#1.12 · no · the recording directory and a kept check's creation and bounds are Not needed, the recorder being unchanged
input-validation#1.13 · yes · the reader is unchanged, but stand_replay reads the two committed Counter checks through it in the fast leg, and a moved layout number may move a kept answer that its script re-records
input-validation#1.14 · no · the driver session label is checked in escher-driver, which the chunk does not change
input-validation#1.15 · yes · the executor's disabled, off-screen and covered refusals and scroll's in_view read the hit and scroll readings the take changes (ebacd68bd, 0980cce5f), witnessed by the re-asserted stand_act checks, while the schema, validation, causes and spans are Not needed
input-validation#1.16 · no · escher-session's argv reader (packages/escher-driver/src/cli.rs, examples/seven_guis/src/session_host.rs) is not reached and the chunk changes no driver code
input-validation#1.17 · yes · results carry a file input's value as the mask, read through stand_mcp_masked in the re-asserted stand checks once pointer.rs moves, while framing, bounds, id echo and protocol errors are Not needed
input-validation#1.18 · no · the browser urlbar (apps/browser/src/nav.rs) is not reached
input-validation#1.19 · no · the navigation method and scheme rule lives in apps/browser/src/nav.rs and packages/dioxus-native/src/link_handler.rs, neither reached
input-validation#1.20 · yes · packages/blitz-dom/src/events/pointer.rs is reached whole and its link navigation (884-902) runs on the click path that the file-input and Enter/Space edits touch
input-validation#1.21 · no · DocumentUrl parsing lives in packages/blitz-dom/src/url.rs, which is not reached
input-validation#1.22 · yes · packages/blitz-dom/src/scrolling.rs is a predicted conflict surface, and its fragment match (805-808) is the anchor scroll the Needs keep beside scroll_into_view and 0980cce5f's missing-node rule
input-validation#1.23 · no · blitz-net's file: read is untouched (blitz-net is not reached), and the ScriptFetcher narrowing is a separate rule
input-validation#1.24 · no · blitz-net's data: decode is untouched (blitz-net is not reached)
input-validation#1.25 · no · the data: provider lives in packages/blitz-shell/src/net.rs, which is not reached
