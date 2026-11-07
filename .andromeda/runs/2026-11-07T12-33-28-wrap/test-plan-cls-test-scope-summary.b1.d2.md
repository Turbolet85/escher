@answer n=1 entity=test-scope-summary#1.1 v=no
why: the CI Python script counts and the apt-install and workflow pins are outside the session's dioxus-native-dom and probe changes
@answer n=2 entity=test-scope-summary#2.1 v=no
why: the apps and examples named hold no tests, and the session adds its one test to packages/dioxus-native-dom, not to an app or example
@answer n=3 entity=test-scope-summary#2.2 v=no
why: the browser tab test sits in apps/browser, which the session does not touch
@answer n=4 entity=test-scope-summary#2.3 v=no
why: the 83 #[test] functions counted here are blitz-dom's own, and the session's 83 is dioxus-native-dom's unit count in a different crate
@answer n=5 entity=test-scope-summary#2.4 v=no
why: the blitz-dom covered behaviours are checks and unit tests on engine files the session does not reach
@answer n=6 entity=test-scope-summary#3.1 v=no
why: the inline-box-with-block-children reader is a blitz-dom layout rule in files the session does not reach
@answer n=7 entity=test-scope-summary#4.1 v=no
why: the upstream commits named here are blitz-dom and blitz-paint commits, and the session ports 286874cc1 into dioxus-native-dom instead
@answer n=8 entity=test-scope-summary#5.1 v=no
why: the non-invertible transform check is a blitz-dom hit-testing check that the session leaves alone
@answer n=9 entity=test-scope-summary#6.1 v=no
why: the multiple-select and group checks are blitz-dom select rules that the session does not touch
@answer n=10 entity=test-scope-summary#7.1 v=no
why: the chosen-row contrast reading concerns default.css and select_interaction, neither reached by the session
@answer n=11 entity=test-scope-summary#8.1 v=no
why: the select driver limits sit in escher-driver's schema, which the session does not reach
@answer n=12 entity=test-scope-summary#9.1 v=no
why: the key press past a dropped anonymous block is a blitz-dom document behaviour the session leaves alone
@answer n=13 entity=test-scope-summary#10.1 v=no
why: key_press_after_container_lost_its_box is a blitz-dom blitz-tests target that the session does not reach
@answer n=14 entity=test-scope-summary#11.1 v=yes mode=contradict
why: the probe's two MountedData should_panic pins are renamed and inverted to the spawned_acts pins over spawned_focus_acts, so this row's hazard pins read false, and F1 (the limit on nodes mounted by an earlier render) is carried here as well
@answer n=15 entity=test-scope-summary#12.1 v=no
why: the windowed select list's stated limits are blitz-dom and residual rows that the session does not move
@answer n=16 entity=test-scope-summary#13.1 v=no
why: the blitz-dom layout test modules and the scrolled-box bounds check are in files the session does not reach
@answer n=17 entity=test-scope-summary#13.2 v=no
why: the pseudo-element flow classification targets are blitz-dom layout checks that the session does not touch
@answer n=18 entity=test-scope-summary#13.3 v=no
why: the node-level in-file tests in blitz-dom's scrollbar, element, range and select modules are outside the session's reach
@answer n=19 entity=test-scope-summary#13.4 v=no
why: the :open flush and its tests are blitz-dom element and mutator code that the session leaves alone
@answer n=20 entity=test-scope-summary#14.1 v=no
why: node/select_list.rs is a blitz-dom file that the session does not reach
@answer n=21 entity=test-scope-summary#15.1 v=no
why: the open_state check is a blitz-dom blitz-tests target that the session does not reach
@answer n=22 entity=test-scope-summary#16.1 v=no
why: the clock.rs unit tests and the iframe clock test are blitz-dom code that the session does not reach
@answer n=23 entity=test-scope-summary#17.1 v=no
why: the blitz-net inline unit tests sit in a crate the session does not touch
@answer n=24 entity=test-scope-summary#18.1 v=no
why: the blitz-paint inline tests and the password mask and caret checks are in blitz-paint and blitz-tests files the session does not reach
@answer n=25 entity=test-scope-summary#19.1 v=no
why: the blitz-paint blending reading concerns mix-blend-mode and isolation, which the session does not touch
