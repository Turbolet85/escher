@answer n=1 entity=quality-gates-coverage-targets#1.0 v=no
why: the dated 2026-10-05 and 2026-10-14 line readings are fixed measurements the session does not reach, and it states no coverage figure
@answer n=2 entity=quality-gates-coverage-targets#2.1 v=yes mode=stale
why: the session's new code and tests in escher-driver and dioxus-native-dom move the current per-crate readings 4959/5058 and 2937/3387 that this entry states
@answer n=3 entity=quality-gates-coverage-targets#3.1 v=no
why: mutation_buffer.rs is not among the session's changed files, so its 116-line, 7-missed reading and the unchanged 83 % floor still hold
@answer n=4 entity=quality-gates-coverage-targets#4.1 v=yes mode=stale
why: stand_actions raises the stand_*.rs file count that this entry names as 52 to 53
@answer n=5 entity=quality-gates-coverage-targets#5.1 v=no
why: the session changes no mutation profile, config, or scoped file among timer.rs, mcp/framing.rs and mutation_writer.rs, so the procedure and its dated readings still hold
@answer n=6 entity=quality-gates-coverage-targets#6.1 v=yes mode=stale
why: the session reaches mcp_tests.rs and stand_requirement_sweep.rs, which hold two of the three named functions, and adds code that can move the 85-over-15 count
