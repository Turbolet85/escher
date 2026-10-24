# Classified — a11y · screen-reader-support.b1

## Answers
screen-reader-support#1.1 · yes · the TextRun labelling of a parent is the name source of the tree builder in accessibility.rs, which Needs re-proves, and stand_diff's text-run exclusion reads it
screen-reader-support#1.2 · yes · the merged name reads (aria-label, with aria-labelledby outranking it) are the accessibility.rs name reads Needs re-proves by accessibility_label and accessibility_description
screen-reader-support#1.3 · yes · the label association and upstream's aria-labelledby loop in accessibility.rs and document.rs name the stand's inputs, which Needs re-proves
screen-reader-support#1.4 · yes · the select option nodes kept under display none are the select option exception Needs re-proves by accessibility_hidden, and stand_act_select reads them
screen-reader-support#2.1 · yes · author_id on element nodes through DioxusDocument's accessibility_tree and the adapter-tree equality are the stand_accessibility_ids and element-id readings Needs names
screen-reader-support#3.1 · yes · the snapshot's role, name, focus, states and password mask read the tree Needs re-proves, through the snapshot checks
screen-reader-support#4.1 · yes · the actionable-key rule, unkeyed_actionable's returned tag, role and remedy, and the non-lean cells task are what stand_actionable_keys and stand_non_lean_tasks read under Needs
screen-reader-support#5.1 · yes · the CRUD listbox and option aria-selected, the flight booker's aria-pressed and the date inputs' aria-invalid are the stand state cues Needs re-reads on the merged stand
screen-reader-support#6.1 · yes · the tree builder writes each element's tag at accessibility.rs:241, the builder Needs re-syncs, and unkeyed_actionable returns that tag (#4.1)
screen-reader-support#6.2 · yes · the Window root at u64::MAX and the focused node as tree focus are what stand_accessibility_ids compares and the snapshot's focus reads
screen-reader-support#6.3 · yes · accessibility_roles is re-proven under Needs, and its goal of roles that AT can navigate by is the purpose that re-proof must keep
screen-reader-support#7.1 · yes · accesskit_xplat's routing of initial-tree, action and deactivation requests is the adapter behaviour the tokio update must not change
screen-reader-support#8.1 · yes · blitz-shell's adapter creation and event forwarding is the adapter path the tokio update must not change
screen-reader-support#8.2 · yes · the accessibility feature gating screen-reader exposure through blitz-shell and the Dioxus crates is the feature wiring Needs keeps reaching the stand through the Dioxus 0.8 manifest move
screen-reader-support#9.1 · no · language and direction are Not needed, and lang_attribute is not an entry in Needs
screen-reader-support#9.2 · no · the dir attribute's bidi-isolation styling is direction, which Not needed excludes
screen-reader-support#9.3 · no · lang on the examples/assets fixtures is language and fixture markup, both Not needed
screen-reader-support#9.4 · no · lang on the example pages is language and example-page markup, both Not needed
screen-reader-support#10.1 · no · an observed-absent record of lang in the s02 fixtures, which Not needed says no measurement in this chunk resolves
screen-reader-support#11.1 · no · the sr-only tooltip in an examples/assets fixture is a text alternative there, which Not needed excludes
screen-reader-support#11.2 · no · descriptive aria-labels in an examples/assets fixture are text alternatives there, which Not needed excludes
screen-reader-support#11.3 · no · aria-hidden and aria-atomic in an examples/assets fixture are that fixture's markup, and Not needed excludes text alternatives there
screen-reader-support#11.4 · no · aria-hidden on the servo-new fixture's burger spans is that fixture's markup, which Not needed excludes
screen-reader-support#11.5 · no · alt text on examples/assets fixture images is a text alternative there, which Not needed excludes
screen-reader-support#11.6 · no · a missing alt on an examples/assets fixture image is a text alternative there, which Not needed excludes
