# Mutation controls — 2026-10-12-self-description

Step 1 of the plan. Each control saved the file's bytes, replaced one site (asserted to occur
once), ran the named test with stderr merged into stdout, wrote the saved bytes back and
compared sha256 before the next control. Run 2026-10-12, between the implement run's first
full gate block and its second; the second full block (all entries green but the operator's)
is the green reading on the restored tree.

| control | file | mutation | test it reddened | cargo exit | restore sha256 equal |
|---|---|---|---|---|---|
| c1 | `packages/escher-driver/src/schema.rs` | `pressed`'s help loses "a mixed toggle reads false" | `schema::tests::the_thirteen_node_keys_are_named_in_order_each_with_its_help` | 101 | yes |
| c2 | `packages/escher-driver/src/error.rs` | `dead`'s row no longer names the kind whose remedy it repeats | `error::tests::the_thirteen_errors_each_state_a_kind_a_meaning_and_a_remedy` | 101 | yes |
| c3 | `packages/escher-driver/src/describe.rs` | a session error's `remedy` written from its meaning | `describe::tests::every_text_in_it_is_its_tables` | 101 | yes |
| c4 | `packages/escher-driver/src/mcp.rs` | the default revision back to `2025-06-18` | `mcp::tests::each_method_gets_its_reply` | 101 | yes |
| c5 | `packages/dioxus-native-dom/src/snapshot_diff.rs` | bounds never read as differing | `stand_diff::the_flight_bookers_book_names_eight_nodes_that_only_moved` | 101 | yes |
| c6 | `packages/escher-driver/src/json.rs` | a session error written without its remedy | `cli_commands::a_state_directory_of_94_bytes_holds_a_session_and_one_of_95_is_refused_with_its_bound` | 101 | yes |
| c7 | `packages/escher-driver/src/error.rs` | the stated Linux bound 94 → 93 | `cli_commands::a_state_directory_of_94_bytes_holds_a_session_and_one_of_95_is_refused_with_its_bound` | 101 | yes |
| c8 | `packages/escher-driver/src/json.rs` | a changed node written without `differs` | `stand_mcp_masked::a_password_typed_over_mcp_reads_the_mask_and_never_the_typed_text` | 101 | yes |
| c9 | `packages/escher-driver/src/json.rs` | a refusal's `remedy` written from its meaning | `stand_mcp_tools::every_cause_a_tool_call_reaches_is_served_with_its_remedy` | 101 | yes |
| c10 | `packages/escher-driver/src/mcp.rs` | the `help` tool answered as no session | `mcp_server::the_newest_revision_is_spoken_and_help_answers_before_any_start` | 101 | yes |
| c11 | `packages/escher-driver/src/cli.rs` | `help --session <dir>` no longer read as `help` | `cli_commands::help_answers_the_tools_description_and_reaches_no_session` | 101 | yes |

Each log held the named test's `... FAILED` line and no compile error. 11 of 11 controls
reddened their test; 11 of 11 restores read the saved sha256.
