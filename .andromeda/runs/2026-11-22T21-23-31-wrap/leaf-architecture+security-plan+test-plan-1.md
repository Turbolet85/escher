@leaf_group group=architecture+security-plan+test-plan-1
@leaf_read leaf=.claude/docs/services/escher-driver.md lines=63
last: - The command line, through the real `escher-session` binary
@leaf_edit n=1 leaf=.claude/docs/services/escher-driver.md line=32 class=omission
leaf-text: as the placing words `select` and `select-list` in a layout answer
source: dec-goyp2c5r «The select list crosses the ratified carriers only as a row's own bounds in the snapshot, the diff and a layout answer, as the layout answer's select-list placing word»
replace-with: as a row's own `bounds` and the placing words `select` and `select-list` in a layout answer
@leaf_edit n=2 leaf=.claude/docs/services/escher-driver.md line=60 class=stale
leaf-text: `command.rs` · `execute.rs`
source: dec-rlfpyplu «An escher-driver module holds its unit tests in a #[path] child file of its own, never a module of the crate's public tree.»
replace-with: `command_tests.rs` · `execute.rs`
@leaf_edit n=3 leaf=.claude/docs/services/escher-driver.md line=51 class=stale
leaf-text: In `command.rs` an assertion over a call carries a row index only
source: dec-rlfpyplu «An escher-driver module holds its unit tests in a #[path] child file of its own, never a module of the crate's public tree.»
replace-with: In `command.rs`'s tests (`command_tests.rs`, its `#[path]` child) an assertion over a call carries a row index only
