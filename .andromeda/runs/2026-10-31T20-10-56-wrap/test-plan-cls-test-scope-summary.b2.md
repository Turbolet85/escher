# Classified — test-plan · test-scope-summary.b2

## Answers
test-scope-summary#14.1 · yes · stale · the session adds lines to dioxus-native-dom's snapshot.rs at 147, 152-155, 245-246 and 250-255, so the cited snapshot.rs ranges (626-627, 670-714, 718-755, 859-912, 942-1024, 295-1025) shift while the 82-test count is not moved
test-scope-summary#14.2 · no · the IME-on-mount test sits in dioxus-native-dom's dioxus_document.rs, which the session does not reach
test-scope-summary#14.3 · no · the absent upstream test concerns dioxus-native-dom's dioxus_document.rs test module, which the session does not reach or add a test to
test-scope-summary#15.1 · no · the session adds no test to dioxus-native or stylo_taffy, and its reached snapshot.rs edits are only shifted citations, so the no-test search still holds
