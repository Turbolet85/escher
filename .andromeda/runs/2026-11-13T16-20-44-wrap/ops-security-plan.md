@row_set n=1 row=decision/ci-actions-pinned-to-sha cell=guard seen=1 fact=F2 detector=D-security-deps
value: WorkflowPinTest in test_ci_workflows.py, reading every workflow file
basis: fct-spbatvsg, fct-uluruybu
because: The pin check moved into a new WorkflowPinTest that reads every workflow, not the CI workflow alone, so the row's guard names the test that now holds the decision.
@row_set n=2 row=decision/unresolvable-stylesheet-href-no-load cell=why seen=1 fact=F3 detector=D-security-input
value: Resolving an href answers none for an unresolvable URL and every caller skips it, so no href resolution panics.
basis: fct-xve6mmf5
because: The engine's href resolver now returns none where it panicked, its callers skipping it, so the row's reason that a panicking resolver is avoided is no longer true.
@row_set n=3 row=decision/hintless-font-face-source-sniffed cell=rule seen=1 fact=F5 detector=D-security-input
value: An @font-face source is chosen by a closed match on its format string, then on its URL extension ignoring query and fragment; an unresolvable or unsupported source is skipped and never fetched, and one with no format hint and no extension is fetched and its format sniffed from its bytes.
basis: fct-xve6mmf5, fct-nctk6ufi
because: The take adds a closed format-string and extension match that skips unsupported font-face sources before any fetch, ratified as upstream's behaviour, so the row says it less than it now holds.
@row_add n=1 row=decision/untrusted-bytes-decoded-without-panic area=input-validation fact=F4 detector=D-security-input
name: Page and stylesheet bytes decoded without panic
rule: A page whose bytes are not UTF-8 is decoded lossily and never panics, and a stylesheet is decoded from its bytes by its BOM and @charset rule.
why: Upstream's behaviour, ratified as an engine change for every document.
basis: fct-xve6mmf5, fct-nctk6ufi, fct-g5byr2t5
because: The take changes how every document's untrusted page and stylesheet bytes are decoded, a ratified widening no row of Input Validation yet states.
@row_add n=2 row=decision/presentational-hints-typed-parse area=input-validation fact=F6 detector=D-security-input
name: Presentational hint attributes read by a typed parse
rule: The font element's face and size, valign, cellspacing and cellpadding are read as presentational hints by a typed parse, and a value that does not parse is ignored.
why: Upstream's behaviour, ratified as an engine change for every document.
basis: fct-xve6mmf5, fct-nctk6ufi, fct-g5byr2t5
because: The take adds new markup attribute readers on page content, remote pages included, whose validation is a typed parse that ignores what does not parse.
@row_none n=1 fact=F1 type=9
what: The RUSTSEC-2026-0459 ignore is an owned residual until the browser moves to rusqlite 0.41 or later, the route's item; the standing per-ID ignore rules hold unchanged.
@row_none n=2 fact=F7 type=4
relied: The accessibility builder reads an img with an empty alt as a decorative generic container, a closed role inside the already ratified adapter crossing.
