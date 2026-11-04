@carried n=1 label=a11y-plan/keyboard-navigation fact=fct-gmhtpdel
why: the probe must re-verify this mechanism and its coordinates at HEAD headlessly, on a key handler spawning a task that writes a signal, and name it in the founder's reading with its fix as his fork, never built.
quote: A `MountedData` call in a task spawned from a key handler that writes a signal panics with `RefCell already borrowed` (`events.rs:162`), the task first polled inside `render_immediate`, measured headlessly and the windowed path read and not run, so Cells calls its handles synchronously, each acting at the call — as measured at escher-0.3.0/chunks/2026-10-31-engine-debts/report.md
@carried n=2 label=a11y-plan/keyboard-navigation fact=fct-ws34tgkb
why: the hazard's reproduction must make a MountedData call such as `set_focus` from the spawned task, and the probe's reading must not count on a focus or blur event from it reaching a handler.
quote: Mounted elements can take or drop focus with `set_focus`; focus/blur events are not queued (TODO) (packages/dioxus-native-dom/src/events.rs:294-306)
@carried n=3 label=a11y-plan/keyboard-navigation fact=fct-v4en4yup
why: the probe must reach the hazard's key handler and link (2)'s key listeners by a KeyDown the bridge delivers to Dioxus as keyboard data, with no new listener or verb.
quote: KeyDown, KeyUp and KeyPress events reach Dioxus as keyboard data with key, code, location, repeat, composing state and modifiers (packages/dioxus-native-dom/src/dioxus_document.rs:391-395; packages/dioxus-native-dom/src/events.rs:339-372)
@carried n=4 label=a11y-plan/keyboard-navigation fact=fct-fanke5ok
why: the windowed reading of the hazard, read and not run, must follow a key through this dispatch to the same `poll` lines the headless run measures.
quote: Every key press and release is dispatched as KeyDown or KeyUp after shell shortcuts are checked (packages/blitz-shell/src/window.rs:690-698)
@carried n=5 label=a11y-plan/keyboard-navigation fact=fct-wgt25dte
why: links (2), (3) and (4) must be read on this path, where Enter or Space on a focused Counter or CRUD button runs the app's onclick before `handle_click`, the handler whose written state the probe follows.
quote: A pressed Enter whose target is the focused node, after the text-input and range arms, dispatches a Click on it when it is no disabled form control (`ElementData::is_disabled`; ratified by the founder (2026-10-18)) and is a `button`, a `summary`, an `input` of type `submit`, `reset` or `button`, or an `a` with `href`; a released Space — `handle_key_up`, KeyUp's first default action, routed by `events/mod.rs` — on the focused node dispatches the same Click for a `button`, a `summary`, or an `input` of type `submit`, `reset`, `button`, `checkbox` or `radio` that is not disabled — a `disabled` attribute on a `summary` or an `a` gating neither key, as HTML does; a link takes no Space, and a text input and a range input take neither key there; the Click is built from `Node::synthetic_click_event_data(modifiers)` and queued by the event driver, so the app's handlers (a Dioxus `onclick`) run, then the default action `handle_click` — since 2026-10-13-stand-keyboard-harness, in every document;
@carried n=5 run=2 label=a11y-plan/keyboard-navigation fact=fct-55wog4z5
quote: `synthetic_click_event` builds a primary mouse click at the node's center (packages/blitz-dom/src/node/node.rs:2028-2058)
@carried n=5 run=3 label=a11y-plan fact=fct-hhqldh6v
quote: the app's handlers run, then `handle_click`, so SC 2.1.1 is met by measurement on the stand's buttons.
@carried n=6 label=a11y-plan/keyboard-navigation fact=fct-6d3ax3jg
why: a pointer click the probe drives on a stand button focuses it and one on a CRUD row clears focus, so the probe's state-to-nodes reading must tell that focus change from the handler's state write.
quote: A pointer click on a `button` (any type) or an `input` of type `submit`, `reset` or `button` focuses it through `generate_focus_events` / `set_focus_to`, then submits the form owner when it is a submit button, and ends the walk matched — since 2026-10-13-stand-keyboard-harness, in every document; before it a submit button submitted without breaking the walk and every other button fell through to the unmatched clear of focus, so an accepted click on one left no element focused (as measured at escher-0.1.0/chunks/2026-10-10-scrolling-box-bounds-and-hit/report.md); a disabled form control still stops the walk first, a disabled ancestor `div` not (ratified by the founder, 2026-10-18), and a click on any other element is unchanged — a CRUD row or the list still clears focus;
@carried n=6 run=2 label=a11y-plan/keyboard-navigation fact=fct-n3cvi4zm
quote: Clicking a non-interactive area clears focus (packages/blitz-dom/src/events/pointer.rs:1093-1096)
@carried n=6 run=3 label=a11y-plan fact=fct-7hugl7cl
quote: a click on any other element is unchanged — a CRUD row or the list still clears focus.
@carried n=7 label=a11y-plan/keyboard-navigation fact=fct-pqyrk5rl
why: the probe must focus its key target by Tab through the harness press before Enter, Space or the hazard's key reaches a handler, and that focus move is a change the changed-set reading drains first.
quote: Tab moves focus to the next node and Shift+Tab to the previous, dispatching focus events (packages/blitz-dom/src/events/keyboard.rs:30-48); `focus_next_node` / `focus_prev_node` move focus to the next/previous focussable node (packages/blitz-dom/src/document.rs:2222-2235)
@carried n=8 label=a11y-plan/keyboard-navigation fact=fct-26wrao3c
why: the CRUD list's `onkeydown` and a row's click writing the selection and the two fields is the handler-to-state surface links (3) and (4) must be read on.
quote: since 2026-10-13-stand-keyboard-harness the list `crud-list` takes `tabindex="0"` and an `onkeydown` — the third CRUD Tab stop — and, focused, ArrowDown selects the next visible row (the first when none is selected), ArrowUp the previous, Home the first and End the last, each setting the selection and the two fields as a row's click does, a key at the end of the list or ArrowUp with no row selected changing nothing;
@carried n=8 run=2 label=a11y-plan fact=fct-utkipxkl
quote: the list `crud-list`, named `People`, is the third CRUD Tab stop and, focused, ArrowDown / ArrowUp / Home / End move the selection as a row's click does;
@carried n=9 label=a11y-plan/keyboard-navigation fact=fct-2gn3is6h
why: the probe must drive its keys through the same harness `press` / `press_with` this check drives on the stand, in both layout modes, and leave this check holding.
quote: A key-press focus check exists on the stand, in both layout modes: after a click on `crud-name` a `Key::Tab` press moves focus to `crud-surname` and Shift+Tab moves it back, the first Tab on a fresh boot focuses `back-btn`, nothing reads focused at boot or after a click on a heading, and each time exactly the focused control's snapshot node reads `focused` and it is the accessibility tree's focus (tests/blitz-tests/tests/stand_snapshot_state.rs:331-367)
@carried n=9 run=2 label=a11y-plan fact=fct-zvvhy2hs
quote: a Tab and Shift+Tab key-press focus check exists on the stand, driven through the test harness's `press` / `press_with` (tests/blitz-tests/tests/stand_snapshot_state.rs:331-367)
@carried n=10 label=a11y-plan/keyboard-navigation fact=fct-bz6mwbin
why: the Enter and Space rows and the CRUD list's keys of `stand_keyboard` witness the handler paths the probe reads, and the probe's key-driven runs must go through the same harness readers without a second one.
quote: Enter and Space on each focused stand button read as its click (16 rows) and keys that do not activate type or change nothing; fixture rows for the controls HTML gives the keys, a button disabled after it took focus taking no key and a button that is itself scrolled taking Enter as its own click; a click focusing each stand button with Tab continuing from it; a `disabled` attribute skipping a form control and not a `<div disabled tabindex=0>`; the CRUD list taking focus and its keys moving the selection;
@carried n=10 run=2 label=a11y-plan fact=fct-ivsezocx
quote: `stand_keyboard` walks each lean task's stated Tab order (`tab_order`, and `ENABLED_TAB_ORDER` for the states enabled by keyboard alone) forward and back in both layout modes,
@binding n=1
**keyboard-framework (the MountedData hazard) ↔ the body's accessibility tree lifecycle**: the hazard's spawned task is first polled inside `render_immediate` on the same poll that refreshes the tree when it takes a non-empty changed set, so the re-verification must read that poll path headlessly, and on the windowed shell read it without running it, with no engine change for every document (a11y-plan/keyboard-navigation fct-gmhtpdel) (a11y-plan/keyboard-navigation fct-fanke5ok) (a11y-plan fct-zyrpeqlm)
@binding n=2
**keyboard-handling (Enter and Space dispatch a Click) ↔ screen-reader-support (the actionable-key rule)**: link (2)'s `click` answer must say whether the listener map reads the onclick that Enter, Space or a pointer click runs, and must not add the snapshot field the residuals line owns (a11y-plan/keyboard-navigation fct-wgt25dte) (a11y-plan fct-77ewqepn)
@binding n=3
**keyboard-focus-order and keyboard-focus-restoration (Tab, a button's click, a CRUD row's click) ↔ a11y-strategy (the engine's changed set)**: a focus change marks the changed set, so a link (4) reading after a Tab, a button click or a CRUD row click must drain the set first and tell the focus marks apart from the nodes that read the written state (a11y-plan/keyboard-navigation fct-pqyrk5rl) (a11y-plan/keyboard-navigation fct-6d3ax3jg) (a11y-plan fct-xhpyuh5z)
@binding n=4
**keyboard-shortcuts (the CRUD list's keys) ↔ screen-reader-support (the stand's states by attribute)**: the selection a CRUD key or row click writes is read in the tree through the option rows' `aria-selected`, so link (4) on CRUD must name those rows as the state's readers through the tree, not by a second role or name mapping (a11y-plan/keyboard-navigation fct-26wrao3c) (a11y-plan fct-ldzd7n5j)
@binding n=5
**keyboard-harness (stand_keyboard, stand_snapshot_state) ↔ the body's A11y Assertion Harness Contract**: the probe must drive keys through the harness `press` / `press_with` the existing checks use, and leave both checks holding in both layout modes (a11y-plan/keyboard-navigation fct-bz6mwbin) (a11y-plan fct-zvvhy2hs)
