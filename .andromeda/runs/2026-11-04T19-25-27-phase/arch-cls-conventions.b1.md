@answer n=1 entity=conventions#1.1 v=yes
why: scope item 18 names the clippy, fmt and rustdoc gates that probe test code must pass
@answer n=2 entity=conventions#2.1 v=no
why: the probe adds no workspace crate and no dependency, so the manifest rules rule over nothing it edits
@answer n=3 entity=conventions#3.1 v=no standing=S4
why: new modules and files open with //! docs, public items carry /// and an example opens with a //! comment
@answer n=4 entity=conventions#4.1 v=no
why: its TODO, FIXME and DO NOT REORDER locations sit in files the chunk does not reach, and the probe adds no marker
@answer n=5 entity=conventions#5.1 v=no
why: the probe adds no crate internal, public item or re-export, so the visibility and naming rules rule over nothing it changes
@answer n=6 entity=conventions#6.1 v=yes
why: scope item 17 names the accessibility feature that gates dioxus-native-dom's snapshot modules, which this entity's feature-gating rule and dioxus-native-dom feature list carry
@answer n=7 entity=conventions#7.1 v=no standing=S6
why: unsafe blocks carry SAFETY comments and borrow conflicts are worked around by taking a child list or layout out and restoring it
@answer n=8 entity=conventions#8.1 v=yes
why: scope item 8 names both layout modes that the probe's stand checks run in, and a served-task check that holds a Session joins the session_common list this entity names
@answer n=9 entity=conventions#9.1 v=no
why: its escher-driver unit-test modules sit in a crate the probe does not edit under scope items 14 and 16
@answer n=10 entity=conventions#9.2 v=no standing=S2
why: tests/blitz-tests/tests/fixtures/mod.rs states the fixture boot and hold once, and a page with one reader stays in its check's file
@answer n=11 entity=conventions#9.3 v=no
why: its row-walk helpers belong to the escher-driver MCP listing test and the blitz-tests requirement-sweep test, neither edited nor named by the scope
@answer n=12 entity=conventions#10.1 v=no standing=S5
why: a Dioxus example declares #[component] components with signal and store hooks, its CSS in const raw strings and fails with unwrap, expect or panic
@answer n=13 entity=conventions#11.1 v=no
why: its HTML fixture pages sit under examples/assets, which the probe does not reach, and the probe reads no HTML page
