@task_prose part=chunk
Engine debts (capability v040-06): four residuals of 0.1.0-0.3.0, each researched with every option's cost and put to the founder as one forks file, then either fixed or kept as a residual by his word with his reason.
Item 1: a control hidden by the hidden attribute, display:none or CSS alone keeps focus and takes keys, because the focus fixup in the mutator's flush reads inert alone; the fork is a focus fixup for a control that no longer renders (focus given up after the next layout) or keeping the limit.
Item 2: a task polled during a Dioxus buffered render sees the document as the previous render left it, so MountedData set_focus on a node of that same render does nothing; the fork is a deferred or replayed act or keeping the limit.
Item 3: a page script's Math.random is the host's, since the run's seeded source reaches Dioxus components alone; the fork is seeding Boa's Math.random from a document-level seed or keeping it. Item 4: the error-boundary registry grows with each remount because dioxus-core's ErrorContext is private; the fork is pruning by a reachable signal, waiting on a dioxus-core release named as such, or keeping it.
A fix is proven headless on the stand or by a fixture in both layout modes; the chunk re-measures the existing limits (the inert focus check and the probe's spawned-acts pins) before and after.
@task_file n=1 path=packages/blitz-dom/src/mutator.rs mark=edits
what: DocumentMutator, through which every DOM mutation goes, and its flush's focus fixup under inert — edits: DocumentMutator::flush's focus clear
@task_file n=2 path=packages/blitz-dom/src/document.rs mark=edits
what: the blitz-dom document core, BaseDocument, holding focus state and the inert reading — edits: clear_focus, focus, is_inert
@task_file n=3 path=tests/blitz-tests/tests/inert_subtree.rs mark=re-reads
what: the integration check of the focus fixup under inert, beside which the item-1 checks stand
@task_file n=4 path=packages/dioxus-native-dom/src/mutation_buffer.rs mark=edits
what: the Dioxus bridge's buffered render, which buffers then replays mutations — edits: the buffered render
@task_file n=5 path=packages/dioxus-native-dom/src/events.rs mark=edits
what: the Dioxus bridge's events and mounted data — edits: MountedData's set_focus
@task_file n=6 path=tests/blitz-tests/tests/dioxus_runtime_probe.rs mark=edits
what: the Dioxus runtime probe that pins the buffered render's measured limits — edits: the spawned-acts pins
@task_file n=7 path=packages/blitz-vibey-script/src/runtime.rs mark=edits
what: the blitz-vibey-script JS runtime on Boa that runs document scripts — edits: the runtime's set-up, Boa's Math.random host hook
@task_file n=8 path=packages/dioxus-native-dom/src/seed.rs mark=edits
what: the run's seeded source, a Dioxus root context, in the dioxus-native-dom public API — edits: SeededRandom, DEFAULT_SEED
@task_file n=9 path=packages/blitz-test-harness/src/harness.rs mark=edits
what: the blitz-test-harness Harness and its harness run seed — edits: from_vdom
@task_file n=10 path=packages/dioxus-native-dom/src/boundary.rs mark=edits
what: the error-boundary registry behind the dioxus-native-dom public boundary_marks and BoundaryMark — edits: the boundary registry, root_error_mark
@task_item n=1 part=needs
The focus fixup under inert and its rule that the fixup fires no events: item 1 either extends the fixup to a focused control that no longer renders (hidden attribute, display:none, CSS alone) or keeps the inert-only reading; the DOM event dispatch contract for the fixup under inert and for focus change binds what a fix may dispatch.
@task_item n=2 part=needs
Focus state and its readers: the focused node, :focus-within on ancestors, the accessibility tree's focus, the snapshot's focused key and the clearing of interaction state of removed nodes all read the focus a fixup clears.
@task_item n=3 part=needs
The layout pass where display:none drops a box, and the rule that hit test and paint skip nodes dropped before resolve: the item-1 option gives focus up after the next layout, so it reads whether a node still renders after resolve.
@task_item n=4 part=needs
inert honoured by presence and the single reading of disabled and focusability: a hidden-control fixup must agree with what is focusable and what the keyboard reaches (Tab, Enter and Space activation of the focused control).
@task_item n=5 part=needs
The Dioxus bridge's buffered render and its mutation buffer replay: item 2's limit by construction, a task polled during a render acting on the document the previous render left; a fix is a deferred or replayed act inside that render, and render with the document unborrowed binds it.
@task_item n=6 part=needs
MountedData set_focus and set_focus_to returning false on a node not yet in the document; mounted listeners fire after build and poll; a stale mounted event is skipped. The probe's spawned-acts pins are the measurement a fix flips or a kept limit re-asserts.
@task_item n=7 part=needs
One seed for a run's randomness: SeededRandom as a Dioxus root context, use_seeded_random and DEFAULT_SEED, the harness run seed and from_vdom; the seeded source reads no environment variable, file or carrier, no seed crosses the driver's carriers, and no PRNG crate is taken.
@task_item n=8 part=needs
The script runtime's crossing for item 3: Boa running document scripts, the runtime's set-up and globals, the ScriptFetcher and runtime rules of the blitz-vibey-script public API; a document-level seed for Math.random must enter through the runtime's set-up without a new global or fetch path.
@task_item n=9 part=needs
The error-boundary registry and the public boundary_marks, BoundaryMark, root_error_mark and RootErrorMark: every boundary built under a document's factory stays registered for the document's life; a prune by a weak handle or generation must keep what those readers answer for live boundaries.
@task_item n=10 part=needs
dioxus-core 0.7.x's private ErrorContext and the held Dioxus 0.8 move: a wait on a dioxus-core release that exposes a dropped boundary is named as a wait on another project's release, never a git dependency or a patched dependency.
@task_item n=11 part=needs
A fix to items 1-3 is an engine behaviour change for every document, remote pages included: it is its own ratification and a direct engine change flagged upstreamable, cited to the spec where one governs it.
@task_item n=12 part=needs
A fix crosses no new carrier, verb, argument, field, refusal cause, log field or span unless the founder rules it: the validated verb set, the refusal causes closed at eight and the driver's span and log rules stay as they are.
@task_item n=13 part=needs
Proof: a fix is proven headless on the seven_guis stand or by a fixture in both layout modes, through the headless test harness on the virtual clock; both layout modes give identical layouts.
@task_item n=14 part=needs
A kept limit is a residual by the founder's word with his reason, stated as measured where an agent reads it (the capability statement and the tool's measured limits) and marked as a known gap in the code.
@task_item n=15 part=needs
The forks process: every option researched and costed first, all four forks written to one founder-forks file in the run dir with the recommended option first, their dependencies and any earlier ruling quoted; technical forks that change nothing of what the product is go to the operator in a dialog.
@task_standing n=1
convention: each new integration test lives in blitz-tests, one test target per file and one behaviour per file, declared in the one test table
home: conventions · standard-contracts
@task_standing n=2
convention: each new scenario runs in both layout modes, asserts its fixture's condition first, and carries a descriptive name
home: conventions
@task_standing n=3
convention: each new fixture uses numbered cases and relative asset URLs, shared fixtures stated once
home: conventions
@task_standing n=4
convention: each new unit test sits at the end of its source file or in a child test module of its module
home: conventions
@task_standing n=5
convention: each new public item and module is documented, each constant carries its rationale, spec URLs are cited at implementations, and each kept gap is marked TODO or FIXME
home: conventions
@task_standing n=6
convention: each new expect or unwrap is justified, each clippy allow kept local, each unsafe block annotated
home: conventions
@task_standing n=7
convention: each new stand check holds the stand-repeat zero-failure budget and gives identical readings on every CI OS
home: stack-and-technologies · standard-contracts
@task_standing n=8
convention: each changed crate holds its per-crate line coverage floor
home: stack-and-technologies · inherited-defaults
@task_item n=1 part=not_needed
The open select list, select model, select keyboard, multiple select and form submission rows: the single select sized as a list box is another chunk's, and no select behaviour changes here.
@task_item n=2 part=not_needed
The driver's verb, answer JSON, snapshot text, lint, layout reason, actions and graph contracts, their argument kinds and key names: the chunk adds nothing to what a verb answers.
@task_item n=3 part=not_needed
The session socket wire, session host, session errors, escher-session command line, MCP server and mcp role, recording directory and kept check file contracts: no carrier is touched.
@task_item n=4 part=not_needed
The agent-run, cold-agent, apt-install, target-sweep and coverage_floor script contracts, their events, exit codes and verdicts: dependency and CI debts are another chunk's.
@task_item n=5 part=not_needed
Paint, canvas, scrollbar painting, renderer, wgpu and screenshot encoding rows: any canvas work is out of scope and no paint output changes.
@task_item n=6 part=not_needed
The browser app's history, persistence, about pages, favicon, toolbar and tab rows: none of the four debts reaches the browser shell.
@task_item n=7 part=not_needed
The WPT runner, its outputs, result kinds, status codes, CI artifacts and PR-body tooling: no WPT behaviour is the subject of these debts.
@task_item n=8 part=not_needed
CSS feature support and layout algorithm rows (grid, flex, tables, fonts, text decoration, writing modes, transforms): only whether display:none drops a box is read.
@task_item n=9 part=not_needed
The networking rows of blitz-net, the HTTP cache, per-host request cap and request initiator: Math.random seeding fetches nothing.
@task_item n=10 part=not_needed
Stable element id tiers, the UI graph and graph registry, signal registry, reached record and handler coverage rows: no debt reaches ids, signals or reached handlers.
@task_item n=11 part=not_needed
The driver explore, step budget and path ending rows: exploration is untouched.
@task_item n=12 part=not_needed
Upstream sync merge, merge base and cherry-pick rows: the chunk takes nothing from another tree.
