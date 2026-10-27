# Classified — tests · unit-test-strategy.b1

## Answers
unit-test-strategy#1.1 · yes · the scope keeps lint.rs's inline unit tests unchanged and edits a test in mcp_tests.rs, a `#[cfg(test)]` `#[path]` child of mcp.rs whose test paths stay unchanged
unit-test-strategy#2.1 · yes · contrast.rs pins the `text_contrast` reader the lint's contrast defect calls unchanged: `None` for a text-less element, the WCAG ratio, and `required_ratio` (3.0 for large text, else 4.5)
unit-test-strategy#2.2 · yes · escher-driver's unit tests pin the findings order the edited `findings` must keep (document order, then `Defect::ALL`, lint.rs:526-604) and the twelve verbs in `VERBS` order that the edited listing test walks (`lint` after `screenshot`, `help` last)
unit-test-strategy#3.1 · yes · the chunk re-runs the doc leg's whole-workspace rustdoc command, the stand-repeat leg's three rounds stopping at the first red, and the escher-driver coverage-floor cases of the coverage leg
unit-test-strategy#4.1 · yes · the scope re-reads `BlitzTestsTargetsTest` in test_blitz_tests_targets.py (no `autotests = false` in `[package]`, one `[[test]]` table `all` at tests/all.rs with `test = false`), which the chunk's edits to an existing target leave holding
unit-test-strategy#5.1 · no · the cold-agent run pipe is in the task's Not needed list and fires no agent run; the chunk changes no tool, verb or answer form the pipe reads
unit-test-strategy#6.1 · no · the capability statement's verb, cause and count rules read docs/capabilities.md and README.md, which the chunk does not reach, and VERBS, the causes and the counts stay unchanged
unit-test-strategy#7.1 · no · blitz-dom's node/ and clock unit tests (scrollbar, text-input scroll, element state, range, select) are in the task's Not needed list and no engine code moves; the lint's named readers are `text_extent` and `text_contrast`
unit-test-strategy#8.1 · no · the WPT runner's unit tests are in the task's Not needed list and the chunk touches no wpt file
unit-test-strategy#9.1 · no · absence of unit tests in slice s03; the chunk adds or removes no test, so the absence holds
unit-test-strategy#9.2 · no · absence of tests in the other seven files of slice s07; the chunk adds or removes no test, so the absence holds
unit-test-strategy#9.3 · no · absence of unit tests in blitz-vibey-script's source files (slice s10); the chunk's reached files lie outside that crate and it adds no test
unit-test-strategy#9.4 · no · absence of in-file `#[cfg(test)]` modules in tests/blitz-tests; the chunk edits a `#[test]` in an integration target, re-reads stand_lint.rs, and adds no in-file module
