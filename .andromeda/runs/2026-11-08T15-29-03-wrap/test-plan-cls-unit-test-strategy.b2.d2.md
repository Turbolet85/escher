@answer n=1 entity=unit-test-strategy#25.1 v=no
why: the session's new unit tests land in escher-driver and dioxus-native-dom, neither of which is slice s03, whose no-unit-tests claim it does not reach
@answer n=2 entity=unit-test-strategy#25.2 v=no
why: the session's new unit tests sit in escher-driver actions.rs and dioxus-native-dom actionable.rs, outside the eight files of slice s07
@answer n=3 entity=unit-test-strategy#25.3 v=no
why: blitz-vibey-script is untouched by the session, and its script tests and their coverage statement are unchanged
@answer n=4 entity=unit-test-strategy#25.4 v=yes mode=stale
why: the session adds stand_actions.rs under tests/blitz-tests, moving the 61-file slice s12 count, so the in-file cfg(test) reading must be re-measured
