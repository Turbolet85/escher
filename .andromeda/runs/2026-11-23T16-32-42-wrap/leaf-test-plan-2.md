@leaf_group group=test-plan-2
@leaf_read leaf=.claude/rules/testing.md lines=78
last: - 2026-11-23: While an editor's cargo (rust-analyzer's `carg
@leaf_read leaf=.claude/rules/verification-harness.md lines=42
last: - 2026-10-31: State what a CI job did from that job's own lo
@leaf_edit n=1 leaf=.claude/rules/testing.md line=36 class=stale
leaf-text: (a shot's PNG bytes differ by OS)
source: dec-bdbzxscn «decision/no-screenshot-in-recorded-check-scripts · No screenshot step in committed recorded checks — The scripts that record the committed checks hold no `screenshot` step, so the kept checks stay byte-equal»
replace-with: (a shot holding document text differs in its PNG bytes by OS)
@leaf_edit n=2 leaf=.claude/rules/testing.md line=36 class=omission
leaf-text: with no shot and no kept check file.
source: dec-gwo4ddwp «decision/canvas-shots-cross-os-by-digest · Canvas shots compared across hosts by digest — Canvas shots are compared across hosts by one stand check: three fixtures (a solid fill, an antialiased edge, canvas glyphs)»
replace-with: with no shot and no kept check file; a shot of a canvas alone is the one shot compared across OSes, by `stand_canvas_cross_os`: three fixtures (a solid fill, an antialiased edge, canvas glyphs), each shot whole and as its element in both layout modes, their answer digests held byte-equal on linux x86_64, windows x86_64 and macOS aarch64 to one committed table, no pixels committed, while a shot holding document text stays out of every cross-OS byte check.
