@compare side=phase doc=design-system marker=2026-11-04-dioxus-runtime-probe e_items=8 held=1 lost_classify=2 lost_strong=1 beside=- cascade_only=2 listed=1 body=3 unplaced=0 held_ids=surface#15.2 only_ids=motion#3.1,surface#15.1
@compare_item n=1 part=lost how=classify entity=motion#1.5 label=design-system/motion fact=fct-z2ugu37c
@carried n=1 label=design-system/motion fact=fct-z2ugu37c
why: The probe's reading of the MountedData hazard must keep apart the stand's synchronous `MountedData` call from a key handler, which stands as recorded, from the hazard's path of a task spawned from a key handler; it must not change or re-record that handler's instant, unanimated scroll.
quote: The stand's Cells grid brings its current cell into view with no animation: its arrow keys' handler calls `MountedData::scroll_to(ScrollBehavior::Instant)` synchronously (tests/blitz-tests/tests/stand_keyboard.rs:293-335; as measured at escher-0.3.0/chunks/2026-10-31-engine-debts/report.md)
@compare_item n=2 part=lost how=classify entity=motion#4.3 label=design-system/motion fact=fct-5d7ppgng
@carried n=2 label=design-system/motion fact=fct-5d7ppgng
why: Link (2)'s reading of handlers and event names must treat animation and transition events as carrying no event data in the bridge. The probe must not dispatch them or build their data, because that would need an engine change, and the reading must name the gap rather than fill it.
quote: Animation and transition event data are `unimplemented!()` in dioxus-native-dom (packages/dioxus-native-dom/src/events.rs:71-73; packages/dioxus-native-dom/src/events.rs:123-125)
@compare_item n=3 part=lost how=strong entity=typography#2.2 label=design-system/typography fact=fct-vdyur6r3
@carried n=2 label=design-system/typography fact=fct-vdyur6r3
why: the probe runs headlessly on the seven_guis stand; the stand's font registration stays an architecture contract, so the probe must not add it or any other value as a design-system typography value.
quote: The seven_guis headless stand's font registration is held in architecture §Standard Contracts → Headless stand, beside `font_ctx()`, not as a design-system typography value
@compare_item n=1 part=listed how=classify entity=typography#2.8 label=design-system/typography fact=fct-zmi56wtl
@carried n=1 label=design-system/typography fact=fct-zmi56wtl
why: a probe check placed in tests/blitz-tests that reads layout (both layout modes) must build blitz-dom with the system-fonts feature, so that text is measured and no font-dependent reading passes vacuously.
quote: `blitz-dom` is built with the `system-fonts` feature for the blitz-tests crate; without it text measures 0x0 and font-dependent assertions pass vacuously
