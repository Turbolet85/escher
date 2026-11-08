@disposition n=1
row: held fct-rekexd3u examples/screenshot.rs:41-51 — refused at 2026-10-10T01-56-50-wrap, not touched since «let file_cont…»
disposition: holds — stands (lines 41-51 still fail with `.unwrap()`)
@disposition n=2
row: held fct-m2wugvlj .github/workflows/publish-browser.yml:122-126 — refused at 2026-10-27T10-45-32-wrap, not touched since «- name: Ins…»
disposition: holds — stands (the step installing the `dx` CLI the bundle step runs)
@disposition n=3
row: stretched fct-bhxvo7z2 packages/dioxus-native-dom/src/lib.rs:5-15 → 5-16 — changed inside
disposition: holds
@disposition n=4
row: stretched fct-mzrxx2a3 packages/escher-driver/src/lib.rs:131-168 → 139-177 — changed inside
disposition: claim false — raised at Validate (the crate now declares eighteen private modules, not seventeen: `mod actions;` with its re-exported `ActionNode` and `VerbReading` is not named)
@disposition n=5
row: stretched fct-4zmppehn tests/blitz-tests/tests/stand_act_spans.rs:1-24 → 1-24 — changed inside
disposition: holds
@disposition n=6
row: stretched fct-4zmppehn tests/blitz-tests/tests/stand_mcp_tools.rs:771-843 → 772-867 — changed inside
disposition: holds
@disposition n=7
row: stretched fct-4zmppehn tests/blitz-tests/tests/stand_requirement_sweep.rs:245-1013 → 245-1013 — changed inside
disposition: holds
@disposition n=8
row: stretched fct-4zmppehn tests/blitz-tests/tests/stand_requirement_sweep.rs:542-837 → 542-837 — changed inside
disposition: holds
@disposition n=9
row: stretched fct-4zmppehn@c29495 tests/blitz-tests/tests/session_common/mod.rs:134-146 → 134-160 — changed inside
disposition: holds
@disposition n=10
row: stretched fct-4zmppehn@c29656 tests/blitz-tests/tests/session_common/mod.rs:119-172 → 119-186 — changed inside
disposition: holds
@disposition n=11
row: stretched fct-qkp2micb examples/seven_guis/tests/host_log.rs:1-21 → 1-22 — changed inside
disposition: holds
@disposition n=12
row: stretched fct-drud2scu examples/seven_guis/tests/mcp_log.rs:55-335 → 57-352 — changed inside
disposition: holds
@disposition n=13
row: stretched fct-drud2scu packages/escher-driver/src/mcp.rs:44-112 → 44-112 — changed inside
disposition: holds
@disposition n=14
row: stretched fct-jzxi3spj examples/seven_guis/tests/common/mod.rs:371-573 → 371-597 — changed inside
disposition: claim false — raised at Validate (the list of what the two log checks read stderr with omits `actions_needles`, which both `host_log` and `mcp_log` now read)
@disposition n=15
row: changed fct-ldsoar27@c1675 README.md:49 → ? — rewritten «An MCP client can drive the same apps: it spawns `target/debu…»
disposition: holds — re-pointed by hand to 49 (the link to `docs/capabilities.md` stands there; the same line now says fourteen driver commands, not the fact's thirteen)
@disposition n=16
row: held fct-k7yo6bgj README.md:49 — refused at 2026-11-08T02-04-13-wrap, not touched since «An MCP client can drive the s…»
disposition: holds — stands (README.md:49 links `docs/capabilities.md`)
@disposition n=17
row: stretched fct-t7zq5feb packages/escher-driver/src/lib.rs:129-168 → 136-177 — changed inside
disposition: claim false — raised at Validate (the crate re-exports 59 names, not 57: `ActionNode` and `VerbReading` from the private `actions` module are not named)
@disposition n=18
row: changed fct-t7zq5feb@c10692 packages/escher-driver/src/schema.rs:550-555 → ? — the start rewritten «/// The verb set, …»
disposition: holds — re-pointed by hand to 575-580 (the `VERBS` array still holds `LINT`)
@disposition n=19
row: held fct-t7zq5feb@c23276 packages/escher-driver/src/schema.rs:197-202 — refused at 2026-11-08T02-04-13-wrap, not touched since «help: "the …»
disposition: claim false — raised at Validate (the help of `snapshot`'s `text` does not read as quoted: it reads "the screen as one text, or the part of it asked for: one line per element, nested by indent, a part's lines as they read in the whole text; …" and stands at 202-207)
@disposition n=20
row: stretched fct-t7zq5feb packages/escher-driver/src/lib.rs:132-168 → 140-177 — changed inside
disposition: holds
