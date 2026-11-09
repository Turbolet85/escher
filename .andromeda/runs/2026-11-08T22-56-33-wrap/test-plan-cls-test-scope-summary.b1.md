@answer n=1 entity=test-scope-summary#1.1 v=yes mode=stale
why: docs/capabilities.md is reached by the change, and the CapabilityStatementTest pin reads its verb and cause names and five counts, so a count can have moved
@answer n=2 entity=test-scope-summary#2.1 v=yes mode=stale
why: host_log's tests move 5 to 6, cli_commands' linux test count moves 17 to 18, and the common module gains coverage_needles
@answer n=3 entity=test-scope-summary#2.2 v=no
why: apps/browser/src/tab.rs is outside the session's reached files and the session does not touch the browser app
@answer n=4 entity=test-scope-summary#2.3 v=no
why: blitz-dom test functions are outside the reached files, the change being confined to dioxus-native-dom, escher-driver and seven_guis
@answer n=5 entity=test-scope-summary#2.4 v=no
why: blitz-dom behaviours and their blitz-tests targets are outside the reached files and no task item or fact names them
@answer n=6 entity=test-scope-summary#3.1 v=no
why: the inline box holding only block children is blitz-dom layout code and its document.rs tests, which the change does not reach
@answer n=7 entity=test-scope-summary#4.1 v=no
why: the upstream blitz-dom and blitz-paint commits with no test of their own are outside the reached files
@answer n=8 entity=test-scope-summary#5.1 v=no
why: the non-invertible transform check is a blitz-dom hit-testing test, which the change does not reach
@answer n=9 entity=test-scope-summary#6.1 v=no
why: select_interaction's multiple-select and group rules are blitz-dom behaviour, which the change does not reach
@answer n=10 entity=test-scope-summary#7.1 v=no
why: the chosen-row contrast pair is a default.css reading in blitz-dom, which the change does not reach
@answer n=11 entity=test-scope-summary#8.1 v=no
why: the press-without-Ctrl limit is pinned in escher-driver's schema.rs, which the change does not reach and which adds no key modifier
@answer n=12 entity=test-scope-summary#9.1 v=no
why: the key press past a dropped block is a blitz-dom and select_popup behaviour outside the reached files
@answer n=13 entity=test-scope-summary#10.1 v=no
why: key_press_after_container_lost_its_box is a blitz-tests target the change does not touch and its claim is blitz-dom behaviour
@answer n=14 entity=test-scope-summary#11.1 v=yes mode=stale
why: its two pins pass on the bridge's buffered render in dioxus-native-dom, a crate the change reaches whole through its dispatch-path edit
@answer n=15 entity=test-scope-summary#12.1 v=yes mode=stale
why: the buffered-render limit is a dioxus-native-dom bridge reading, and the change reaches that crate whole through its dispatch-path edit
@answer n=16 entity=test-scope-summary#13.1 v=yes mode=add-to
why: the change adds the reached-handler record to dioxus-native-dom's graph readers, beside the unconfirmed-flag limit, and the cited common module is reached
@answer n=17 entity=test-scope-summary#14.1 v=no
why: the windowed select list's limits are blitz-dom select-popup readings outside the reached files
@answer n=18 entity=test-scope-summary#15.1 v=no
why: the blitz-dom layout modules and the scrolled-box bounds test are outside the reached files
@answer n=19 entity=test-scope-summary#15.2 v=no
why: pseudo_element_flow_classification is a blitz-tests target and blitz-dom layout code the change does not reach
@answer n=20 entity=test-scope-summary#15.3 v=no
why: the blitz-dom node inline tests and the range and select unit tests are outside the reached files
@answer n=21 entity=test-scope-summary#15.4 v=no
why: the :open state path is blitz-dom element and mutator code the change does not reach
@answer n=22 entity=test-scope-summary#16.1 v=no
why: node/select_list.rs is a blitz-dom module the change does not reach
@answer n=23 entity=test-scope-summary#17.1 v=no
why: open_state is a blitz-tests target over blitz-dom's :open, which the change does not reach
@answer n=24 entity=test-scope-summary#18.1 v=no
why: clock.rs and the iframe clock test are blitz-dom code the change does not reach
@answer n=25 entity=test-scope-summary#19.1 v=no
why: blitz-net's inline unit tests are outside the reached files
