@row_add n=1 row=decision/stand-checks-in-both-layout-modes area=test-scope-summary rec=1
name: Stand checks run in both layout modes
rule: A stand check that reads a screen asserts its readings in both layout modes, and the two modes' readings are held equal where the check compares them.
@row_add n=2 row=decision/typed-password-never-leaves area=test-scope-summary rec=1
name: A typed password never leaves the instance
rule: A typed password reads only as the fixed mask and occurs in no snapshot, snapshot text, diff, accessibility-tree field, recording, span field, MCP result or screenshot pixels.
guard: stand_snapshot_state, stand_act_filled, stand_mcp_masked, stand_session_recording, password_painted_masked
@row_add n=3 row=decision/no-screenshot-step-in-committed-checks area=test-scope-summary rec=1
name: No screenshot step in a committed check
rule: A committed recorded check holds no screenshot step; screenshot repeatability is asserted on one host only.
why: a shot's PNG bytes differ between linux x86_64 and macOS aarch64, so a shot is byte-identical on one host only
guard: stand_screenshot_repeat
@row_add n=4 row=decision/advance-moves-exactly-the-ms-asked area=test-scope-summary rec=1
name: Advance moves exactly the time asked
rule: An advance reports exactly the ms asked whatever the step does: the step is handed exactly ms, advanced_ms equals ms, and the clock moves by exactly ms.
why: ratified
guard: stand_act_timer
@row_add n=5 row=decision/mcp-causes-served-with-remedy area=test-scope-summary rec=1
name: Refusal causes served with their remedy over MCP
rule: Every refusal cause a tool call reaches is served through MCP with its remedy.
why: ratified
guard: stand_mcp_tools
except: unknown-verb is not served through MCP
@row_add n=6 row=decision/one-virtual-clock area=test-scope-summary rec=1
name: One virtual clock drives engine time
rule: The scrollbar fade, double-click timing and smooth scroll move only with the document's clock, which a session moves by advance alone; a document built with no clock runs by the wall clock.
why: ratified
guard: virtual_clock_engine, stand_virtual_clock
@row_add n=7 row=decision/snapshot-text-budget area=test-scope-summary rec=1
name: Snapshot text within a byte budget
rule: A screen's snapshot text is read whole when inside SNAPSHOT_TEXT_BUDGET (10,000 bytes); a larger screen is read in parts, each within the budget, the parts concatenating to the whole text.
guard: stand_snapshot_text, stand_snapshot_parts
@row_add n=8 row=decision/owner-only-session-files area=test-scope-summary rec=1
name: Owner-only session socket and recordings
rule: A session's state directory is 0700 and its socket 0600, and a recording directory open to its group is refused with nothing written.
guard: stand_session_lifecycle, stand_session_recording
@row_add n=9 row=decision/telemetry-target-allowlist area=test-scope-summary rec=1
name: Telemetry sink keeps stdout silent behind an allowlist
rule: Stdout stays silent, and a record from a target outside the sink's allowlist writes nothing at any level even under a RUST_LOG directive naming it; an escher record prints and an engine record prints redacted.
guard: telemetry_stdout_silent, telemetry_drop
@row_add n=10 row=contract/driver-refusal-causes/not-found area=test-scope-summary rec=1
boundary: driver refusal causes
item: not-found
promise: An id no screen of the session read, or that no element carries, is refused not-found.
@row_add n=11 row=contract/driver-refusal-causes/stale area=test-scope-summary rec=1
boundary: driver refusal causes
item: stale
promise: An id an earlier screen of the session read that names no element now is refused stale.
@row_add n=12 row=contract/driver-refusal-causes/malformed area=test-scope-summary rec=1
boundary: driver refusal causes
item: malformed
promise: An id longer than 1024 bytes is refused malformed before any lookup.
@row_add n=13 row=contract/driver-refusal-causes/fonts-unavailable area=test-scope-summary rec=1
boundary: driver refusal causes
item: fonts-unavailable
promise: A screenshot or lint on an app with no usable font is refused fonts-unavailable with nothing changed.
@row_add n=14 row=contract/driver-command-span/one-line-per-call area=test-scope-summary rec=1
boundary: driver command span
item: command line
promise: Each call run through the session leaves exactly one escher_driver line at INFO, its span reading command, carrying no field outside the stated eight and none of the ids, names or supplied texts.
