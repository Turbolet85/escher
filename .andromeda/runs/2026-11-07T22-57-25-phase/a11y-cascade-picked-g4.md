@account n=1 entity=keyboard-navigation#3.2 v=not_in_scope
why: the engine's range-input keys, which the chunk only uses and the scope names nowhere, a re-check every change owes
@account n=2 entity=keyboard-navigation#3.6 v=not_in_scope
why: the select list's row cap and scroll, an engine behaviour the chunk neither edits nor re-measures, a re-check every change owes
@account n=3 entity=keyboard-navigation#3.17 v=not_in_scope
why: the engine's Enter and Space activation, a component the chunk only uses whose behaviour the scope does not name, a re-check every change owes
@account n=4 entity=keyboard-navigation#3.21 v=not_in_scope
why: a label's default click in pointer.rs, an engine behaviour the chunk leaves unchanged and does not re-read, a re-check every change owes
@account n=5 entity=keyboard-navigation#4.8 v=carried
why: the signal registry's wrapper write runs in key handlers and their spawned tasks on a Dioxus document in dioxus-native-dom, so it must keep the buffered, unborrowed render the pins measure
@account n=6 entity=keyboard-navigation#4.12 v=not_in_scope
why: the bridge's falsy boolean attribute removal in mutation_writer.rs, which the chunk only reads through and never edits, a re-check every change owes
@account n=7 entity=keyboard-navigation#5.8 v=carried
why: Cells' `current` signal moves onto the registry, an edit of the arrow handler that writes it, so its clamped arrows and synchronous scroll_to must keep holding
@account n=8 entity=keyboard-navigation#5.10 v=carried
why: the CRUD list's onkeydown writes the selection and field signals the chunk moves onto the registry, so its keys must keep moving the selection as a row's click does
@account n=9 entity=keyboard-navigation#7.1 v=not_in_scope
why: the stand_snapshot_state focus check reads only `focused`, which the chunk neither edits nor re-pins, a re-check every change owes
@account n=10 entity=keyboard-navigation#7.2 v=carried
why: stand_keyboard is the witness that the stand tasks the chunk edits keep their stated Tab orders and their CRUD and Cells keys
@account n=11 entity=keyboard-navigation#7.3 v=carried
why: the shared module's stated Tab sequences are the tables whose Tab orders the edited stand tasks must keep as they are
@account n=12 entity=keyboard-navigation#7.4 v=not_in_scope
why: the range-input key and click checks over the unchanged engine, re-run as a re-check every change owes
@account n=13 entity=keyboard-navigation#7.6 v=not_in_scope
why: the select key and typeahead checks over an engine model the chunk does not touch, a re-check every change owes
@account n=14 entity=keyboard-navigation#7.7 v=not_in_scope
why: the select list's own check counts, which the chunk adds no test to and does not re-measure, a re-check every change owes
@account n=15 entity=keyboard-navigation#7.8 v=not_in_scope
why: the flight-type select's arrow pin and stand_keyboard's test count, which the chunk adds nothing to, a re-check every change owes
