# Classified — security-plan · input-validation.b1

## Answers
input-validation#1.1 · no · the URL argument's parse sits in examples/screenshot.rs and examples/paint_bench.rs, neither reached by the session
input-validation#1.2 · no · the numeric arguments sit in examples/paint_bench.rs, examples/screenshot.rs and examples/multi_doc.rs, none reached
input-validation#1.3 · no · the output filename rule sits in examples/screenshot.rs, which is not reached
input-validation#1.4 · no · the preact_script path check sits in examples/preact_script.rs, which is not reached
input-validation#1.5 · no · agent-run.sh is not reached and its existing-test-file-stem rule still holds for the new test files
input-validation#1.6 · no · cold-agent.sh is not reached and its fixed prompt and allowlist are unchanged; the covered effect on its run belongs to another item
input-validation#1.7 · yes · stale · test_ci_workflows.py is reached and its new stand-repeat assertion at line 367 sits above the cited lines 427-461 and 481-497, which the report does not show held
input-validation#1.8 · no · coverage_floor.py and its tests are not reached and the session changes no coverage floor
input-validation#1.9 · no · cold-agent.sh is not reached and the twelve-verb table its check reads is unmoved
input-validation#1.10 · yes · add-to · the socket's snapshot and diff now carry a row's own bounds and a shot of an open list carries option and group labels as pixels, and the highlighted row is in no snapshot, tree or lint (F11)
input-validation#1.11 · yes · stale · examples/seven_guis/tests/cli_commands.rs is reached and gains a hosted-shot pin, so the cited state-dir bound test lines 766-770 are not shown held
input-validation#1.12 · no · session.rs, recording.rs and cold-agent.sh are not reached and no tool call or result field names the recording directory
input-validation#1.13 · no · recording.rs, replay.rs and cold-agent.sh are not reached and the two committed checks gain no pixel
input-validation#1.14 · no · session.rs is not reached and the label rule is unaffected
input-validation#1.15 · yes · add-to · the covered cause's meaning gains a clause naming an option of a closed select, with its name, remedy and order unchanged
input-validation#1.16 · yes · stale · examples/seven_guis/tests/cli_commands.rs is reached and gains a hosted-shot pin, so the cited test lines 374-417 and 696-758 are not shown held
input-validation#1.17 · yes · stale · the restated click, type and press help is in the tools/list text, so the measured 2,984 → 8,844 byte listing figure moved
input-validation#1.18 · no · apps/browser/src/nav.rs is not reached
input-validation#1.19 · no · apps/browser/src/nav.rs and packages/dioxus-native/src/link_handler.rs are not reached
input-validation#1.20 · yes · stale · packages/blitz-dom/src/events/pointer.rs is reached with lines added from line 220 onward, so the cited href lines 1008-1026 moved
input-validation#1.21 · no · packages/blitz-dom/src/url.rs is not reached
input-validation#1.22 · yes · stale · packages/blitz-dom/src/scrolling.rs is reached and gains the open list's scroll branches, so the cited fragment lines 829-832 may have moved and the report does not show them held
input-validation#1.23 · no · blitz-net is not reached and the session fetches nothing
input-validation#1.24 · no · blitz-net is not reached and the session fetches nothing
input-validation#1.25 · no · packages/blitz-shell/src/net.rs is not reached
