@answer n=1 entity=secret-management#1.1 v=no
why: CI signing, keystore and token secrets in the publish-browser and wpt workflows are not_needed items 4 and 12; the probe adds no CI job or secret source
@answer n=2 entity=secret-management#1.2 v=no
why: the cold-agent pipe and the operator's login reach the model provider, not_needed item 7; the probe runs no model session
@answer n=3 entity=secret-management#2.1 v=no
why: credential-word census over the s02 files, the at-rest and cookie surface of not_needed item 1; the probe reads and writes no such data
@answer n=4 entity=secret-management#2.2 v=yes
why: the env census over the s03 files binds because the probe's new tests add only registered build-time env reads (item 10), so the census must be restated for any chunk file it lands in
@answer n=5 entity=secret-management#2.3 v=yes
why: cells.rs (the Cells example of the seven_guis set) sits in this credential-word census and the chunk edits examples/seven_guis whole, so the new example code must add no credential word
@answer n=6 entity=secret-management#2.4 v=yes
why: the only password occurrence maps to a role, so the engine masks no password value and the probe must keep MASKED_VALUE on password inputs by its own code (item 4)
@answer n=7 entity=secret-management#2.5 v=no
why: env and api_key census over the s06 files, outside the chunk's reached files (tests/blitz-tests, examples/seven_guis, and the dioxus_document.rs and events.rs re-reads)
@answer n=8 entity=secret-management#2.6 v=no
why: env and token census over the s07 files, outside the chunk's reached files
@answer n=9 entity=secret-management#2.7 v=no
why: credential-word census over the s08 files, the browser navigation and form surfaces of not_needed item 8
@answer n=10 entity=secret-management#2.8 v=no
why: env and credential census over the s09 files, the engine layout and paint group of not_needed item 9; the probe changes no engine code
@answer n=11 entity=secret-management#2.9 v=yes
why: the s10 env census records the one registered test-time read, and item 10 names the probe's env reads, so the probe's new tests must keep to that registered set
@answer n=12 entity=secret-management#2.10 v=no
why: env and auth census over the s11 fixture files, the fixture pages of not_needed item 11 that the probe does not touch
@answer n=13 entity=secret-management#2.11 v=no
why: credential-word census over the s12 files, outside the chunk's reached files
@answer n=14 entity=secret-management#2.12 v=no
why: credential-word census over the s13 listed files, outside the chunk's reached files
@answer n=15 entity=secret-management#2.13 v=no standing=S5
why: the committed-evidence rule of no host path and no e-mail address, applied to the docs and the pin
