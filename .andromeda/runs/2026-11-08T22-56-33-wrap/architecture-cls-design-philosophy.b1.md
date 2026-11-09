@answer n=1 entity=design-philosophy#1.1 v=yes mode=stale
why: its cited line 3 of dioxus-native-dom lib.rs sits in a reached file that gains the ungated reached module and the ReachedHandler re-export, 35 to 36, so the citation can have moved
@answer n=2 entity=design-philosophy#2.1 v=yes mode=stale
why: its cited lines 60 and 86-87 of dioxus_document.rs sit in a reached file that gains record_reached and the bridge handle_event record write, so those citations can have moved
@answer n=3 entity=design-philosophy#3.1 v=no
why: none of its spec and engine citations sit in a file the session reached, and the bubbling count is a stated limit, not a spec claim
@answer n=4 entity=design-philosophy#4.1 v=no
why: its cited blitz-dom and incremental-test files are not reached, and the reached record stores dioxus-side stable ids, not blitz NodeIds
@answer n=5 entity=design-philosophy#5.1 v=no
why: the new blitz-tests target sits under tests/ as the entity already states, and its cited harness files are not reached
