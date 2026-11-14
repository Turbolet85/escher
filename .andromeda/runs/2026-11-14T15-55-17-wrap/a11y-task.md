@task_prose part=session
The session adds auto-exploration to the session library: one in-process call explores the actions the actions verb lists, breadth-first, within a step budget (1 to 10,000 acting calls), each path on a fresh session.
A path takes one child per actionable node and explored verb (click, type, scroll); screenshot is left out because its result holds an image field, judged by the field's kind and never its name.
Each path ends accepted (settled or not), refused by its driver cause, panicked (a handler panic caught) or diverged; a refused, panicked or diverged path is never expanded, and screens are deduplicated by a digest of the snapshot text form.
The report holds only closed words, indices and calls, with no screen text, node id or panic payload, and logs nothing; each path can be kept as a replayable check in an owner-only directory.
The kept check's opening record gains a format key (written 1, read strictly, absent read as 1); the two committed Counter checks were re-recorded with it alone.
No verb, role, argument, cause, node key, tool, span or log site was added; verbs stay fourteen, causes eight.
@task_file n=1 path=packages/escher-driver/src/explore.rs
what: the session library's exploration of the actionable nodes, its step budget, its closed path endings and the snapshot-text screen digest
@task_file n=2 path=packages/escher-driver/src/explore_tests.rs
what: unit checks of the exploration's budget bounds and of which driver verbs it explores (no image-result verb)
@task_file n=3 path=packages/escher-driver/src/lib.rs
what: the session library's public surface, re-exporting the exploration and the kept check's format number
@task_file n=4 path=packages/escher-driver/src/actions.rs
what: the actions verb, listing actionable nodes with the verbs that target them and the call naming each; two helpers now shared with the exploration, behaviour unchanged
@task_file n=5 path=packages/escher-driver/src/recording.rs
what: the kept check's writer and reader (session recording and replay), its opening record now carrying the format key
@task_file n=6 path=packages/escher-driver/src/recording_tests.rs
what: unit checks of the kept check's opening record, its format rows and a two-target verb's step read back in schema order
@task_file n=7 path=tests/blitz-tests/tests/stand_explore.rs
what: the stand test target exploring every served task in both layout modes, with a refusing control page and a panicking handler page
@task_file n=8 path=tests/blitz-tests/checks/counter-cli.json
what: the committed kept check of the Counter recorded over the command-line carrier
@task_file n=9 path=tests/blitz-tests/checks/counter-mcp.json
what: the committed kept check of the Counter recorded over the MCP carrier
@task_file n=10 path=docs/capabilities.md
what: the capability statement, edited by hand, now stating the in-process exploration, its limits and its absences (no carrier, no navigation)
@task_item n=1 part=touches
The actionable-nodes listing and the actions answer: the exploration is a new consumer of the listing, driving one call per listed node and verb through the call naming the actions verb gives; the actions verb itself still changes nothing.
@task_item n=2 part=touches
The driver refusal causes, among them disabled: a refused path is reported by its closed cause (a refusing control page is met and reported by its cause); the cause count stays eight.
@task_item n=3 part=touches
A screenshot changes nothing: the screenshot verb is now excluded from exploration by its image result field, so an image-producing verb is never explored.
@task_item n=4 part=touches
The snapshot text form and its parts: the exploration deduplicates screens by a digest over the snapshot text, so the text form is now also the exploration's notion of a reached screen.
@task_item n=5 part=touches
One snapshot model with many readers, and snapshot readers behind the accessibility feature: the exploration is a new in-process reader of the snapshot inside the session library, which always enables accessibility.
@task_item n=6 part=touches
Password never revealed and the snapshot's value readings: the exploration report holds no screen text and typed text is only the fixed in-bound values the call naming gives.
@task_item n=7 part=touches
Driver click, type and scroll rules (synthetic click at the node centre, type focusing a select with no click, a disabled control stopping the click walk): exploration drives these verbs unchanged on every served task.
@task_item n=8 part=touches
Stand expectations stated once and the served tasks: a new stand target explores every served task in both layout modes; the expectation tables are unchanged.
@task_item n=9 part=touches
Stand checks repeated three rounds and the a11y test leg: a new stand-prefixed blitz-tests target joins the stand set; the count of named targets outside the stand prefix is unmoved.
@task_item n=10 part=touches
Accessibility reach edited by hand in the capability statement: the statement was edited by hand for exploration; its accessibility reach and its five counts are unchanged.
@task_item n=11 part=touches
Handler coverage and the reached-handler record: kept checks now carry the format key and their readers (replay, session recording, start in a state, handler coverage) stay green.
@task_fact n=1 slug=screen-reader-support
fact: Auto-exploration reads the snapshot and the actions listing only, explores the target verbs whose result holds no image field (click, type, scroll; never screenshot), and its report holds closed words, indices and calls, never screen text, node ids or values.
@task_fact n=2 slug=screen-reader-support
fact: Exploration identifies a reached screen by a digest of the snapshot text form, so two screens with the same snapshot text are one screen to it.
@task_item n=1 part=untouched
Contrast, the visual lint, colour pairs, focus outlines and the WCAG thresholds: no paint, style or contrast reading moved.
@task_item n=2 part=untouched
Role mapping, accessible name precedence, ARIA state and property reads and hidden-content exclusion: the accessibility tree's construction is unchanged.
@task_item n=3 part=untouched
Keyboard navigation contracts (Tab order, Enter and Space activation, range input keys, select list keys, the Cells grid and CRUD list keys): no key handling changed.
@task_item n=4 part=untouched
The AccessKit platform adapter, the accesskit_xplat choice and the absence of an adapter on the stand binaries: no adapter wiring moved.
@task_item n=5 part=untouched
The log format and its target allowlist: exploration adds no span, field or log site.
@task_item n=6 part=untouched
Cognitive accessibility, the invalid-state rules and the accessibility tier and scope decisions: untouched.
@task_item n=7 part=untouched
The UI graph and its state registration: no graph node, state or registration rule changed.
