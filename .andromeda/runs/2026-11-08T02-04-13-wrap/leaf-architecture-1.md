@leaf_group group=architecture-1
@leaf_read leaf=.claude/docs/conventions.md lines=61
last: - Architecture: `.andromeda/architecture.md` · directory lay
@leaf_read leaf=.claude/docs/gotchas.md lines=68
last: - Decisions and rationale: `.andromeda/architecture.md` §Est
@leaf_read leaf=.claude/docs/services/blitz-dom.md lines=44
last: - `.andromeda/architecture.md` · `.claude/docs/stack.md` · `
@leaf_read leaf=.claude/docs/services/blitz-paint.md lines=35
last: - `.andromeda/architecture.md` · `.claude/docs/services/blit
@leaf_read leaf=.claude/docs/services/blitz-shell.md lines=34
last: - `.andromeda/architecture.md` · `.claude/docs/services/blit
@leaf_read leaf=.claude/docs/services/blitz-traits.md lines=33
last: - `.andromeda/architecture.md` · `.claude/docs/services/blit
@leaf_edit n=1 leaf=.claude/docs/conventions.md line=25 class=omission
leaf-text: its `snapshot`, `snapshot_diff`, `snapshot_text` and `actionable` modules)
source: fct-dw3hon6z «gates its `accessibility_tree` override and its `snapshot`, `snapshot_diff`, `snapshot_text`, `actionable` and `graph` modules, its `signal_registry` module ungated»
replace-with: its `snapshot`, `snapshot_diff`, `snapshot_text`, `actionable` and `graph` modules, its `signal_registry` module ungated, so an app registers state whatever its features)
@leaf_edit n=2 leaf=.claude/docs/conventions.md line=58 class=omission
leaf-text: state via `use_signal` / `use_store` / `use_memo`;
source: fct-m3flvoi7 «seven_guis' tasks register each signal through `tasks::use_graph_signal(name, writers, init)`, wrapping `use_signal` and `use_graph_state`»
replace-with: state via `use_signal` / `use_store` / `use_memo` — seven_guis' tasks register each signal through `tasks::use_graph_signal(name, writers, init)` (`use_signal` left to the stand shell's `active` and `stand.rs`'s `linked`), their writers built by the `const fn` `tasks::writes(id, event)` and written `const { &[…] }` or as a `const` item, since a bare `&[writes(..)]` is not promoted to `'static` and errors `E0716`;
@leaf_edit n=3 leaf=.claude/docs/services/blitz-dom.md line=15 class=omission
leaf-text: is what focusability (`ElementData::flush_is_focussable`), the pointer gates and the activation keys read
source: fct-nl4nv5ny «the Enter and Space activation gates and the snapshot's `NodeState.clickable` (true where the bridge's vdom walk holds a current `click` listener and the element is no form control»
replace-with: is what focusability (`ElementData::flush_is_focussable`), the pointer gates, the activation keys and dioxus-native-dom's snapshot `NodeState.clickable` read (a disabled `button` with a `click` listener reading not clickable, a `<div disabled>` with one clickable)
