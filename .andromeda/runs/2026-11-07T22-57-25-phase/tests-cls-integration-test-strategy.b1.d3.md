@answer n=1 entity=integration-test-strategy#1.1 v=no
why: pseudo-element layout tests live in blitz-dom's layout engine, which the chunk does not change
@answer n=2 entity=integration-test-strategy#2.1 v=no
why: blitz-vibey-script's script DOM tests are outside the chunk, and no script or DOM surface is named
@answer n=3 entity=integration-test-strategy#3.1 v=no
why: the radio change event is blitz-vibey-script's script DOM, which the chunk does not touch
@answer n=4 entity=integration-test-strategy#4.1 v=no
why: URLSearchParams tests are blitz-vibey-script's, outside the chunk
@answer n=5 entity=integration-test-strategy#5.1 v=yes
why: a caller-built Call run through Session::run returns after settle with the before/after diff, a refused call leaves the instance unchanged and echoes none of its bytes, and a password reads MASKED_VALUE, which is the held-instance contract of the scope's item 9
@answer n=6 entity=integration-test-strategy#5.2 v=no
why: the socket host's start, attach, stop, served-count and directory edges are the host's own lifecycle, which the chunk does not change, and a new verb is answered by the same loop
@answer n=7 entity=integration-test-strategy#5.3 v=yes
why: it names host_log, mcp_log and stand_act_spans, which the scope's log-scrub item requires to read 0 id, name and typed-text needles at trace, with no log site carrying a node, component, handler, state or value
@answer n=8 entity=integration-test-strategy#6.1 v=yes
why: host_log is named by the log-scrub item, and its select-list test at trace asserts that no option is logged, the same no-value rule the chunk's read must honour
@answer n=9 entity=integration-test-strategy#7.1 v=no
why: cli_commands' hosted-list check is a process check the scope does not name, and the chunk changes no CLI or host path
@answer n=10 entity=integration-test-strategy#8.1 v=no
why: the multiple-select rules run over a select that is not one of the stand tasks whose signals move, and the chunk changes no select behaviour
@answer n=11 entity=integration-test-strategy#9.1 v=yes
why: stand_act_popup is a stand check driving a select's list through a held stand session, and the scope's stand item requires every stand check over those tasks to stay green in both layout modes after the signals move
@answer n=12 entity=integration-test-strategy#10.1 v=no
why: select_popup is harness input over blitz-dom's select list, which the chunk does not change
@answer n=13 entity=integration-test-strategy#11.1 v=no
why: select_popup_painted is paint pixel testing, which the chunk does not need
@answer n=14 entity=integration-test-strategy#12.1 v=no
why: render-blocking stylesheet tests cover blitz-tests DOM resource loading that the chunk does not reach
@answer n=15 entity=integration-test-strategy#13.1 v=yes
why: the probe's link-2 check, the vdom walk's dynamic attributes reading each element's listeners exactly with mounted included, is the mechanism the graph reader is built on, and the probe is named as a re-read with its link-1, link-2 and subscriber mechanisms
@answer n=16 entity=integration-test-strategy#14.1 v=yes
why: the probe's links 3 and 4 check, a held signal's subscribers read as its reader scopes, is the subscriber mechanism the new signal registry and reader rest on, and the probe is named as a re-read
@answer n=17 entity=integration-test-strategy#15.1 v=no standing=S1
why: the apps' test placement is the unit-test home convention, cfg(test) mod tests in src, which every new test owes, and the chunk adds no app integration test
