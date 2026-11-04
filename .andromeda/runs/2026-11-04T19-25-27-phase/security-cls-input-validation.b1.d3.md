@answer n=1 entity=input-validation#1.1 v=no
why: upstream CLI example screenshot.rs and paint_bench.rs URL argument; not reached by the chunk and named not_needed n=6
@answer n=2 entity=input-validation#1.2 v=no
why: numeric argument fallbacks in paint_bench, screenshot and multi_doc; upstream examples the chunk does not touch (not_needed n=6)
@answer n=3 entity=input-validation#1.3 v=no
why: screenshot output filename in examples/screenshot.rs; upstream example not reached (not_needed n=6)
@answer n=4 entity=input-validation#1.4 v=no
why: preact_script path canonicalization in an upstream example; script execution and upstream examples are not_needed n=3 and n=6
@answer n=5 entity=input-validation#1.5 v=no
why: agent-run.sh selection rule is that script's own input check; the scope names no agent-run.sh run and the script is not a reached file
@answer n=6 entity=input-validation#1.6 v=no
why: cold-agent.sh verb and task allowlist; the cold-agent pipe is not_needed n=7
@answer n=7 entity=input-validation#1.7 v=no
why: apt-install.sh package names and options in ci.yml run steps; the chunk adds no CI apt call
@answer n=8 entity=input-validation#1.8 v=no
why: coverage_floor.py counts source lines under three crate directories; the probe adds no line to them and reads no floor
@answer n=9 entity=input-validation#1.9 v=no
why: cold-agent check reply parsing; the cold-agent pipe is not_needed n=7
@answer n=10 entity=input-validation#1.10 v=yes
why: the driver socket's closed grammar, answer bounds and masked control values (password and file input as the mask) are the carrier the served task's probe must stay inside (needs n=3, n=4)
@answer n=11 entity=input-validation#1.11 v=no
why: an open select list's highlighted row crosses no carrier; the scope names no select list and the probe reads no such row
@answer n=12 entity=input-validation#1.12 v=yes
why: a served task run through the driver session must create its state directory 0700, its socket 0600 and take the path as a parameter (needs n=9)
@answer n=13 entity=input-validation#1.13 v=no standing=S2
why: the convention a new test making a recording directory owes (0700 directory, 0600 files, non-recursive removal); the scope names no recording directory
@answer n=14 entity=input-validation#1.14 v=no
why: strict reader of a kept check's bytes and its bounds over the two committed checks; the probe reads and adds no kept check, a re-check every change owes
@answer n=15 entity=input-validation#1.15 v=yes
why: a served task's session label must be 1 to 32 bytes of a-z, 0-9 and dash, checked before boot (needs n=9)
@answer n=16 entity=input-validation#1.16 v=yes
why: every probe call passes the twelve-verb schema's validate inside Session::run and adds no verb, argument or field (needs n=3, n=9)
@answer n=17 entity=input-validation#1.17 v=yes
why: the escher-session command line a served task runs through reads no environment variable and writes no caller-typed value back (needs n=10, n=4)
@answer n=18 entity=input-validation#1.18 v=no
why: MCP server JSON-RPC framing and tools/list; the probe makes no MCP call and adds no verb to the listing, and the scope does not name MCP
@answer n=19 entity=input-validation#1.19 v=no
why: browser urlbar parsing is a browser navigation surface, not_needed n=8
@answer n=20 entity=input-validation#1.20 v=no
why: navigation to the external browser, method and scheme; browser navigation surface, not_needed n=8
@answer n=21 entity=input-validation#1.21 v=no
why: link href resolution is a browser navigation surface, not_needed n=8
@answer n=22 entity=input-validation#1.22 v=no
why: DocumentUrl parsing; the probe builds in-process Dioxus documents from fixed inputs and the scope names no document URL
@answer n=23 entity=input-validation#1.23 v=no
why: URL fragment percent-decoding is a browser navigation surface, not_needed n=8
@answer n=24 entity=input-validation#1.24 v=no
why: file: URL path reading in the net provider; the probe fetches nothing and it is not_needed n=2
@answer n=25 entity=input-validation#1.25 v=no
why: data: URL decoding in the net provider; the probe fetches nothing and it is not_needed n=2
