# Classified — tests · mocking-stubbing-discipline.b1

## Answers
mocking-stubbing-discipline#1.1 · yes · mutator.rs:1769-1778 is a conflict surface, so its test-local ShellProvider fake counting redraw requests binds
mocking-stubbing-discipline#1.2 · no · interaction_state_teardown.rs is not reached and the scope names no IME or shell behaviour
mocking-stubbing-discipline#1.3 · yes · select_popup is a named select-list contract check; the restored key press is a list move that this CountingShell counts as a redraw, so its count binds
mocking-stubbing-discipline#1.4 · no · blitz-shell event.rs is not reached and the scope does not name winit beta.4 changing the proxy wake this test reads
mocking-stubbing-discipline#1.5 · no · link_rel_attribute.rs is not reached and its URL-recording net provider is outside the scope
mocking-stubbing-discipline#1.6 · yes · render_blocking_stylesheet.rs is upstream's test file that the take carries under the fork's blitz-tests rule, and its file-private ManualNetProvider must hold under that run
mocking-stubbing-discipline#1.7 · no · fixtures/mod.rs is not reached and the scope does not name the load counting HeldNet carries; the stand checks use it only through their own code
mocking-stubbing-discipline#1.8 · no · stand_load_wait.rs is not reached and LateNet's delayed answers are outside the named contracts
mocking-stubbing-discipline#1.9 · no · seven_guis stand.rs is not reached; LoadNet is a load-page double outside the checks the scope names
mocking-stubbing-discipline#1.10 · no · blitz-test-harness settle.rs is not reached; the scope re-reads only contrast.rs in that crate
mocking-stubbing-discipline#1.11 · no · the WPT runner is upstream-only and out of scope under Not needed
mocking-stubbing-discipline#1.12 · yes · dom.rs runs on the result as a named script-surface suite, and its MapFetcher answers the script fetch path that the bootstrap.rs conflict surface wires, so its NotFound behaviour binds
mocking-stubbing-discipline#1.13 · no · WPT navigation is upstream-only and out of scope under Not needed
mocking-stubbing-discipline#1.14 · no · the blitz-traits no-op providers are not reached and no scope line names their behaviour
mocking-stubbing-discipline#1.15 · no · harness.rs is not reached; the harness's dispatch contracts are driven only through standing checks and dispatch_recorded is not named
mocking-stubbing-discipline#1.16 · no · the rule lives in scrollbar_drag.rs, which is not reached and is not a conflict surface, and the scope names no scrollbar drag check
mocking-stubbing-discipline#1.17 · no · custom_widget_layout.rs is not reached and the Widget stand-ins are not named in the scope
mocking-stubbing-discipline#1.18 · no · paint_tree_bench.rs is an ignored benchmark, under Not needed
mocking-stubbing-discipline#1.19 · no · the WPT runner's virtual timers are upstream-only and out of scope under Not needed
mocking-stubbing-discipline#1.20 · yes · the scope names the clock invariant (stand_virtual_clock); this entity states that advance moves the session's VirtualClock by exactly ms, which the invariant rules over, and the timer step that drives it
mocking-stubbing-discipline#1.21 · no · debug_timer is not reached and no scope line names its zero-cost dummy timer
mocking-stubbing-discipline#1.22 · no · the WPT JS harness is upstream-only and out of scope under Not needed
mocking-stubbing-discipline#1.23 · no · the browser url_suggestions worker is outside the chunk's reach; the scope changes nothing there
mocking-stubbing-discipline#1.24 · no · the browser url_suggestions notify closure is outside the chunk's reach; the scope changes nothing there
mocking-stubbing-discipline#1.25 · no · .github/scripts/wpt_diff_to_pr.py is not reached and the GitHub dry-run is not in the scope
