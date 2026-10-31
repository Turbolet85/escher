# Classified — a11y · a11y-strategy.b1

## Answers
a11y-strategy#1.1 · yes · the tree refreshes only on a non-empty changed set, so an open or a close the tree must report has to reach that set (Needs: the engine's changed set)
a11y-strategy#1.2 · yes · a select's changed choice marks the set and apply_select_choice's marking is what the list's choice reads (keyboard.rs is reached); the drain-first check clause is standing S6 and not carried
a11y-strategy#1.3 · no · ActionRequested and AccessibilityDeactivated are shell TODOs; blitz-shell is not reached and the chunk adds no adapter
a11y-strategy#1.4 · no · window focus and bounds forwarding to the adapter in blitz-shell; the chunk reaches no shell or adapter path
a11y-strategy#2.1 · yes · the HTML-AAM role mapping is what gives an optgroup Role::Group, which the group label reading rests on
a11y-strategy#2.2 · no · the semantic page's Unknown check holds no select or optgroup, and the chunk adds no role that could read Unknown
a11y-strategy#2.3 · yes · an option reads its chosen state as `selected` from select::is_chosen, which the list's chosen option and the hidden-content carve-out rely on
a11y-strategy#2.4 · no · native checkbox and radio Toggled reads; the chunk touches no checkable control
a11y-strategy#2.5 · yes · aria-expanded is read as a boolean, one read per attribute, and is the source of the tree's expanded state for the open list
a11y-strategy#3.1 · no · accesskit_xplat platform module selection; the Not needed adapter item, the chunk adds no adapter
a11y-strategy#3.2 · no · accesskit_xplat developer warning; no adapter change in the chunk
a11y-strategy#3.3 · no · Android and Windows/macOS adapter no-ops; the Not needed adapter item
a11y-strategy#3.4 · no · Android adapter Rect import; the Not needed adapter item
a11y-strategy#3.5 · no · Android surface field in the adapter; the Not needed adapter item
a11y-strategy#4.1 · yes · the accessibility feature gates the snapshot model and unkeyed_actionable, which the snapshot.rs option bounds and the options' actionable-key check sit behind
