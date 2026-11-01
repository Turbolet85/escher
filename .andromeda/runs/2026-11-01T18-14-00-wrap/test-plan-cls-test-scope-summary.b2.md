# Classified — test-plan · test-scope-summary.b2

## Answers
test-scope-summary#16.1 · yes · add-to · the new tests/url_search_params.rs joins dom.rs, selection.rs and preact.rs in blitz-vibey-script's test inventory (Touches, §1)
test-scope-summary#17.1 · yes · stale · dioxus_document.rs is a merge surface and this entity cites its test lines (:448-484, :518-835), which the merge can move
test-scope-summary#17.2 · yes · stale · dioxus_document.rs is a merge surface and this entity cites the IME test at :745-779, which the merge can move
test-scope-summary#17.3 · yes · add-to · F7: upstream's element_removed_before_its_mounted_event_is_flushed is left out (reads 1 mounted event where it asserts 0), so the 6ef4b532d guard has no test
test-scope-summary#18.1 · no · the claim covers the dioxus-native crate and stylo_taffy, and no reached file lies in either; the reached dioxus_document.rs is dioxus-native-dom
