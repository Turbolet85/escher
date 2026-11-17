# design · standing conventions

- S1 · each new test and fixture check builds the engine with system fonts, and a screenshot it takes never paints with text skipped · typography
  contract/driver-screenshot-refusals/fonts-unavailable · driver screenshot refusals · fonts-unavailable — A screenshot is refused fonts-unavailable, with nothing painted, when the app's font context resolves no sans-serif family.
  contract/driver-screenshot/fonts-unavailable · driver screenshot · fonts-unavailable — A `screenshot` of an app whose font context resolves no `sans-serif` family is refused `fonts-unavailable` with nothing painted; a shot never paints with text skipped.
  decision/screenshot-never-skips-text · Screenshot never paints with text skipped — A driver screenshot of an app whose font context resolves no sans-serif family is refused with nothing painted, so a shot never paints text skipped.
  decision/test-engine-with-system-fonts · Test engine built with system fonts — The blitz-dom engine is built with the system-fonts feature for the blitz-tests crate. · why: Without system fonts text measures 0x0 and font-dependent assertions pass vacuously.
  decision/tests-built-with-system-fonts · Tests built with system fonts — The blitz-tests crate builds `blitz-dom` with the `system-fonts` feature, which is also enabled by default when testing the whole workspace. · why: Without system fonts text measures 0x0 and font-dependent assertions pass vacuously.
- S2 · each new stand check moves time only through the driver's one virtual clock (advance, and the clock moves of click, press and type), never wall-clock waits · motion-calibrated-to-expression-level-unrecorded · motion
  contract/driver-verbs-and-virtual-clock/advance · driver verbs and the virtual clock · advance — The driver's advance moves the VirtualClock by exactly its ms and, through the step a session's caller hands it, the app's own time together.
  contract/driver-verbs-and-virtual-clock/click · driver verbs and the virtual clock · click — The driver's click leaves the VirtualClock where it was.
  contract/driver-verbs-and-virtual-clock/press · driver verbs and the virtual clock · press — The driver's press leaves the VirtualClock where it was.
  contract/driver-verbs-and-virtual-clock/type · driver verbs and the virtual clock · type — The driver's type leaves the VirtualClock where it was.
  decision/animation-driven-by-virtual-clock · Animation driven by the virtual clock — The test harness drives animation from the VirtualClock its documents read, and settle reads animation time only through that clock, never advancing it.
  decision/driver-advance-moves-one-clock · Driver advance moves the one virtual clock — The driver's advance moves the harness virtual clock by exactly its ms and the app's own time together, while click, type and press leave the clock where it was. · why: Ratified.
  decision/settle-does-not-wait-on-animation · Settle does not wait on animation — A harness settle answers every member of the document's animating set the same way: none holds it open, none is waited on or advanced; settle returns Settled with animating reading the document's animating flag.
- S3 · each new check reads escher-session's answers as an agent does, its exit status from the schema's table and its stdout lines in their stated form, with no colour or terminal test · surface · surface-cli
  contract/escher-session-exit-codes/1 · escher-session exit codes · 1 — The command was refused.
  contract/escher-session-exit-codes/2 · escher-session exit codes · 2 — A usage error.
  contract/escher-session-exit-codes/3 · escher-session exit codes · 3 — A session error; an mcp role ending in Err also exits 3.
  contract/escher-session-exit-status/1 · escher-session exit status · 1 — Refused.
  contract/escher-session-exit-status/2 · escher-session exit status · 2 — Usage.
  contract/escher-session-exit-status/3 · escher-session exit status · 3 — Session error, the recording's record-dir-not-private and recording-failed among them, with no exit of their own.
  contract/escher-session-mcp/stdout · escher-session mcp role · stdout — stdout holds newline-delimited JSON-RPC 2.0 protocol lines only, a reply per request that is not a notification, every reply one uncoloured line with no ESC byte; its end writes nothing there.
  contract/escher-session-stdout/command-line-line · escher-session stdout · command line output — One JSON line per command: the answer, the refusal or a session error's object.
  contract/escher-session-stdout/mcp-line · escher-session stdout · mcp role output — Newline-delimited JSON-RPC 2.0 lines.
  decision/driver-output-for-agents · Driver command line output read by agents — The escher-session command line and its mcp role write only JSON to stdout, never colour or an ESC byte, and no code tests for a terminal, so the output is the same piped or not. · why: These surfaces are read by an agent, not a person.
  decision/escher-session-agent-read-surface · escher-session is read by an agent, not a person — An escher-session command prints one line of JSON on stdout, the answer, the refusal or a session error's object keyed by the driver schema's own words, and nothing else there.
  decision/exit-statuses-from-schema-table · Exit statuses read from the schema's table — The exit statuses are read from the driver schema's EXIT_STATUSES table, whose meanings help serves.
