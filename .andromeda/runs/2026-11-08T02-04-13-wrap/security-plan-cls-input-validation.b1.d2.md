@answer n=1 entity=input-validation#1.1 v=no
why: the URL argument rule lives in examples/screenshot.rs and examples/paint_bench.rs, which the change does not reach and the example tools are untouched
@answer n=2 entity=input-validation#1.2 v=no
why: the numeric argument fallbacks live in the example tools, which the change does not reach and which are untouched
@answer n=3 entity=input-validation#1.3 v=no
why: the output filename rule lives in examples/screenshot.rs, which the change does not reach
@answer n=4 entity=input-validation#1.4 v=no
why: the preact_script path rule lives in examples/preact_script.rs, which the change does not reach
@answer n=5 entity=input-validation#1.5 v=no
why: agent-run.sh is not reached by the change and its argument handling is untouched
@answer n=6 entity=input-validation#1.6 v=no
why: the cold-agent script's own verbs, task allowlist, fixed STAND_PROMPT and preconditions still hold, the change adding graph only to the pipe's VERBS table, which this row does not state
@answer n=7 entity=input-validation#1.7 v=no
why: apt-install.sh is not reached by the change and its argument handling is untouched
@answer n=8 entity=input-validation#1.8 v=no
why: coverage_floor.py is not reached by the change and its three fixed floors are unchanged
@answer n=9 entity=input-validation#1.9 v=yes mode=stale
why: the pipe validates each check step against the schema, which now holds thirteen verbs with graph added, so the count of twelve verbs reads stale
@answer n=10 entity=input-validation#1.10 v=yes mode=stale
why: the node keys go from thirteen to fourteen with clickable, so the NODE_FIELDS and node-key counts read stale, and the socket now carries the UI graph answer
@answer n=11 entity=input-validation#1.11 v=no
why: the highlighted row still crosses no carrier, since the graph holds only ids, names, state names, event names, flag words and booleans, and clickable is a boolean
@answer n=12 entity=input-validation#1.12 v=no
why: the state and recording directory rules live in host.rs and session.rs, which the change does not reach, and neither the graph nor clickable touches permissions or paths
@answer n=13 entity=input-validation#1.13 v=yes mode=stale
why: the recorder's rule that a call outside the eight instance verbs is not recorded reads stale, since graph is the ninth instance verb and is recorded
@answer n=14 entity=input-validation#1.14 v=yes mode=stale
why: the strict reader's tool word is one of the eight instance verbs, now nine with graph, and a kept check may now hold a graph step and its answer
@answer n=15 entity=input-validation#1.15 v=no
why: the session label rule is unchanged, session.rs is not reached, and the graph carries no caller string
@answer n=16 entity=input-validation#1.16 v=yes mode=stale
why: the closed table of twelve verbs and the seven and eight instance-verb counts move to thirteen and nine with graph added as the ninth instance verb taking no argument
@answer n=17 entity=input-validation#1.17 v=yes mode=stale
why: the escher-session usage line gains graph after lint, and graph answers one uncoloured JSON line on stdout with status 0
@answer n=18 entity=input-validation#1.18 v=yes mode=stale
why: tools/list now carries thirteen verbs with graph after lint, and the MCP results carry the UI graph as a text block
@answer n=19 entity=input-validation#1.19 v=no
why: the browser urlbar lives in apps/browser, which is untouched and not reached by the change
@answer n=20 entity=input-validation#1.20 v=no
why: external navigation lives in apps/browser and dioxus-native link_handler.rs, neither reached by the change
@answer n=21 entity=input-validation#1.21 v=no
why: link href resolution lives in blitz-dom, which the change did not touch
@answer n=22 entity=input-validation#1.22 v=no
why: DocumentUrl parsing lives in blitz-dom, which the change did not touch
@answer n=23 entity=input-validation#1.23 v=no
why: fragment decoding lives in blitz-dom scrolling, which the change did not touch
@answer n=24 entity=input-validation#1.24 v=no
why: file URL reading lives in blitz-net, which the change did not touch
@answer n=25 entity=input-validation#1.25 v=no
why: data URL decoding lives in blitz-net, which the change did not touch
