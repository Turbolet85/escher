# Working Route — escher-0.3.0

### Epoch 1 — Foundation

[2026-10-26-codebase-health] Codebase health — the lint's findings walk and two table-asserting tests under the complexity ceiling; test functions counted, engine function untouched (v030-01; Epoch 5 audit M1)
[2026-10-27-readme-and-records] README and records — README shows 0.2.0 done, 0.3.0 planned; capabilities statement on the 2026-10-25 run; ledger refs, design stand rows, incubator ignore (v030-02, v030-16)

### Epoch 2 — Engine debts

[2026-10-27-upstream-sync-engine-debts] Upstream sync ahead of engine debts — the founder's chosen take; merge-surface owners kept; CI, a11y, audit green; per-OS tallies, sink allowlist re-read (v030-03)
[2026-10-31-engine-debts] Engine debts — transformed scrolling box, `inert` focus fixup, native checkedness, select limits, select-option overlap, served pages' keyboard and contrast rows; each the founder's fork (v030-04)
[2026-10-31-select-popup] Select popup — a closed select opens a painted list of its options, chosen by pointer and key; the founder's fork (v030-17)

### Epoch 3 — The UI graph

[2026-11-01-upstream-sync-ui-graph] Upstream sync ahead of the UI graph — upstream/main merged, changes mostly additive; CI, a11y leg and audit green, per-OS tallies re-read, sink allowlist re-checked
[2026-11-04-dioxus-runtime-probe] Dioxus runtime probe — components, handlers and state readable without a fork-level change; the reading reaches the founder before the graph (v030-05)
[2026-11-07-mounteddata-buffered-render] MountedData buffered render — a task polled during render reaches the document: upstream `286874cc1`'s buffered render on the Dioxus 0.7 bridge, its test the witness, the hazard pin inverted; every Dioxus document
[2026-11-07-ui-graph] The UI graph — per screen: node, component, handlers, written state, its readers; "reacts to a click" reading; values masked, never logged (v030-05)
[2026-11-08-what-can-i-do-here] What can I do here — actionable nodes' verbs and current refusal causes, from graph and snapshot; actionable-key check, an operator-asked crossing, if it fits (v030-06)
[2026-11-08-handler-coverage] Handler coverage — one command: the handlers no recorded check exercised, read from the UI graph (v030-07)

### Epoch 4 — Determinism and exploration

[2026-11-13-upstream-sync-determinism-and-exploration] Upstream sync ahead of determinism and exploration — upstream/main merged, changes mostly additive; CI, a11y leg and audit green, per-OS tallies re-read, sink allowlist re-checked
[2026-11-13-cognitive-load-corrective] Cognitive load corrective — handle_click (97), execute (32) and graph (50) brought under or toward cognitive 15; no file split, mutator.rs and document.rs kept whole
[2026-11-13-local-files-for-local-documents] Local files for local documents — blitz-net's file: branch serves a file: document alone; a remote page reads no local file (the founder, 2026-11-12, «Как в коде + работа»)
[2026-11-14-seeded-randomness] Seeded randomness — a run's randomness from one seed; two runs of one seed read the same state (v030-13)
[2026-11-14-bundled-fonts] Bundled fonts — a run's fonts bundled from the stand's DejaVu Sans; `fonts-unavailable` kept; every CI OS reads one state, never compared by shot bytes (v030-14)
[2026-11-14-start-in-a-state] Start in a state — a session starts in a given fixture state, a bounded validated argument, on 0.2.0's shared fixtures (v030-08)
[2026-11-14-auto-exploration] Auto-exploration — breadth-first over available actions within a step budget; panics, refusals caught; each path an owner-only replayable check, no shot; navigation a boundary fork (v030-09)
[2026-11-14-remote-pages-local-files] Remote pages and local files in the browser — a remote page's favicon and its navigation to a `file:` URL read no local file

### Epoch 5 — Errors and explanation

[2026-11-15-upstream-sync-errors-and-explanation] Upstream sync ahead of errors and explanation — upstream/main merged, changes mostly additive; CI, a11y leg and audit green, per-OS tallies re-read, sink allowlist re-checked
[2026-11-15-browser-corrective] Browser corrective — the navigation gate's page-URL hand-off (`make_doc_config`) proven by the browser's own tests; every browser `expect_used` allow carries its reason
[2026-11-15-taffy-from-upstream-git] Taffy from upstream's git — Taffy moved to upstream's git pin with #1303 and the left-out chain, measured before planned; the founder's Taffy-only exception to the no-git rule
[2026-11-16-taffy-past-1372] Taffy past #1372 — reversed scroll origins — upstream's Taffy from #1372 on with its scroll-origin change, reconciled with escher's scrolled-box bounds, hit and position readings; the five later bumps behind it
[2026-11-15-session-binary-smoke] Session-binary smoke — the boot-path smoke through `escher-session` reachable by the agent-run contract, so a plan can name a `role=smoke` entry for it
[2026-11-16-errors-with-the-action] Errors with the action — panic, error or handler log it caused, in the answer by node and handler; scrub held; widening the founder's fork (v030-10)
[2026-11-16-error-text-in-the-answer] Error text in the answer — `caused` carries each record's message from every source, framework and engine beside the app, scrubbed and bounded; whether every error boundary can be read, measured first
[2026-11-16-error-field-in-the-answer] Error field in the answer — `caused` reads its own scrub set without `error`, the other content names still hidden, the sink's lines unchanged (v030-10)
[2026-11-17-layout-on-the-carriers] Layout on the carriers — the verb `layout --id` on every ratified carrier, one id in, stable ids, closed words and numbers out, changing nothing (v030-11)
[2026-11-16-layout-explained] Layout explained — why an element sits where it does at its size, from the style and layout chain; new fields per the widening rule (v030-11)
[2026-11-17-a-component-in-isolation] A component in isolation — one component alone with given props, a validated widening, at the stand's viewport and font, across the graph's component boundary (v030-12)

### Epoch 6 — Polish & ship

[2026-11-17-upstream-sync-ahead-of-polish-and-ship] Upstream sync ahead of polish and ship — upstream/main merged, changes mostly additive; CI, a11y leg and audit green, per-OS tallies re-read, sink allowlist re-checked
[2026-11-17-answer-writer-by-family] Answer writer by family — the driver's answer writer split by answer family (caused, layout, snapshot, diff), every carrier's answers unchanged; Blitz's `document.rs` left whole
[2026-11-17-stand-requirement-sweep] Stand requirement sweep — 0.3.0 capabilities proven on stand or fixture inside stand-repeat; floors held, agent-run re-counted; stderr content-free; masks, stand layout held (intent §Principles)
[2026-11-18-a11y-re-read] A11y re-read — SC 2.1.1, 2.4.3, 1.4.3 re-asserted on stand and served pages after engine debts and bundled fonts; checks in the `a11y` leg (a11y-plan §9)
[2026-11-18-capability-statement] Capability statement — what 0.3.0 adds, its stated limits and the README's 0.3.0 done, where Viola's planning reads it (intent §Principles)
[2026-11-18-cold-agent-bar] Cold-agent bar — cold-agent pipe widened past its twelve verbs, span counts with it; fresh agent over 0.3.0's capabilities; founder's bar from 0.2.0's baseline (v030-15)
[2026-11-18-served-error-page] Served error page — a seventh served stand app whose actions panic, return an error and log, so a cold agent meets `caused`: its stand checks, a goal in the stand prompt and a count in the verdict (v030-10's reach)
[2026-11-18-cold-agent-live-run] Cold-agent live run — one fresh agent over 0.3.0's capabilities and the error page, judged against the 5 % wrong-call bar; the README and the capability page state the run as measured, with its reach (v030-15)
