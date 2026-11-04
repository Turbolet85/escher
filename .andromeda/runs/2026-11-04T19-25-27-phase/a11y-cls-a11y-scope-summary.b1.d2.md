@answer n=1 entity=a11y-scope-summary#1.1 v=yes
why: names accesskit_xplat as a workspace member, the platform adapter the headless probe must not take in (item 10)
@answer n=2 entity=a11y-scope-summary#1.2 v=yes
why: names the adapter crate the probe must not build, so its test code stays headless (item 10)
@answer n=3 entity=a11y-scope-summary#1.3 v=yes
why: the blitz-dom accessibility tree build is the tree the probe reads role, name, focus and author_id from (items 1, 3, 5)
@answer n=4 entity=a11y-scope-summary#1.4 v=no
why: the select list overlay is neither driven nor read by the probe, which reads no select list (not-needed item 2)
@answer n=5 entity=a11y-scope-summary#1.5 v=no
why: the custom-widget tree hook is a commented-out TODO and the probe builds no custom widget node
@answer n=6 entity=a11y-scope-summary#1.6 v=no
why: blitz-html's feature forwarding is not on the path the probe reads, which runs through dioxus-native-dom to blitz-dom
@answer n=7 entity=a11y-scope-summary#1.7 v=yes
why: the windowed shell's AccessibilityState and adapter are the path the tree refresh is read on, not run (item 5)
@answer n=8 entity=a11y-scope-summary#1.8 v=no
why: the blitz umbrella's default feature is not named by the probe, which reaches the feature through escher-driver (item 10)
@answer n=9 entity=a11y-scope-summary#1.9 v=yes
why: NodeId::as_u64 is the integer form the probe's addressing of nodes by stable element id as author_id relies on (item 1)
@answer n=10 entity=a11y-scope-summary#1.10 v=yes
why: states how the accessibility feature reaches seven_guis through escher-driver with no platform adapter, which the headless run depends on (item 10)
@answer n=11 entity=a11y-scope-summary#1.11 v=yes
why: the served Cells and load pages are the one served task the probe runs beside the lean task, with their Tab orders and keyed rows
@answer n=12 entity=a11y-scope-summary#1.12 v=no
why: the browser app is not the seven_guis stand the probe runs on
@answer n=13 entity=a11y-scope-summary#1.13 v=yes
why: the test crate's accessibility features and accesskit dev-dependency are what the probe's test-code reads of the tree compile against
@answer n=14 entity=a11y-scope-summary#1.14 v=no
why: the input fixture is not part of the seven_guis stand the probe runs on
@answer n=15 entity=a11y-scope-summary#1.15 v=no
why: the capability statement's reach is unchanged by a headless probe, and its leg file count moves only if a file joins the leg, which is S3's own duty
@answer n=16 entity=a11y-scope-summary#2.1 v=no
why: an absence search over the examples states no rule the probe must honour, and the probe names no role or ARIA attribute
@answer n=17 entity=a11y-scope-summary#2.2 v=no
why: an absence search over the s06 files states no rule the probe must honour
@answer n=18 entity=a11y-scope-summary#2.3 v=no
why: an absence search over the s07 files states no rule the probe must honour
@answer n=19 entity=a11y-scope-summary#2.4 v=no
why: an absence search over the s13 files, whose only hits are wpt quarantine lines, states no rule the probe must honour
