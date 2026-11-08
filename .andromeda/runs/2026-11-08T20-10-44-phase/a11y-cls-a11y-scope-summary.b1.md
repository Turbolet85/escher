@answer n=1 entity=a11y-scope-summary#1.1 v=no
why: the workspace manifest entry for accesskit and accesskit_xplat; the chunk adds no dependency and states no rule over the crate version.
@answer n=2 entity=a11y-scope-summary#1.2 v=no
why: names the platform adapter crate; the chunk builds no adapter (task item 6, not needed).
@answer n=3 entity=a11y-scope-summary#1.3 v=yes
why: BaseDocument::build_accessibility_tree builds the AccessKit tree under the accessibility feature, and the snapshot model reads role, name, focus and states through that tree, so the replayed actions answer reads its output.
@answer n=4 entity=a11y-scope-summary#1.4 v=no
why: the select list is an engine overlay outside the DOM graph; the chunk's graph and actions read no overlay and change no select behaviour.
@answer n=5 entity=a11y-scope-summary#1.5 v=no
why: the custom widget tree hook is a commented-out TODO the scope does not name; the chunk reads no custom widget.
@answer n=6 entity=a11y-scope-summary#1.6 v=no
why: blitz-html's feature forwarding is a manifest fact the chunk does not change.
@answer n=7 entity=a11y-scope-summary#1.7 v=no
why: the shell's adapter state belongs to the windowed shell; the chunk builds no adapter.
@answer n=8 entity=a11y-scope-summary#1.8 v=no
why: blitz's feature forwarding is a manifest fact the chunk does not change.
@answer n=9 entity=a11y-scope-summary#1.9 v=yes
why: names NodeId::as_u64 as the AccessKit integer-id interop, the call the coverage key must avoid, since the coverage keys on the stable graph id and never on an engine NodeId.
@answer n=10 entity=a11y-scope-summary#1.10 v=yes
why: the dioxus-native-dom accessibility feature reaches escher-driver's snapshot and the accessibility_tree override carrying author_id, which the coverage read and the replay read.
@answer n=11 entity=a11y-scope-summary#1.11 v=yes
why: the served pages' actionables are keyed and their stand checks read those keys, so the coverage list's handlers on the stand are keyed on them.
@answer n=12 entity=a11y-scope-summary#1.12 v=no
why: the browser app is not on the stand and the chunk touches no browser code.
@answer n=13 entity=a11y-scope-summary#1.13 v=no
why: blitz-tests' feature and dev-dependency are manifest facts; the replay test sits in the driver and the scope names no test crate.
@answer n=14 entity=a11y-scope-summary#1.14 v=no
why: the input fixture exercises focusable divs, and the chunk changes no focus.
@answer n=15 entity=a11y-scope-summary#1.15 v=no
why: the capability statement publishes reach by hand, and the chunk's read-only coverage names no reach or limit in a document the scope names.
@answer n=16 entity=a11y-scope-summary#2.1 v=no
why: an absence search over the examples' ARIA attributes; the chunk adds no example and edits no example source.
@answer n=17 entity=a11y-scope-summary#2.2 v=no
why: an absence search over the s06 slice's accessibility words; the search slice the entity names contains none of the chunk's reached files.
@answer n=18 entity=a11y-scope-summary#2.3 v=no
why: an absence search over the s07 slice's accessibility words; the search slice the entity names contains none of the chunk's reached files.
@answer n=19 entity=a11y-scope-summary#2.4 v=no
why: an absence search over the s13 files' ARIA, role and focus words, whose only hits are quarantined wpt paths; the chunk reaches none of them.
