# Classified — tests · test-scope-summary.b2

## Answers
test-scope-summary#16.1 · yes · the script tests dom.rs and selection.rs run on the result of the bootstrap.rs and events.rs conflict resolution, as the Needs name
test-scope-summary#17.1 · yes · the dioxus-native-dom unit tests of element ids, snapshot text, parts and diff, actionables, and the bridge and document events bind, since the Needs name that crate as most modified and dioxus_document.rs is a conflict surface
test-scope-summary#17.2 · yes · upstream's input_focused_on_mount_enables_ime sits in dioxus_document.rs, a conflict surface, so a resolution there must keep IME enabled at editor construction
test-scope-summary#17.3 · yes · commit 286874cc1 is left out of the fork, and its upstream test lands in the conflict surface dioxus_document.rs if the take carries that commit, which the left-out delta re-read covers
test-scope-summary#18.1 · no · the absence of tests in stylo_taffy is a gap the chunk does not fill; the scope names no stylo_taffy test or behaviour, and the moved Stylo and Taffy are proven through the stand checks
