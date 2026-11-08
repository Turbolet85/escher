@answer n=1 entity=log-coverage#1.1 v=no
why: the closed-span line format, its allowlist and its field order are unchanged, and the session adds no span, field or line
@answer n=2 entity=log-coverage#2.1 v=no
why: the verdict field set and the counted verbs lint, screenshot and advance are unchanged, and actions joins the pipe uncounted
@answer n=3 entity=log-coverage#2.2 v=yes mode=stale
why: the pipe's verb list gains actions after graph, so its thirteen words become fourteen, uncounted and in no verdict field
@answer n=4 entity=log-coverage#3.1 v=no
why: span_counts still reads only the counted verbs lint, screenshot and advance, and actions is not one of them
@answer n=5 entity=log-coverage#3.2 v=no
why: the recording directory, its mode and the pipe's never-printed kept checks are unchanged by the session
@answer n=6 entity=log-coverage#3.3 v=no
why: tasks_met still reads accepted snapshot answers only, and the session changes no snapshot path
@answer n=7 entity=log-coverage#4.1 v=no
why: the apps/browser log sites are upstream of the session and the session changes none of them
@answer n=8 entity=log-coverage#5.1 v=no
why: the fixed endings, their help-after-stop order and the examples' print sites hold, and the session adds no print or log site
@answer n=9 entity=log-coverage#6.1 v=yes mode=stale
why: the verb table moves from thirteen words, nine instance-level, to fourteen and ten, with actions the tenth instance verb after graph, so both counts in the command span and tool_call lines are stale
@answer n=10 entity=log-coverage#7.1 v=no
why: docs/capabilities.md states the span set, the closed-word fields and the scrub limits, none moved by the session, and it states no verb count
@answer n=11 entity=log-coverage#8.1 v=no
why: blitz-dom is diffed unchanged and the actions read makes no focus call, so no focus line appears or moves
@answer n=12 entity=log-coverage#8.2 v=no
why: blitz-dom events and util are diffed unchanged and no event or log site in them is reached
@answer n=13 entity=log-coverage#8.3 v=no
why: blitz-dom layout is diffed unchanged and the session reaches no layout log site
@answer n=14 entity=log-coverage#8.4 v=no
why: blitz-dom node is diffed unchanged and the session adds no print or log site there
@answer n=15 entity=log-coverage#8.5 v=no
why: blitz-net is diffed unchanged and the session fetches nothing new
@answer n=16 entity=log-coverage#8.6 v=no
why: blitz-html is diffed unchanged and the session parses no HTML through its sink
@answer n=17 entity=log-coverage#8.7 v=no
why: blitz-shell and blitz-paint are diffed unchanged and their error and warn sites are not moved
@answer n=18 entity=log-coverage#8.8 v=no
why: blitz and blitz-vibey-script are diffed unchanged and their log sites and bootstrap globals are not reached
@answer n=19 entity=log-coverage#8.9 v=no
why: the mutation writer's trace sites are unchanged, the session adds no log site to dioxus-native-dom, and the data-dioxus-id mark is still written by the bridge
@answer n=20 entity=log-coverage#8.10 v=no
why: the mutation buffer holds no log site and the session adds none to it or to initial_build and poll
@answer n=21 entity=log-coverage#8.11 v=yes mode=add-to
why: the new dioxus-native-dom readings join this no-site group, F1 with the stand_actions check and actionable_nodes holding no log site, subscriber, env read or span and escher-driver keeping its 2 info_span! sites, and F4 with the read making no pump, dispatch, scroll_into_view or focus call
@answer n=22 entity=log-coverage#8.12 v=no
why: DioxusEventHandler::handle_event is unchanged and the actions read dispatches nothing
@answer n=23 entity=log-coverage#8.13 v=no
why: the blitz-tests println diagnostics stand and stand_actions prints nothing, so no diagnostic line is added or moved
@answer n=24 entity=log-coverage#8.14 v=no
why: the wpt runner is diffed unchanged and the session reaches none of its log calls
@answer n=25 entity=log-coverage#8.15 v=no
why: publish-browser.yml is outside the session's change and CI stays at 17 checks with no leg or artifact moved
