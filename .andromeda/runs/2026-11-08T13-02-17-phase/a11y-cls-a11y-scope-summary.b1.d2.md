@answer n=1 entity=a11y-scope-summary#1.1 v=no
why: the workspace accesskit and accesskit_xplat declarations are not changed by a read that takes its tree from the snapshot and builds no platform adapter
@answer n=2 entity=a11y-scope-summary#1.2 v=no
why: accesskit_xplat is the platform adapter crate, which the read builds and reads nothing of (not_needed item 4)
@answer n=3 entity=a11y-scope-summary#1.3 v=yes
why: the read takes role, name and focus from the accessibility tree that build_accessibility_tree builds under the accessibility feature (needs item 1)
@answer n=4 entity=a11y-scope-summary#1.4 v=yes
why: the select list is answered first by hit_with_scrollbar, which decides covered on the shared action_point path and the open list's rows the read must list (needs item 9)
@answer n=5 entity=a11y-scope-summary#1.5 v=no
why: a commented-out TODO hook for custom widgets, which the read does not reach and the scope does not name
@answer n=6 entity=a11y-scope-summary#1.6 v=no
why: blitz-html's feature forwarding is a build edge the chunk's code does not call
@answer n=7 entity=a11y-scope-summary#1.7 v=no
why: the shell's AccessibilityState and its adapter are not built or read by the driver's read (not_needed item 4)
@answer n=8 entity=a11y-scope-summary#1.8 v=no
why: blitz's default accessibility feature forwarding is a build edge the read does not name
@answer n=9 entity=a11y-scope-summary#1.9 v=no
why: a note on NodeId::as_u64 interop, and the read keys nothing on a DOM NodeId, so the entity's rule does not reach the chunk's code
@answer n=10 entity=a11y-scope-summary#1.10 v=yes
why: the driver's accessibility feature on dioxus-native-dom carries author_id through the accessibility_tree override and the snapshot model the read lists ids from (needs items 1, 4, 13)
@answer n=11 entity=a11y-scope-summary#1.11 v=yes
why: the Cells and load pages are stand tasks, so unkeyed_actionable must return nothing on them and their rows are read keyed (needs item 3)
@answer n=12 entity=a11y-scope-summary#1.12 v=no
why: the browser app is neither in the driver's graph nor named by the read, and its accessibility feature is not in its default set
@answer n=13 entity=a11y-scope-summary#1.13 v=yes
why: the read's stand proof and minimal fixture are new checks in blitz-tests, which must build with the accessibility feature and accesskit dev-dependency the snapshot-based proof needs (needs item 12)
@answer n=14 entity=a11y-scope-summary#1.14 v=no
why: the input fixture is an existing example the chunk neither edits nor uses, and its tabindex content states no rule the read must meet
@answer n=15 entity=a11y-scope-summary#1.15 v=yes
why: docs/capabilities.md is reached whole, and its published accessibility reach (SC set and the a11y leg's file count) must match the new read's stand check and any file it adds to the leg
@answer n=16 entity=a11y-scope-summary#2.1 v=no
why: a search record of current absence of ARIA attributes in the examples, stating no rule the chunk's code must meet beyond the scope's own no-second-mapping item
@answer n=17 entity=a11y-scope-summary#2.2 v=no
why: a search record of current absence of accessibility code over a file slice, stating no rule the chunk must meet; the no-second-role-mapping rule is carried by the scope's own items
@answer n=18 entity=a11y-scope-summary#2.3 v=no
why: a search record of current absence of accessibility handling over the s07 slice, stating no rule the chunk must meet
@answer n=19 entity=a11y-scope-summary#2.4 v=no
why: a search record of current absence over the s13 slice whose only hits are a quarantine reason and a quarantined test path, neither reached by the chunk
