# Classified — tests · test-scope-summary.b2

## Answers
test-scope-summary#16.1 · yes · blitz-vibey-script's dom.rs and selection.rs tests are the ones the chunk names to run on the URLSearchParams result, where bootstrap.rs and events.rs conflict
test-scope-summary#17.1 · yes · the dioxus-native-dom unit tests of the element id, snapshot text, parts and diff, actionables and bridge that the chunk names, and dioxus_document.rs's test module, a conflict surface
test-scope-summary#17.2 · yes · input_focused_on_mount_enables_ime lives in dioxus_document.rs's test module, a conflict surface, so the resolution there must keep it holding
test-scope-summary#17.3 · yes · names left-out commit 286874cc1, which the chunk re-reads commit by commit in the left-out delta, and its absent test would arrive in dioxus_document.rs, a conflict surface, on a take
test-scope-summary#18.1 · no · dioxus-native and stylo_taffy are not reached by the chunk and the scope names neither, so their absence of tests states no rule the chunk's code or tests must honour
