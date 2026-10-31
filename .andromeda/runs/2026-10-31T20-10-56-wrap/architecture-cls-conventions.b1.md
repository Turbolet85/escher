# Classified — architecture · conventions.b1

## Answers
conventions#1.1 · yes · stale · the session's insertion at render.rs:7 moves the cited `#[expect(unused)]` at render.rs:718-719 down past it; the fmt and clippy gates stay green at 715798ce2 (CI 17/17)
conventions#2.1 · no · no manifest, `Cargo.toml`, `Cargo.lock` or `deny.toml` is reached and no dependency is added; every cited manifest is unreached
conventions#3.1 · no · the cited `//!` and `///` lines sit in unreached files or above every edited hunk (scrolling.rs:1-2; stylo.rs:993-1105 before the 1124-1139 hunk) and the report moves no doc rule
conventions#4.1 · yes · stale · the TODO citations stylo.rs:1232 and render.rs:1334 sit past the session's insertions at stylo.rs:1124-1139 and render.rs:7, so both lines moved
conventions#5.1 · yes · add-to · the session adds blitz-dom `node::select_list` and blitz-paint `render::select_list`, one module per concern each, joining the split; the render.rs:1-6 citation stands and document.rs:245-416 moves past its 343-344 insertion
conventions#6.1 · yes · stale · the feature-gate citation events/pointer.rs:1047 sits past the session's pointer.rs hunks at 220-656, so its line moved
conventions#7.1 · yes · stale · the cited stylo.rs:1342 and :1414-1419 sit past the session's stylo.rs hunk at 1124-1139, so both lines moved
conventions#8.1 · yes · stale · the count "the fifteen driver-action checks `stand_act_*`" at lines 52-54 goes to sixteen with the new `stand_act_popup`, and the cited document.rs:3527-3528 moves past the document.rs hunks
conventions#9.1 · no · the eight file-split unit-test modules are unchanged; the cited cli.rs and recording.rs are unreached, and schema_tests.rs was edited in place with its path unchanged; the report names no new file-split test module
conventions#9.2 · no · fixtures/mod.rs is unreached and states no count the three new targets move
conventions#9.3 · no · the cited mcp_tests.rs and stand_requirement_sweep.rs are unreached and their row-helper claim holds
conventions#10.1 · no · every cited app and example file is unreached; examples/seven_guis/src was not edited, only tests beside it
conventions#11.1 · no · the cited examples/assets HTML fixtures are unreached and no asset is in the reached files
