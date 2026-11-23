@leaf_group group=architecture+test-plan-3
@leaf_read leaf=CLAUDE.md lines=166
last: <!-- USER:session-learnings end -->
@leaf_edit n=1 leaf=CLAUDE.md line=122 class=stale
leaf-text: a shot's PNG bytes differ between linux x86_64 and macOS aarch64, so a check compared byte for byte across OSes holds no `screenshot` step
source: dec-vlmnnziw «A shot holding document text differs in PNG bytes between linux x86_64 and macOS aarch64; a canvas-only shot reads equal on linux x86_64, windows x86_64 and macOS aarch64.»
replace-with: a shot holding document text differs in its PNG bytes between linux x86_64 and macOS aarch64, so a check compared byte for byte across OSes holds no such `screenshot` step (a canvas-only shot reads byte-equal on linux x86_64, windows x86_64 and macOS aarch64, its answer digests, whole and as its element in both layout modes, held to one committed table by `stand_canvas_cross_os`, no pixels committed)
