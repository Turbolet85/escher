# Classified — tests · ci-integration.b1

## Answers
ci-integration#1.1 · yes · the chunk's gates are these legs (fast, doc, audit, msrv, coverage, stand-repeat, a11y), every cargo leg run --locked, and fork CI must be green on every job
ci-integration#2.1 · no · CI caching is named in the task's Not needed list as untouched by the sync, and the chunk reads no cache timing
ci-integration#3.1 · no · apt-install.sh's bounded package installs are named in the task's Not needed list as untouched by the sync, so no install step the chunk must honour changes
ci-integration#4.1 · yes · the task's needs name the engine-features-by-runner rule (tracing and writing-mode on only in the workspace build), so the workspace green and the per-package green are separate readings and sink captures count lines by target
ci-integration#5.1 · yes · the task needs the per-OS tallies re-read by the recorded method (linux over macOS by the doc-test lines, macOS over windows by the unix-gated binaries, ios and android build only) with each moved count explained
ci-integration#6.1 · yes · the chunk's local gates run --locked and a re-run workspace test count is read against the recorded 1098 passed and 12 ignored, while the 2026-10-05 timings bind nothing
ci-integration#7.1 · yes · the task needs the coverage leg with its per-crate floors (dioxus-native-dom 83, escher-driver 97, blitz-test-harness 90) and the keyboard_compat.rs exclusion, which drops only if the take carries 286874cc1
