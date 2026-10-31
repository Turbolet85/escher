# Classified — test-plan · test-scope-summary.b2

## Answers
test-scope-summary#14.1 · yes · stale · the session edits packages/dioxus-native-dom/src/snapshot.rs at lines 147, 152-155, 245-246 and 250-255, so the snapshot.rs ranges cited here (942-1024, 718-755, 859-912, 626-627, 670-714, 295-1025) shift; no dioxus-native-dom test count is stated as moved
test-scope-summary#14.2 · no · the IME-on-mount test sits in dioxus_document.rs, which the session does not reach, and nothing in the session touches IME
test-scope-summary#14.3 · no · the absent upstream test is named against dioxus_document.rs, which the session does not reach, and the session adds no such test
test-scope-summary#15.1 · no · the session reaches no file of the dioxus-native or stylo_taffy crates this absence names, and the report adds no test to dioxus-native-dom
