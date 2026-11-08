@answer n=1 entity=existing-scopes#1.1 v=no
why: the platform AccessKit adapter, which the chunk does not touch (the accessibility adapter is out of scope)
@answer n=2 entity=existing-scopes#1.2 v=no
why: the debug timer, and the read takes no timing and changes none
@answer n=3 entity=existing-scopes#1.3 v=no
why: a module list only, and the never-a-NodeId boundary comes from the task's own needs, not from this listing
@answer n=4 entity=existing-scopes#1.4 v=no standing=S1
why: states blitz-dom's cfg(test) child modules in files of their own, the test-home convention every new test owes
@answer n=5 entity=existing-scopes#1.5 v=no
why: stylesheet editing and getComputedStyle, and the read reads no style and the chunk changes none
@answer n=6 entity=existing-scopes#1.6 v=no
why: form owner reset, iframe loading and query selectors, none of which the read uses (the owner is a select's, not a form's)
@answer n=7 entity=existing-scopes#1.7 v=yes
why: hit testing, the open select list answered first in hit_with_scrollbar and the clipping hit that the shared refusal path must keep (task item 6)
@answer n=8 entity=existing-scopes#1.8 v=no
why: accessibility tree building and taffy debug printing, which the read does not run
@answer n=9 entity=existing-scopes#1.9 v=no
why: the event submodules that dispatch input, and the read dispatches nothing and runs no step
@answer n=10 entity=existing-scopes#1.10 v=yes
why: the visible-region reader in scrolling.rs is the off-screen source the read and the executor share (task item 6, seeing read through visible_region)
@answer n=11 entity=existing-scopes#1.11 v=no
why: Servo styling, Device and Stylo conversions, and the chunk changes no style and the read computes none
@answer n=12 entity=existing-scopes#1.12 v=no
why: versioned node storage and traversal, and the read is keyed on no NodeId and walks through the UI graph instead
@answer n=13 entity=existing-scopes#1.13 v=no
why: box construction, damage, inline and paint layout passes, and the chunk changes no layout behaviour
@answer n=14 entity=existing-scopes#1.14 v=yes
why: ElementData::is_disabled and option_disabled, the select rules and the open-list geometry that keep a closed single select's option refused covered (task items 6 and 7)
@answer n=15 entity=existing-scopes#1.15 v=no
why: HTML parsing through a TreeSink, and the read parses no markup and the stand is built from Dioxus
@answer n=16 entity=existing-scopes#1.16 v=no
why: HTTP, filesystem and data-URI fetching, and the read makes no fetch
@answer n=17 entity=existing-scopes#1.17 v=no
why: painting, including the select list paint, and the read paints nothing and takes no shot
@answer n=18 entity=existing-scopes#1.18 v=no
why: the Boa script runtime and its tests, and the read touches no script
@answer n=19 entity=existing-scopes#1.19 v=no
why: the shell's application, window and event modules, and the read runs headless through the driver session
@answer n=20 entity=existing-scopes#1.20 v=no
why: re-exports of the blitz sub-crates, and the chunk adds no import through this crate
@answer n=21 entity=existing-scopes#1.21 v=no
why: the Stylo-to-Taffy wrappers, and the chunk changes no layout conversion
@answer n=22 entity=existing-scopes#1.22 v=no
why: a module list and a Cargo dependency edge, and the chunk reads the bridge through dioxus-native-dom and changes no dependency
@answer n=23 entity=existing-scopes#1.23 v=yes
why: the graph, snapshot and actionable modules the chunk reaches, with GraphNode and its optional owner, virtual_node_id's reserved owner form, UnkeyedActionable and the accessibility gate they sit under
@answer n=24 entity=existing-scopes#1.24 v=no
why: the harness, input, inspect, settle, screenshot and contrast modules, and the read steps nothing and takes no shot
@answer n=25 entity=existing-scopes#1.25 v=yes
why: the target allowlist and field scrub that the command span's printed line passes through (task item 9, the command span records verb and cause alone)
