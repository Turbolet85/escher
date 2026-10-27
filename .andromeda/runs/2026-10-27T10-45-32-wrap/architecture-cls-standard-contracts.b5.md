# Classified — architecture · standard-contracts.b5

## Answers
standard-contracts#24.1 · yes · stale · dioxus_document.rs is a conflict surface of this take, so the lines this entity cites in it can have moved
standard-contracts#25.1 · no · snapshot.rs, its only cited file, is not reached, and the password and file mask it tests is unchanged
standard-contracts#25.2 · yes · stale · dioxus_document.rs is a conflict surface of this take, so the Input and Change routing cited at :377-382 can have moved
standard-contracts#26.1 · yes · stale · apps/browser/src/main.rs is a merge surface of this take, so the lines cited at :60-76 and :126-130 can have moved
standard-contracts#26.2 · no · examples/multi_doc.rs is not reached, and its mode and iteration arguments, env reads, timing and summary line are unchanged
standard-contracts#27.1 · yes · add-to · the take changed apps/browser/src/tab.rs, and tab lenses now survive a webview props replacement, which this browser contract must carry
standard-contracts#28.1 · no · no WPT runner file is reached, and the only moved item, the upstream WPT pin, is stated by no line of this entity
