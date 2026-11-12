@row_add n=1 row=decision/ci-scripts-unit-tested area=1-test-scope-summary rec=1,2
name: CI Python scripts unit-tested
rule: The CI Python scripts are unit-tested by unittest discovery over the CI scripts directory, run as the fast ci-scripts leg, contract tests of the CI install, coverage floor and blitz-tests manifest scripts among them.
guard: ci-scripts leg
@row_add n=2 row=decision/ci-package-installs-bounded area=1-test-scope-summary rec=1,3
name: CI package installs bounded
rule: Every package install of the CI workflow is bounded, a workflow invariant held by the CI scripts' unit tests.
guard: test_ci_workflows.py
@row_add n=3 row=decision/capability-statement-pinned area=1-test-scope-summary rec=1,4
name: Capability statement pinned to the driver schema
rule: The capability statement names every driver verb and refusal cause as a code span and states the schema's verb, cause, session-error, node-key and exit-status counts in words; the README states the count of driver commands an MCP client gets as tools.
guard: test_capability_statement.py
@row_add n=4 row=decision/expiry-wait-watches-state-dir area=2-test-strategy rec=5,6
name: Expiry waits watch the state directory
rule: A check that waits for a host's expiry watches the state directory rather than asking the host.
why: An attach, and so the status command, is an answered request and starts the expiry count again.
@row_add n=5 row=decision/recorded-checks-replayed-as-tests area=2-test-strategy rec=7,8
name: Recorded checks replayed as tests
rule: A check a session recorded is committed as data and run as an ordinary cargo test target, every step re-validated through the session and red at the first step whose acceptance or kept answer differs.
@row_add n=6 row=decision/recorded-checks-kept-fresh area=2-test-strategy rec=7,8
name: Recorded checks kept fresh
rule: A committed recorded check is kept fresh by a byte-for-byte comparison with what the binary records.
guard: seven_guis recording
@row_add n=7 row=contract/driver-refusals/fonts-unavailable area=2-test-strategy rec=9,10
boundary: Driver refusals
item: fonts-unavailable
promise: A shot of a document whose font context resolves no sans-serif family is refused with nothing painted and no skip path, and the actions read names the same fonts-unavailable for screenshot.
@row_add n=8 row=contract/stand-boot/boot-timer area=3-test-harness-contract rec=11
boundary: seven_guis headless stand
item: boot_timer
promise: boot_timer(HarnessOptions) boots the Timer as a fresh VirtualDom over from_vdom rendering the task in TaskShell, and returns the harness with its TimerTicks.
@row_add n=9 row=contract/harness-screenshot/screenshot-rect area=3-test-harness-contract rec=13
boundary: Harness screenshot
item: screenshot_rect
promise: screenshot_rect(PixelRect) paints like screenshot at the harness's own time, the rectangle clipped to the viewport; an empty rectangle is EmptyRect.
@row_add n=10 row=decision/one-contrast-formula area=3-test-harness-contract rec=14,15
name: One contrast formula
rule: Session lint's contrast reads the same as the harness's text_contrast, with no second formula.
