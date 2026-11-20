@row_set n=1 row=decision/unit-tests-in-source-file cell=rule seen=1 detector=D-tests-framework
value: Unit tests sit in a test-only module inside the source file, or out of line in a sibling file with the _tests suffix declared test-only by path at its parent; a module moved out of line keeps its test paths unchanged.
basis: fct-grexume2, fct-v2hfin6i, fct-ktvznktr
because: The chunk's new unit tests were written out of line in a sibling _tests file declared by path, on the operator's review, which the row's rule allows only for a module moved out of line, so the rule must name the sibling-file placement.
@row_add n=1 row=decision/mutation-filter-anchored-on-verb area=quality-gates-coverage-targets fact=F1 detector=none
name: Scoped mutation run filter anchored on the verb
rule: A cargo-mutants run scoped to one function anchors its filter on the mutant name's verb, the replace verb then the function's name and a space, and its mutant listing is read first and must show exactly the mutants the run is meant to test.
why: The filter is an unanchored regex over the whole mutant name, location included, so a bare function name matches other functions holding it as a substring.
basis: fct-wew7vhwb, fct-zgzblpov
because: A run filtered on the bare function name tested a mutant outside its scope, and the mended filter anchored on the verb, checked by its listing first, is now the convention for scoped mutation runs.
