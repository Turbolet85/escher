# Hand mutation controls — 2026-10-16-fast-failing-mutants

Each of the six timeout-only mutants, applied by hand exactly as cargo-mutants 27.1.0 writes it (`cargo mutants
--list --diff` at the base `494abd258fee55528006cfeaef68c145fd291de6`, the new test modules in place): the file's
bytes saved and their sha256 taken, the mutation written, the direct test run alone through `cargo test --locked -p
{pkg} --lib {test name}` and read red (the named test `FAILED` on its own assertion or panic, cargo exit 101, the
crate recompiled), the saved bytes written back and their sha256 compared, and only then the same run read green
(the named test `ok`, exit 0). The seconds are the red run's wall time, the recompile included.

| mutant | direct test | red | seconds | restore | green |
|---|---|---|---|---|---|
| examples/seven_guis/src/tasks/timer.rs:29:9: replace TimerTicks::next with () | a_delivered_tick_is_taken_once_and_an_empty_wait_parks | red | 1.1 s | sha256 equal | green |
| examples/seven_guis/src/tasks/timer.rs:32:31: replace -= with += in TimerTicks::next | a_delivered_tick_is_taken_once_and_an_empty_wait_parks | red | 1.1 s | sha256 equal | green |
| examples/seven_guis/src/tasks/timer.rs:32:31: replace -= with /= in TimerTicks::next | a_delivered_tick_is_taken_once_and_an_empty_wait_parks | red | 1.1 s | sha256 equal | green |
| packages/escher-driver/src/mcp/framing.rs:47:53: replace + with - in read_request | read_request_takes_each_line_with_its_terminator | red | 0.9 s | sha256 equal | green |
| packages/escher-driver/src/mcp/framing.rs:47:53: replace + with * in read_request | read_request_takes_each_line_with_its_terminator | red | 0.9 s | sha256 equal | green |
| packages/dioxus-native-dom/src/mutation_writer.rs:172:9: replace <impl WriteMutations for MutationWriter<'_>>::create_text_node with () | a_created_text_node_is_mapped_and_left_on_the_stack | red | 1.1 s | sha256 equal | green |

After the six, the three files read their base sha256 again: `timer.rs` `face26a4…` (the test module appended),
`mcp/framing.rs` `88bf9276…`, `mutation_writer.rs` `e40d0a1e…`.

## The mutant test command (step 7)

Read from the first mutant's log of the `timer.rs` run (`target/tmp/ffm-timer-20261016T203018Z`, the log of
`timer.rs:21:9`, line 501). The test phase ran `cargo nextest run --verbose --package=blitz-tests@0.3.0-beta.3
--package=seven_guis@0.1.0 --lib`, then the 50 `--test=` arguments of the unit's list, `--test=cli_commands` through
`--test=stand_timer`, the 50 lines after the package line, as `cargo.rs:160-162` builds it. The nextest
run read `nextest profile: mutants`, `Starting 201 tests across 52 binaries (7 tests skipped)`, the count the unmutated
baseline entry read. `tasks::timer::tests::a_delivered_tick_is_taken_once_and_an_empty_wait_parks` started in the
first batch of four, as test 1/201, and failed in 0.028 s.
