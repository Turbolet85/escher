@disposition n=1
row: held fct-rekexd3u examples/screenshot.rs:41-51 — refused at 2026-10-10T01-56-50-wrap, not touched since «let file_cont…»
disposition: holds — stands (lines 41-51 still fail with `.unwrap()` on the file read and the fetch)
@disposition n=2
row: held fct-m2wugvlj .github/workflows/publish-browser.yml:122-126 — refused at 2026-10-27T10-45-32-wrap, not touched since «- name: Ins…»
disposition: holds — stands (122-126 is the `Install Dioxus CLI` step that provides `dx`, beside `Install Rust` at 94-98 and the `dx bundle` step at 152-157)
@disposition n=3
row: stretched fct-kaaobbkf@c2830 packages/dioxus-native-dom/src/lib.rs:18-35 → 18-36 — changed inside
disposition: claim false — raised at Validate (the module list omits the new module `reached` at lib.rs:28, whose `ReachedHandler` is re-exported at lib.rs:45)
@disposition n=4
row: stretched fct-mzrxx2a3 packages/escher-driver/src/lib.rs:139-177 → 142-182 — changed inside
disposition: claim false — raised at Validate (the crate now holds nineteen private modules, not eighteen: `coverage`, with `Coverage`, `HandlerRef` and `handler_coverage` re-exported, is unnamed)
@disposition n=5
row: changed fct-qkp2micb examples/seven_guis/src/session_host.rs:1-5 → ? — the end rewritten «//! The `escher-session` bin…»
disposition: holds — re-pointed by hand to 1-7 (the doc comment now ends at line 7, adding a `coverage` role the fact does not name)
@disposition n=6
row: changed fct-qkp2micb examples/seven_guis/tests/host_log.rs:1-22 → ? — the end rewritten «//! The `escher-session` host…»
disposition: holds — re-pointed by hand to 1-24 (the doc comment gains a coverage-run sentence and now ends at line 24)
@disposition n=7
row: stretched fct-jzxi3spj examples/seven_guis/tests/common/mod.rs:371-597 → 371-624 — changed inside
disposition: claim false — raised at Validate (the listed readers omit the new `coverage_needles` at common/mod.rs:458, used for `host_log`'s coverage run)
@disposition n=8
row: held fct-k7yo6bgj README.md:49 — refused at 2026-11-08T02-04-13-wrap, not touched since «An MCP client can drive the s…»
disposition: holds — stands (README.md:49 links `docs/capabilities.md`)
@disposition n=9
row: stretched fct-t7zq5feb packages/escher-driver/src/lib.rs:136-177 → 139-182 — changed inside
disposition: claim false — raised at Validate (the crate now re-exports 62 names, not 59: `Coverage`, `HandlerRef` and `handler_coverage` from the private module `coverage` are unnamed)
@disposition n=10
row: held fct-t7zq5feb@c23929 packages/escher-driver/src/schema.rs:197-202 — refused at 2026-11-08T02-04-13-wrap, not touched since «help: "the …»
disposition: holds — re-pointed by hand to 203-208 (the `text` field's help literal stands there, unchanged)
@disposition n=11
row: stretched fct-t7zq5feb packages/escher-driver/src/lib.rs:140-177 → 143-182 — changed inside
disposition: holds (the new `coverage` module carries no `cfg` gate either; command, refusal and schema are still ungated)
@disposition n=12
row: stretched fct-t7zq5feb packages/escher-driver/src/cli.rs:46-101 → 51-116 — changed inside
disposition: holds (`command_line`'s doc still states `start` spawning `serve <app> --session <dir>` with its streams closed; it now also states a `coverage` role)
@disposition n=13
row: stretched fct-t7zq5feb packages/escher-driver/src/cli.rs:137-203 → 154-237 — changed inside
disposition: claim false — raised at Validate (the reader now takes a `coverage` first word as the coverage role after `mcp` and before the verb table, which the fact's fixed rule order omits, so `coverage` no longer reads `unknown-verb`)
@disposition n=14
row: stretched fct-t7zq5feb packages/escher-driver/src/cli.rs:341-368 → 375-403 — changed inside
disposition: holds (the usage line still names the host role, the mcp role and `start [--record <dir>]`, and now a coverage role too)
@disposition n=15
row: stretched fct-t7zq5feb packages/escher-driver/src/json.rs:416-420 → 417-445 — changed inside
disposition: holds (`shot_without_image` stands at 443-445; the range now also holds `impl Coverage`, under the shot's stray doc comment at 417-418)
@disposition n=16
row: stretched fct-fltopt6e packages/escher-driver/src/replay.rs:40-70 → 40-80 — changed inside
disposition: holds (`replay` now delegates to the crate-private `replay_each` with a no-op hook; the label check, the run through `Session::run` and the stop at the first change are unchanged)
@disposition n=17
row: stretched fct-kllkxz3d packages/dioxus-native-dom/src/element_id.rs:269-408 → 269-425 — changed inside
disposition: holds (the added `element_listener_ids` reuses the same walk; the id derivation is unchanged)
@disposition n=18
row: stretched fct-kllkxz3d packages/dioxus-native-dom/src/lib.rs:42-49 → 43-51 — changed inside
disposition: holds (the snapshot re-exports stand at 47-51, beside the new `reached` re-export)
@disposition n=19
row: stretched fct-uzcbujjs packages/dioxus-native-dom/src/dioxus_document.rs:446-457 → 447-468 — changed inside
disposition: holds (the registry's dispatch is unchanged; a `ReachedRecord` context, present only when coverage enables it, now records before it)
@disposition n=20
row: stretched fct-ie5oemgh packages/dioxus-native-dom/src/element_id.rs:253-311 → 253-328 — changed inside
disposition: holds (the added `element_listener_ids` also reads the vdom walk's listeners, never the listener mark)
