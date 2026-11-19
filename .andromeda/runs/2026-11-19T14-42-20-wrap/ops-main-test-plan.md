@row_add n=1 row=decision/build-directory-held-by-sweep area=9-ci-integration detector=none
name: Build directory held by a sweep, resolutions kept
rule: The build directory's growth is held by sweeping superseded unit hashes after a green fast leg, never by merging feature resolutions; the workspace, blitz-tests, embolden and seven_guis resolutions each stay a build of their own.
why: Cargo mints a unit hash per feature resolution, lockfile and toolchain epoch, never per source edit, and collects none it supersedes; each resolution is load-bearing, so merging two was measured only.
basis: fct-rik5xvof, fct-7uih3hbs, fct-e4mdu4wl
because: The plan's reviewed expected amendment for test-plan §9 names the cause measured and the sweep chosen over a unification, and no proposer gave it a row.
@row_set n=1 row=decision/local-baseline cell=why seen=1 detector=none
value: It is the reference CI is compared against; on it a cold workspace build fills 54.61 GB of target/debug, and a replayed working day grows it 57.90 GB unswept, −2.04 GB swept by fast.
basis: fct-e4mdu4wl
because: The plan's reviewed expected amendment says the local-baseline row gains the measured footprint, which the report states and no proposer carried.
@row_add n=2 row=decision/ci-set-fast-builds-apart area=9-ci-integration detector=none
name: A CI-set fast leg builds under hashes of its own
rule: A fast leg run on the dev host with CI set builds its units under new hashes, about 48 GB measured, and runs no sweep; the next dev-host fast leg's sweep removes them; its mechanism is not found and stays open.
why: Measured, not chosen: the project source reads no CI, so the hash change comes from outside it; a measured limit carried on the route.
basis: fct-suavrma7, fct-exgth54l
because: The wrap directive asks the open CI-set rebuild to stand as a measured limit with a carry, and no master states it.
