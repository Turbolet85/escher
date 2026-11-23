@leaf_group group=architecture-3
@leaf_read leaf=.claude/docs/services/seven_guis.md lines=44
last: - `.claude/rules/a11y.md` (the stand is in its paths) · `.cla
@leaf_read leaf=.claude/docs/stack.md lines=59
last: 4. Commit as `chore: bump {crate} to {version}`.
@leaf_read leaf=.claude/docs/workflow.md lines=47
last: A reference-based CLAUDE.md: a thin index, with depth in `.c
@leaf_edit n=1 leaf=.claude/docs/services/seven_guis.md line=11 class=stale
leaf-text: no `screenshot` step, a shot's PNG bytes differing by OS)
source: dec-r6p4rs24 «why: A shot holding document text differs in its PNG bytes by OS, so no screenshot step is kept.»
replace-with: no `screenshot` step, a shot holding document text differing in its PNG bytes by OS)
@leaf_edit n=2 leaf=.claude/docs/services/seven_guis.md line=38 class=stale
leaf-text: a shot's PNG bytes differ between linux x86_64 and macOS aarch64, so a check compared byte for byte across OSes holds no `screenshot` step;
source: dec-vlmnnziw «A shot holding document text differs in PNG bytes between linux x86_64 and macOS aarch64; a canvas-only shot reads equal on linux x86_64, windows x86_64 and macOS aarch64.»
replace-with: a shot holding document text differs in PNG bytes between linux x86_64 and macOS aarch64, so a check compared byte for byte across OSes holds no `screenshot` step, save a canvas-only shot, held byte-equal on linux x86_64, windows x86_64 and macOS aarch64 by one committed digest table, no pixels committed;
