# Classified — arch · standard-contracts.b5

## Answers
standard-contracts#24.1 · yes · dioxus_document.rs is a merge surface, so the rules that live there bind: the data-dioxus-id event routing, the four-tier stable ids, the accessibility_tree author_id override, and the snapshot, its text, parts, diff and unkeyed_actionable that the Needs keep and the stable-id checks decide
standard-contracts#25.1 · yes · is_masked_input is the mask predicate the snapshot's value reader calls, and the snapshot is part of the bridge the Needs keep; the password and file-input mask that the snapshot's state proofs assert, which the chunk re-runs, reads through it
standard-contracts#25.2 · yes · dioxus_document.rs:377-382, a merge surface, routes a select's Input and Change to onchange, and the Needs keep the select arms through the same conflicts
standard-contracts#26.1 · yes · only escher-session's main binds: it installs the telemetry sink before its argv, and that install carries the capture checks the Needs keep at 0 needles on the host and mcp role; the example and app CLIs sit in no reached file, and the driver's verbs and exit statuses are Not needed
standard-contracts#26.2 · no · multi_doc is a root example in no reached file and named by no Need, and its Instant::now() sits outside blitz-dom, the one crate the clock invariant covers
standard-contracts#27.1 · no · apps/browser and apps/readme sit in no reached file, the Needs name apps/browser only as a surface a take may carry under the publish-job guard, and its history persistence is Not needed, so no Need names a clause of this entity
standard-contracts#28.1 · no · the WPT runner's outputs, kinds and bucketing are Not needed and wpt/runner sits in no reached file; WPT_COMMIT is a conflict surface, but this entity states no commit rule
