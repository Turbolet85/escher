@task_prose part=chunk
Researches four engine debts left by 0.1.0-0.3.0 and puts each to the founder as a fork, fix or keep, with every option's cost measured first.
Debt 1: a focused control that no longer renders (hidden, display:none or CSS alone) keeps focus and takes keys; the fork is a focus fixup that gives focus up after the next layout, or keep the limit.
Debt 2: a task polled during a buffered Dioxus render sees the document as the previous render left it, so set_focus on a node made in that render does nothing; the fork is a deferred or replayed act, or keep the limit.
Debt 3: a page script's Math.random is unseeded by the run's seeded source; debt 4: the error-boundary registry grows with each remount; each fork is a fix (a document-level seed; a prune by a weak handle or generation, or a wait on a dioxus-core release) or keep.
A debt fixed is proven headless on the stand or as a fixture in both layout modes; a debt kept stays a residual with the founder's reason.
@task_file n=1 path=packages/blitz-dom/src/mutator.rs mark=edits
what: the document mutator, whose flush clears focus from a control that turned inert — edits: DocumentMutator::flush's focus clear
@task_file n=2 path=packages/blitz-dom/src/document.rs mark=edits
what: the document, holding focus state and the inert test — edits: clear_focus, focus, is_inert
@task_file n=3 path=packages/dioxus-native-dom/src/ mark=edits
what: the dioxus-native DOM integration: the buffered render and its replay, MountedData's set_focus, the seeded source SeededRandom and DEFAULT_SEED, the error-boundary registry and root_error_mark
@task_file n=4 path=packages/blitz-vibey-script/ mark=edits
what: the page script runtime over Boa: its set-up and Math.random host hook
@task_file n=5 path=tests/blitz-tests/tests/inert_subtree.rs mark=re-reads
what: the inert-subtree focus checks, beside which a focus fixup's checks would stand
@task_item n=1 part=needs
The focus held by a control that stops rendering: whether a focused control with no box keeps focus and takes keys, including a focused select with no box, and the fixup that would give focus up after the next layout.
@task_item n=2 part=needs
The buffered render, replayed under one borrow: a task polled during a render acts on the document as the previous render left it.
@task_item n=3 part=needs
MountedData calls during a render: the document state they see and set_focus_to answering false on a node created in that same render; a deferred or replayed act would change both.
@task_item n=4 part=needs
The run's seeded random source, a Dioxus root context that reaches Dioxus components alone and reads no environment variable, file or carrier; seeding a page script's Math.random from a document-level seed would extend its reach to the script runtime.
@task_item n=5 part=needs
The script runtime's crossing for page scripts: a Math.random hook must stay inside the security plan's script fetcher and runtime rules.
@task_item n=6 part=needs
The error-boundary registry under a document's factory and its root error mark: every boundary stays registered for the document's life; a prune needs a reachable signal since dioxus-core's error context fields are private.
@task_item n=7 part=needs
A fix to debts 1-3 is an engine change for every document, remote pages included, and so its own ratification.
@task_item n=8 part=needs
A fix crosses no new carrier, driver verb, argument, field, cause, log field or span unless the founder rules it.
@task_item n=9 part=needs
A fixed debt is proven headless on the stand, or as a fixture run in both layout modes; a kept debt stays a named residual with the founder's reason, a wait on another project's release named as such.
@task_standing n=1
convention: each new headless test or fixture builds the test engine with system fonts
home: typography
@task_item n=1 part=not_needed
Colour palette rows: engine colours, colour-scheme recascade and select list colours; no debt changes a colour.
@task_item n=2 part=not_needed
The open select list's overlay, its paint order, hit-testing, placement, gestures and list events; the single select sized as a list box is another chunk.
@task_item n=3 part=not_needed
The WPT runner's output and result colours; no debt touches the WPT runner.
@task_item n=4 part=not_needed
Shell provider window controls, cursor icons and favicon; no debt touches the window shell or iconography.
@task_item n=5 part=not_needed
Scroll motion, overlay scrollbar fades and smooth scroll; no debt changes scrolling or animation.
@task_item n=6 part=not_needed
Presentational hints for body margins, table spacing and font elements; no debt changes layout hints or spacing.
