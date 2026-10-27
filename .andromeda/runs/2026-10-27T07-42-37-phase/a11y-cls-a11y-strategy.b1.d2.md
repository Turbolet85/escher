# Classified — a11y · a11y-strategy.b1

## Answers
a11y-strategy#1.1 · yes · the shell poll drains the changed set and rebuilds the platform tree through `Document::accessibility_tree`, a Need that survives the redraw routing in window.rs and lib.rs (merge surfaces)
a11y-strategy#1.2 · yes · what marks the changed set and what does not, drained by `take_changed_nodes`, is a Need that survives the mutator.rs conflict, read through document.rs (merge surface) and re-run by stand_settle
a11y-strategy#1.3 · no · the two events' TODOs sit in application.rs, which no reached file names, and no Need reads either event; the chunk fixes no debt
a11y-strategy#1.4 · yes · the window-event forwarding at window.rs:585-588 lives in a merge surface and is part of the adapter wiring the Needs keep
a11y-strategy#2.1 · yes · the HTML-AAM role mapping is what accessibility_roles re-reads and re-asserts after the take
a11y-strategy#2.2 · yes · the assertion at accessibility_roles.rs:182-198 is the re-read check's own rule, re-run after the take
a11y-strategy#2.3 · yes · the builder's aria-selected, aria-pressed, aria-checked and aria-invalid reads are a Need re-asserted by accessibility_states and by the snapshot's three states
a11y-strategy#2.4 · yes · the merged upstream reads and the label rules are a Need re-asserted by accessibility_label and accessibility_description
a11y-strategy#3.1 · no · accesskit_xplat's platform selection is a component the chunk uses and leaves unchanged; no reached file is in that crate and no Need names the selection
a11y-strategy#3.2 · no · a doc-comment note on accesskit_xplat's approach; no reached file is in that crate and no Need names it
a11y-strategy#3.3 · no · the empty and no-op set_focus and set_window_bounds belong to an unreached component, and no Need names those calls
a11y-strategy#3.4 · no · an Android compile detail in an unreached adapter; the Needs name no Android build
a11y-strategy#3.5 · no · an Android field type in an unreached adapter; the Needs name no Android path
a11y-strategy#4.1 · yes · the accessibility feature's gate on the Dioxus accessibility_tree override and snapshot family is a Need through dioxus_document.rs (merge surface), and its reach through the manifests is a Need
