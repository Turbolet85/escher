# Classified — a11y · a11y-strategy.b1

## Answers
a11y-strategy#1.1 · yes · the poll-time refresh through `take_changed_nodes` and the `Document::accessibility_tree` override is the changed-set contract and tree refresh the Needs name, through the `mutator.rs` and `dioxus_document.rs` conflicts
a11y-strategy#1.2 · yes · what marks `changed_nodes`, what does not, the select list's marks and `has_changes()` are the changed-set contract and select model the Needs name, through the `mutator.rs` and `node.rs` conflicts
a11y-strategy#1.3 · no · the ActionRequested and AccessibilityDeactivated TODOs sit in blitz-shell's `application.rs` (84-89), no reached file, and no need or check names action requests
a11y-strategy#1.4 · no · window focus and bounds forwarding in blitz-shell's `accessibility.rs` and `window.rs` is not a reached file, and no stand check asserts those values
a11y-strategy#2.1 · yes · the role mapping is what the re-read `accessibility_roles.rs` under `accessibility_*` witnesses, so that check's rule is re-run on the take
a11y-strategy#2.2 · yes · the semantic page's only-`html`-and-`body` `Role::Unknown` assertion is a row the re-read `accessibility_roles.rs` holds
a11y-strategy#2.3 · yes · the three states on every element, the option's `selected` from `select::is_chosen` and the per-role toggles are what the re-read `accessibility_states.rs` checks, with the select model's option state under `select_popup`
a11y-strategy#2.4 · yes · a native checkbox's or radio's `Toggled` from `checkbox_input_checked()` is what `accessibility_states`' Space pin reads, and the tier-0 target needs Space activation
a11y-strategy#2.5 · yes · upstream's six reads (`aria-labelledby`, `aria-describedby`, `aria-description`, `aria-expanded`, `aria-disabled`, `aria-level`) are what the re-read `accessibility_label.rs` and `accessibility_description.rs` witness, and `aria-expanded` on a single select writes the select model's `expanded`
a11y-strategy#3.1 · no · the platform adapter selection in accesskit_xplat's `mod.rs` is not a reached file, and the chunk builds and adds no platform adapter
a11y-strategy#3.2 · no · a comment in accesskit_xplat's `lib.rs` that no reached file or check reads
a11y-strategy#3.3 · no · the empty or no-op adapter methods in accesskit_xplat are not a reached file, and no need or check names those methods
a11y-strategy#3.4 · no · the Android `Rect` import and parameters are in accesskit_xplat's `android.rs`, not a reached file, and no need or check names Android
a11y-strategy#3.5 · no · the Android `mSurfaceView` field is in accesskit_xplat's `android.rs`, not a reached file, and no need or check names it
a11y-strategy#4.1 · yes · the `accessibility` feature gating dioxus-native-dom's `accessibility_tree` override and snapshot model, forwarded by dioxus-native's `Cargo.toml`, is the surface the stable-id and snapshot checks use, and `Cargo.toml` is reached
