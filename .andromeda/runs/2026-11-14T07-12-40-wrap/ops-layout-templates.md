@row_add n=1 row=decision/stand-boot-default-run-seed area=surface-desktop-native fact=F1 detector=none
name: Stand boot default run seed
rule: The stand boot gives every harness-built document a run seed, the default seed 0 where the builder gave none, so two runs of one task read the same randomness at every step, in both layout modes and across processes; outside any run the windowed shell seeds from the host.
guard: stand_seeded_randomness
basis: fct-uaxpbl54, fct-ow6mpevg
because: The stand boot every session role shares now supplies a default run seed as it supplies the virtual clock, a new determinism condition of the stand boot that future stand work must respect.
@row_add n=2 row=decision/no-seed-on-driver-carriers area=surface-cli fact=F2 detector=none
name: No seed on the driver's carriers
rule: No run seed crosses the driver's carriers: the command line, the MCP tools, the actions and the kept-check file take no seed, and every session reads the default seed.
state: provisional
basis: fct-gn7pklgp, fct-uaxpbl54
because: Fork F1 was answered A, no seed on any carrier, provisional until the founder confirms or replaces it, and no cli row says so.
@row_none n=1 fact=F3 type=9
