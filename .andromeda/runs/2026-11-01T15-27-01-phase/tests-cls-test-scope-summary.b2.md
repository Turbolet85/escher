# Classified — tests · test-scope-summary.b2

## Answers
test-scope-summary#16.1 · yes · dom.rs and selection.rs of blitz-vibey-script bind, since the scope names them as running on the result of 91ef65e21 where bootstrap.rs and events.rs conflict
test-scope-summary#17.1 · yes · dioxus-native-dom's unit tests on element ids, the snapshot with its text, parts and diff, unkeyed actionables and the bridge and document events bind, as the Needs name them and dioxus_document.rs is a conflict surface
test-scope-summary#17.2 · yes · the IME-on-mount test sits in dioxus_document.rs's test module, a conflict surface, so its rule binds on the resolution
test-scope-summary#17.3 · yes · the absence of upstream's task_polled_during_render_can_access_document, commit 286874cc1 left out, is a left-out-delta reading in dioxus_document.rs, a conflict surface, which the Needs require to be re-read commit by commit
test-scope-summary#18.1 · no · an observed absence of tests in dioxus-native and stylo_taffy states no rule the chunk must meet, and neither crate is a reached file or a named coverage floor
