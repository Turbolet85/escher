@leaf_group group=test-plan-2
@leaf_read leaf=.claude/rules/testing.md lines=78
last: - 2026-11-23: While an editor's cargo (rust-analyzer's `carg
@leaf_read leaf=.claude/rules/verification-harness.md lines=42
last: - 2026-10-31: State what a CI job did from that job's own lo
@leaf_edit n=1 leaf=.claude/rules/testing.md line=36 class=stale
leaf-text: three fixtures (a solid fill, an antialiased edge, canvas glyphs), each shot whole and as its element in both layout modes, their answer digests held byte-equal on linux x86_64, windows x86_64 and macOS aarch64 to one committed table, no pixels committed
source: dec-gwo4ddwp «Canvas shots are compared across hosts by one stand check: three canvas-alone fixtures shot whole and as their element, and Circle Drawer's canvas shot as its element, in both layout modes»
replace-with: three canvas-alone fixtures (a solid fill, an antialiased edge, canvas glyphs), each shot whole and as its element, and Circle Drawer's canvas shot as its element, in both layout modes, their answer digests held byte-equal on linux x86_64, windows x86_64 and macOS aarch64 to committed tables, no pixels committed
@leaf_edit n=2 leaf=.claude/rules/testing.md line=36 class=omission
leaf-text: while a shot holding document text stays out of every cross-OS byte check.
source: dec-ldhhfcng «Text an app paints on a canvas that a cross-host shot comparison holds is drawn with its glyph origins on whole device pixels»
replace-with: while a shot holding document text stays out of every cross-OS byte check; text an app paints on a canvas such a comparison holds (Circle Drawer's diameter label) is drawn with its glyph origins on whole device pixels.
@leaf_edit n=3 leaf=.claude/rules/testing.md line=30 class=omission
leaf-text: the served pages through `stand::boot_cells`, `stand::boot_load` and `stand::boot_order` —
source: ctr-ymqabqx2 «seven_guis::stand, native only, boots the four lean tasks, Cells, Circle Drawer, a load page, the order page and one component alone in a pinned 800x600 Light viewport with one bundled font»
replace-with: the served pages through `stand::boot_cells`, `stand::boot_load` and `stand::boot_order`, and Circle Drawer, in process only and served by no escher-session role, through `stand::boot_circle_drawer` —
@leaf_edit n=4 leaf=.claude/rules/verification-harness.md line=36 class=omission
leaf-text: and beside the lean tasks Cells through `stand::boot_cells`,
source: ctr-ymqabqx2 «seven_guis::stand, native only, boots the four lean tasks, Cells, Circle Drawer, a load page, the order page and one component alone in a pinned 800x600 Light viewport with one bundled font»
replace-with: and beside the lean tasks Cells through `stand::boot_cells`, Circle Drawer in process only, served by no escher-session role, through `stand::boot_circle_drawer(HarnessOptions)`,
@leaf_stands n=1 leaf=.claude/rules/testing.md line=18
because: The line names the stand's LoadNet and `stand::boot_load` as stand-ins and boots, which the contract still states (a load page booted by seven_guis::stand), and lists no boot set the change to the promise could make stale.
