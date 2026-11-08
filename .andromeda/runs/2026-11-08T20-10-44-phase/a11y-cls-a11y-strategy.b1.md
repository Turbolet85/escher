@answer n=1 entity=a11y-strategy#1.1 v=yes
why: the DioxusDocument accessibility_tree override that carries the stable author ids to the platform tree sits in dioxus_document.rs, which the chunk edits, and needs item 3 keeps that author_id path; the shell refresh clause does not bind
@answer n=2 entity=a11y-strategy#1.2 v=no
why: the changed-set marks and the settle that leaves them undrained bind only a check that reads has_changes, and the chunk's coverage read and replay test read no changed set
@answer n=3 entity=a11y-strategy#1.3 v=no
why: unhandled AccessibilityDeactivated and ActionRequested platform events are not read by the coverage command, the actions verb or the replay test
@answer n=4 entity=a11y-strategy#1.4 v=no
why: window focus and bounds forwarding to the adapter is shell plumbing the chunk does not touch
@answer n=5 entity=a11y-strategy#2.1 v=no
why: HTML-AAM role mapping is the tree builder derivation the task's not-needed item 4 names, and the replay compares two runs of the same mapping so the chunk asserts no value of it
@answer n=6 entity=a11y-strategy#2.2 v=no
why: the semantic-page check that only html and body map to Role Unknown is a builder role-mapping check the chunk neither changes nor re-measures, per the task's not-needed item 4
@answer n=7 entity=a11y-strategy#2.3 v=no
why: the write rules for aria-selected, aria-pressed, aria-checked and aria-invalid are ARIA state derivation in the tree builder, named not needed by item 4; the chunk reads the states through the unchanged snapshot model and the replay compares two runs of one derivation
@answer n=8 entity=a11y-strategy#2.4 v=no
why: native checkbox and radio toggled writes are the builder state derivation of item 4, read by the chunk only through the unchanged snapshot
@answer n=9 entity=a11y-strategy#2.5 v=yes
why: the clause that the snapshot's enabled reads the disabled attribute and that aria-disabled moves no snapshot reading binds the driver's disabled refusal of needs item 7; the labelledby, description and other upstream naming and derivation reads do not bind
@answer n=10 entity=a11y-strategy#3.1 v=no
why: the platform module's per-OS adapter choice is the accesskit_xplat adapter, which the chunk builds no part of, per the task's not-needed item 6
@answer n=11 entity=a11y-strategy#3.2 v=no
why: the accesskit_xplat design warning is adapter-internal and the chunk builds no adapter, not-needed item 6
@answer n=12 entity=a11y-strategy#3.3 v=no
why: the empty Android set_focus and the no-op Windows and macOS adapter calls are adapter behaviour the chunk does not reach, not-needed item 6
@answer n=13 entity=a11y-strategy#3.4 v=no
why: the Android Rect import gap is adapter-internal and the chunk builds no adapter, not-needed item 6
@answer n=14 entity=a11y-strategy#3.5 v=no
why: the Android SurfaceView field type is adapter-internal and the chunk builds no adapter, not-needed item 6
@answer n=15 entity=a11y-strategy#4.1 v=yes
why: the new coverage read in the graph module, DioxusDocument::graph and DioxusDocument::actionable_nodes sit under the accessibility feature gate that the chunk's code and tests must keep, and that gate shares the actionable rule the coverage key reads
