@row_add n=1 row=decision/a11y-leg-verified-skia-archive area=9-ci-integration fact=F1 detector=none
name: Verified skia archive for the a11y leg
rule: The a11y CI leg builds skia only from the prebuilt archive verified against the committed sha256 record, and a refused archive ends the leg before any test runs.
guard: test_skia_legs_build_from_the_verified_archive; test_a_refused_archive_ends_the_leg_before_cargo
basis: fct-z3khe52c
because: The leg runner now fetches and verifies skia's prebuilt archive first for every skia-building leg, the a11y leg among them, and ends the leg on a refused archive, a gate no a11y-plan row states.
