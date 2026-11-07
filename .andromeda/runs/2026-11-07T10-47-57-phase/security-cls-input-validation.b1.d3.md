@answer n=1 entity=input-validation#1.1 v=no
why: the examples' URL argument parsing is not reached; the chunk edits no example

@answer n=2 entity=input-validation#1.2 v=no
why: the examples' numeric argument fallbacks are not reached by the chunk

@answer n=3 entity=input-validation#1.3 v=no
why: screenshot's output filename is in an example the chunk does not reach

@answer n=4 entity=input-validation#1.4 v=no
why: preact_script's path canonicalization is in an example the chunk does not reach

@answer n=5 entity=input-validation#1.5 v=no
why: agent-run.sh's verb and selection grammar is not edited and the chunk adds no test file

@answer n=6 entity=input-validation#1.6 v=no
why: cold-agent.sh's verb and task allowlist is not edited and the chunk adds no task

@answer n=7 entity=input-validation#1.7 v=no
why: apt-install.sh's package and option grammar is not reached; no ci.yml step is edited

@answer n=8 entity=input-validation#1.8 v=no
why: coverage_floor.py's argv grammar is not edited and the chunk passes it no floor

@answer n=9 entity=input-validation#1.9 v=no
why: the cold-agent check's extraction and replay are not reached by the chunk

@answer n=10 entity=input-validation#1.10 v=yes
why: the driver's answers carry the masked password and file-input values that the scope keeps identical (task item 7), and the buffer must never carry an attribute value as typed

@answer n=11 entity=input-validation#1.11 v=no
why: an open select list's highlighted row crosses no carrier and the chunk reaches no select popup

@answer n=12 entity=input-validation#1.12 v=no
why: the driver state directory's creation and mode rules are untouched by the chunk

@answer n=13 entity=input-validation#1.13 v=no
why: the driver recording directory and its check files are untouched by the chunk

@answer n=14 entity=input-validation#1.14 v=yes
why: the entity names stand_replay, a stand_* check the chunk re-runs as its witness, and that replay reads the committed checks the port must keep accepting

@answer n=15 entity=input-validation#1.15 v=no
why: the driver session label validation is not reached by the chunk

@answer n=16 entity=input-validation#1.16 v=yes
why: the entity names stand_act_spans, a stand_* check the chunk re-runs as its witness, for the command span's eight fields and the masked control values in Outcome's Debug output

@answer n=17 entity=input-validation#1.17 v=no
why: the escher-session argv reader and its command endings are not edited and not reached

@answer n=18 entity=input-validation#1.18 v=yes
why: the MCP results carry the masked password and file-input values that the scope keeps identical (task item 7), while the framing and tool table are untouched

@answer n=19 entity=input-validation#1.19 v=no
why: browser urlbar parsing is in apps/browser, which the chunk does not reach

@answer n=20 entity=input-validation#1.20 v=no
why: external navigation's method and scheme limits are in link_handler, which the chunk does not change

@answer n=21 entity=input-validation#1.21 v=no
why: link href resolution in blitz-dom's pointer events is not edited and the scope names no href

@answer n=22 entity=input-validation#1.22 v=no
why: DocumentUrl's parse error handling in blitz-dom is not edited and the chunk adds no URL parse

@answer n=23 entity=input-validation#1.23 v=no
why: URL fragment decoding in blitz-dom's scrolling is not edited and the scope names no fragment

@answer n=24 entity=input-validation#1.24 v=no
why: file: URL reads in the net provider are not reached, since the buffered render fetches nothing

@answer n=25 entity=input-validation#1.25 v=no
why: data: URL decoding in the net provider is not reached, since the buffered render fetches nothing
