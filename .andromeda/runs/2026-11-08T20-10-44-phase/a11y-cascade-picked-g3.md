@account n=1 entity=keyboard-navigation#3.5 v=not_in_scope
why: the open select list's choosing keys in blitz-dom, a component the chunk only drives, with no key behaviour added
@account n=2 entity=keyboard-navigation#3.13 v=not_in_scope
why: optgroup option reach and disabled rows in blitz-dom, which the chunk neither reaches nor re-measures
@account n=3 entity=keyboard-navigation#3.15 v=not_in_scope
why: the driver's type on a select in execute.rs, a file the chunk does not reach and a behaviour its scope does not name
@account n=4 entity=keyboard-navigation#3.17 v=carried
why: an Enter or Space press runs the app's onclick, so a check's key press reaches a click handler the coverage record must count as exercised
@account n=5 entity=keyboard-navigation#3.22 v=not_in_scope
why: where the synthetic click lands is the engine's own, used unchanged; the coverage reads which handler a dispatch reached, not its point
@account n=6 entity=keyboard-navigation#4.5 v=carried
why: the keyboard-event forwarding lives in dioxus_document.rs, reached whole, and a keydown handler's dispatch is one the coverage must count
@account n=7 entity=keyboard-navigation#4.6 v=carried
why: the registry's dispatch record in signal_registry.rs and graph.rs is the premise the chunk's exercised record is read against, and must hold on every app
@account n=8 entity=keyboard-navigation#4.7 v=carried
why: the focus-event forwarding lives in dioxus_document.rs, reached whole, and focus handlers are graph handlers the coverage must count
@account n=9 entity=keyboard-navigation#4.8 v=not_in_scope
why: Mounted set_focus in events.rs is a file the chunk does not reach, and mounted is outside the listener rule the graph reads
@account n=10 entity=keyboard-navigation#4.13 v=not_in_scope
why: the falsy boolean attribute removal in mutation_writer.rs, a component the chunk only reads through, its scope naming no attribute write
@account n=11 entity=keyboard-navigation#5.10 v=carried
why: crud-list's onkeydown is one (node, event) handler of the stand that the command's exact unexercised list reads
@account n=12 entity=keyboard-navigation#7.1 v=not_in_scope
why: the stand's Tab focus check is a re-check every change owes; the chunk changes no focus
@account n=13 entity=keyboard-navigation#7.2 v=not_in_scope
why: stand_keyboard's walks are a re-check every change owes; the chunk adds no key or focus behaviour
@account n=14 entity=keyboard-navigation#7.4 v=not_in_scope
why: the range input key and click checks are a re-check every change owes; the chunk touches no range input
@account n=15 entity=keyboard-navigation#7.6 v=not_in_scope
why: the select key and typeahead checks are a re-check every change owes; the chunk touches no select
