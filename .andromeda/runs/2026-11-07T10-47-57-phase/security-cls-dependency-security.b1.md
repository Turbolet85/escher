@answer n=1 entity=dependency-security#1.0 v=yes
why: the cargo-deny audit leg's command, deny.toml advisory ignores and graph are what scope item 8 names as the leg the changed code must still pass.
@answer n=2 entity=dependency-security#2.1 v=yes
why: scope item 8 requires the audit leg to pass over the changed code, and this gives the local reproduction (ci-leg.sh audit) to run before a push.
@answer n=3 entity=dependency-security#3.1 v=no
why: records the 2026-10-10 upstream sync's lock additions; the chunk moves no lock entry, so nothing stated is a rule its code or tests must honour.
@answer n=4 entity=dependency-security#3.2 v=no
why: records the 2026-10-13 sync's taffy and lock move as history; the chunk moves no package.
@answer n=5 entity=dependency-security#3.3 v=no
why: records the 2026-10-17 sync's keyboard-types and toml moves as history; nothing the chunk's code or tests must honour.
@answer n=6 entity=dependency-security#3.4 v=no
why: records the 2026-10-18 cherry-pick's unchanged lock and taffy pin as history; taffy is not reached by the chunk.
@answer n=7 entity=dependency-security#3.5 v=no
why: records the 2026-10-19 cherry-pick's unchanged lock as history; the chunk changes no dependency.
@answer n=8 entity=dependency-security#3.6 v=yes
why: keeps Dioxus at 0.7.3 and leaves 0.8.0-alpha.3 (upstream 286874cc1) unported, which scope item 8 names; the chunk ports the buffered render on 0.7 only.
@answer n=9 entity=dependency-security#3.7 v=yes
why: its pins line holds Dioxus "0.7.3" with no git dependency, the value scope item 8 names; the chunk must not bump, patch or vendor Dioxus.
@answer n=10 entity=dependency-security#3.8 v=yes
why: its pins line holds Dioxus "0.7.3" with no git dependency, the value scope item 8 names; the chunk must leave it unchanged.
@answer n=11 entity=dependency-security#4.1 v=no
why: serde_json reads the MCP stdin and kept checks; the buffered render parses no JSON and the chunk adds no serde_json use.
@answer n=12 entity=dependency-security#5.1 v=no
why: the png crate's shot encoder and blitz-tests dev-dependency; the chunk writes no image and adds no dependency.
@answer n=13 entity=dependency-security#6.1 v=no
why: the color crate in blitz-test-harness's contrast reader; the chunk touches no contrast reader and adds no dependency.
@answer n=14 entity=dependency-security#7.1 v=yes
why: the licence table is part of the cargo-deny audit leg the scope names, so the changed code must still read licences ok over the unchanged graph.
@answer n=15 entity=dependency-security#7.2 v=no
why: a CI free-disk action step in ci.yml; the chunk edits no workflow and this step is not the audit leg or a cargo leg.
@answer n=16 entity=dependency-security#7.3 v=no
why: the matrix_test free-disk step and its pin test; the chunk edits no workflow or test_ci_workflows.py.
@answer n=17 entity=dependency-security#7.4 v=yes
why: the embolden select_popup_painted leg is a --locked cargo leg, and scope item 8 names every cargo leg running with --locked as one that must still pass over the changed code.
