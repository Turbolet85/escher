@carried n=1 label=a11y-plan/keyboard-navigation fact=fct-wgt25dte
why: a recorded check's Enter or Space press must count as exercising the click handler it reaches, so the coverage record must read the dispatch the key synthesizes, not only a pointer click.
quote: the Click is built from `Node::synthetic_click_event_data(modifiers)` and queued by the event driver, so the app's handlers (a Dioxus `onclick`) run, then the default action `handle_click`
@carried n=1 run=2 label=a11y-plan fact=fct-hhqldh6v
quote: A pressed Enter on the focused node dispatches a Click when it is no disabled form control and is a `button`, a `summary`, an `input` of type `submit`, `reset` or `button`, or an `a` with `href`; a released Space dispatches it for a `summary`, or for a `button` or an `input` of type `submit`, `reset`, `button`, `checkbox` or `radio` that is no disabled form control, never a link; the app's handlers run, then `handle_click`, so SC 2.1.1 is met by measurement on the stand's buttons.
@carried n=2 label=a11y-plan/keyboard-navigation fact=fct-wfj657us
why: the exercised record must hold on every app, registry or not, so the chunk must not lean on the registry's dispatch alone and must keep a page with no registered state dispatching every event as before.
quote: where a `SignalRegistry` is registered, the bridge's `DioxusEventHandler::handle_event` sets the registry's dispatch (its chain and its event) around `runtime.handle_event` and clears it after, so each signal write is recorded against that dispatch and the writing scope, and resolves to the first element of its chain carrying a listener of its event, to nothing where a node of the chain before that element no longer resolves; a write is flagged `undeclared` where no declaration names it, and a declared write `unconfirmed` only once a dispatch of the handler's event passed through its element and no run wrote the state; a page with no registered state reaches no registry and dispatches every event as before
@carried n=3 label=a11y-plan/keyboard-navigation fact=fct-v4en4yup
why: the keyboard forwarding in dioxus_document.rs, a reached file, must keep reaching Dioxus unchanged, and a forwarded key event reaching a keydown handler must count that handler as exercised.
quote: KeyDown, KeyUp and KeyPress events reach Dioxus as keyboard data with key, code, location, repeat, composing state and modifiers (packages/dioxus-native-dom/src/dioxus_document.rs:414-418; packages/dioxus-native-dom/src/events.rs:339-372)
@carried n=4 label=a11y-plan/keyboard-navigation fact=fct-zo6wthev
why: the focus forwarding in dioxus_document.rs, a reached file, must keep holding, and a forwarded focus event reaching a focus handler must count that handler as exercised.
quote: Focus, Blur, FocusIn and FocusOut events are forwarded as focus data (packages/dioxus-native-dom/src/dioxus_document.rs:409-412)
@carried n=5 label=a11y-plan/keyboard-navigation fact=fct-26wrao3c
why: the command's exact unexercised list on the stand must count `crud-list`'s keydown as one (node, event) handler, exercised only where a check pressed a key on the focused list.
quote: since 2026-10-13-stand-keyboard-harness the list `crud-list` takes `tabindex="0"` and an `onkeydown` — the third CRUD Tab stop — and, focused, ArrowDown selects the next visible row (the first when none is selected), ArrowUp the previous, Home the first and End the last, each setting the selection and the two fields as a row's click does
@carried n=5 run=2 label=a11y-plan fact=fct-utkipxkl
quote: the list `crud-list`, named `People`, is the third CRUD Tab stop and, focused, ArrowDown / ArrowUp / Home / End move the selection as a row's click does
@binding n=1
**keyboard-handling ↔ screen-reader-support (actionable keys)**: a handler a key press reaches through a synthesized Click (a11y-plan/keyboard-navigation fct-wgt25dte) is counted exercised under the same (graph node id, event) key the graph reads from the vdom walk's exact current listeners, never the `data-dioxus-id` mark or a `NodeId` (a11y-plan fct-77ewqepn).
@binding n=2
**keyboard-framework ↔ screen-reader-support (snapshot and graph)**: the registry's dispatch record and the graph's `confirmed`/`unconfirmed` reading (a11y-plan/keyboard-navigation fct-wfj657us) stay unchanged while the coverage reads its own exercised record over every app, the graph keeping one node per snapshot node and no second role or name mapping (a11y-plan fct-uz4kdss3).
@binding n=3
**keyboard-framework ↔ keyboard-shortcuts**: the key and focus events the bridge forwards (a11y-plan/keyboard-navigation fct-v4en4yup) are what reach the stand's keyboard handlers such as `crud-list`'s `onkeydown` (a11y-plan/keyboard-navigation fct-26wrao3c), so the exact list counts such a handler exercised only when a check's key dispatch reached it.
