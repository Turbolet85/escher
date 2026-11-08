@answer n=1 entity=integration-test-strategy#1.1 v=no
why: pseudo_element_flow_classification is a blitz-dom layout test, and the chunk changes no blitz-dom engine code (task not_needed 5)
@answer n=2 entity=integration-test-strategy#2.1 v=no
why: blitz-vibey-script script DOM tests are not reached, the browser-side script crate being outside the chunk (task not_needed 3)
@answer n=3 entity=integration-test-strategy#3.1 v=no
why: a radio's change event in the script DOM is blitz-vibey-script behaviour the chunk does not touch (task not_needed 3)
@answer n=4 entity=integration-test-strategy#4.1 v=no
why: the URLSearchParams tests cover blitz-vibey-script globals the chunk does not touch (task not_needed 3)
@answer n=5 entity=integration-test-strategy#5.1 v=yes
why: a Session::run call on a held instance returns after settle with the diff, a refused call leaves the instance unchanged and echoes none of its bytes, and a password reads MASKED_VALUE and never leaves in an answer, all of which the read's Session path must honour (task items 9 and 10)
@answer n=6 entity=integration-test-strategy#5.2 v=no
why: the session socket's start, attach, stop, directory modes, Dead and served-count edges are host bookkeeping, and the chunk's read is checked through Session rather than the socket (task item 8)
@answer n=7 entity=integration-test-strategy#5.3 v=yes
why: the log scrub the scope names, host_log, mcp_log and stand_act_spans reading 0 id, name and typed-text needles at trace, and the driver's one escher_driver line per call holding counts only, binds any new log site or verb word (task items 5 and 11)
@answer n=8 entity=integration-test-strategy#6.1 v=yes
why: the host_log and mcp_log select-list tests read no option logged at trace, an option being a node, so the no-node-in-logs rule over host_log and mcp_log that the scope names (task items 5 and 11) binds
@answer n=9 entity=integration-test-strategy#7.1 v=no
why: the hosted popup open, shot equality and row choose through cli_commands is select-list behaviour the chunk does not change, and the socket path carries no read check (task item 8)
@answer n=10 entity=integration-test-strategy#8.1 v=no
why: the multiple select's driver click, Shift span and type behaviour is select behaviour the chunk does not change, and the scope names no select check
@answer n=11 entity=integration-test-strategy#9.1 v=no
why: the list open, choose, Escape and covered refusals of stand_act_popup are popup behaviour the chunk's Session path only uses, and the entity's screen-read clause describes the open list rather than the new read's contract
@answer n=12 entity=integration-test-strategy#10.1 v=no
why: the select_popup harness event pipeline is blitz-dom and harness event behaviour the chunk does not touch, and the reacts-to-click field reads Dioxus listeners rather than the engine's select handling (task not_needed 4 and 5)
@answer n=13 entity=integration-test-strategy#11.1 v=no
why: select_popup_painted reads blitz-paint frames the chunk does not touch (task not_needed 4)
@answer n=14 entity=integration-test-strategy#12.1 v=no
why: render_blocking_stylesheet is a resource-loading check in blitz-dom and blitz-net, which the chunk does not touch (task not_needed 5 and 6)
@answer n=15 entity=integration-test-strategy#13.1 v=yes
why: the vdom walk's dynamic attributes read each element's current listeners exactly, mounted included, which is the link-2 mechanism the graph reader is built on and that dioxus_runtime_probe pins, so the reader must not read the over-reporting listener attribute (task prose line 3 and task_file n=3)
@answer n=16 entity=integration-test-strategy#14.1 v=yes
why: the held signal's subscribers reached through ReactiveContext origin_scope give links 3 and 4, which the new signal registry reads and the seven_guis stand's signals move onto, with hook-local use_signal left out (task prose line 3 and task item 5)
@answer n=17 entity=integration-test-strategy#15.1 v=no standing=S1
why: the entity states that unit tests sit in #[cfg(test)] mod tests in src across the apps, the placement convention S1 names, and the chunk adds no apps integration test
