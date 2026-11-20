@row_add n=1 row=decision/source-file-size-bar area=conventions fact=F1 detector=none
name: Source files at most 800 code lines
rule: Escher's own source files stay at or under 800 code lines.
why: The exceptions are ratified.
except: Escher's seven oversize integration-test targets are kept as they are, and upstream-authored files over the bar are kept as upstream's.
basis: fct-w3akakwb, fct-gffzjga5
because: The founder ruled escher's own files bound by the 800-line bar with its two named exceptions, a convention future work must respect that no row of architecture states.
@row_set n=1 row=decision/cognitive-complexity-ceiling cell=except seen=1 fact=F2 detector=none
value: Upstream-authored functions over the ceiling are kept as upstream's; the ceiling binds escher's own functions alone.
basis: fct-w3akakwb, fct-sbsb2qha
because: The founder ruled upstream's functions over cognitive 15 kept as upstream's, so the ceiling row must name that the bar binds escher's own functions alone.
@row_add n=2 row=decision/upstream-tests-not-deduplicated area=conventions fact=F3 detector=none
name: Upstream tests not deduplicated
rule: Escher does not deduplicate upstream-authored tests; upstream's duplicated test pairs in the cdp-server and vibey-script crates are kept as upstream's.
why: Ratified.
basis: fct-w3akakwb, fct-wh2egjta
because: The founder ruled upstream's cdp-server and vibey-script test clone pairs kept as upstream's, a convention bounding future duplication work that no row of architecture states.
@row_add n=3 row=decision/coverage-reading-leaves-out-tests-files area=stack-and-technologies fact=F4 detector=none
name: Coverage reading leaves out only tests-suffixed files
rule: A crate's coverage reading leaves out only files named with the `_tests` suffix, so a `#[path]` test child of any other name is counted in the reading.
basis: fct-jdhzbl7x, fct-q2q55jhq
because: The chunk measured that a path test child not named with the tests suffix is counted in its crate's coverage reading, a guarantee of the coverage configuration that binds how future test children are named.
