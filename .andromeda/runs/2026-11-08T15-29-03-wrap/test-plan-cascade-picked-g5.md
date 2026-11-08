@account n=1 entity=quality-gates-coverage-targets#2.1 v=proposed
why: its «read now» per-crate figures are 2026-11-07's while this chunk added code to escher-driver and dioxus-native-dom, and the report reads only that CI#42493052318's Coverage report passed, the figures unread
@account n=2 entity=quality-gates-coverage-targets#4.1 v=proposed
why: it counts 52 stand_*.rs files in the stand-repeat leg, and stand_actions makes 53 through the existing glob (report stand-tally 52 → 53)
@account n=3 entity=quality-gates-coverage-targets#6.1 v=proposed
why: it names mcp_tests.rs's the_listing_is_the_twelve_verbs_of_the_table, a test this chunk renamed (thirteen → the_listing_is_the_fourteen_verbs_of_the_table), so the named function no longer exists by that name
