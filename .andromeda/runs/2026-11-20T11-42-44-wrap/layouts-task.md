@task_prose part=session
The session restated one heading of the project's front page: the 0.3.0 version under Status moved from built, its cold-agent run passed, to done and verified.
The three closed versions 0.1.0, 0.2.0 and 0.3.0 now share one heading form, done and verified; the closed count under Status moved from 2 to 3.
The interim 0.3.0 wording was retired; no audit or mutation sentence stands under 0.3.0, and the 0.4.0 section is unchanged.
No code, symbol, API, crate, dependency, schema, config, harness or gate surface changed; no stand, session command, MCP server or CLI contract moved.
CI on the pushed head read green, and the fork's default branch read build/escher-0.4.0, while one harness doc still names build/escher-0.3.0.
@task_file n=1 path=README.md
what: the project's front page — its Status section with one heading per version and that version's state
@task_item n=1 part=touches
Any row stating the 0.3.0 version's state as built, interim, or its cold-agent run passed: that state is now done and verified.
@task_item n=2 part=touches
Any row holding the number of closed versions, or the form a closed version's heading takes: it is now 3 closed versions, each as done and verified.
@task_item n=3 part=touches
Indirect, through the version state: a cold-agent row that treats a passed cold-agent run as a version's interim mark or its close; the front page no longer names the cold-agent run as 0.3.0's state.
@task_item n=4 part=touches
Any row naming the fork's default branch as build/escher-0.3.0: the default branch is build/escher-0.4.0.
@task_item n=1 part=untouched
The desktop-native stand decisions and the stand stable element ids: no stand control, id, role, keyboard region, viewport or layout moved.
@task_item n=2 part=untouched
The ShellProvider window chrome controls: no window control changed.
@task_item n=3 part=untouched
The escher-session command line, exit codes, refusals, recording, served tasks, snapshot, layout, lint, graph and MCP server contracts: none changed.
@task_item n=4 part=untouched
The cold-agent.sh and agent-run.sh contracts, and the cold-agent decisions on prompt, pipe and driver: their usage, exits, outputs and record dirs are unchanged.
@task_item n=5 part=untouched
The paint_bench and multi_doc command lines, the arbitrary index.html templates absence, and the not-yet-measured mobile-native, cli and web-spa surface overviews: no fact reaches them.
