@answer n=1 entity=dependency-security#1.0 v=yes
why: names the cargo-deny audit leg (advisories and licences, --locked, over the resolved workspace graph) that task item 16 names for this chunk
@answer n=2 entity=dependency-security#2.1 v=yes
why: the audit leg the scope names can go red on newly published advisories, and its local reproduction is the check to run before a push
@answer n=3 entity=dependency-security#3.1 v=no
why: records the 2026-10-10 upstream sync's dependency set, a past sync the chunk does not repeat and that adds no package
@answer n=4 entity=dependency-security#3.2 v=no
why: records the 2026-10-13 upstream sync's taffy and workspace bump, a past sync the chunk does not repeat
@answer n=5 entity=dependency-security#3.3 v=no
why: records the 2026-10-17 upstream sync's keyboard-types and test-that bumps, a past sync the chunk does not repeat
@answer n=6 entity=dependency-security#3.4 v=no
why: records the 2026-10-18 cherry-pick, which moved no dependency and states no rule over the chunk's code or lock
@answer n=7 entity=dependency-security#3.5 v=no
why: records the 2026-10-19 cherry-picks, which moved no dependency and states no rule over the chunk's code or lock
@answer n=8 entity=dependency-security#3.6 v=no
why: records a past sync's pins and the commits it took or left out, and the chunk takes no sync
@answer n=9 entity=dependency-security#3.7 v=no
why: records a past sync that moved no package, and the chunk takes no sync
@answer n=10 entity=dependency-security#3.8 v=no
why: records a past sync that moved no package, and the chunk takes no sync
@answer n=11 entity=dependency-security#4.1 v=yes
why: serde_json with no features is the reader of the MCP server's stdin and of the recorded step through Check::from_bytes, which task items 8 and 10 name
@answer n=12 entity=dependency-security#5.1 v=no
why: png encodes a shot in escher-driver, and the graph read does not touch the shot path
@answer n=13 entity=dependency-security#6.1 v=no
why: color is the luminance source of the contrast reader in blitz-test-harness, which the chunk does not touch
@answer n=14 entity=dependency-security#7.1 v=yes
why: the licence table is part of the cargo-deny audit leg the scope names (item 16), which the chunk's dependency check must pass
@answer n=15 entity=dependency-security#7.2 v=no
why: a CI free-disk action in the stand-checks job, which the chunk's scope does not name and it does not change
@answer n=16 entity=dependency-security#7.3 v=no
why: a CI test pinning the free-disk action in the matrix_test job, and the chunk changes no CI job
@answer n=17 entity=dependency-security#7.4 v=no
why: the embolden test leg is a locked cargo test, not the audit leg, and runs no graph code the chunk touches
