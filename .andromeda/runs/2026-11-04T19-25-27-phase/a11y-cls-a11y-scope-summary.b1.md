@answer n=1 entity=a11y-scope-summary#1.1 v=yes
why: the probe reads author_id as AccessKit 0.25 types, and accesskit_xplat must stay out of the stand graph (task item 10)
@answer n=2 entity=a11y-scope-summary#1.2 v=yes
why: the platform adapter crate is the one the probe must keep out of every stand binary (task item 10)
@answer n=3 entity=a11y-scope-summary#1.3 v=yes
why: BaseDocument::build_accessibility_tree builds the tree that the probe reads through accessibility_tree and refreshes (task items 1, 3, 5)
@answer n=4 entity=a11y-scope-summary#1.4 v=no
why: the select overlay is not a node of the element graph, and the probe opens no select list (task not_needed 2)
@answer n=5 entity=a11y-scope-summary#1.5 v=no
why: the custom widget tree hook is a commented-out TODO and the probe uses no custom widget
@answer n=6 entity=a11y-scope-summary#1.6 v=no
why: a manifest forwarding of blitz-html's feature that the chunk adds no edge to; the reach it carries is task item 10's, through escher-driver
@answer n=7 entity=a11y-scope-summary#1.7 v=yes
why: the windowed shell's AccessibilityState and adapter carry the tree refresh that task item 5 reads (read, not run)
@answer n=8 entity=a11y-scope-summary#1.8 v=no
why: a manifest default-feature forwarding for blitz; the chunk adds no dependency on blitz and the stand's reach is task item 10's, through escher-driver
@answer n=9 entity=a11y-scope-summary#1.9 v=no
why: a doc note on NodeId::as_u64; the probe's id is the author_id that task item 1 names, not this helper
@answer n=10 entity=a11y-scope-summary#1.10 v=yes
why: the Dioxus accessibility features and the escher-driver edge are what reach the accessibility_tree override and the no-platform-adapter rule (task item 10)
@answer n=11 entity=a11y-scope-summary#1.11 v=yes
why: the load page's Enter and Space on load-fetch and its empty unkeyed_actionable are served-task reads that task items 7 and 2 name
@answer n=12 entity=a11y-scope-summary#1.12 v=no
why: the browser app's feature set lies outside the stand and the probe
@answer n=13 entity=a11y-scope-summary#1.13 v=yes
why: blitz-tests holds the headless accessibility tests the probe runs beside, with the accessibility features and accesskit dev-dependency it needs
@answer n=14 entity=a11y-scope-summary#1.14 v=no
why: the focus input fixture is not the seven_guis stand the probe runs on
@answer n=15 entity=a11y-scope-summary#1.15 v=no
why: the capability statement's reach claims stay unchanged, since the probe asserts no SC and adds no a11y-leg file the scope names
@answer n=16 entity=a11y-scope-summary#2.1 v=no
why: an absence search over existing examples' ARIA attributes, a result rather than a rule; the probe writes no attribute path
@answer n=17 entity=a11y-scope-summary#2.2 v=no
why: an absence search over the s06 files, a result with no rule the probe must meet
@answer n=18 entity=a11y-scope-summary#2.3 v=no
why: an absence search over the s07 files, a result with no rule the probe must meet
@answer n=19 entity=a11y-scope-summary#2.4 v=no
why: an absence search whose only hits are a wpt quarantine reason and path, unrelated to the probe
