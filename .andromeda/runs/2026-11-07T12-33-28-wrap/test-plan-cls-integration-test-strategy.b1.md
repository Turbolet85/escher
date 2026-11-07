@answer n=1 entity=integration-test-strategy#1.1 v=no
why: the session reaches no pseudo-element flow classification file; the blitz-tests target sits outside every file the change reached.
@answer n=2 entity=integration-test-strategy#2.1 v=no
why: blitz-vibey-script's dom.rs and its script-to-DOM tests are outside every file the change reached.
@answer n=3 entity=integration-test-strategy#3.1 v=no
why: the radio change event test in blitz-vibey-script dom.rs is outside every file the change reached.
@answer n=4 entity=integration-test-strategy#4.1 v=no
why: the URLSearchParams tests in blitz-vibey-script are outside every file the change reached.
@answer n=5 entity=integration-test-strategy#5.1 v=no
why: the stand session tests assert driver settle, refusal and screenshot behaviour, which the change to the dioxus-native-dom render path leaves unchanged and which no reached file names.
@answer n=6 entity=integration-test-strategy#5.2 v=no
why: the session host socket and binary path, which the change does not touch, carry no reached file.
@answer n=7 entity=integration-test-strategy#5.3 v=no
why: the session host's stdout, stderr and sink writes are unchanged, since the change adds no log line and touches no sink.
@answer n=8 entity=integration-test-strategy#6.1 v=no
why: the host_log select-list tests are outside every file the change reached.
@answer n=9 entity=integration-test-strategy#7.1 v=no
why: the cli_commands select-popup test is outside every file the change reached.
@answer n=10 entity=integration-test-strategy#8.1 v=no
why: stand_act_select's multiple-select test is outside every file the change reached.
@answer n=11 entity=integration-test-strategy#9.1 v=no
why: stand_act_popup is outside every file the change reached, and the change does not move its select-list behaviour.
@answer n=12 entity=integration-test-strategy#10.1 v=no
why: select_popup.rs is not a reached file, and the harness input helpers and their callers are unmoved.
@answer n=13 entity=integration-test-strategy#11.1 v=no
why: select_popup_painted.rs is not a reached file, and the painted select list is untouched by the change.
@answer n=14 entity=integration-test-strategy#12.1 v=no
why: render_blocking_stylesheet.rs is not a reached file, and the change does not touch stylesheet resource loading.
@answer n=15 entity=integration-test-strategy#13.1 v=yes mode=stale
why: dioxus_runtime_probe.rs gained lines at 12-17, moving the probe lines 369-384 and 386-428 this record cites, and the listener pin itself still holds.
@answer n=16 entity=integration-test-strategy#14.1 v=yes mode=stale
why: the same probe insertion at lines 12-17 moves the probe lines 462-469 and 471-512 this record cites, while the held-signal readers claim still holds.
@answer n=17 entity=integration-test-strategy#15.1 v=no
why: the session's one new test sits inside dioxus_document.rs's cfg(test) module in src, and no file of the 86-file apps slice changed, so every hit still sits in a cfg(test) mod in src.
