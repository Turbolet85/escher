@carried n=1 label=a11y-plan/keyboard-navigation fact=fct-gmhtpdel
why: the target state turns this hazard around: a MountedData call from a task polled during poll's render must move focus with no panic, in both layout modes, with the same precondition still asserted first.
quote: A `MountedData` call in a task spawned from a key handler that writes a signal panics with `RefCell already borrowed`: `DioxusDocument::poll` holds `inner.borrow_mut()` across `vdom.render_immediate`, dioxus-core 0.7.10 first polls the task there through `handle_task_wakeup`, and its `set_focus`, borrowing through `doc_mut`, panics, while the same handler calling `set_focus` with no task moves focus and does not panic — measured headlessly in both layout modes by `dioxus_runtime_probe`'s two `should_panic` pins, their precondition asserted first (the handle held, the key target focused by Tab)
@carried n=2 label=a11y-plan/keyboard-navigation fact=fct-ws34tgkb
why: the witness and the inverted pins call set_focus on a held MountedData and must read the focus it moves straight off the engine, because no focus or blur event is queued to observe.
quote: Mounted elements can take or drop focus with `set_focus`; focus/blur events are not queued (TODO) (packages/dioxus-native-dom/src/events.rs:294-306)
@carried n=3 label=a11y-plan/keyboard-navigation fact=fct-2nvw4wcm
why: the replayed buffer must route every set-attribute call through MutationWriter's dynamic and static template paths, so a falsy boolean attribute is still removed and mutation_writer.rs stays byte-identical to upstream at 434e0661 unless a named reason changes it.
quote: A falsy Dioxus value (Bool false, Text `"false"`, Int 0, Float 0.0, None) of one of the bridge's 28 boolean attributes — upstream's private `is_bool_attr` predicate, a closed `matches!` set, since 2026-10-13-upstream-sync-polish-and-ship, which retired the bridge's own 27-name `BOOLEAN_ATTRIBUTES` list and left `mutation_writer.rs` byte-identical to upstream's at `434e0661`; `disabled`, `checked`, `hidden`, `readonly`, `required`, `multiple`, `selected`, `open`, `autofocus` and `inert` among them — removes the attribute, on the dynamic and the static template attribute path alike
@carried n=3 run=2 label=a11y-plan fact=fct-fi2um7zb
quote: A falsy Dioxus value (Bool false, Text `"false"`, Int 0, Float 0.0, None) of one of the bridge's 28 boolean attributes — upstream's `is_bool_attr` set, `disabled`, `hidden` and `inert` among them — removes the attribute on both attribute paths, so an enabled control does not match `:disabled` and `hidden: false` keeps an element laid out and in the tree
@carried n=3 run=3 label=a11y-plan fact=fct-2bz7tofn
quote: HTML's focus fixup is built, so a node holding focus when it comes under `inert`, a subtree moved under an inert box included, gives focus up as the mutator flushes, no DOM `blur` or `focusout` firing.
@carried n=4 label=a11y-plan/keyboard-navigation fact=fct-6luzygdm
why: scroll_to is one of the MountedData calls the scope names, and Cells' synchronous call from its key handler must keep acting at the call under the buffered render, as stand_keyboard re-asserts.
quote: Cells: `cells-grid` is one Tab stop, and its arrows move `current`, clamped, and bring the cell into view through a synchronous `MountedData::scroll_to(ScrollBehavior::Instant)` (tests/blitz-tests/tests/stand_keyboard.rs:193-200; tests/blitz-tests/tests/stand_keyboard.rs:293-335)
@carried n=4 run=2 label=a11y-plan fact=fct-5i3imozy
quote: the arrows move `current`, clamped, and bring the cell into view
@carried n=5 label=a11y-plan/keyboard-navigation fact=fct-bz6mwbin
why: stand_keyboard is one of the stand_* checks the scope names as the witness of unchanged behaviour, and it must pass unchanged in both layout modes over documents now rendered through the buffer.
quote: A keyboard check runs on the stand in both layout modes, since 2026-10-13-stand-keyboard-harness: `stand_keyboard` walks each lean task's stated sequence (`tab_order`) forward and back with Tab and Shift+Tab, with wrap, the engine's focus, the snapshot's `focused` node and the tree's focus agreeing at every step
@carried n=5 run=2 label=a11y-plan fact=fct-ivsezocx
quote: `stand_keyboard` walks each lean task's stated Tab order (`tab_order`, and `ENABLED_TAB_ORDER` for the states enabled by keyboard alone) forward and back in both layout modes, the engine's focus, the snapshot's `focused` node and the tree's focus agreeing at every step
@carried n=6 label=a11y-plan/keyboard-navigation fact=fct-2gn3is6h
why: this stand_* check re-asserts the Tab focus and the snapshot's focused reading the chunk must keep unchanged, which is also the Tab-focus precondition shape the inverted pins assert first.
quote: A key-press focus check exists on the stand, in both layout modes: after a click on `crud-name` a `Key::Tab` press moves focus to `crud-surname` and Shift+Tab moves it back, the first Tab on a fresh boot focuses `back-btn`, nothing reads focused at boot or after a click on a heading, and each time exactly the focused control's snapshot node reads `focused` and it is the accessibility tree's focus
@carried n=7 label=a11y-plan/keyboard-navigation fact=fct-wgt25dte
why: a key-dispatched Dioxus onclick re-renders through poll, the render site the chunk edits, so the stand rows whose click changes the screen must still change it through the replayed buffer.
quote: the Click is built from `Node::synthetic_click_event_data(modifiers)` and queued by the event driver, so the app's handlers (a Dioxus `onclick`) run, then the default action `handle_click` — since 2026-10-13-stand-keyboard-harness, in every document; SC 2.1.1 on the stand's buttons is met by measurement: Enter and Space on each focused stand button read as its click (16 rows, `counter-increment`, `flight-book` and `crud-create` changing the screen
@carried n=7 run=2 label=a11y-plan fact=fct-hhqldh6v
quote: the app's handlers run, then `handle_click`, so SC 2.1.1 is met by measurement on the stand's buttons.
@binding n=1
**keyboard-framework ↔ a11y-strategy**: a set_focus issued from a task polled during render must move the engine's focus (a11y-plan/keyboard-navigation fct-ws34tgkb) and still mark the changed set, so a poll that takes a non-empty set rebuilds the tree (a11y-plan fct-xhpyuh5z).
@binding n=2
**keyboard-framework ↔ keyboard-focus-order**: the replay must route set-attribute calls through MutationWriter's two paths (a11y-plan/keyboard-navigation fct-2nvw4wcm) and flush the mutator once per applied buffer, so the inert focus fixup still runs (a11y-plan fct-2bz7tofn).
@binding n=3
**keyboard-harness ↔ sr-snapshot**: the inverted pins and the re-run stand_keyboard and stand_snapshot_state checks must find the engine's focus, the snapshot's focused node and the tree's focus agreeing (a11y-plan/keyboard-navigation fct-bz6mwbin), and the snapshot must keep reading focus from accessibility_tree (a11y-plan fct-uz4kdss3).
@binding n=4
**keyboard-framework ↔ keyboard-shortcuts**: lifting the render-time borrow must leave the synchronous handler-time MountedData calls acting at the call, both Cells' scroll_to (a11y-plan/keyboard-navigation fct-6luzygdm) and the no-task set_focus (a11y-plan/keyboard-navigation fct-gmhtpdel).
