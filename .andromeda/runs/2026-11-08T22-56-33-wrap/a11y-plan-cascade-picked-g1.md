@account n=1 entity=a11y-scope-summary#1.10 v=proposed
why: F4 (the reached record and element_listener_ids compile without accessibility, element_facts gated) belongs beside the feature-reach entity; no manifest or lockfile moved, so its own claim holds
@account n=2 entity=a11y-scope-summary#1.11 v=not_touched
why: the report adds to tests/blitz-tests/tests/common/mod.rs only at 335-412 and leaves stand.rs, the Tab orders and the served pages' rows as they were, so the claim holds
@account n=3 entity=a11y-scope-summary#1.15 v=not_touched
why: docs/capabilities.md gains the handler-coverage limits (273-281) and two other blocks, while the a11y leg's fourteen files and test_capability_statement.py stay unchanged, so the published reach and its by-hand rule hold
@account n=4 entity=a11y-strategy#4.1 v=proposed
why: the gated/ungated module list now misses the new ungated reached module and its ungated listener reader beside signal_registry
@account n=5 entity=aria-patterns-roles#5.2 v=not_touched
why: a historical s06 slice search; the chunk adds no ARIA attribute or role
@account n=6 entity=aria-patterns-roles#5.3 v=not_touched
why: a historical s07 slice search; the chunk adds no ARIA attribute or role
@account n=7 entity=aria-patterns-roles#5.4 v=not_touched
why: a historical s08 slice search; the chunk adds no ARIA attribute or role
@account n=8 entity=aria-patterns-roles#5.6 v=not_touched
why: a historical s10 slice search; the chunk adds no ARIA attribute or role
@account n=9 entity=aria-patterns-roles#5.7 v=not_touched
why: a historical s11 slice search; the chunk adds no ARIA attribute or role
@account n=10 entity=keyboard-navigation#1.1 v=not_touched
why: the focusability rule lives in blitz-dom, which the change does not reach, and unkeyed_actionable is unchanged
@account n=11 entity=keyboard-navigation#3.7 v=not_touched
why: the select list's thumb is unchanged; the capabilities.md additions are handler-coverage limits, so the claim holds
@account n=12 entity=keyboard-navigation#3.17 v=not_touched
why: the engine's Enter and Space activation (keyboard.rs, events/mod.rs) is unchanged; F2's keyboard-activation coverage check is placed beside the harness entity keyboard-navigation#7.2
@account n=13 entity=keyboard-navigation#4.5 v=not_touched
why: dioxus_document.rs gains the reached record at 448-460 only, and the keyboard data conversion is unchanged, so the claim holds
@account n=14 entity=keyboard-navigation#4.6 v=proposed
why: the report states fct-wfj657us holds as written and gains the reached record beside the registry dispatch (E2), which is F1
@account n=15 entity=keyboard-navigation#4.7 v=not_touched
why: the focus event forwarding in dioxus_document.rs is unchanged beside the record's lines
@account n=16 entity=keyboard-navigation#4.9 v=not_touched
why: the report states poll, initial_build and flush_queued_mounted_events are untouched and dioxus_runtime_probe green
@account n=17 entity=keyboard-navigation#4.10 v=not_touched
why: the render-polled task limit is unchanged; the record takes no borrow across render_immediate
@account n=18 entity=keyboard-navigation#4.12 v=not_touched
why: handle_event's other arms are unchanged beside the record, so the IME TODO holds
@account n=19 entity=keyboard-navigation#7.2 v=proposed
why: F2 (stand_handler_coverage asserting a keyboard activation exercises its click) belongs beside the stand keyboard harness entity
@account n=20 entity=keyboard-navigation#7.3 v=not_touched
why: the shared module gains tables at 335-412 only, and LOAD_TAB_ORDER and CELLS_TAB_ORDER are unchanged
@account n=21 entity=keyboard-navigation#7.6 v=not_touched
why: the select checks and the leg lists are unchanged (no change to ci-leg.sh)
@account n=22 entity=keyboard-navigation#7.7 v=not_touched
why: cli_commands and host_log each gain a coverage test, not a select-list test, so one select-list test each still holds
@account n=23 entity=keyboard-navigation#7.9 v=not_touched
why: the test file and the legs are unchanged
@account n=24 entity=keyboard-navigation#8.3 v=not_touched
why: a historical s07 slice search; no s07 file is reached
