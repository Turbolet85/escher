@answer n=1 entity=log-coverage#1.1 v=no
why: escher-telemetry's span line format and scrub are untouched, since no telemetry crate code, target prefix or field set moved
@answer n=2 entity=log-coverage#2.1 v=no
why: the verdict fields of the cold-agent pipe are unchanged, since no harness, leg or CI step moved
@answer n=3 entity=log-coverage#3.1 v=no
why: span_counts in the cold-agent pipe is unchanged, since the driver and harness are untouched
@answer n=4 entity=log-coverage#3.2 v=no
why: the cold-agent record directory and its pipe reads are unchanged, since no harness or leg moved
@answer n=5 entity=log-coverage#3.3 v=no
why: tasks_met and the replay's expect match in the cold-agent pipe are unchanged, since no harness file is reached
@answer n=6 entity=log-coverage#4.1 v=no
why: apps/browser's loader, toolbar and persistence log sites lie in files the session does not reach
@answer n=7 entity=log-coverage#5.1 v=no
why: the examples' print calls and escher-session's lines are unchanged, since no example or driver file is reached
@answer n=8 entity=log-coverage#6.1 v=no
why: the escher-driver's command and tool_call spans are unchanged, since the driver is untouched and the log stream is the same
@answer n=9 entity=log-coverage#7.1 v=no
why: docs/capabilities.md restates the driver's span reach and scrub limits, and the session moves neither
@answer n=10 entity=log-coverage#8.1 v=no
why: the blitz-dom log sites stand, since no blitz-dom file is reached and the session adds no log site to any of them
@answer n=11 entity=log-coverage#8.2 v=no
why: blitz-dom's event and util log sites are unchanged, since no blitz-dom file is reached
@answer n=12 entity=log-coverage#8.3 v=no
why: blitz-dom's layout log sites are unchanged, since no layout file is reached
@answer n=13 entity=log-coverage#8.4 v=no
why: blitz-dom's node log sites and the select list's silence record stand, since no node file is reached
@answer n=14 entity=log-coverage#8.5 v=no
why: blitz-net's log sites are unchanged, since no blitz-net file or dependency moved
@answer n=15 entity=log-coverage#8.6 v=no
why: blitz-html's parse-error log is unchanged, since no blitz-html file is reached
@answer n=16 entity=log-coverage#8.7 v=no
why: blitz-shell's and blitz-paint's log sites are unchanged, since no file of either crate is reached
@answer n=17 entity=log-coverage#8.8 v=no
why: blitz's launch log and blitz-vibey-script's logs are unchanged, since neither crate is reached
@answer n=18 entity=log-coverage#8.9 v=yes mode=add-to
why: the new private mutation_buffer holds no log site and the replay re-fires MutationWriter's trace! calls, so the emitter line set is unchanged (F1)
@answer n=19 entity=log-coverage#8.10 v=no
why: the named tests/blitz-tests files and the log-phase-times feature are unchanged, and the probe's added pins print nothing (census 0)
@answer n=20 entity=log-coverage#8.11 v=no
why: wpt/runner's log calls are unchanged, since no WPT file is reached
@answer n=21 entity=log-coverage#8.12 v=no
why: the publish workflow's CARGO_LOG setting and CI steps are unchanged, since no CI step moved
@answer n=22 entity=log-coverage#9.1 v=no
why: the console absence stands, since the added lines hold no console token and no file is merged or conflicted
@answer n=23 entity=log-coverage#9.2 v=no
why: the structured-logging absence over the examples slice stands, since the session adds no tracing token and reaches no example
@answer n=24 entity=log-coverage#9.3 v=no
why: the 0.2.0 requirement record's test additions lie outside the session's reached files, so their absence is not moved
@answer n=25 entity=log-coverage#9.4 v=yes mode=add-to
why: the probe's two renamed pins and helper spawned_focus_acts install no subscriber, read no env var, print nothing and bind no listener, the census reading 0 (F3)
