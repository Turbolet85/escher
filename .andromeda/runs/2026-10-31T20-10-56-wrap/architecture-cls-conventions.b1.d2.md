# Classified — architecture · conventions.b1

## Answers
conventions#1.1 · yes · stale · the session inserts `mod select_list;` at render.rs line 7, moving the `#[expect(unused)]` citation at render.rs:718-719, while ci-leg.sh:20-25 and :37 sit above the new lines 59-60 and stay
conventions#2.1 · no · no manifest moved, Cargo.toml, Cargo.lock and deny.toml are unchanged, and the cited manifests are not reached
conventions#3.1 · no · the session adds modules and public items but moves no documentation rule, and the cited lines in reached scrolling.rs:1-2 and stylo.rs:993-1105 sit above its insertions
conventions#4.1 · yes · stale · render.rs gains a line at 7 and stylo.rs one at 1124-1139, moving the TODO citations at render.rs:1334 and stylo.rs:1232; the markers themselves stand
conventions#5.1 · yes · add-to · render::select_list and blitz-dom's select_list join the one-module-per-concern split the Touches name, and the document.rs:245-416 citation's end moves past the new BaseDocument select_list field at 343-344
conventions#6.1 · yes · stale · pointer.rs gains lines at 220-656, moving the cfg-gated citation at events/pointer.rs:1047; no feature is added and the driver's hand-built spans stand
conventions#7.1 · yes · stale · stylo.rs gains lines at 1124-1139, moving the unsafe-block citations at stylo.rs:1342 and 1414-1419; the SAFETY comments stand
conventions#8.1 · yes · stale · the session adds stand_act_popup, so the count of stand_act_* checks goes from fifteen to sixteen, and the three new targets move the stand_*.rs and mod-declarer counts
conventions#9.1 · no · the eight own-file test modules are in cli.rs, recording.rs, mcp, schema and dioxus-native-dom files, cli.rs and recording.rs are unreached, and the session adds no own-file test module
conventions#9.2 · no · fixtures/mod.rs is not reached and no fixture page or HeldNet moved, and the three new targets add mod fixtures declarers, a count this entity does not state
conventions#9.3 · no · mcp_tests.rs and stand_requirement_sweep.rs are not reached and the session adds no sweep row
conventions#10.1 · no · the cited app and example sources are not reached, and examples/seven_guis/src was not edited
conventions#11.1 · no · no examples/assets file is reached, and the cited HTML fixtures are unchanged
