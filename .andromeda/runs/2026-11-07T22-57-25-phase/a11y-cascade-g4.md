@carried n=1 label=a11y-plan/keyboard-navigation fact=fct-gmhtpdel
why: The signal registry's wrapper write, recorded for link 3, runs inside key handlers and the tasks they spawn on every Dioxus document, so it must take no document borrow of its own and leave the buffered render intact.
quote: A `MountedData` call in a task spawned from a key handler that writes a signal acts and does not panic, on every Dioxus document: `DioxusDocument::initial_build` and `Document::poll` render into a mutation buffer (`buffered_mutations`, a `MutationBuffer`) with the document unborrowed, then take `inner.borrow_mut()` once and replay the buffer through one `MutationWriter`
@carried n=2 label=a11y-plan/keyboard-navigation fact=fct-26wrao3c
why: The CRUD list's onkeydown writes the selection and the two field signals the chunk moves onto the registry, and its keys must keep acting exactly as stated once they do.
quote: since 2026-10-13-stand-keyboard-harness the list `crud-list` takes `tabindex="0"` and an `onkeydown` — the third CRUD Tab stop — and, focused, ArrowDown selects the next visible row (the first when none is selected), ArrowUp the previous, Home the first and End the last, each setting the selection and the two fields as a row's click does, a key at the end of the list or ArrowUp with no row selected changing nothing; the tree's focus is the list, never the option
@carried n=2 run=2 label=a11y-plan fact=fct-utkipxkl
quote: the list `crud-list`, named `People`, is the third CRUD Tab stop and, focused, ArrowDown / ArrowUp / Home / End move the selection as a row's click does
@carried n=3 label=a11y-plan/keyboard-navigation fact=fct-6luzygdm
why: Cells' `current` signal moves onto the registry, and its arrow handler must keep clamping and keep bringing the cell into view through the synchronous scroll_to call.
quote: Cells: `cells-grid` is one Tab stop, and its arrows move `current`, clamped, and bring the cell into view through a synchronous `MountedData::scroll_to(ScrollBehavior::Instant)`
@carried n=3 run=2 label=a11y-plan fact=fct-5i3imozy
quote: the arrows move `current`, clamped, and bring the cell into view; Enter opens the editor on `current`, focused, and the editor's Enter commits and returns focus to the grid; Tab leaving an open editor commits it
@carried n=4 label=a11y-plan/keyboard-navigation fact=fct-bz6mwbin
why: The stand tasks whose signals the chunk moves must keep the Tab orders stand_keyboard walks, with the engine's focus, the snapshot's `focused` node and the tree's focus agreeing at every step.
quote: `stand_keyboard` walks each lean task's stated sequence (`tab_order`) forward and back with Tab and Shift+Tab, with wrap, the engine's focus, the snapshot's `focused` node and the tree's focus agreeing at every step
@carried n=4 run=2 label=a11y-plan/keyboard-navigation fact=fct-ovpaig3d
quote: `LOAD_TAB_ORDER` and `CELLS_TAB_ORDER`, the stated Tab sequences of the load page and Cells, live in the shared module beside `tab_order` and `ENABLED_TAB_ORDER`
@carried n=4 run=3 label=a11y-plan fact=fct-ivsezocx
quote: `stand_keyboard` walks each lean task's stated Tab order (`tab_order`, and `ENABLED_TAB_ORDER` for the states enabled by keyboard alone) forward and back in both layout modes, the engine's focus, the snapshot's `focused` node and the tree's focus agreeing at every step, every lean task's control lying in one of the two lean orders
@binding n=1
**keyboard-framework ↔ the scope's signal registry**: the registry's wrapper write that records link 3 must run in a key handler, and in a task such a handler spawns, with no borrow of the document of its own, so the buffered render and its two `dioxus_runtime_probe` pins keep holding in both layout modes (a11y-plan/keyboard-navigation fct-gmhtpdel); the graph read through `VirtualDom::in_runtime` must take the document no other way than the render does (none).
@binding n=2
**keyboard-shortcuts ↔ screen-reader-support**: the CRUD list's `onkeydown` and the Cells grid's keys must keep acting as stated after their signals move onto the registry (a11y-plan/keyboard-navigation fct-26wrao3c) (a11y-plan/keyboard-navigation fct-6luzygdm), and they are handlers the graph's link 2 names on actionable elements that keep their author keys under the actionable-key rule (a11y-plan fct-77ewqepn).
@binding n=3
**keyboard-harness ↔ screen-reader-support**: the stand's stated Tab orders must stay as they are in the shared module (a11y-plan/keyboard-navigation fct-ovpaig3d), and the snapshot's `focused` node that stand_keyboard compares at every step must keep coming from the same snapshot model the new «reacts to a click» field joins, with no second role or name mapping (a11y-plan/keyboard-navigation fct-bz6mwbin) (a11y-plan fct-uz4kdss3).
