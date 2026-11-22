@row_add n=1 row=contract/skia-archive-exit-codes/0 area=standard-contracts fact=F1 detector=D-arch-resources
boundary: skia_archive.py exit codes
item: 0
promise: A verified archive for the target is in DIR, a matching local copy reused with no fetch; stdout carries one line, the SKIA_BINARIES_URL template `file://{DIR}/skia-binaries-{key}.tar.gz`.
basis: fct-z3khe52c
because: The chunk lands the skia archive script, whose exit codes are a contract the leg runner and the matrix step read, and no row registers it.
@row_add n=2 row=contract/skia-archive-exit-codes/1 area=standard-contracts fact=F1 detector=D-arch-resources
boundary: skia_archive.py exit codes
item: 1
promise: No verified archive after three attempts; stderr prints `skia-archive: attempt {n} of 3 failed in {fetch|verify}` for each failed attempt, then `skia-archive: no verified archive`.
basis: fct-z3khe52c
because: The chunk lands the skia archive script, whose exit codes are a contract the leg runner and the matrix step read, and no row registers it.
@row_add n=3 row=contract/skia-archive-exit-codes/2 area=standard-contracts fact=F1 detector=D-arch-resources
boundary: skia_archive.py exit codes
item: 2
promise: A usage error or a target with no recorded archive, nothing fetched; stderr prints `skia-archive: usage` or `skia-archive: no recorded archive for this target`.
basis: fct-z3khe52c
because: The chunk lands the skia archive script, whose exit codes are a contract the leg runner and the matrix step read, and no row registers it.
@row_add n=4 row=decision/skia-legs-verify-archive-first area=standard-contracts fact=F2 detector=D-arch-resources
name: Skia-building legs verify skia's archive first
rule: The clippy, test, build, msrv, doc, coverage and a11y legs of the CI leg runner each first verify skia's prebuilt archive for the rustc host and export SKIA_BINARIES_URL, and a refused archive ends the leg before any cargo runs.
guard: test_skia_legs_build_from_the_verified_archive
basis: fct-z3khe52c
because: The leg runner gains a skia arm over seven named legs, a contract of the runner no standing row states.
@row_add n=5 row=name/other/skia-binaries-url area=occupied-resources fact=F3 detector=D-arch-resources
class: other
name: SKIA_BINARIES_URL
basis: fct-z3khe52c
because: The leg runner and the matrix test step now set SKIA_BINARIES_URL for skia-bindings' build script, an occupied environment variable not yet registered.
@row_add n=6 row=decision/skia-verified-prebuilt-archive area=infrastructure-patterns fact=F4 detector=D-arch-decisions
name: Skia built from a verified prebuilt archive
rule: Skia's prebuilt archive is fetched once per platform from the rust-skia skia-binaries release at the locked skia-bindings tag and used only when its sha256 equals the committed record for its target.
except: A failed read or an unrecorded key falls back to a source build of skia, a stated limit.
guard: test_skia_archive.py
basis: fct-sods3b6s
because: The chunk adopts a verified prebuilt skia archive with a committed per-target record, a new infrastructure pattern no row states.
@row_add n=7 row=decision/no-forced-skia-download area=infrastructure-patterns fact=F4 detector=D-arch-decisions
name: Skia download never forced
rule: FORCE_SKIA_BINARIES_DOWNLOAD is never set by any CI leg or workflow step.
why: In a crates.io build the flag routes skia-bindings to a full source build instead of a download.
basis: fct-uvfujhlk
because: The plan's forced download was disproved by measurement and dropped, a deliberate choice future CI work must respect.
@row_add n=8 row=decision/matrix-skia-archive-witness area=infrastructure-patterns fact=F5 detector=none
name: Matrix witness of the verified skia archive
rule: The matrix test job verifies skia's archive after setup and points the build at it, and after the tests a witness step reddens the row unless a skia-bindings build output, read by content, shows the build read the verified file:// archive and its install succeeded.
why: A file:// archive is read whatever key is asked and a failed read falls silently to a source build, so the witness is what reads that fallback red.
guard: test_p_matrix_builds_skia_from_the_verified_archive
basis: fct-xp4s2p66
because: The matrix job gains the verified-archive and witness steps, a gate the CI now guarantees that no row states.
@row_add n=9 row=decision/cargo-under-verified-skia-url area=infrastructure-patterns fact=F6 detector=none
name: Every skia-building cargo call under the verified URL
rule: Every cargo call in a skia-building leg, the fast leg's closing target sweep included, runs under the verified SKIA_BINARIES_URL.
why: skia-bindings' build script declares rerun-if-env-changed, so a cargo run on the shared target directory without the variable re-runs it.
guard: test_fast_leg_sweeps_under_the_verified_archive
basis: fct-mhyeompk
because: The sweep hazard was measured and fixed in the leg runner, a rule future legs and sweeps must keep.
@row_add n=10 row=decision/build-output-checks-layout-agnostic area=stack-and-technologies fact=F7 detector=none
name: Build-output checks pin no build layout
rule: A check that reads a package's build-script output must not pin one build-directory layout.
why: The CI runners' Cargo 1.100 writes it under build/{package}/{hash}/run/stdout while the dev host's Cargo 1.99 keeps build/{package}-{hash}/output.
basis: fct-vry5wipu
because: A witness pinned to one layout read red on the CI toolchain, a measured hazard that binds every later build-output check.
