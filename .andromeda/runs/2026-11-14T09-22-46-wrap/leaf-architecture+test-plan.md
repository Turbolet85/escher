@leaf_group group=architecture+test-plan
@leaf_read leaf=.claude/docs/commands.md lines=82
last: - Fonts missing in headless tests → `system-fonts` feature /
@leaf_read leaf=.claude/docs/services/blitz-test-harness.md lines=37
last: - `.claude/rules/verification-harness.md` · `.claude/docs/te
@leaf_edit n=1 leaf=.claude/docs/commands.md line=28 class=omission
leaf-text: --test stand_non_lean_tasks` — the headless-stand checks, package-alone
source: dec-fcya4gfl «decision/cross-os-readings-by-answer-digest · Cross-OS readings of the served tasks — The six served tasks are read on every CI OS by one stand check that compares the driver's answer digests for snapshot (every part), lint and actions»
replace-with: --test stand_non_lean_tasks --test stand_bundled_font_families --test stand_cross_os_readings` — the headless-stand checks, package-alone (`stand_cross_os_readings`: the six served tasks' driver answer digests for snapshot (every part), lint and actions, at boot and after one acting step in both layout modes, compared on every CI OS to one committed table, with no shot and no kept check file)
@leaf_edit n=2 leaf=.claude/docs/commands.md line=82 class=omission
leaf-text: they boot with the bundled DejaVu Sans (`seven_guis::stand::options`)
source: dec-pgrj4xim «decision/stand-typography-dejavu-sans · Stand typography is DejaVu Sans — Every stand boot loads one font, the bundled DejaVu Sans, with system fonts off, so every generic family and any unavailable named family lays out in DejaVu Sans by fallback»
replace-with: they boot with the bundled DejaVu Sans alone, system fonts off, every generic family and any unavailable named family laying out in it by fallback (`seven_guis::stand::options`; checked by `stand_bundled_font_families`)
