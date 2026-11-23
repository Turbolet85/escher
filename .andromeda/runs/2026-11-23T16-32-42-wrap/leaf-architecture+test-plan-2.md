@leaf_group group=architecture+test-plan-2
@leaf_read leaf=.claude/docs/services/escher-driver.md lines=63
last: - The command line, through the real `escher-session` binary
@leaf_edit n=1 leaf=.claude/docs/services/escher-driver.md line=61 class=stale
leaf-text: a shot's PNG bytes differ between linux x86_64 and macOS aarch64 (CI#39225375577), so a check compared byte for byte across OSes holds no `screenshot` step
source: dec-6vuh2wac «A shot holding document text differs in PNG bytes between linux x86_64 and macOS aarch64; a canvas-only shot reads equal on linux x86_64, windows x86_64 and macOS aarch64.»
replace-with: a shot holding document text differs in PNG bytes between linux x86_64 and macOS aarch64 (CI#39225375577), so a check compared byte for byte across OSes holds no such `screenshot` step, while a shot of a canvas alone, its PNG bytes held equal on linux x86_64, windows x86_64 and macOS aarch64, is compared across OSes by its answer digest — `stand_canvas_cross_os`, three canvas-only fixtures shot whole and as their element in both layout modes, held to one committed digest table, no pixels committed
