@row_set n=1 row=decision/cross-os-checks-hold-no-shot cell=except seen=1 detector=none
value: A shot of a canvas alone, its PNG bytes held equal on linux x86_64, windows x86_64 and macOS aarch64, is compared across OSes by its answer digest.
basis: fct-rpckbvls, fct-y2t2bg5i, fct-u34pici2
because: A canvas-only shot is now compared byte for byte across OSes and held equal on all three CI hosts on a ratified requirement of byte equality, so the row's flat ban gains that exception.
@row_set n=2 row=decision/no-shot-in-cross-os-byte-checks cell=except seen=1 detector=none
value: A shot of a canvas alone, its PNG bytes held equal on linux x86_64, windows x86_64 and macOS aarch64, is compared across OSes by its answer digest.
basis: fct-rpckbvls, fct-y2t2bg5i, fct-u34pici2
because: A canvas-only shot is now compared byte for byte across OSes and held equal on all three CI hosts on a ratified requirement of byte equality, so the row's flat ban gains that exception.
@row_set n=3 row=decision/no-screenshot-in-cross-os-checks cell=except seen=1 detector=none
value: A `screenshot` of a canvas alone, its PNG bytes held equal on linux x86_64, windows x86_64 and macOS aarch64, is compared across OSes by its answer digest.
basis: fct-rpckbvls, fct-y2t2bg5i, fct-u34pici2
because: A canvas-only shot is now compared byte for byte across OSes and held equal on all three CI hosts on a ratified requirement of byte equality, so the row's flat ban gains that exception.
@row_set n=4 row=decision/no-screenshot-step-in-committed-checks cell=why seen=1 detector=none
value: a shot holding document text differs in PNG bytes between linux x86_64 and macOS aarch64, so such a shot is byte-identical on one host only
basis: fct-rpckbvls, fct-v6nbc4k3
because: The stated limit on a shot's PNG bytes across hosts is now scoped to a shot holding document text, a canvas-only shot held equal, so the rule's reason narrows to what was measured while the rule stands.
@row_add n=1 row=decision/canvas-shots-cross-os-by-digest area=test-strategy fact=F1 detector=none
name: Canvas shots compared across hosts by digest
rule: Canvas shots are compared across hosts by one stand check: three fixtures (a solid fill, an antialiased edge, canvas glyphs), each a canvas alone, shot whole and as its element in both layout modes, their answer digests held byte-equal to one committed table, no pixels committed.
why: Ratified as byte equality required, not a tolerance; held on linux x86_64, windows x86_64 and macOS aarch64.
guard: stand_canvas_cross_os
basis: fct-w4fut5je, fct-o4fpla67, fct-y2t2bg5i, fct-u34pici2
because: The chunk brings a ratified cross-host digest comparison of canvas shots that no standing row states, so the test strategy gains it.
@row_none n=1 fact=F2 type=9
what: the unmeasured conditions of a canvas shot's cross-host equality (another SIMD level, a scale other than 1.0, content beyond the three fixtures) are a not-yet-measured limit the capability statement holds; the document-text difference is carried by the except and why sets of the no-shot rows
