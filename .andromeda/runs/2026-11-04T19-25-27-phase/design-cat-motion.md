@carried n=1 label=design-system/motion fact=fct-z2ugu37c
why: The probe's reading of the MountedData hazard must keep apart the stand's synchronous `MountedData` call from a key handler, which stands as recorded, from the hazard's path of a task spawned from a key handler; it must not change or re-record that handler's instant, unanimated scroll.
quote: The stand's Cells grid brings its current cell into view with no animation: its arrow keys' handler calls `MountedData::scroll_to(ScrollBehavior::Instant)` synchronously (tests/blitz-tests/tests/stand_keyboard.rs:293-335; as measured at escher-0.3.0/chunks/2026-10-31-engine-debts/report.md)
@carried n=2 label=design-system/motion fact=fct-5d7ppgng
why: Link (2)'s reading of handlers and event names must treat animation and transition events as carrying no event data in the bridge. The probe must not dispatch them or build their data, because that would need an engine change, and the reading must name the gap rather than fill it.
quote: Animation and transition event data are `unimplemented!()` in dioxus-native-dom (packages/dioxus-native-dom/src/events.rs:71-73; packages/dioxus-native-dom/src/events.rs:123-125)
@carried n=3 label=design-system fact=fct-za7xpowz
why: The probe's runs on the stand must read the state a handler writes through steps that keep to the virtual clock's rule: a click, type or press must not advance time, and only an `advance` moves the clock and the app's time together.
quote: the driver's `advance` moves it by exactly its `ms` and, through the step a session's caller hands it, the app's own time together, while `click`, `type` and `press` leave it where it was.
@carried n=4 label=design-system fact=fct-vjxch6lk
why: A probe check that reads state after a step must not wait on the document's animating set or take `Settled` as proof that an animation finished.
quote: A harness settle answers every member of the document's animating set the same way — none holds it open, none is waited on or advanced: it returns `Settled`, `animating` reading the document's animating flag.
@binding n=1
**MountedData use at the stand (motion) ↔ the body's scroll rule**: the Cells key handler's synchronous instant `scroll_to` is the recorded working case beside the hazard. A fix the founder chooses must keep it instant and unanimated (design-system/motion fct-z2ugu37c), and it must keep the body's into-view rule (design-system fct-tnwqvola).
@binding n=2
**Measured verdicts (motion) ↔ the body's animation runtime**: each verdict's stand run must follow the harness clock and settle rules from the body (design-system fct-za7xpowz) (design-system fct-vjxch6lk) and the detailed clock fact (design-system/motion fct-e2e2e6vg).
