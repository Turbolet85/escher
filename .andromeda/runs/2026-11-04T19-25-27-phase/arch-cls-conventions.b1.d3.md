@answer n=1 entity=conventions#1.1 v=yes
why: scope item 18 names the clippy, fmt and rustdoc gates that any probe test or example must pass, so these gate rules bind
@answer n=2 entity=conventions#2.1 v=no
why: the chunk adds no dependency, workspace crate or manifest entry (scope item 16), so the dependency declaration rules rule over nothing it changes
@answer n=3 entity=conventions#3.1 v=no standing=S4
why: the new-module, new-file and example `//!` and `///` doc convention the task lists as S4
@answer n=4 entity=conventions#4.1 v=no
why: its TODO and DO NOT REORDER markers sit in mutation_writer.rs and the launch-path files, which the change does not reach, and no re-read check of dioxus_document.rs or events.rs rules over them
@answer n=5 entity=conventions#5.1 v=no
why: its visibility and naming rules govern the engine crates' internals and re-exports, which the probe does not edit, and the probe adds no public item
@answer n=6 entity=conventions#6.1 v=yes
why: scope item 17 names the accessibility feature that gates the dioxus-native-dom snapshot modules a probe check reads, and this entity states the feature gating of those crates
@answer n=7 entity=conventions#7.1 v=no standing=S6
why: the unsafe and borrow-workaround convention the task lists as S6; the chunk adds no unsafe block and makes no MountedData fix (scope item 11)
@answer n=8 entity=conventions#8.1 v=yes
why: the probe adds an integration test that runs in both layout modes and holds a Session beside the served task, so the session_common list and the one-target-per-file rule must record it
@answer n=9 entity=conventions#9.1 v=no
why: it names escher-driver's moved unit-test modules, a crate the chunk does not reach, since the probe adds no verb, argument, cause or exit
@answer n=10 entity=conventions#9.2 v=no standing=S2
why: the fixtures/mod.rs boot-and-hold convention the task lists as S2; the probe adds no fixture page that a second check reads
@answer n=11 entity=conventions#9.3 v=no
why: it names the row-walk helpers of escher-driver's mcp listing test and the requirement sweep, which the chunk does not reach
@answer n=12 entity=conventions#10.1 v=no standing=S5
why: the Dioxus example convention the task lists as S5; the probe adds no example crate, binary or binary name
@answer n=13 entity=conventions#11.1 v=no
why: the HTML fixture forms of the google and graphite pages are out of scope under not_needed item 13, and the probe reads no HTML fixture
