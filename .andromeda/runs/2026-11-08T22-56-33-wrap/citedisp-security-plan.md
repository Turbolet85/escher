@disposition n=1
row: held fct-73ojmpsr apps/browser/Cargo.toml:14 — refused at 2026-10-24T12-21-29-wrap, not touched since «default = ["hyb…»
disposition: holds — stands (line 14 is the default feature list, holding `cookies` and `cache`)
@disposition n=2
row: held fct-4kkbivro packages/blitz-dom/src/layout/table.rs:535-539 — refused at 2026-10-13T15-50-37-wrap, not touched since «let Some(di…»
disposition: holds — stands (535-539 is the `let Some(display) … else` that logs at info and returns)
@disposition n=3
row: stretched fct-v4gvn7w2 packages/escher-driver/src/replay.rs:40-70 → 40-80 — changed inside
disposition: holds (`Check::replay` and `replay_each` run each step through `Session::run`, nothing on another app, stopping at the first change)
@disposition n=4
row: stretched fct-gucmzkph packages/escher-driver/src/cli.rs:137-203 → 154-237 — changed inside
disposition: holds (`read` keeps the stated order of rules; the new `coverage` role it now also reads is not named by the fact)
@disposition n=5
row: stretched fct-d2yejoup packages/dioxus-native-dom/src/element_id.rs:269-408 → 269-425 — changed inside
disposition: holds (`element_ids`, `placed_ids`, `place_children` and `element_id` keep the author-key, anchored and positional rules and the `None` path)
@disposition n=6
row: held fct-d2yejoup tests/blitz-tests/tests/stand_snapshot.rs:1-5 — refused at 2026-10-25T17-25-04-wrap, not touched since «//! The sna…»
disposition: holds — stands (lines 1-5 are the snapshot stand's module doc)
