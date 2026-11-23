@leaf_group group=architecture+test-plan-1
@leaf_read leaf=.claude/docs/commands.md lines=82
last: - Fonts missing in headless tests → `system-fonts` feature /
@leaf_read leaf=.claude/docs/gotchas.md lines=68
last: - Decisions and rationale: `doc:architecture` (the records s
@leaf_read leaf=.claude/docs/services/blitz-test-harness.md lines=37
last: - `.claude/rules/verification-harness.md` · `.claude/docs/te
@leaf_edit n=1 leaf=.claude/docs/commands.md line=61 class=stale
leaf-text: a shot's PNG bytes differ between linux x86_64 and macOS aarch64, so a check compared byte for byte across OSes holds no `screenshot` step
source: dec-6vuh2wac «A `screenshot` of a canvas alone, its PNG bytes held equal on linux x86_64, windows x86_64 and macOS aarch64, is compared across OSes by its answer digest.»
replace-with: a shot holding document text differs in its PNG bytes between linux x86_64 and macOS aarch64, so a check compared byte for byte across OSes holds no `screenshot` step, save a `screenshot` of a canvas alone, its PNG bytes held equal on linux x86_64, windows x86_64 and macOS aarch64 and compared across OSes by its answer digest
@leaf_edit n=2 leaf=.claude/docs/commands.md line=61 class=omission
leaf-text: --test stand_canvas_shot` (`stand_canvas_shot`:
source: dec-gwo4ddwp «Canvas shots are compared across hosts by one stand check: three fixtures (a solid fill, an antialiased edge, canvas glyphs), each a canvas alone»
replace-with: --test stand_canvas_shot --test stand_canvas_cross_os` (`stand_canvas_cross_os`: three fixtures, each a canvas alone (a solid fill, an antialiased edge, canvas glyphs), shot whole and as its element in both layout modes, their answer digests held byte-equal on linux x86_64, windows x86_64 and macOS aarch64 to one committed table, no pixels committed; `stand_canvas_shot`:
