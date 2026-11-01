# Classified — a11y · a11y-strategy.b1

## Answers
a11y-strategy#1.1 · yes · the shell's poll-time tree refresh on a non-empty changed set and the `accessibility_tree` trait path, which the task's "tree refresh a non-empty set drives" names
a11y-strategy#1.2 · yes · the changed-set contract (what marks `changed_nodes`, what does not, `has_changes()`), which the task names as unchanged through the `mutator.rs` conflict; its drain-first clause is S5's own rule, carried by standing
a11y-strategy#1.3 · no · the AccessibilityDeactivated and ActionRequested TODOs sit in blitz-shell's `application.rs`, which the chunk does not reach and no chunk check reads
a11y-strategy#1.4 · no · window focus and bounds forwarding sit in unreached blitz-shell files; the chunk's keyboard checks read the engine's focus, not window focus
a11y-strategy#2.1 · yes · the HTML-AAM role mapping that `accessibility_roles.rs` asserts, a re-read check under `accessibility_*`
a11y-strategy#2.2 · yes · only `<html>` and `<body>` map to `Role::Unknown`, asserted by `accessibility_roles.rs`, a file the chunk re-reads
a11y-strategy#2.3 · yes · the three ARIA state reads and the per-role toggle, which `accessibility_states.rs` asserts and the chunk re-reads
a11y-strategy#2.4 · yes · the native checkbox and radio `Toggled` write, pinned by `accessibility_states`' `a_click_and_a_space_move_a_native_control_in_the_tree`, a check the chunk re-reads
a11y-strategy#2.5 · yes · upstream's six reads (`aria-labelledby`, `aria-describedby`, `aria-description`, `aria-expanded`, `aria-disabled`, `aria-level`), which `accessibility_label` and `accessibility_description` assert; both files are re-read and upstream's own under a take that carries them, and the engine-debts merge surfaces are re-read row by row
a11y-strategy#3.1 · no · platform adapter selection in unreached `accesskit_xplat`; no stand binary builds a platform adapter and this chunk adds none
a11y-strategy#3.2 · no · `accesskit_xplat`'s developer warning, a comment in a crate the chunk does not reach
a11y-strategy#3.3 · no · Android, Windows and macOS adapter no-ops in unreached `accesskit_xplat`, built by no stand binary
a11y-strategy#3.4 · no · Android adapter `Rect` imports in unreached `accesskit_xplat`, built by no stand binary
a11y-strategy#3.5 · no · Android adapter `mSurfaceView` field type in unreached `accesskit_xplat`, built by no stand binary
a11y-strategy#4.1 · yes · the `accessibility` feature gating of dioxus-native-dom's `accessibility_tree` override, snapshot, `to_text`, `text_parts`, `diff` and `unkeyed_actionable`, which the `dioxus_document.rs` conflict must keep, and the feature forwarding in the reached `Cargo.toml`
