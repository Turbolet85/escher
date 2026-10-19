- line: .andromeda/test-plan.md:27
old: «and two shared modules that are no target — `tests/common/mod.rs`, the stand checks' tables and helpers, each stated once, and `tests/session_common/mod.rs`, what the checks that hold a session share (§3 Crate-local test helpers) (tests/blitz-tests/lib.rs:1; tests/blitz-tests/Cargo.toml:3; tests/blitz-tests/tests/common/mod.rs:1-6; tests/blitz-tests/tests/session_common/mod.rs:1-7)»
new: «and three shared modules that are no target — `tests/common/mod.rs`, the stand checks' tables and helpers, each stated once, `tests/session_common/mod.rs`, what the checks that hold a session share, and, since 2026-10-19-shared-fixtures, `tests/fixtures/mod.rs`, the fixture boot and hold, the fixture pages more than one check reads and the held network stand-in `HeldNet`, each stated once (§3 Crate-local test helpers) (tests/blitz-tests/lib.rs:1; tests/blitz-tests/Cargo.toml:3; tests/blitz-tests/tests/common/mod.rs:1-6; tests/blitz-tests/tests/session_common/mod.rs:1-7; tests/blitz-tests/tests/fixtures/mod.rs:17-107)»
- line: .andromeda/test-plan.md:27
old: «on two in-file fixtures booted with the stand's options, a typed password and a file input's authored path»
new: «on two fixtures booted with the stand's options — the shared password page `fixtures::password_labelled` and an in-file file-input page —, a typed password and a file input's authored path»
- line: .andromeda/test-plan.md:27
old: «on three in-file fixtures booted with the stand's options, an element hidden and shown by attribute leaving and returning with its descendants, a typed password named once and only by the fixed mask, and a rewritten label naming the input it labels —»
new: «on three fixtures booted with the stand's options — two in-file and the shared password page `fixtures::password_labelled` —, an element hidden and shown by attribute leaving and returning with its descendants, a typed password named once and only by the fixed mask, and a rewritten label naming the input it labels —»
- line: .andromeda/test-plan.md:27
old: «`covered` read on an in-file overlay fixture»
new: «`covered` read on the shared overlay page `fixtures::covered`»
- line: .andromeda/test-plan.md:68
old: «and two shared modules that are no target — `tests/common/mod.rs`, read through»
new: «and three shared modules that are no target — `tests/common/mod.rs`, read through»
- line: .andromeda/test-plan.md:68
old: «session_common/mod.rs:1-7), and a third file that is no built target»
new: «session_common/mod.rs:1-7), and, since 2026-10-19-shared-fixtures, `tests/fixtures/mod.rs`, read through `mod fixtures;` by 19 checks — 18 `stand_*.rs` (`stand_settle`, `stand_boot`, `stand_act_diff`, `stand_diff`, `stand_snapshot_text`, `stand_mcp_masked`, `stand_session_recording`, `stand_act_spans`, `stand_mcp_tools`, `stand_act_filled`, `stand_act_screenshot`, `stand_act_obstructed`, `stand_act_scroll`, `stand_act_transformed`, `stand_act_timer`, `stand_replay`, `stand_keyboard`, `stand_snapshot_state`) and `font_face_sniffed_source.rs` (tests/blitz-tests/tests/fixtures/mod.rs:17-107), and a fourth file that is no built target»
- line: .andromeda/a11y-plan.md:285
old: «the input a test-only element of `stand_mcp_masked`'s fixture, no stand task or app gaining one with it (tests/blitz-tests/tests/stand_mcp_masked.rs:23-30;»
new: «the input a test-only element of the shared password page `fixtures::password_named`, no stand task or app gaining one with it (tests/blitz-tests/tests/fixtures/mod.rs:37-44;»
