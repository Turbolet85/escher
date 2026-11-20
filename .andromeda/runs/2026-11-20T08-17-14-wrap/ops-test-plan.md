@row_set n=1 row=decision/inline-cfg-test-modules cell=except seen=2 fact=F5 detector=D-tests-framework
value: Unit tests may sit out of line in a sibling file with the _tests suffix, declared as a cfg(test) module by path at their parent, and fixture builders in a child module of that test module.
basis: fct-nmmch6lh, fct-xmrlwmoz, fct-w6h4uegz
because: The chunk moved the snapshot reader's tests out of line into a _tests sibling and gave the driver JSON tests a fixtures child, so the convention's exception now names the _tests sibling and the fixtures child.
@row_set n=2 row=decision/inline-unit-test-modules cell=except seen=1 detector=D-tests-framework
value: A source file's unit tests may sit out of line in a sibling file with the _tests suffix, declared as a cfg(test) module by path, and fixture builders may sit in a child module of that test module.
basis: fct-nmmch6lh, fct-xmrlwmoz, fct-w6h4uegz
because: The twin section's exception says the out-of-line form less than the convention now holds, with its _tests sibling and fixtures child.
@row_set n=3 row=decision/complexity-ceiling-host-only cell=rule seen=2 detector=none
value: The cognitive-complexity ceiling of 15 is read on the dev host alone, test functions counted, and no CI leg runs it; escher's own functions are held under it, and upstream-authored functions over it are kept as upstream wrote them.
basis: fct-w3akakwb, fct-gffzjga5
because: The chunk brought escher's own functions under the ceiling and the upstream-authored ones over it were ruled kept as upstream's, so the row's «binds no function» no longer holds.
@row_add n=1 row=decision/upstream-oversize-files-kept area=quality-gates-coverage-targets fact=F1 detector=none
name: Upstream-authored oversize files kept as upstream's
rule: Upstream-authored files over 800 code lines are kept as upstream wrote them; the 800-line file ceiling binds escher's own files only.
why: Ratified.
basis: fct-w3akakwb, fct-sbsb2qha
because: The ruling F1 keeps upstream's oversize files as upstream's, a standing decision no row of the section states.
@row_add n=2 row=decision/own-file-size-ceiling area=quality-gates-coverage-targets fact=F2 detector=none
name: File-size ceiling of 800 code lines on escher's own files
rule: Escher's own files are held under 800 code lines.
why: Ratified.
except: The integration-test targets already over the ceiling are kept whole, with no route entry to split them.
basis: fct-w3akakwb, fct-gffzjga5
because: The chunk held escher's own files under 800 code lines save the integration-test targets the ruling F4 keeps whole, a bar no row of the section states.
@row_add n=3 row=decision/upstream-test-clone-pairs-kept area=quality-gates-coverage-targets fact=F3 detector=none
name: Upstream's duplicated test pairs kept as upstream's
rule: Upstream's cdp-server and vibey-script duplicated test pairs are kept as upstream wrote them, and the duplication reading is not held against them.
why: Ratified.
basis: fct-w3akakwb, fct-wh2egjta
because: The ruling F3 keeps upstream's test clone pairs and discharges the duplication watch, a standing decision no row of the section states.
@row_add n=4 row=decision/test-files-out-of-coverage-by-suffix area=quality-gates-coverage-targets fact=F4 detector=D-tests-coverage
name: Test files left out of coverage by their _tests suffix
rule: A unit-test file included by path is left out of its crate's coverage reading only when its name carries the _tests suffix; a test child module of any other name is counted in the reading.
basis: fct-q2q55jhq, fct-jdhzbl7x
because: The chunk's fixtures child, named without the suffix, was counted and moved escher-driver's reading, so which test files the coverage reading leaves out is a convention the plan must state.
