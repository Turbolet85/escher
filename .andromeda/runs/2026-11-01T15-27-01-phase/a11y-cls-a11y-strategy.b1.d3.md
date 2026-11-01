# Classified — a11y · a11y-strategy.b1

## Answers
a11y-strategy#1.1 · yes · the shell's poll takes the changed set and rebuilds the tree on a non-empty set through `Document::accessibility_tree`, the tree refresh a non-empty set drives that the Needs name, with the wrapper's stable-id override reaching it
a11y-strategy#1.2 · yes · the changed-set contract (what marks `changed_nodes`, what does not, `has_changes()`, settle neither reading nor draining it) is the contract the Needs name; the drain-first clause is S5 and is not carried by this line
a11y-strategy#1.3 · no · the AccessibilityDeactivated and ActionRequested TODOs sit in blitz-shell's application.rs, a file the chunk does not reach and no check reads
a11y-strategy#1.4 · no · window focus and bounds forwarding sits in blitz-shell's accessibility.rs and window.rs, neither reached nor named by the Needs
a11y-strategy#2.1 · yes · the role mapping (links, lists, tables, labels, landmarks off `Role::Unknown`) is asserted by accessibility_roles.rs, one of the `accessibility_*` files the chunk re-runs
a11y-strategy#2.2 · yes · only `<html>` and `<body>` map to `Role::Unknown`, asserted at accessibility_roles.rs:182-198, a re-run `accessibility_*` check that witnesses it
a11y-strategy#2.3 · yes · the three state reads and a select option's `selected` from `is_chosen` bind through the accessibility_states.rs re-run and the Needs' select model in the tree
a11y-strategy#2.4 · yes · a native checkbox or radio's `Toggled` from `checkbox_input_checked()`, pinned by accessibility_states' native-control row in the re-run `accessibility_*` set
a11y-strategy#2.5 · yes · upstream's six reads (aria-labelledby, aria-describedby, aria-description, aria-expanded with the select's `expanded` from its list, aria-disabled, aria-level) are the ones accessibility_label and accessibility_description, upstream's own test files the Needs name, test
a11y-strategy#3.1 · no · the platform module selection in accesskit_xplat is not reached, and the chunk builds no platform adapter
a11y-strategy#3.2 · no · accesskit_xplat's developer note is not reached, and no check reads it
a11y-strategy#3.3 · no · the Android, Windows and macOS adapter no-ops in accesskit_xplat are not reached, and the chunk builds no adapter nor moves any per-OS count they affect
a11y-strategy#3.4 · no · the Android `Rect` import in accesskit_xplat is not reached, and the chunk builds no adapter
a11y-strategy#3.5 · no · the Android `mSurfaceView` field type in accesskit_xplat is not reached, and the chunk builds no adapter
a11y-strategy#4.1 · yes · the `accessibility` feature gating dioxus-native-dom's `accessibility_tree`, snapshot, text, parts, diff and `unkeyed_actionable`, with dioxus-native's forwarding in Cargo.toml, bounds the dioxus_document.rs conflict and the Cargo.toml reach
