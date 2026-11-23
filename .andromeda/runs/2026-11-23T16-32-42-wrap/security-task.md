@task_prose part=session
The session added one test target to the blitz-tests harness that paints three canvas-only fixtures (a solid fill, an antialiased edge, glyphs drawn with the document's font context) through custom widgets and reads each one's screenshot answer digest, whole and by element, in both layout modes, against one committed table of twelve digest rows.
It measured those twelve rows byte-equal on CI's linux x86_64, windows x86_64 and macOS aarch64 hosts (CI run green 17/17); no PNG is committed, only digests.
It scoped the capability statement's shot-bytes limit: a shot holding document text still differs between linux x86_64 and macOS aarch64, while a canvas-only shot is held equal on the three hosts, and the statement now lists the unmeasured canvas conditions (another SIMD level, a scale other than 1.0, gradients).
The founder ratified byte-for-byte equality for canvas-only shots; the no-shot cross-OS rules gain a canvas-only exception in test-plan and architecture, not here.
No product symbol, driver verb, argument, carrier, field, refusal cause, session error, exit code, log field, span, env var, port, socket, dependency, CI script or leg changed; nothing retired.
@task_file n=1 path=tests/blitz-tests/tests/stand_canvas_cross_os.rs
what: added (new file) — a committed stand check of the test harness, its custom widgets painting canvas content with the document's font context, reading the driver session's screenshot answer digest against a table of twelve rows
@task_file n=2 path=docs/capabilities.md
what: edited — the committed capability statement, its limit on a shot's PNG bytes across OSes and its list of unmeasured conditions
@task_item n=1 part=touches
Canvas content crosses only as shot pixels: the new check reads canvas content solely through the screenshot answer's digest; confirm the row stands unmoved (the canvas still crosses only as pixels).
@task_item n=2 part=touches
The committed statement of the MCP crossing: the capability statement was edited (the shot-bytes limit scoped to shots with document text, the canvas-only exception, unmeasured canvas conditions); any row holding the statement's shot limit text.
@task_item n=3 part=touches
Screenshot answer png contract and the shot encoder rows (png encodes a screenshot, png as the shot encoder): the shot's bytes are now stated byte-equal across three hosts for canvas-only content; a row whose why says shot bytes differ by OS.
@task_item n=4 part=touches
Screenshot carried in band and screen pixels in band: the new check consumes the in-band screenshot answer's digest; no new carrier.
@task_item n=5 part=touches
Committed stand-Counter checks and any row stating a committed check holds no screenshot step or that a shot repeats on one host only: the rule stands, but its reason now holds for page-text shots alone.
@task_item n=6 part=touches
Custom widget handed the document's font context: a test widget's fonts hook keeps the document's font context to draw canvas glyphs.
@task_item n=7 part=touches
Custom widget painted and pointed in its content box: the fixtures place a canvas at (8, 8, 200, 100) and assert it paints alone with no named node.
@task_item n=8 part=touches
Harness turns on blitz-paint's custom-widget feature: the new harness target relies on it; no manifest line moved.
@task_item n=9 part=touches
font-embolden tested by a locked fast leg: the digest table is also read green under blitz-paint/font-embolden.
@task_item n=10 part=touches
Lint findings hold no page content / lint findings crossing: the fixtures assert no lint finding through the session's lint call.
@task_item n=11 part=touches
CI matrix runs every job: the cross-host reading rests on the linux, windows and macOS test jobs of one run; no job or leg changed.
@task_item n=12 part=touches
blitz-tests not published and the test harness taking no tracing and no style: the new target lives in that unpublished harness, adds no dependency and prints nothing.
@task_item n=13 part=touches
Locked builds: every gate of the session ran with --locked; no Cargo.toml or Cargo.lock line moved.
@task_item n=14 part=untouched
Authentication and authorization, CI tokens and publish secrets: no credential, token or publish job was reached.
@task_item n=15 part=untouched
Secret management and environment variables: no env var read or added.
@task_item n=16 part=untouched
Outbound network, fetch, CORS, same-origin, navigation and request limits: no network path was reached.
@task_item n=17 part=untouched
Driver verbs, argument kinds, refusal causes, session errors, exit codes and the session socket or MCP wire grammar: none moved.
@task_item n=18 part=untouched
Select lists, form controls, masking of password and file values, stable element id and snapshot text: no such behaviour was reached.
@task_item n=19 part=untouched
Logging, telemetry sink allowlists and spans: no log field or span added.
@task_item n=20 part=untouched
Data protection rows on history, cookies, HTTP cache, TLS and kept check files: no stored data or kept check moved.
@task_item n=21 part=untouched
Dependency audit, advisory ignores, licence gate and dependency pins: no dependency added or bumped.
