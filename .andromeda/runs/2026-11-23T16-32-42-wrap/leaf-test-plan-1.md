@leaf_group group=test-plan-1
@leaf_read leaf=.claude/docs/tests-summary.md lines=56
last: **Full plan:** `.andromeda/test-plan.md`. Path-scoped rules:
@leaf_edit n=1 leaf=.claude/docs/tests-summary.md line=21 class=stale
leaf-text: a shot's PNG bytes differ between linux x86_64 and macOS aarch64 (CI#39225375577), so a committed check holds no `screenshot` step
source: dec-acdz5ghz «why: a shot holding document text differs in PNG bytes between linux x86_64 and macOS aarch64, so such a shot is byte-identical on one host only»
replace-with: a shot holding document text differs in PNG bytes between linux x86_64 and macOS aarch64 (CI#39225375577), so such a shot is byte-identical on one host only and a committed check holds no `screenshot` step
@leaf_edit n=2 leaf=.claude/docs/tests-summary.md line=26 class=stale
leaf-text: a shot's PNG bytes differ between linux x86_64 and macOS aarch64 (CI#39225375577), so a check compared byte for byte across OSes holds no shot,
source: dec-7qmunnbr «A check compared byte for byte across OSes holds no shot. · why: A shot holding document text differs in PNG bytes between linux x86_64 and macOS aarch64; a canvas-only shot reads equal on linux x86_64»
replace-with: a shot holding document text differs in PNG bytes between linux x86_64 and macOS aarch64 (CI#39225375577), so a check compared byte for byte across OSes holds no shot, save a shot of a canvas alone, its PNG bytes held equal on linux x86_64, windows x86_64 and macOS aarch64 and compared across OSes by its answer digest (`stand_canvas_cross_os`: three canvas-only fixtures — a solid fill, an antialiased edge, canvas glyphs — shot whole and as its element in both layout modes, their answer digests held byte-equal to one committed table, no pixels committed),
