@answer n=1 entity=log-coverage#1.1 v=no
why: the closed-span line format of escher-driver's spans is unchanged, and the session touches no telemetry crate and no driver code
@answer n=2 entity=log-coverage#2.1 v=no
why: the cold-agent verdict fields and their reasons live in scripts the session does not change
@answer n=3 entity=log-coverage#3.1 v=no
why: span_counts reads the cold-agent pipe's server log, a path the session does not touch
@answer n=4 entity=log-coverage#3.2 v=no
why: the live server child's record directory is set up in scripts/cold-agent.sh, which the session does not change
@answer n=5 entity=log-coverage#3.3 v=no
why: tasks_met reads snapshot answers in scripts/cold-agent.sh, which the session does not change
@answer n=6 entity=log-coverage#4.1 v=no
why: apps/browser's log sites sit in the browser app and its persistence crate, neither reached by the dioxus-native-dom change
@answer n=7 entity=log-coverage#5.1 v=no
why: the example output and log sites are unchanged, as the session touches no example and no driver code
@answer n=8 entity=log-coverage#6.1 v=no
why: the driver's command and tool_call spans, their fields and their emitters are unchanged, the driver being untouched
@answer n=9 entity=log-coverage#7.1 v=no
why: docs/capabilities.md restates the driver's spans and the scrub's limits, and the session changes neither the spans nor the scrub
@answer n=10 entity=log-coverage#8.1 v=no
why: the blitz-dom files holding these log sites are not reached by the change, and the focus line still fires once per focus move
@answer n=11 entity=log-coverage#8.2 v=no
why: blitz-dom's events and util log sites sit in files the change does not reach
@answer n=12 entity=log-coverage#8.3 v=no
why: blitz-dom's layout log sites sit in files the change does not reach
@answer n=13 entity=log-coverage#8.4 v=no
why: blitz-dom's node log sites and its select-list census sit in files the change does not reach
@answer n=14 entity=log-coverage#8.5 v=no
why: blitz-net's log sites sit in a crate the change does not reach
@answer n=15 entity=log-coverage#8.6 v=no
why: blitz-html's parse-error logging sits in a crate the change does not reach
@answer n=16 entity=log-coverage#8.7 v=no
why: blitz-shell and blitz-paint log sites sit in crates the change does not reach
@answer n=17 entity=log-coverage#8.8 v=no
why: the blitz and blitz-vibey-script log sites sit in crates the change does not reach
@answer n=18 entity=log-coverage#8.9 v=yes mode=add-to
why: the bridge now writes mutations through the new buffer and replays them, re-firing the same trace! calls, and F1 joins this emitter row because the buffer holds no log site and the log stream is unchanged
@answer n=19 entity=log-coverage#8.10 v=no
why: the blitz-tests diagnostics named here sit in other test files the change does not touch
@answer n=20 entity=log-coverage#8.11 v=no
why: the WPT runner's log calls sit in a crate the change does not reach
@answer n=21 entity=log-coverage#8.12 v=no
why: the publish-build logging sits in CI workflow files the session does not change
@answer n=22 entity=log-coverage#9.1 v=no
why: the console-call census covers the s02 files, none of which the change reaches, and the change adds no console call
@answer n=23 entity=log-coverage#9.2 v=no
why: the structured-logging census covers the examples slice, which the change does not reach
@answer n=24 entity=log-coverage#9.3 v=no
why: its census covers the 0.2.0 requirement record's test additions, none of which the session touches
@answer n=25 entity=log-coverage#9.4 v=yes mode=add-to
why: the probe's two renamed pins and their helper spawned_focus_acts install no subscriber, read no env var, print nothing and bind no listener, so F3 joins this silence record of blitz-tests test targets with the census over the probe reading 0
