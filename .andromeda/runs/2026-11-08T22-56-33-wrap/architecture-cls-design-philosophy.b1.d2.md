@answer n=1 entity=design-philosophy#1.1 v=yes mode=stale
why: the session adds a reached module and re-exports to dioxus-native-dom's lib.rs, a reached file, so the cited crate description at lib.rs:3 can have moved
@answer n=2 entity=design-philosophy#2.1 v=yes mode=stale
why: the session adds record_reached and a handle_event record write inside dioxus_document.rs, so the cited lines 60 and 86-87 can have moved
@answer n=3 entity=design-philosophy#3.1 v=no
why: the session reaches no blitz-dom, blitz-paint or blitz-net file, and none of its cited files is in the change
@answer n=4 entity=design-philosophy#4.1 v=no
why: its cited files are blitz-dom and blitz-tests files the session does not reach, and the versioned-id and single-pipeline contracts are untouched
@answer n=5 entity=design-philosophy#5.1 v=no
why: the headless harness and tests/ directory it names are unchanged, the new stand target sits in that same directory, and no cited file is reached
