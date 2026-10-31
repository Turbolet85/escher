# Classified — security-plan · input-validation.b1

## Answers
input-validation#1.1 · no · the URL argument rule lives in examples/screenshot.rs and examples/paint_bench.rs, neither reached by the session
input-validation#1.2 · no · the numeric-argument rule lives in examples/paint_bench.rs, examples/screenshot.rs and examples/multi_doc.rs, none reached by the session
input-validation#1.3 · no · the output filename rule lives in examples/screenshot.rs, which the session did not reach
input-validation#1.4 · no · the preact_script path rule lives in examples/preact_script.rs, which the session did not reach
input-validation#1.5 · no · scripts/agent-run.sh is unreached and its name rule reads test-file existence live, so the five new targets change no rule
input-validation#1.6 · no · scripts/cold-agent.sh is unreached, and its fixed prompt names no element id, so the cells-grid change does not reach it
input-validation#1.7 · yes · stale · test_ci_workflows.py is a reached file, so its InstallScriptTest citations at :427-461 and :481-497 can have moved
input-validation#1.8 · no · coverage_floor.py and its tests are unreached and unedited, and no edit moved the three fixed floors
input-validation#1.9 · no · scripts/cold-agent.sh is unreached, and the twelve-verb set its check reads stands, the session adding no verb
input-validation#1.10 · yes · add-to · F4 and F5: an optgroup's non-empty label is a tree name inside the ratified crossing, and a multiple select's value reads its first chosen option in tree order
input-validation#1.11 · no · the state-directory rule lives in host.rs and error.rs, neither reached, and no state rule moved
input-validation#1.12 · no · the recording-directory rule lives in session.rs and recording.rs, neither reached, and the recording rule did not move
input-validation#1.13 · no · the strict check reader lives in recording.rs and replay.rs, neither reached, and the committed checks and recording rule did not change
input-validation#1.14 · no · the session-label rule lives in session.rs, which the session did not reach, and no label rule moved
input-validation#1.15 · yes · add-to · F6: type on a multiple select focuses with no click and clears the typeahead, so the elements that can take type now include a multiple select, while the twelve verbs stand
input-validation#1.16 · yes · add-to · F4 and F5: an optgroup's label and a multiple select's chosen value now reach the answer's names and values
input-validation#1.17 · yes · add-to · F4 and F5: the MCP results' accessible names and control values now include an optgroup's label and a multiple select's first chosen option
input-validation#1.18 · no · the urlbar rule lives in apps/browser/src/nav.rs, which the session did not reach
input-validation#1.19 · no · the navigation rule lives in apps/browser/src/nav.rs and packages/dioxus-native/src/link_handler.rs, neither reached
input-validation#1.20 · yes · stale · pointer.rs is reached whole, so the href citation at pointer.rs:913-931 can have moved
input-validation#1.21 · no · the document URL rule lives in packages/blitz-dom/src/url.rs, which the session did not reach
input-validation#1.22 · yes · stale · scrolling.rs is reached whole for its visible_region change, so the fragment citation at scrolling.rs:821-824 can have moved
input-validation#1.23 · no · the file URL rule lives in packages/blitz-net/src/lib.rs, and no file in packages/blitz-net changed
input-validation#1.24 · no · the data URL rule lives in packages/blitz-net/src/lib.rs, and no file in packages/blitz-net changed
input-validation#1.25 · no · the shell data URL rule lives in packages/blitz-shell/src/net.rs, which the session did not reach (only event.rs is reached)
