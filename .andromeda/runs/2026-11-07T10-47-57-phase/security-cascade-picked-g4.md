@account n=1 entity=data-protection#1.1 v=not_in_scope
why: kept checks at rest; the chunk records, reads and changes no kept check, and its buffer reaches no carrier
@account n=2 entity=data-protection#2.1 v=not_in_scope
why: the two committed Counter checks; the chunk writes, reads and changes neither
@account n=3 entity=api-security#1.0 v=carried
why: the scope's «Nothing crosses» names no listener and no port, so the served-API count it states must hold at zero
@account n=4 entity=api-security#2.6 v=not_in_scope
why: settle and load waits are a component the chunk only drives; the scope names no settle behaviour, a re-check every change owes
@account n=5 entity=dependency-security#1.0 v=not_in_scope standing=S2
why: every cargo build and test passes --locked and the cargo-deny audit leg
@account n=6 entity=dependency-security#2.1 v=not_in_scope standing=S2
why: every cargo build and test passes --locked and the cargo-deny audit leg, its local reproduction before a push
@account n=7 entity=dependency-security#3.3 v=carried
why: records Dioxus 0.7.3 keeping its own keyboard-types 0.7, which the scope keeps by leaving the keyboard-types 0.8 move unported
@account n=8 entity=dependency-security#3.4 v=not_in_scope
why: a past upstream sync's lock record over Taffy commits; the chunk takes no sync and names no Taffy pin
@account n=9 entity=dependency-security#3.5 v=not_in_scope
why: a past upstream sync's lock record over Taffy commits; the chunk takes no sync and names no Taffy pin
@account n=10 entity=dependency-security#3.6 v=carried
why: records the Dioxus "0.7.3" pin and upstream 286874cc1 (Dioxus 0.8.0-alpha.3) as not taken, which the chunk's port re-asserts
@account n=11 entity=dependency-security#3.7 v=carried
why: records the Dioxus "0.7.3" pin the scope re-asserts staying unbumped
@account n=12 entity=dependency-security#3.8 v=carried
why: records the Dioxus "0.7.3" pin the scope re-asserts staying unbumped
@account n=13 entity=dependency-security#7.1 v=not_in_scope standing=S2
why: every cargo build and test passes the cargo-deny audit leg, its licences check included
@account n=14 entity=dependency-security#7.4 v=not_in_scope
why: the embolden leg runs select_popup_painted, a check the chunk neither reaches nor re-reads
@account n=15 entity=secret-management#2.1 v=not_in_scope
why: a slice's search record for credentials; the chunk adds no secret and reads no env var
@account n=16 entity=secret-management#2.2 v=not_in_scope
why: a slice's search record for credentials and env reads; the chunk adds no secret and reads no env var
@account n=17 entity=secret-management#2.3 v=not_in_scope
why: a slice's search record for credentials; the chunk adds no secret
@account n=18 entity=secret-management#2.7 v=not_in_scope
why: a slice's search record for credentials; the chunk adds no secret
@account n=19 entity=secret-management#2.8 v=not_in_scope
why: a slice's search record for credentials and env reads; the chunk adds no secret and reads no env var
@account n=20 entity=secret-management#2.10 v=not_in_scope
why: a slice's search record for credentials and env reads; the chunk adds no secret and reads no env var
@account n=21 entity=secret-management#2.11 v=not_in_scope
why: a slice's search record for credentials; the chunk adds no secret
@account n=22 entity=secret-management#2.12 v=not_in_scope
why: a slice's search record for credentials; the chunk adds no secret
@account n=23 entity=error-handling#6.1 v=carried
why: the measured MountedData hazard the chunk closes and whose two should_panic pins it inverts
@account n=24 entity=error-handling#7.1 v=carried
why: flush_queued_mounted_events' skip of a removed element, which the two render sites must keep after the replay
