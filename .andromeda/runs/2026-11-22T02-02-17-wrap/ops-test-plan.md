@row_add n=1 row=decision/skia-legs-build-from-verified-archive area=9-ci-integration fact=F1 detector=none
name: Skia-building legs build from the verified archive
rule: The leg script's skia-building legs (clippy, test, build, msrv, doc, coverage and a11y) fetch and verify the recorded skia archive and run every cargo call, the fast leg's target sweep included, under SKIA_BINARIES_URL; a refused archive ends the leg before cargo.
why: A cargo run without SKIA_BINARIES_URL on the shared target re-runs skia-bindings' build script, and the file:// check reads only the last run.
guard: LegScriptTest
except: A leg that builds no skia sets no skia variable.
basis: fct-z3khe52c, fct-o7ywsm7s, fct-mhyeompk
because: The chunk made every skia-building leg and the fast leg's sweep build skia from the verified archive, a CI guarantee no row of §9 states.
@row_add n=2 row=decision/matrix-skia-from-verified-archive area=9-ci-integration fact=F2 detector=none
name: Matrix builds skia from the verified archive
rule: Every row of the native platform matrix fetches and verifies its target's recorded skia archive after setup, and after test a witness step reads red unless a skia-bindings build output names a file:// read and DOWNLOAD AND INSTALL SUCCEEDED.
guard: test_p_matrix_builds_skia_from_the_verified_archive
basis: fct-vpyradka, fct-pnbxoiv3, fct-xp4s2p66
because: The matrix job gained the verified-archive step and its witness, a guarantee of the CI configuration no row of §9 states.
@row_add n=3 row=decision/skia-prebuilt-fallback-stated-limit area=9-ci-integration fact=F3 detector=none
name: Skia prebuilt fallback a stated limit
rule: A failed skia archive read or a build asking for an unrecorded key falls back to a full source build of skia with no panic, read red by the matrix witness on CI and by the dev host's file:// output check locally; FORCE_SKIA_BINARIES_DOWNLOAD is never set.
why: In a crates.io build FORCE_SKIA_BINARIES_DOWNLOAD routes skia-bindings to a full source build, not a panic.
basis: fct-pufyyo3s, fct-twf6j6ew, fct-uvfujhlk, fct-n5rhrxdw
because: The operator ruled the fallback a stated limit and dropped the force flag, a decision future CI work must respect.
@row_add n=4 row=decision/skia-witness-reads-by-content area=9-ci-integration fact=F4 detector=none
name: Skia archive witness reads the build tree by content
rule: The skia archive witness finds skia-bindings' build output by reading its build tree by content, never by a pinned path.
why: A CI toolchain ahead of the dev host moves the layout: Cargo 1.100 writes build-script output to run/stdout under a hashed folder, Cargo 1.99 the output file.
guard: test_p_matrix_builds_skia_from_the_verified_archive
basis: fct-xp4s2p66, fct-vry5wipu
because: The witness was rewritten after two reds to read by content, a choice with a stated reason that a later edit of the witness must keep.
@row_add n=5 row=contract/skia-archive-exit-codes/0 area=ci-integration fact=F5 detector=none
boundary: skia archive script exit codes
item: 0
promise: Prints one stdout line, the SKIA_BINARIES_URL template.
basis: fct-z3khe52c
because: The new skia archive script's exit codes are a contract the leg script and the matrix step read, stated by no row.
@row_add n=6 row=contract/skia-archive-exit-codes/1 area=ci-integration fact=F5 detector=none
boundary: skia archive script exit codes
item: 1
promise: No verified archive after 3 attempts, with a fixed stderr line for each attempt.
basis: fct-z3khe52c
because: The new skia archive script's exit codes are a contract the leg script and the matrix step read, stated by no row.
@row_add n=7 row=contract/skia-archive-exit-codes/2 area=ci-integration fact=F5 detector=none
boundary: skia archive script exit codes
item: 2
promise: A bad call or an unrecorded target, with nothing fetched.
basis: fct-z3khe52c
because: The new skia archive script's exit codes are a contract the leg script and the matrix step read, stated by no row.
@row_add n=8 row=decision/hosted-runner-cancel-hand-rerun area=9-ci-integration fact=F6 detector=none
name: Hosted-runner cancel answered by a hand re-run
rule: A CI run cancelled with no runner is a hosted-runner limit, answered by a hand re-run.
basis: fct-gyir65wd, fct-sods3b6s
because: The chunk stated the no-runner cancel as a hosted-runner limit with the hand re-run, carried to test-plan §9, which holds no row of it.
