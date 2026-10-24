# Classified — architecture · conventions.b1

## Answers
conventions#1.1 · yes · stale · the session moved layout/construct.rs lines after 6, so the lint allow cited at construct.rs:571 is off its line
conventions#2.1 · no · the root Cargo.toml is byte-identical and the session's manifest edits (browser features, WPT runner dependency) sit outside the lines this entry cites
conventions#3.1 · yes · stale · the session moved stylo.rs lines after 29 and every line of wpt/runner/src/main.rs (+3), so the citations at stylo.rs:991, :1021-1024, :1103 and main.rs:56-63 are off
conventions#4.1 · yes · stale · the session moved lines in stylo.rs, construct.rs and dioxus-native lib.rs and turned layout/inline.rs into a directory, so the TODO citations at stylo.rs:1216, construct.rs:427, :612, lib.rs:107, inline.rs:278, :501 are off or gone
conventions#5.1 · no · no visibility item cited here sits in a file the session changed, and its new pub fields and pub(crate) items are not among the cited ones
conventions#6.1 · yes · stale · the session shifted mutator.rs (after 375), resolve.rs (after 29) and construct.rs (after 6), moving the tracing fallbacks at mutator.rs:1321-1322, resolve.rs:44-45 and construct.rs:480-488
conventions#7.1 · yes · stale · the session shifted stylo.rs, construct.rs and node.rs lines and turned layout/inline.rs into a directory, so the unsafe and borrow citations at stylo.rs:751-758, :1326, :1398-1403, construct.rs:264-273, node.rs:138-180, :1503-1504 and inline.rs:245-251 are off or gone
conventions#8.1 · yes · stale · the session shifted mutator.rs (after 375), node/element.rs (after 45), node/node.rs (after 125) and wpt/runner/src/main.rs (+3), moving the test citations at mutator.rs:1512-1513, element.rs:1054-1188, node.rs:2054-2140 and main.rs:618-622
conventions#9.1 · no · escher-driver is byte-unchanged and the session's one new test is an integration target, so the three escher-driver test modules stand as cited
conventions#9.2 · no · fixtures/mod.rs and the fixture pages it states are unchanged, the session adding only a check file of its own
conventions#10.1 · no · the session changes none of the binaries, Dioxus examples or browser sources this entry cites, and multi_doc.rs is a root example outside its list
conventions#11.1 · no · no examples/assets file is changed or cited by the session
