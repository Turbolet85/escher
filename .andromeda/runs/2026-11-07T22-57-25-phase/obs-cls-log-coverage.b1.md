@answer n=1 entity=log-coverage#1.1 v=yes
why: the new verb's command span is a closed span in this line format, and each of its pairs passes the scrub under escher_driver (task_item 4, 10)
@answer n=2 entity=log-coverage#2.1 v=yes
why: counts_agree needs the new MCP tool to emit one tool_call line per escher tool use (task_item 5, 9)
@answer n=3 entity=log-coverage#3.1 v=no
why: the cold-agent pipe reads accepted tool_call lines of lint, screenshot and advance only, and the new verb and tool are not among them
@answer n=4 entity=log-coverage#3.2 v=yes
why: the recorded step writes each answered call's answer, including the new outcome, into the kept check this directory holds, which the pipe never prints or copies (task_item 8)
@answer n=5 entity=log-coverage#3.3 v=no
why: tasks_met reads accepted snapshot answers only, and the new verb's answer is not a snapshot
@answer n=6 entity=log-coverage#4.1 v=no
why: apps/browser is not touched by the chunk (not_needed item 1)
@answer n=7 entity=log-coverage#5.1 v=yes
why: escher-session's usage line built from VERBS, its one-line JSON stdout for accepted and refused calls, and describe()'s one line gain the new verb and field (task_item 11, 13)
@answer n=8 entity=log-coverage#6.1 v=yes
why: the new verb's Session::run command span and the new MCP tool's tool_call span are the closed-span lines this rule sets, with closed-word fields (task_item 4, 5)
@answer n=9 entity=log-coverage#7.1 v=no
why: the chunk adds no span and changes no scrub limit, the trigger for a hand edit of docs/capabilities.md, and that file is not reached
@answer n=10 entity=log-coverage#8.1 v=no
why: blitz-dom's focus, resolve and hit-test logs are engine sites the chunk only reads through and leaves unchanged (not_needed item 8)
@answer n=11 entity=log-coverage#8.2 v=no
why: blitz-dom's IME, link and WOFF log sites are engine logs the chunk does not touch (not_needed item 8)
@answer n=12 entity=log-coverage#8.3 v=no
why: blitz-dom's layout log sites are engine logs the chunk does not touch (not_needed item 8)
@answer n=13 entity=log-coverage#8.4 v=no
why: the logged property values are CSS values in blitz-dom's node code, not a state or control value the graph's masking rule covers
@answer n=14 entity=log-coverage#8.5 v=no
why: blitz-net fetch logs are not reached, since the graph reads no network (not_needed item 7)
@answer n=15 entity=log-coverage#8.6 v=no
why: blitz-html parse-error logs are an engine site the chunk does not touch (not_needed item 8)
@answer n=16 entity=log-coverage#8.7 v=no
why: blitz-shell and blitz-paint error and warn sites are engine logs the chunk does not touch (not_needed item 8)
@answer n=17 entity=log-coverage#8.8 v=no
why: blitz and blitz-vibey-script logs are not reached, the chunk makes no script runtime change (not_needed item 3)
@answer n=18 entity=log-coverage#8.9 v=yes
why: mutation_writer's trace calls record text-node contents and attribute values in dioxus-native-dom, and the graph code must add no such site (task_item 2)
@answer n=19 entity=log-coverage#8.10 v=no
why: the mutation_buffer silence census is a prior chunk's measurement of a module the chunk does not change
@answer n=20 entity=log-coverage#8.11 v=no
why: paint_tree_bench and br_trailing_line diagnostics are other test files, and stand_non_lean_tasks is edited only at its pinned-reading unit
@answer n=21 entity=log-coverage#8.12 v=no
why: no wpt code is touched (not_needed item 2)
@answer n=22 entity=log-coverage#8.13 v=no
why: the chunk adds no CI job or artifact (not_needed item 5)
@answer n=23 entity=log-coverage#9.1 v=no
why: a console-call absence over the s02 files is an earlier chunk's past measurement the chunk does not re-run (not_needed item 10)
@answer n=24 entity=log-coverage#9.2 v=yes
why: the structured-logging silence census over the examples slice covers the seven_guis stand, whose census at 0 the chunk must keep (task_item 12)
@answer n=25 entity=log-coverage#9.3 v=no standing=S1
why: each new or extended test check installs no subscriber, reads no env var and prints nothing, with the silence census reading 0 over the file
