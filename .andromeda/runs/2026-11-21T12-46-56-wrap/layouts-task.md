@task_prose part=session
The engine now gives focus up, after each layout and with no blur or focusout event, from a focused control that no longer shows (display none on it or an ancestor, or visibility not visible); the changed set marks that control.
Tab and Shift+Tab now skip any focusable node that does not show, an engine change for every document on the founder's word; a control whose own focus-within rule hides its container still takes Tab and loses it at the next layout, stated as a limit.
A script document can be built with a seed, which replaces the page's Math.random with a seeded sequence; an unseeded document still draws from the host; no carrier, env read or log site is added.
The Dioxus error-boundary registry is pruned of dropped boundaries before each render, so it stays bounded over remounts (measured length 3 over 20 remounts); a boundary dropped in a step still reads in that step's marks until the next render.
A task that wakes its own waker during a render livelocks inside that render; this busy-yield limit is stated in the capability statement, and the bridge's same-render focus limit is re-stated to its reachable case.
The a11y CI leg goes from eighteen to nineteen test files, its reading from 18 · 163 · 0 · 0 to 19 · 177 · 0 · 0; no IPC method, endpoint, event, port, env var, crate or dependency is added.
@task_file n=1 path=packages/blitz-dom/src/node/node.rs
what: the engine's DOM node, now holding the reading of whether a node shows (in the document, under no display none, visibility visible)
@task_file n=2 path=packages/blitz-dom/src/layout_reason.rs
what: the engine's layout-reason reading, whose has-a-box check now reads the shared display-none reading, answers unchanged
@task_file n=3 path=packages/blitz-dom/src/document.rs
what: the engine document's focus handling: the post-layout focus give-up for a control that no longer shows, and the Tab and Shift+Tab walk that now skips unshown nodes
@task_file n=4 path=packages/blitz-dom/src/resolve.rs
what: the engine document's resolve pass, which now runs the post-layout focus give-up right after the select list is reconciled
@task_file n=5 path=packages/blitz-vibey-script/src/lib.rs
what: the page-script engine's module list, gaining the crate-private seeded random module
@task_file n=6 path=packages/blitz-vibey-script/src/state.rs
what: the page-script runtime state, gaining the seeded random state
@task_file n=7 path=packages/blitz-vibey-script/src/runtime.rs
what: the page-script runtime, which now can replace the page's Math.random with a seeded native function
@task_file n=8 path=packages/blitz-vibey-script/src/document.rs
what: the script document's public building API, gaining the with-seed builder
@task_file n=9 path=packages/blitz-vibey-script/src/seeded_random.rs
what: new crate-private seeded random source for page scripts, with its unit checks
@task_file n=10 path=packages/blitz-vibey-script/tests/seeded_random.rs
what: new check that one seed reads one sequence, two seeds two, and an unseeded document draws from the host
@task_file n=11 path=packages/dioxus-native-dom/src/boundary.rs
what: the Dioxus bridge's error-boundary registry and boundary marks, now pruned of dropped boundaries before each render
@task_file n=12 path=packages/dioxus-native-dom/src/dioxus_document.rs
what: the Dioxus document's poll step, which now prunes the boundary registry right before the immediate render
@task_file n=13 path=tests/blitz-tests/tests/focus_fixup_unshown.rs
what: new a11y check that a focused control which stops showing gives focus up with no blur, and that Tab and Shift+Tab skip a control that does not show
@task_file n=14 path=tests/blitz-tests/tests/key_press_after_container_lost_its_box.rs
what: the a11y check of a key on a control whose container lost its box, now re-focusing the control before the key
@task_file n=15 path=tests/blitz-tests/tests/dioxus_runtime_probe.rs
what: the Dioxus runtime probe check, gaining a task that awaits the handle of a node its handler shows and one polled in the render that removes its target
@task_file n=16 path=.github/scripts/ci-leg.sh
what: the CI leg script, whose a11y leg test list gains the focus fixup check (nineteen files)
@task_file n=17 path=.github/scripts/test_ci_workflows.py
what: the CI workflow check, pinning the a11y leg's file list
@task_file n=18 path=.github/scripts/test_capability_statement.py
what: the capability statement check, pinning the a11y leg's file count as a literal (now 19)
@task_file n=19 path=docs/capabilities.md
what: the capability statement: the focus limits, the bridge's same-render limit, the busy-yield limit, and the a11y leg's count and reading
@task_item n=1 part=touches
Any row on keyboard focus over stand controls: a focused stand control that is hidden (display none or visibility not visible) now loses focus after the next layout with no blur or focusout, and is no longer a Tab or Shift+Tab stop.
@task_item n=2 part=touches
decision/crud-list-keyboard-region, indirectly through the Tab walk: Tab and Shift+Tab now skip a focusable node that does not show, so a hidden row or control in a keyboard region is no Tab stop.
@task_item n=3 part=touches
decision/flight-type-select-boot-reading and decision/open-list-rows-own-bounds, indirectly through the select: a focused select whose container lost its box gives focus up after the layout, and the focus give-up runs right after the select list is reconciled; a key reaches it only once re-focused.
@task_item n=4 part=touches
decision/stand-a11y-by-attributes-alone and decision/stand-ids-as-attributes-alone, indirectly: the engine's focus state now changes after a layout as well, so a focused-state reading of a stand control can drop where the control is hidden.
@task_item n=5 part=touches
contract/escher-session-snapshot/answer and contract/escher-session-actions/answer, indirectly: the focused node a snapshot reads after a layout, and where a Tab or Shift+Tab action lands, now skip a control that does not show.
@task_item n=6 part=touches
decision/order-page-built-to-cause and contract/stand-element-ids/order-failed, indirectly through the error-boundary registry: the boundary marks a cause reads are pruned of dropped boundaries before each render, and a boundary dropped in a step still reads until the next render.
@task_item n=7 part=touches
decision/every-stand-control-keyed and decision/held-instance-reads-as-fresh-boot, indirectly: remounting components no longer grows the error-boundary registry; it stays bounded (measured 3 over 20 remounts).
@task_item n=8 part=touches
decision/session-recording-replayable-check, indirectly: a script document can now be seeded so page-script Math.random reads one sequence per seed; an unseeded document still draws from the host.
@task_item n=9 part=touches
decision/scroll-into-view-moves-every-scroller and decision/visible-region-through-clipping-boxes, only where a row ties focus to what shows: the engine's new shows reading is display none and visibility alone, not clipping or scroll position.
@task_item n=1 part=untouched
The ShellProvider window chrome contracts (drag, maximize, minimize, close, decorations): no window or shell behaviour moved.
@task_item n=2 part=untouched
The stand element ids themselves: no id was added, renamed or freed, and no stand page or component was edited.
@task_item n=3 part=untouched
decision/stand-accent-colour, decision/headless-stand-pinned-viewport, decision/fixed-document-skeleton-main-mount and absence/arbitrary-index-html-templates: no colour, viewport, skeleton or template moved.
@task_item n=4 part=untouched
The CLI contracts (agent-run, cold-agent, escher-session exit codes, refusals, recording, MCP server, served tasks, multi_doc, paint_bench) and the cold-agent and MCP decisions: no command line, exit code, refusal, tool, verb or served task moved.
@task_item n=5 part=untouched
decision/load-page-only-network and decision/stand-lints-clean-at-boot: no network provider, lint or boot reading moved.
