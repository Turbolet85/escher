@row_add n=1 row=contract/skia-archive-lines/success area=ci-integration fact=F1 detector=none
boundary: The skia archive step's lines
item: success
promise: On success the skia archive step exits 0 and prints exactly one stdout line, the SKIA_BINARIES_URL file:// template, and nothing on stderr.
basis: fct-z3khe52c
because: The chunk adds the skia archive fetch-and-verify step with a fixed stdout line on success, a contract its CI callers read.
@row_add n=2 row=contract/skia-archive-lines/attempt area=ci-integration fact=F1 detector=none
boundary: The skia archive step's lines
item: attempt
promise: Each failed attempt writes one stderr line, skia-archive: attempt {n} of 3 failed in fetch or verify, kept in the CI step or leg log.
basis: fct-z3khe52c
because: The chunk adds fixed stderr lines for the skia archive step, kept in the step or leg log, which obs-plan §9 is owed.
@row_add n=3 row=contract/skia-archive-lines/exit-1 area=ci-integration fact=F1 detector=none
boundary: The skia archive step's lines
item: exit 1
promise: When no attempt yields a verified archive the step writes skia-archive: no verified archive on stderr and exits 1.
basis: fct-z3khe52c
because: The chunk adds the skia archive step's exit 1 with its fixed stderr line.
@row_add n=4 row=contract/skia-archive-lines/exit-2 area=ci-integration fact=F1 detector=none
boundary: The skia archive step's lines
item: exit 2
promise: A bad call writes skia-archive: usage and a target with no recorded archive writes skia-archive: no recorded archive for this target, each on stderr at exit 2 with nothing fetched.
basis: fct-z3khe52c
because: The chunk adds the skia archive step's exit 2 with its two fixed stderr lines.
@row_add n=5 row=contract/skia-archive-witness-line/match area=ci-integration fact=F2 detector=none
boundary: The matrix witness step's lines
item: match
promise: The matrix job's witness step reads each row's skia-bindings build output by content and prints skia-archive: build output {path}:{line} for each match.
basis: fct-xp4s2p66
because: The chunk adds the matrix job's witness step, whose printed lines are what CI shows of a row's skia build reading the verified archive.
@row_add n=6 row=contract/skia-archive-witness-line/no-match area=ci-integration fact=F2 detector=none
boundary: The matrix witness step's lines
item: no match
promise: When no skia-bindings build read the verified archive the witness step prints skia-archive: no skia-bindings build read the verified archive; its build tree, out/ not descended: and a listing of the skia-bindings build paths, then fails the row.
basis: fct-xp4s2p66
because: The chunk adds the witness step's red path with its fixed line and build-tree listing.
@row_add n=7 row=decision/skia-source-fallback-read-red area=ci-integration fact=F3 detector=none
name: Skia source-build fallback observed only as red
rule: A skia build that misses its verified archive, by a failed read or an unrecorded key, falls back to a full source build of skia with no panic; the fallback is observed only as red, by the witness step on CI and by the skia-bindings output check on the dev host.
why: Forcing the binaries download in a crates.io build routes skia-bindings to a full source build rather than a panic, so the fallback is stated as a limit.
basis: fct-pufyyo3s
because: The chunk drops the planned forced download and states the source-build fallback as a limit read red, a decision on how this failure is observed.
@row_add n=8 row=decision/refused-skia-archive-ends-leg area=ci-integration fact=F4 detector=none
name: Refused skia archive ends the leg before cargo
rule: Every skia-building CI leg and the fast leg's closing sweep run cargo under the verified SKIA_BINARIES_URL, and a refused archive ends the leg before any cargo runs, its reason being the skia-archive lines in the leg log.
basis: fct-z3khe52c
because: The chunk makes the leg runner export the verified archive URL and stop on a refusal, with the reason observable only in the leg log.
