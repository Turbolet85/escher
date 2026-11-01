# Classified — tests · unit-test-strategy.b1

## Answers
unit-test-strategy#1.1 · yes · the `bridge_tests` `#[path]` declaration at the end of dioxus_document.rs (lines 834-836) sits in a conflict surface a resolution must keep; the other unit-test locations are S6's list
unit-test-strategy#2.1 · no · blitz-shell's redraw-wake test: the crate is not reached, and the scope names winit's pin, not this wake behaviour
unit-test-strategy#3.1 · yes · contrast.rs's fixture that an element with no text reads `None` is the re-read of `Harness::text_contrast`'s None conditions; screenshot.rs's opaque-white unpainted default and whole-viewport shot are the painted default the font-embolden witness reads; the settle loop is not reached
unit-test-strategy#3.2 · no · escher-driver's wire, session, error, refusal, schema, recording and CLI pins are escher-owned code outside every predicted conflict, which the scope keeps out
unit-test-strategy#4.1 · no · the help strings of click, type, press and covered are fixed escher wording in escher-driver, a file not reached and not moved by the take
unit-test-strategy#5.1 · no · the MCP listing helpers test escher-driver's tool listing, outside every reached file and the scope's MCP wire form
unit-test-strategy#6.1 · no · type's help on a select is a fixed escher-driver string in a file not reached
unit-test-strategy#7.1 · yes · the ci.yml invariants and ci-leg.sh's legs pinned by test_ci_workflows.py: the matrix_test free-disk `if:` edit and the font-embolden gate joining ci-leg.sh, both named by the scope
unit-test-strategy#8.1 · yes · BlitzTestsTargetsTest pins the blitz-tests target list (one `[[test]]` table, `all` at tests/all.rs with `test = false`), which the scope names as a re-read and which a take carrying upstream's tests/all.rs moves
unit-test-strategy#9.1 · no · the cold-agent pipe's tests run escher-owned scripts and driver code outside every predicted conflict, and the scope excludes the pipe
unit-test-strategy#10.1 · no · the capability statement's pin over docs/capabilities.md and README.md, which the scope excludes as no verb, cause or tool count moves
unit-test-strategy#11.1 · yes · the four DISABLED/ENABLED tests at node.rs:2090-2169 live in node.rs, a predicted conflict surface, and a resolution there must keep them
unit-test-strategy#12.1 · yes · node/select.rs's step, span and group-order rules are the select model under the select-list merge surface the Needs name
unit-test-strategy#13.1 · yes · node/select_list.rs is the select list the merge-surface row re-reads, and its placement, width, edge, size, scroll and point rules must hold after a resolution
unit-test-strategy#14.1 · no · form.rs's submitted-entries tests belong to form submission, a file no reached-file or merge-surface entry names and no contract the scope names
unit-test-strategy#15.1 · no · WPT runner tests are upstream-only and out of the chunk's scope
unit-test-strategy#16.1 · no · an absence over slice s03 names no reached file, and the chunk adds no unit test that S6 would place there
unit-test-strategy#16.2 · no · an absence over the other seven files of slice s07 names no reached file, and the chunk adds no unit test
unit-test-strategy#16.3 · yes · bootstrap.rs and events.rs, both predicted conflict surfaces in blitz-vibey-script, hold no test module, and the script tests stand in tests/ as the Needs' script-engine security entry names
unit-test-strategy#16.4 · yes · the zero count of in-file cfg(test) modules over tests/blitz-tests covers the predicted conflict file pseudo_element_flow_classification.rs and the edited select_popup.rs, and upstream's carried test files must keep it
