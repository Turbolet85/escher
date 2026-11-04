@answer n=1 entity=test-strategy#1.1 v=no
why: the harness wrapping of blitz_dom documents is unchanged, the session only adds one more caller of Harness::from_vdom
@answer n=2 entity=test-strategy#2.1 v=no
why: the three #[ignore] host children and their idle expiry are unchanged, and the new probe adds no host child
@answer n=3 entity=test-strategy#2.2 v=no
why: the command-line checks and the sh flow scripts are unchanged, and the probe runs no CLI or script
@answer n=4 entity=test-strategy#3.1 v=no
why: the recorded-check replay of stand_replay and its committed checks are unchanged, and the new target has no stand_ prefix
@answer n=5 entity=test-strategy#4.1 v=no
why: docs/capabilities.md and the checks it names are unchanged, and the session adds no capability claim
@answer n=6 entity=test-strategy#5.1 v=no
why: the pixel and frame checks are unchanged, and the probe paints no frame and moves no screenshot test
@answer n=7 entity=test-strategy#6.1 v=no
why: select_popup_painted and its font-embolden step are unchanged
@answer n=8 entity=test-strategy#7.1 v=no
why: the font-dependent checks and their skip paths are unchanged, and the probe reads no font
@answer n=9 entity=test-strategy#8.1 v=no
why: the WPT runner and its conformance reading are unchanged, and the session touches no wpt file
@answer n=10 entity=test-strategy#9.1 v=yes mode=stale
why: the target counts 157 .rs files and 156 test targets move to 158 and 157, and dioxus_runtime_probe.rs becomes the latest target
@answer n=11 entity=test-strategy#10.1 v=no
why: the Tab sequence tables in tests/common/mod.rs are unchanged, and the probe's Tab focus lives in its own file
