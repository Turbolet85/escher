@task_prose part=chunk
Settles four engine debts left by 0.1.0–0.3.0, each put to the founder as a fork after every option's cost is researched, all forks written into one founder-forks file in the run dir.
Debt 1: a focused control hidden by the hidden attribute, inline display:none or CSS alone keeps focus and takes keys; fork is a focus fixup for a control that no longer renders, or keep the limit.
Debt 2: a task polled during a Dioxus render sees the previous render's document, so set_focus on a node made in that render does nothing; fork is a deferred or replayed act, or keep the limit.
Debt 3: a page script's Math.random is unseeded (the host's); fork is seeding Boa's Math.random from a document-level seed, or keep. Debt 4: the error-boundary registry grows with each remount; fork is prune by a reachable signal, wait on a dioxus-core release, or keep.
Each debt ends fixed and proven headless on the stand or as a fixture in both layout modes, or kept as a residual by the founder's word with his reason.
@task_file n=1 path=packages/blitz-dom/src/mutator.rs mark=edits
what: the document mutator that applies a render's changes to the document — edits: DocumentMutator::flush's focus clear (the focus fixup reading is_inert alone)
@task_file n=2 path=packages/blitz-dom/src/document.rs mark=edits
what: the document, its focus state and inertness — edits: clear_focus, focus, is_inert
@task_file n=3 path=tests/blitz-tests/tests/inert_subtree.rs
what: the headless check of focus in an inert subtree, beside which the hidden-focused-control checks stand
@task_file n=4 path=packages/dioxus-native-dom/src mark=edits
what: the Dioxus native document layer — edits: the buffered render, MountedData's set_focus (set_focus_to), the seeded source SeededRandom and DEFAULT_SEED, the error-boundary registry and root_error_mark
@task_file n=5 path=packages/blitz-vibey-script mark=edits
what: the page-script runtime on Boa — edits: Boa's Math.random host hook, the runtime's set-up
@task_item n=1 part=needs
A focused control that stops rendering (hidden attribute, inline display:none, CSS alone): whether it keeps focus and takes keys, and a focus fixup that gives focus up after the next layout where the layout pass drops its box.
@task_item n=2 part=needs
Which boxes count as visible: a control whose box is dropped or clipped out of the visible region is the state the focus fixup reads, so the visible-region rule through clipping boxes binds what "no longer renders" means.
@task_item n=3 part=needs
Focus moved by script during a render: MountedData set_focus on a node made in the same buffered render returns false today; a fix makes such a task act on the render's own nodes by a deferred or replayed act.
@task_item n=4 part=needs
Page-script randomness: Boa's Math.random seeded from a document-level seed, reaching the page script the way the run's seeded source already reaches Dioxus components, so a headless run's script output is repeatable.
@task_item n=5 part=needs
The seeded source reads no environment variable, file or carrier; a document-level seed for page scripts stays inside that rule.
@task_item n=6 part=needs
The error-boundary registry under a document's factory: pruning a dropped boundary by a reachable signal (a weak handle, a generation) so a remounting app no longer grows it, or a named wait on a dioxus-core release.
@task_item n=7 part=needs
A fix to debts 1–3 is an engine change for every document, remote pages included; its proof must hold for any loaded page, not the stand's tasks alone.
@task_item n=8 part=needs
A fix crosses no new carrier, verb, argument, field, cause, log field or span unless the founder rules it: the driver's session commands, its answers and its log stay as they are.
@task_item n=9 part=needs
Proof headless on the stand: each fixed debt is shown by a headless check on the stand at its pinned viewport, the instance read as a fresh boot.
@task_item n=10 part=needs
Proof as a fixture in both layout modes: a fixture built for a debt runs in each layout mode and is read in each.
@task_item n=11 part=needs
A fixture or stand page built for these checks keeps the fixed document skeleton with its main mount, its controls keyed and its ids carried as attributes alone.
@task_item n=12 part=needs
Keyboard input to the focused control: the keys a hidden control takes today, and where they go once focus is given up, including into-view scrolling that focus may trigger.
@task_standing n=1
convention: each new headless stand check runs at the stand's pinned viewport and reads a held instance as a fresh boot
home: surface-desktop-native
@task_standing n=2
convention: each new stand control or fixture control is keyed, carries its id as an attribute alone, and names its role and state by attributes alone
home: surface-desktop-native
@task_standing n=3
convention: each new stand page or fixture lints clean at boot
home: surface-desktop-native
@task_item n=1 part=not_needed
The window chrome controls of the shell provider (drag, maximize, minimize, close, decorations): no debt touches the window around the document.
@task_item n=2 part=not_needed
The agent-run and cold-agent pipe contracts and the cold-agent decisions: the chunk runs no cold agent and changes no test pipe.
@task_item n=3 part=not_needed
The multi_doc and paint_bench command lines: the chunk builds no benchmark and no canvas work.
@task_item n=4 part=not_needed
The served-task specifics of the stand's existing tasks (counter, timer, crud, flight booker, cells, order, load, circle drawer, temperature converter) and their per-task ids: no debt changes what any existing task shows or does.
@task_item n=5 part=not_needed
The single select sized as a list box, the open list rows' bounds and the flight-type select's boot reading: the select debt is out of this chunk.
@task_item n=6 part=not_needed
The mobile-native and web-spa surfaces: they hold no rows and the chunk reaches neither.
