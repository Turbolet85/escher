@task_prose part=session
The session added a seeded randomness source to the Dioxus document crate: one seed per run, every component of one document drawing from one shared stream, so two runs of one seed read the same state at every step.
The source is a new module of that crate, compiled UNGATED (outside the accessibility feature), beside the reached-handler record and the signal registry; with no run it falls back to the host's own randomness.
The test harness's build of a Dioxus-backed harness now provides a default run seed (0) as a root context before the document is built, wherever the builder gave none, as it already supplies the one virtual clock; its signature and options are unchanged, and every driver session boots through it.
A new stand check proves one seed reads one state in both layout modes, two seeds differ, a run given no seed reads the default, a second process (a re-exec of the test binary) reads the same state, and a kept check replays its seeded run.
No verb, carrier field, kept-check key, MCP tool field, span, log site, env read or dependency was added; no seed crosses the driver's carriers (the founder's fork answered A, provisional). Page-script Math.random seeding was left out, owned as a residual for next.
@task_file n=1 path=packages/dioxus-native-dom/src/seed.rs
what: the seeded randomness source of the Dioxus document — the default run seed, the shared-stream generator and the hook a component reads it through; logs nothing, compiled outside the accessibility feature
@task_file n=2 path=packages/dioxus-native-dom/src/lib.rs
what: the Dioxus document crate's module list and exports — which modules stand behind the accessibility feature and which stay ungated (the reached-handler record, UI graph state registration, now the seed)
@task_file n=3 path=packages/blitz-test-harness/src/harness.rs
what: the test harness — building a Dioxus-backed harness over a document, the one virtual clock, settle and pump; the boot every driver session and stand check goes through
@task_file n=4 path=tests/blitz-tests/tests/stand_seeded_randomness.rs
what: a stand check, run in both layout modes and in the stand-repeat leg — one seed reads one state, a second process and a kept-check replay read the same
@task_item n=1 part=touches
The rows on the accessibility feature gate (screen-reader integration behind the accessibility feature, snapshot readers behind it, accessibility as a default-on feature): a new module of the Dioxus document crate now compiles outside the feature, beside the reached-handler record and UI graph state registration — an add-to where a row names which modules stay ungated.
@task_item n=2 part=touches
The Dioxus document reached-handler record and UI graph state registration contracts: they share the crate's ungated surface with the new seed module; their own promise is unchanged, a row listing the ungated parts gains the seed.
@task_item n=3 part=touches
Rows over the harness building a Dioxus document (Dioxus render into a mutation buffer, harness settle leaves the changed set, double click read on the document's clock): the harness build now also provides a root seed context before the document is built, beside the one virtual clock; render, settle and clock behaviour unchanged.
@task_item n=4 part=touches
Stand checks repeated three rounds (CI integration): a new stand check joins the stand-repeat leg by its file glob; stand check files 54 to 55 — a touch only where a row's cell holds that count.
@task_item n=5 part=touches
The keyed field on the carriers and the driver schema rows: no seed field was added to any carrier, verb or kept-check key — a seed on the carriers is absent, provisionally, pending the founder's word.
@task_item n=6 part=touches
Every driver session (command line, host, MCP, coverage, replay) now boots with the default run seed; handler coverage answers, MCP tool results and kept-check replay read nothing of it and are unchanged.
@task_item n=7 part=touches
The obs log format described under the assertion harness contract: the seed module and the harness's provide add no log site, span, field or env read.
@task_item n=8 part=touches
The new check's fixture carries a keyed roll button (id roll), read through snapshot and click; the actionable-key and interactive-nodes-named rules hold over it unchanged.
@task_item n=1 part=untouched
ARIA role mapping, role-from-attribute rules and accessible name precedence: no role, name or ARIA read changed.
@task_item n=2 part=untouched
Accessibility tree states, hidden and inert content and the AccessKit adapter rows: no tree building or adapter code changed.
@task_item n=3 part=untouched
Keyboard navigation contracts (range input, single and multiple select lists, cells grid, CRUD list, Tab order, keyboard activation): no key or focus handling changed.
@task_item n=4 part=untouched
Visual design verification — contrast lint, WCAG thresholds, stand accent colour, select list colours and motion: nothing visual changed.
@task_item n=5 part=untouched
Snapshot fields, snapshot text form, snapshot diff and the actions answer: no field or token added or moved.
@task_item n=6 part=untouched
Cognitive accessibility, accessibility tier and reach, and the a11y CI leg and gate: none reached.
@task_item n=7 part=untouched
Rows using seed in another sense (a range input's value, the stylist device seeded from the root): not this source.
