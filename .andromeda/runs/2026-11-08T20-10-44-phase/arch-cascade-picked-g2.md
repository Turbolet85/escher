@account n=1 entity=established-decisions#4.1 v=not_in_scope
why: renderer backends and the harness screenshot are only used, and the scope's kept checks and replay test hold no screenshot step whose renderer it reads
@account n=2 entity=established-decisions#5.1 v=carried
why: the coverage read and the exercised record land in dioxus-native-dom beside the UI graph, which its default `accessibility` feature gates, so the new code must sit inside that default set and leave it unchanged
@account n=3 entity=established-decisions#6.1 v=not_in_scope
why: document defaults (incremental layout, media type, quirks mode, Stylo prefs, colour scheme) are only used by the stand's documents and nothing the chunk reads or re-measures reads them
@account n=4 entity=established-decisions#7.1 v=not_in_scope
why: resource loading is only used by the load page's boot, and the coverage read and replay test act on no load, cap or stylesheet ordering
@account n=5 entity=established-decisions#10.1 v=carried
why: keyboard activation and the range input dispatch Click and `input` to the app's handlers, so which (element, event) handlers a run reached must count those engine-dispatched events, the record kept on the bridge's dispatch path
@account n=6 entity=established-decisions#11.1 v=not_in_scope
why: the seven engine changes (inert, toggle read, contain-paint hit, password caret, transformed rect) dispatch no app event the coverage record must attribute and the chunk edits none of their files
@account n=7 entity=established-decisions#11.2 v=carried
why: the engine's `change` on a checkbox and a radio reaches the app's `change` handler with no driver call naming that event, so the exercised record must count it as reached
@account n=8 entity=established-decisions#11.3 v=not_in_scope
why: the anonymous-block `layout_parent` fix is engine layout bookkeeping that dispatches no event and lives in no file the chunk reaches
@account n=9 entity=established-decisions#11.5 v=carried
why: every select choice fires `input` then `change` through `apply_select_choice` and a choice of the chosen option fires nothing, which fixes which select handlers a run reaches and the record must count exactly that
@account n=10 entity=established-decisions#11.6 v=not_in_scope
why: the select list's opening and painting is engine behaviour the chunk only uses, stating no event dispatched to an app handler
@account n=11 entity=established-decisions#11.7 v=not_in_scope
why: an open list's closes are engine behaviour the chunk only uses, and dispatching no event they leave the exercised record untouched by their own code
@account n=12 entity=established-decisions#11.8 v=carried
why: an open list's Enter, Space and row click choose through `apply_select_choice`, so its `input` and `change` reach the app's handlers and the record must count them
@account n=13 entity=established-decisions#11.10 v=not_in_scope
why: the select list's standing limits (thumb, contrast, font clip, highlight, size) touch no handler, graph node or check the chunk reads
@account n=14 entity=established-decisions#11.12 v=not_in_scope
why: a `multiple` select's key model is engine behaviour the chunk only uses, stating which rows a key chooses and no event dispatched to an app handler
