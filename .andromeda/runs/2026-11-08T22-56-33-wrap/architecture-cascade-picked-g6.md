@account n=1 entity=existing-scopes#1.23 v=proposed
why: its module list lacks the new ungated reached module, element_id's new element_listener_ids, and bridge_tests moved from eight to nine tests
@account n=2 entity=existing-scopes#1.26 v=proposed
why: escher-driver gains its nineteenth module coverage, cli's coverage role, replay's crate-private replay_each and json's Coverage::to_json
@account n=3 entity=existing-scopes#1.31 v=proposed
why: stand_replay moved from 3 to 4 tests, common/mod.rs gained its tables and its twenty-fourth reader, and the new stand_handler_coverage target is absent
@account n=4 entity=existing-scopes#1.32 v=proposed
why: escher-session gains its coverage role and the shared tests/common/mod.rs gains coverage_needles()
@account n=5 entity=existing-scopes#3.1 v=not_touched
why: the report puts the coverage role's new unit test in cli_tests.rs, still cli's #[path] child, and coverage.rs holds no test child, so the claim still holds on the changed file
@account n=6 entity=existing-scopes#3.2 v=proposed
why: the report measures the mod common; declarers moving from 23 to 24 and the session_common declarers from 34 to 35
@account n=7 entity=existing-scopes#4.3 v=not_touched
why: the report adds dioxus_document.rs lines only at 10, 448-449 and 452-460, and shows cargo test -p dioxus-native-dom and dioxus_runtime_probe green, so the upstream test claim still holds
