@row_add n=1 row=decision/canvas-shot-digest-equal-across-hosts area=3-observability-harness-contract fact=F1 detector=none
name: Canvas-only shot digest equal across hosts
rule: The screenshot answer's digest for a canvas-only shot is byte-equal on linux x86_64, windows x86_64 and macOS aarch64, held by a committed table of answer digests; a shot holding document text still differs between linux x86_64 and macOS aarch64.
why: Ratified: byte equality is required of a canvas-only shot across hosts, rather than a tolerance.
guard: stand_canvas_cross_os
basis: fct-rpckbvls, fct-u6a6lt4g, fct-u34pici2
because: The chunk measured and committed a digest table holding canvas-only screenshot answers byte-equal on the three CI hosts, scoping the shot-bytes limit to shots holding document text, and no row of the area states it.
