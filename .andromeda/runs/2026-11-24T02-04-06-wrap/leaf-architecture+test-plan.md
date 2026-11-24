@leaf_group group=architecture+test-plan
@leaf_read leaf=.claude/docs/commands.md lines=82
last: - Fonts missing in headless tests → `system-fonts` feature / 
@leaf_read leaf=.claude/docs/services/blitz-test-harness.md lines=37
last: - `.claude/rules/verification-harness.md` · `.claude/docs/te
@leaf_edit n=1 leaf=.claude/docs/commands.md line=61 class=omission
leaf-text: (`stand_canvas_cross_os`: three fixtures, each a canvas alone (a solid fill, an antialiased edge, canvas glyphs), shot whole and as its element in both layout modes, their answer digests held byte-equal on linux x86_64, windows x86_64 and macOS aarch64 to one committed table, no pixels committed;
source: dec-gwo4ddwp «Canvas shots are compared across hosts by one stand check: three canvas-alone fixtures shot whole and as their element, and Circle Drawer's canvas shot as its element, in both layout modes»
replace-with: (`stand_canvas_cross_os`: three fixtures, each a canvas alone (a solid fill, an antialiased edge, canvas glyphs), shot whole and as its element, and Circle Drawer's canvas shot as its element, in both layout modes, their answer digests held byte-equal on linux x86_64, windows x86_64 and macOS aarch64 to committed tables, no pixels committed;
@leaf_edit n=2 leaf=.claude/docs/services/blitz-test-harness.md line=16 class=omission
leaf-text: `seven_guis::stand::{boot, boot_alone, boot_timer, boot_cells, boot_load, boot_order, options, start_in}`
source: ctr-537tgfbo «stand::boot_circle_drawer(HarnessOptions) boots Circle Drawer in process alone, served by no escher-session role.»
replace-with: `seven_guis::stand::{boot, boot_alone, boot_timer, boot_cells, boot_load, boot_order, boot_circle_drawer, options, start_in}` — `boot_circle_drawer` booting Circle Drawer in process alone, served by no escher-session role
@leaf_stands n=1 leaf=.claude/docs/commands.md line=61
because: The line names only the stand's served and in-process checks and the load page's boot, never the full set of stand boots, so the contract's added Circle Drawer boot (in process only, served by no escher-session role) contradicts nothing it states.
