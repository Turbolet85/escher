@answer n=1 entity=quality-gates-coverage-targets#1.0 v=no
why: the first-reading figures are dated 0.1.0 and 2026-10-14 measurements that the session neither re-reads nor moves
@answer n=2 entity=quality-gates-coverage-targets#2.1 v=no
why: the session reports the three coverage floors held with per-crate figures not re-read, and coverage_floor.py and ci-leg.sh are unchanged
@answer n=3 entity=quality-gates-coverage-targets#3.1 v=no
why: mutation_buffer.rs is not among the files the session changes and its dated CI reading is not re-read
@answer n=4 entity=quality-gates-coverage-targets#4.1 v=yes mode=stale
why: stand_handler_coverage is a new stand_*.rs file, moving the stand-repeat count from 53 files to 54
@answer n=5 entity=quality-gates-coverage-targets#5.1 v=no
why: the session leaves the mutation tier unchanged and its scoped targets timer.rs, mcp/framing.rs and mutation_writer.rs are not reached
@answer n=6 entity=quality-gates-coverage-targets#6.1 v=yes mode=stale
why: the change reaches escher-driver and dioxus-native-dom files whose complexity readings this entity cites, and no re-read shows they still hold
