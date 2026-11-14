@leaf_group group=architecture-3
@leaf_read leaf=.claude/docs/services/seven_guis.md lines=45
last: - `.claude/rules/a11y.md` (the stand is in its paths) · `.cla
@leaf_read leaf=.claude/docs/stack.md lines=60
last: 4. Commit as `chore: bump {crate} to {version}`.
@leaf_read leaf=.claude/docs/workflow.md lines=48
last: A reference-based CLAUDE.md: a thin index, with depth in `.cl
@leaf_edit n=1 leaf=.claude/docs/services/seven_guis.md line=38 class=omission
leaf-text: so a check compared byte for byte across OSes holds no `screenshot` step;
source: dec-n3ep46dv «decision/stand-readings-same-on-every-ci-os · Stand readings identical on every CI OS — The six served stand tasks read the same driver answer digests of snapshot, lint and actions on linux x86_64, windows x86_64 and macOS aarch64,»
replace-with: so a check compared byte for byte across OSes holds no `screenshot` step; `stand_cross_os_readings` holds the six served stand tasks' driver answer digests of snapshot, lint and actions the same on linux x86_64, windows x86_64 and macOS aarch64, in both layout modes, at boot and after one step, compared with no screenshot;
@leaf_edit n=2 leaf=.claude/docs/services/seven_guis.md line=19 class=omission
leaf-text: `font_ctx()` (bundled DejaVu Sans, system fonts off);
source: dec-pgrj4xim «decision/stand-typography-dejavu-sans · Stand typography is DejaVu Sans — Every stand boot loads one font, the bundled DejaVu Sans, with system fonts off, so every generic family and any unavailable named family lays out in DejaVu Sans by fallback;»
replace-with: `font_ctx()` (bundled DejaVu Sans, the one font, system fonts off — every generic family and any unavailable named family lays out in it by fallback, guarded by `stand_bundled_font_families`);
