# Classified — test-plan · test-scope-summary.b1

## Answers
test-scope-summary#1.1 · no · the CI-scripts leg re-read green at 108 tests, and BlitzTestsTargetsTest stays green on the changed all.rs with the new target, while ci.yml, ci-leg.sh and apt-install.sh are unchanged
test-scope-summary#2.1 · yes · stale · apps/browser/src/tab.rs gains a test module (F8), so the #[test] search's browser hits are six files, not the five this line states
test-scope-summary#2.2 · yes · stale · mutator.rs, a conflict surface, gains +8 lines above the cited test module, so that module now starts eight lines lower than :1543
test-scope-summary#2.3 · yes · stale · mutator.rs, a conflict surface, gains +8 lines above the cited :1543-1990 tests, so that range moves eight lines lower
test-scope-summary#3.1 · yes · add-to · F1: the new blitz-tests target pseudo_element_flow_classification (3 tests: every box-generating ::before and ::after votes in flow classification) joins this blitz-dom layout group
test-scope-summary#3.2 · yes · stale · node/element.rs, a merge surface, gains +10 lines at 488-489 and 543-550, moving the cited disabled-state tests at :1081-1215 ten lines lower, and F9 (no test for the :open state) joins this element-state group
test-scope-summary#4.1 · no · clock.rs, iframe.rs and the virtual_clock_engine check are not in the reached files, and the report names no engine time read that the session adds
test-scope-summary#5.1 · no · blitz-paint is not in the reached files, so gradient.rs, css_box.rs and the render.rs password mask stand as cited
test-scope-summary#6.1 · no · the blitz-test-harness crate is not reached, and its API, settle, screenshot and contrast readers and tests are unchanged
test-scope-summary#7.1 · no · escher-telemetry is byte-unchanged, so its 15 inline unit tests and the sink's target allowlist stand as cited
test-scope-summary#7.2 · no · escher-driver is byte-unchanged, so its unit tests, their counts and the stand_act_* files it cites stand as cited
test-scope-summary#8.1 · yes · stale · tests/dom.rs gains three #[test] functions (27 to 30) and lines at 455-488 and 859-950, so the cited count and the :23-858 range both move
test-scope-summary#9.1 · yes · stale · the dioxus-native-dom lib test count moves 81 to 82 with input_focused_on_mount_enables_ime in dioxus_document.rs (F7), which moves the cited :498-815 range, and F11 (upstream's task_polled_during_render_can_access_document, absent from the fork) joins this inventory
test-scope-summary#10.1 · no · the matching files are unchanged, since the new IME test adds to dioxus_document.rs, already a match, and dioxus-native and stylo_taffy are not reached
