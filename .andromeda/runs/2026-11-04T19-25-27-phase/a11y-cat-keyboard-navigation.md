@carried n=1 label=a11y-plan/keyboard-navigation fact=fct-gmhtpdel
why: The founder's reading must name the MountedData hazard as this record states it, re-verified at HEAD, and each fix option brought to him must say how it changes Cells' synchronous-handle workaround.
quote: A `MountedData` call in a task spawned from a key handler that writes a signal panics with `RefCell already borrowed` (`events.rs:162`), the task first polled inside `render_immediate`, measured headlessly and the windowed path read and not run, so Cells calls its handles synchronously, each acting at the call — as measured at escher-0.3.0/chunks/2026-10-31-engine-debts/report.md
@carried n=2 label=a11y-plan/keyboard-navigation fact=fct-6luzygdm
why: Cells' keyboard behaviour rests on synchronous MountedData calls, so the probe must leave it unchanged (no engine change) and the fork's options must name what each does to it.
quote: Cells: `cells-grid` is one Tab stop, and its arrows move `current`, clamped, and bring the cell into view through a synchronous `MountedData::scroll_to(ScrollBehavior::Instant)` (tests/blitz-tests/tests/stand_keyboard.rs:193-200; tests/blitz-tests/tests/stand_keyboard.rs:293-335) — as measured at escher-0.3.0/chunks/2026-10-31-engine-debts/report.md
@carried n=2 run=2 label=a11y-plan/keyboard-navigation fact=fct-gqyqow26
quote: Cells' editor: Enter on the grid opens the editor on `current`, focused; its Enter commits, stops propagation and returns focus to the grid; a click inside a cell's own open editor returns early, so typed text stays (examples/seven_guis/src/tasks/cells.rs:425-433; tests/blitz-tests/tests/stand_keyboard.rs:202-249)
@carried n=3 label=a11y-plan/keyboard-navigation fact=fct-ws34tgkb
why: The probe reads the bridge's `events.rs` MountedData surface; its focus methods and the unqueued focus/blur TODO must be described as they are recorded, and no probe may change them.
quote: Mounted elements can take or drop focus with `set_focus`; focus/blur events are not queued (TODO) (packages/dioxus-native-dom/src/events.rs:294-306)
@carried n=4 label=a11y-plan/keyboard-navigation fact=fct-v4en4yup
why: Link (2), node to handlers, must cover the key and focus event names that the bridge already forwards to Dioxus, using these bridge entry points as the named API.
quote: KeyDown, KeyUp and KeyPress events reach Dioxus as keyboard data with key, code, location, repeat, composing state and modifiers (packages/dioxus-native-dom/src/dioxus_document.rs:391-395; packages/dioxus-native-dom/src/events.rs:339-372)
@carried n=4 run=2 label=a11y-plan/keyboard-navigation fact=fct-zo6wthev
quote: Focus, Blur, FocusIn and FocusOut events are forwarded as focus data (packages/dioxus-native-dom/src/dioxus_document.rs:386-389)
@carried n=5 label=a11y-plan/keyboard-navigation fact=fct-wgt25dte
why: The "reacts to a click" answer for link (2) must allow for Enter and Space on a focused button reaching the same Dioxus `onclick`, so reading a click listener covers keyboard activation too.
quote: the Click is built from `Node::synthetic_click_event_data(modifiers)` and queued by the event driver, so the app's handlers (a Dioxus `onclick`) run, then the default action `handle_click` — since 2026-10-13-stand-keyboard-harness, in every document;
@carried n=6 label=a11y-plan/keyboard-navigation fact=fct-26wrao3c
why: The CRUD lean task gives the probe a key handler that writes state other nodes read (`crud-list`'s `onkeydown`), and links (2) to (4) must be measured on it as recorded.
quote: since 2026-10-13-stand-keyboard-harness the list `crud-list` takes `tabindex="0"` and an `onkeydown` — the third CRUD Tab stop — and, focused, ArrowDown selects the next visible row (the first when none is selected), ArrowUp the previous, Home the first and End the last, each setting the selection and the two fields as a row's click does, a key at the end of the list or ArrowUp with no row selected changing nothing;
@carried n=7 label=a11y-plan/keyboard-navigation fact=fct-bz6mwbin
why: The probe adds only test or example code, so the stand keyboard check must still pass unchanged in both layout modes.
quote: `stand_keyboard` walks each lean task's stated sequence (`tab_order`) forward and back with Tab and Shift+Tab, with wrap, the engine's focus, the snapshot's `focused` node and the tree's focus agreeing at every step
@carried n=8 label=a11y-plan/keyboard-navigation fact=fct-2nvw4wcm
why: An "exposed with a cost" verdict that would change the bridge must say whether it breaks `mutation_writer.rs`'s byte-identity with upstream.
quote: since 2026-10-13-upstream-sync-polish-and-ship, which retired the bridge's own 27-name `BOOLEAN_ATTRIBUTES` list and left `mutation_writer.rs` byte-identical to upstream's at `434e0661`;
@binding n=1
**MountedData hazard ↔ the scope's folded CARRY and the founder's fork**: the reading must state the hazard exactly as recorded (a task spawned from a key handler, first polled inside `render_immediate`, panicking at `events.rs:162`; headless measured, windowed read and not run) after P3 re-verifies it at HEAD. Each fix option must say what it does to Cells' synchronous `MountedData` calls and to its editor's focus return. The chunk decides none of them (a11y-plan/keyboard-navigation fct-gmhtpdel) (a11y-plan/keyboard-navigation fct-6luzygdm) (a11y-plan/keyboard-navigation fct-gqyqow26)
@binding n=2
**Keyboard activation to onclick ↔ §7's actionable-key rule and the "reacts to a click" residual**: link (2)'s answer for `click` must account for Enter and Space reaching a Dioxus `onclick` through the queued synthetic Click. The body's rule that no snapshot field says an element reacts to a click is owned by a residuals line, and the probe adds no such field (a11y-plan/keyboard-navigation fct-wgt25dte) (a11y-plan fct-77ewqepn)
@binding n=3
**Bridge key/focus forwarding ↔ the probe's link (2) verdicts and the "nothing crosses" boundary**: the key and focus event names forwarded by `dioxus_document.rs`/`events.rs` are the handler surface the probe reads. No listener, field or log line may be added, and no handler value may be printed (a11y-plan/keyboard-navigation fct-v4en4yup) (a11y-plan/keyboard-navigation fct-zo6wthev) (a11y-plan/keyboard-navigation fct-ws34tgkb)
@binding n=4
**Stand keyboard check ↔ §9 CI a11y leg**: `stand_keyboard` runs in the `a11y` leg and three rounds in `stand-repeat`. The probe's check in `tests/blitz-tests/` must leave it passing, and whether the probe joins a leg is the plan's choice (a11y-plan/keyboard-navigation fct-bz6mwbin) (a11y-plan fct-6kcxmj7u)
