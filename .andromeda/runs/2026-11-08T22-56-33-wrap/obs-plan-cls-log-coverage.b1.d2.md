@answer n=1 entity=log-coverage#1.1 v=no
why: the session adds no span, field or target to the closed-span line format, which escher-telemetry keeps
@answer n=2 entity=log-coverage#2.1 v=no
why: the verdict fields belong to the cold-agent pipe, which the session does not reach
@answer n=3 entity=log-coverage#2.2 v=no
why: VERBS stays at fourteen words, the coverage role being a third command-line role and not a verb
@answer n=4 entity=log-coverage#3.1 v=no
why: span_counts reads the cold-agent server log, which the session does not touch
@answer n=5 entity=log-coverage#3.2 v=no
why: the cold-agent recording directory and its replay children are outside the session's change
@answer n=6 entity=log-coverage#3.3 v=no
why: tasks_met reads cold-agent snapshot answers, a path the session does not reach
@answer n=7 entity=log-coverage#4.1 v=no
why: apps/browser lies outside the session's changed files and its log sites are unmoved
@answer n=8 entity=log-coverage#5.1 v=yes mode=add-to
why: the coverage role's endings (coverage line on stdout status 0, malformed status 1, usage status 2, session error status 3) join escher-session's endings list (F3)
@answer n=9 entity=log-coverage#6.1 v=yes mode=add-to
why: the coverage role's replayed steps leave command span lines through Session::run (F3), and host_log gains a coverage run at trace reading 0 needles (F4)
@answer n=10 entity=log-coverage#7.1 v=no
why: the session adds no span, field or scrub limit, so the capability statement's span and scrub claims still hold on the changed file
@answer n=11 entity=log-coverage#8.1 v=no
why: its blitz-dom sites lie in files the session does not change, and the measured census adds none
@answer n=12 entity=log-coverage#8.2 v=no
why: blitz-dom events and util files are outside the session's changes, which add no log site
@answer n=13 entity=log-coverage#8.3 v=no
why: blitz-dom layout files are outside the session's changes, which add no log site
@answer n=14 entity=log-coverage#8.4 v=no
why: blitz-dom node files are outside the session's changes, which add no log site
@answer n=15 entity=log-coverage#8.5 v=no
why: blitz-net is outside the session's changed files and its log sites are unmoved
@answer n=16 entity=log-coverage#8.6 v=no
why: blitz-html is outside the session's changed files and its log site is unmoved
@answer n=17 entity=log-coverage#8.7 v=no
why: blitz-shell and blitz-paint are outside the session's changed files and their log sites are unmoved
@answer n=18 entity=log-coverage#8.8 v=no
why: blitz and blitz-vibey-script are outside the session's changed files and their log sites are unmoved
@answer n=19 entity=log-coverage#8.9 v=no
why: mutation_writer.rs, which holds these trace sites, is not reached and the session adds no log site
@answer n=20 entity=log-coverage#8.10 v=yes mode=add-to
why: the reached module and crate-private element_listener_ids join the crate-private modules with no log site or print (F1), beside the mutation buffer's group, in dioxus_document.rs, a reached file
@answer n=21 entity=log-coverage#8.11 v=yes mode=add-to
why: the coverage module's to_json, never logged or fielded, joins the Outcome::Graphed note (F2), and the stand_handler_coverage check joins the silence census beside stand_graph (F5)
@answer n=22 entity=log-coverage#8.12 v=no
why: actions.rs is not a reached file and the session adds no pump, dispatch or focus call to the read
@answer n=23 entity=log-coverage#8.13 v=yes mode=add-to
why: the reached record is written in handle_event beside the registry dispatch on the same path, before runtime.handle_event, with no log site (F1)
@answer n=24 entity=log-coverage#8.14 v=no
why: the session's new stand file and reached common module print nothing, so the println diagnostics claim holds
@answer n=25 entity=log-coverage#8.15 v=no
why: wpt/runner is outside the session's changed files and its log calls are unmoved
