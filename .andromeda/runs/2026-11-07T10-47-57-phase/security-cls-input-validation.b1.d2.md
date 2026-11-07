@answer n=1 entity=input-validation#1.1 v=no
why: URL argument parsing in examples/screenshot.rs and paint_bench.rs; the chunk edits no example and the scope puts the examples' CLI readers out of reach.
@answer n=2 entity=input-validation#1.2 v=no
why: numeric argument fallbacks in the examples; not reached by the chunk.
@answer n=3 entity=input-validation#1.3 v=no
why: screenshot output filename derivation in examples/screenshot.rs; not reached by the chunk.
@answer n=4 entity=input-validation#1.4 v=no
why: preact_script path canonicalization in examples/preact_script.rs; not reached by the chunk.
@answer n=5 entity=input-validation#1.5 v=no
why: agent-run.sh verb and selection grammar; the scope puts agent-run.sh's argv grammar out of reach and the chunk changes no script.
@answer n=6 entity=input-validation#1.6 v=no
why: cold-agent.sh verb, task allowlist and fixed prompt; the chunk edits no script and the pipe's grammar is not reached.
@answer n=7 entity=input-validation#1.7 v=no
why: apt-install.sh package and option grammar in CI; the chunk edits no workflow or CI script.
@answer n=8 entity=input-validation#1.8 v=no
why: coverage_floor.py argv grammar and its floors; the coverage script is not a file the chunk reaches and the scope names no coverage witness.
@answer n=9 entity=input-validation#1.9 v=no
why: cold-agent check extraction and replay of the agent's reply; the pipe and its MCP children are not reached by the chunk.
@answer n=10 entity=input-validation#1.10 v=no
why: driver session socket request grammar and bounds; escher-driver is not touched and the chunk adds no verb, carrier or listener.
@answer n=11 entity=input-validation#1.11 v=no
why: highlighted row of an open select list crosses no structured carrier; the chunk changes no select or lint path.
@answer n=12 entity=input-validation#1.12 v=no
why: driver state directory path and mode checks; escher-driver is not touched.
@answer n=13 entity=input-validation#1.13 v=no
why: driver recording directory creation and mode checks; escher-driver is not touched.
@answer n=14 entity=input-validation#1.14 v=yes
why: Check::from_bytes reads the committed stand_replay checks and Check::replay runs them through Session::run on the Dioxus stand; the chunk re-runs stand_* as a witness, so the buffered render must keep every kept answer equal.
@answer n=15 entity=input-validation#1.15 v=no
why: driver session label validation; escher-driver is not touched.
@answer n=16 entity=input-validation#1.16 v=no
why: driver command schema, validation order and executor refusals; the chunk edits no driver code and the stand witnesses do not measure these refusal rules.
@answer n=17 entity=input-validation#1.17 v=no
why: escher-session argv reader and its endings; not touched by the chunk.
@answer n=18 entity=input-validation#1.18 v=no
why: MCP server framing, tool argument checks and its no-panic probe; the chunk adds nothing to the MCP side.
@answer n=19 entity=input-validation#1.19 v=no
why: browser urlbar parsing in apps/browser; apps/browser is not reached.
@answer n=20 entity=input-validation#1.20 v=no
why: navigation method and scheme limits in link_handler.rs; the buffered render changes no link handling and that file is not reached.
@answer n=21 entity=input-validation#1.21 v=no
why: link href resolution in blitz-dom pointer events; not reached by the chunk.
@answer n=22 entity=input-validation#1.22 v=no
why: DocumentUrl parse error handling in blitz-dom; not reached by the chunk.
@answer n=23 entity=input-validation#1.23 v=no
why: URL fragment percent-decoding for scroll-to matching in blitz-dom; a separate path from the MountedData scroll ops the chunk relies on.
@answer n=24 entity=input-validation#1.24 v=no
why: file: URL reads in the net provider; the buffered render fetches nothing.
@answer n=25 entity=input-validation#1.25 v=no
why: data: URL decoding in the net provider; the buffered render fetches nothing.
