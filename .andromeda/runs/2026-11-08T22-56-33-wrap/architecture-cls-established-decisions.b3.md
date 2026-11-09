@answer n=1 entity=established-decisions#25.2 v=no
why: the buffered render, initial_build and poll are untouched and the reached record sits only in handle_event, so the row's limit still holds
@answer n=2 entity=established-decisions#25.3 v=no
why: the reached walk sits in handle_event before runtime handle_event and takes no borrow across render_immediate, so the poll and borrow sequence this row records still holds
@answer n=3 entity=established-decisions#26.1 v=no
why: the owning-component walk through public API in the probe file is not reached, and the new reached module reads listeners rather than owners
@answer n=4 entity=established-decisions#26.2 v=no
why: a held signal's readers, writes and the standing registry dispatch they run against are unchanged, the reached record sitting beside that dispatch
@answer n=5 entity=established-decisions#27.1 v=yes mode=contradict
why: the reached record now adds a ReachedRecord lookup on every document's dispatch, so the change no longer does nothing without a registry
@answer n=6 entity=established-decisions#28.1 v=no
why: the browser app's history, tabs and about pages are outside the files the session reaches
@answer n=7 entity=established-decisions#29.1 v=no
why: tab lenses and webview props are browser app code the session does not reach
@answer n=8 entity=established-decisions#30.1 v=no
why: the WPT runner's targets, harness and report are not reached by the session
@answer n=9 entity=established-decisions#31.1 v=yes mode=add-to
why: the coverage role adds a kept-check reading from the caller's argv files, a ratified boundary widening, and the command line becomes a second reader of kept-check bytes
@answer n=10 entity=established-decisions#31.2 v=no
why: the select list's bounds and screenshot crossings are unchanged by the session
@answer n=11 entity=established-decisions#31.3 v=no
why: the coverage line carries only handler ids, owner ids and event names that the graph answer already crosses under the ratified carriers, and the graph answer itself is unchanged
@answer n=12 entity=established-decisions#31.4 v=yes mode=add-to
why: F5 places handler coverage's reading of every screen the replays reach, held PROVISIONAL until the epoch boundary, beside the actions keyed reading
@answer n=13 entity=established-decisions#32.1 v=no
why: the replay path still runs through Session run alone with its behaviour unchanged, and the two committed Counter checks are unchanged
