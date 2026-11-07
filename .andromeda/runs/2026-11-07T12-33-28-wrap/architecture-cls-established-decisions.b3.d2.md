@answer n=1 entity=established-decisions#26.1 v=yes mode=stale
why: the probe file is reached whole and the walk test's cited lines 198-279 in dioxus_runtime_probe.rs cannot be shown to hold after the pin rewrite, so the citation may have moved
@answer n=2 entity=established-decisions#26.2 v=yes mode=stale
why: the probe file is reached whole and the signal-readers test's cited lines 471-512 in dioxus_runtime_probe.rs cannot be shown to hold after the pin rewrite, so the citation may have moved
@answer n=3 entity=established-decisions#27.1 v=no
why: the browser app's cited files are not reached by the session, which touches only the bridge file, the crate root and the probe
@answer n=4 entity=established-decisions#28.1 v=no
why: the tab lens claim cites apps/browser/src/tab.rs, which the session does not reach
@answer n=5 entity=established-decisions#29.1 v=no
why: the WPT runner's cited files are not reached and its claims do not touch the Dioxus bridge
@answer n=6 entity=established-decisions#30.1 v=no
why: the driver session's cited files are not reached, and its boundary, dependencies and bridge calls stay as they were
@answer n=7 entity=established-decisions#30.2 v=no
why: the select list's crossing and its escher-0.3.0 measurement are unaffected by the buffer change
@answer n=8 entity=established-decisions#31.1 v=no
why: the kept-check recording is driver-side and the session does not reach it
