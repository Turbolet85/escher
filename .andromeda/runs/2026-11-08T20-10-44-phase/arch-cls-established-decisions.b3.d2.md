@answer n=1 entity=established-decisions#25.2 v=no
why: a focus-timing limit of the buffered render; the chunk calls no set_focus_to and changes no render path
@answer n=2 entity=established-decisions#25.3 v=yes
why: the new record wraps runtime.handle_event on the event path, where a task spawned from an event handler is polled inside render_immediate's borrow, so the record must not borrow during that call
@answer n=3 entity=established-decisions#26.1 v=yes
why: the optional owner the coverage list keys on is read by this public-API walk, and a deleted row resolving to none bounds that owner value
@answer n=4 entity=established-decisions#26.2 v=yes
why: the scope requires the exercised record to hold on apps with no registry, and this entity states what a registry-less app leaves unreachable and what use_graph_state reaches
@answer n=5 entity=established-decisions#27.1 v=yes
why: the chunk changes the bridge's handle_event, which this entity says sets the registry dispatch only where a registry stands and leaves the rest untouched
@answer n=6 entity=established-decisions#28.1 v=no
why: browser app history, favicons and about pages are not reached by the coverage read or the replay test
@answer n=7 entity=established-decisions#29.1 v=no
why: browser app tab lenses are not reached by the chunk
@answer n=8 entity=established-decisions#30.1 v=no
why: WPT runner rules are not reached by the driver chunk
@answer n=9 entity=established-decisions#31.1 v=yes
why: the driver socket and its call and answer carriers, the no-NodeId crossing, and the tool_call and command span fields bound the new coverage verb and the owed replay
@answer n=10 entity=established-decisions#31.2 v=no
why: the select list's bounds and shot crossing rule concerns select rows and open-list pixels, which the chunk neither reads nor paints
@answer n=11 entity=established-decisions#31.3 v=yes
why: the graph verb's answer holds the stable ids, owner ids, component names, state names and handler event names the coverage list is built from, never a value or NodeId
@answer n=12 entity=established-decisions#31.4 v=yes
why: the owed replay test reads the actions answer and compares it key by key, and this entity states that answer's ids, owner ids, verb words and cause words
@answer n=13 entity=established-decisions#32.1 v=yes
why: the owed test records a session's answered calls as an owner-only kept check, reads it back and replays it through Check::replay and Session::run, which this entity states
