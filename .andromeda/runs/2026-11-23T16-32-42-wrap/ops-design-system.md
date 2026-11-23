@row_add n=1 row=contract/screenshot/canvas-only-bytes-across-hosts area=surface fact=F1 detector=none
boundary: The screenshot verb
item: PNG bytes of a canvas-only shot
promise: A canvas-only shot, glyphs drawn with the document's font context included, reads the same PNG bytes on linux x86_64, windows x86_64 and macOS aarch64, whole and by id, in both layout modes, at scale 1.0 on the default CPU renderer; a shot with document text still differs.
basis: fct-rpckbvls, fct-w26btbps, fct-u6a6lt4g, fct-u34pici2
because: The capability statement now promises byte equality across the three hosts for a canvas-only shot, ratified as a requirement, and no row of the screenshot surface says what a shot's bytes hold across hosts.
@row_none n=1 fact=F2 type=9
what: the canvas conditions not measured (another SIMD level, a scale other than 1.0, gradient or image content) are an unmeasured note the capability statement and the route hold, not a decision or a contract.
