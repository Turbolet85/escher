@disposition n=1
row: held fct-keyqn7iz apps/browser/Cargo.toml:14 — refused at 2026-10-24T15-00-29-cites-backlog, not touched since «default = […»
disposition: holds — stands (line 14 is the default feature set, which still lacks `accessibility`)
@disposition n=2
row: stretched fct-d5bs73pt packages/dioxus-native-dom/src/lib.rs:7-12 → 7-13 — changed inside
disposition: holds (lines 7-13 still name accesskit support, the snapshot, `to_text`, `text_parts`, `diff`, `unkeyed_actionable` and the UI graph; they now also name `actionable_nodes`)
@disposition n=3
row: stretched fct-6opcw6lo packages/escher-driver/src/execute.rs:242-246 → 251-255 — changed inside
disposition: holds (lines 251-255 are the guard that types no text opening on a space into a select)
@disposition n=4
row: stretched fct-kf4qrij2 tests/blitz-tests/tests/common/mod.rs:17-129 → 17-140 — changed inside
disposition: holds (lines 17-140 still hold the `controls`, `INPUT_NAMES` and `REGION_NAMES` tables, now beside the added `ACTIONABLE_AT_BOOT` and Tab-order tables)
@disposition n=5
row: held fct-ge4n5uhw tests/blitz-tests/tests/stand_snapshot.rs:1-5 — refused at 2026-10-25T17-25-04-wrap, not touched since «//! The sna…»
disposition: holds — stands (lines 1-5 are the test's module doc: snapshot role and name are the accessibility node's, in both layout modes)
@disposition n=6
row: stretched fct-pfylw46r packages/dioxus-native-dom/src/actionable.rs:55-89 → 93-157 — changed inside
disposition: claim false — raised at Validate (`unkeyed_actionable` no longer reads the listener mark `data-dioxus-id`. It now reads the vdom's current listeners other than `mounted`, so a listener dropped by a re-render is not counted and the check no longer over-includes, as lines 86-90 and `Readings::of` at line 74 show)
@disposition n=7
row: stretched fct-g5whxvnn packages/dioxus-native-dom/src/lib.rs:7-12 → 7-13 — changed inside
disposition: holds (line 7 still states accesskit support behind the `accessibility` feature)
