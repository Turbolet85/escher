@task_prose part=session
The session measured a canvas shot's bytes on CI's three hosts (linux x86_64, windows x86_64, macOS aarch64) and found a canvas-only shot, one holding no document text, byte-equal on all three; CI run 45209188300 read every digest row green on each host.
It added one stand test target that holds a twelve-row digest table: three canvas fixtures (a solid fill, an antialiased edge, glyphs painted on a canvas from the document's font context), each read as a whole-screen shot and as an element shot, in both layout modes, each row the screenshot answer's digest, with no pixels committed.
It scoped the capability statement's shot-bytes limit: shot bytes still differ between linux x86_64 and macOS aarch64 when the shot holds document text, while a canvas-only shot is held equal on all three hosts; the statement now names the unmeasured canvas conditions (another SIMD level of the CPU renderer, a scale other than 1.0, gradients).
The founder ratified byte equality for canvas-only shots (his F1, answer A), so the no-shot rule for cross-OS byte checks gains a canvas-only exception; a shot of page text stays outside it.
No product symbol, verb, argument, carrier, field, refusal cause, session error, exit code, log field, span, dependency, crate, CI leg or workflow line moved; nothing was retired.
@task_file n=1 path=tests/blitz-tests/tests/stand_canvas_cross_os.rs
what: a new blitz-tests stand check target, one file one target: the cross-OS canvas shot digest table, three canvas custom-widget fixtures in whole and element shot forms, read in both layout modes, its fixture precondition asserted first
@task_file n=2 path=docs/capabilities.md
what: the capability statement, its measured limit on a shot's PNG bytes across OSes, now scoped to shots holding document text, with the canvas-only exception, its witness and the unmeasured canvas conditions
@task_item n=1 part=touches
The existing-scopes decision no-screenshot-in-cross-os-byte-check gains the canvas-only exception: a cross-OS byte check may hold a shot whose content is canvas alone, compared by digest, ratified by the founder's F1; a shot holding document text stays excluded, and the row's reason is restated to what was measured.
@task_item n=2 part=touches
The occupied-resources decision committed-stand-checks: its reason that a shot's PNG bytes differ by OS now holds for a shot of page text alone; the rule that a committed check holds no screenshot step stands unchanged.
@task_item n=3 part=touches
The project-intent decision capability-statement and the capability-statement sections contract: the statement's shot-bytes limit was rescoped and gained the measured canvas exception and a list of unmeasured canvas conditions, held green by the statement's schema check.
@task_item n=4 part=touches
The decision measured-limits-stated-in-the-tool: the measured shot-bytes limit an agent reads is restated with its scope and witness.
@task_item n=5 part=touches
The standard-contracts decision stand-readings-same-on-every-ci-os: a new set of stand readings, the canvas shot digests, now reads identical on linux, windows and macOS.
@task_item n=6 part=touches
The decisions headless-screenshot-cpu-renderer and screenshots-render-on-cpu: the cross-host equality holds for canvas shots on the Vello CPU renderer at its default SIMD level and scale 1.0; other SIMD levels and scales are stated unmeasured.
@task_item n=7 part=touches
The decision canvas-crosses-as-pixels: canvas content still crosses only as pixels, and those pixels are now held byte-equal across hosts for canvas-only shots; the rule itself is unmoved.
@task_item n=8 part=touches
The decisions harness-composites-over-white, harness-screenshot-white-after-paint and png-encodes-shots: the digest table reads shots composited over white and png-encoded, so their bytes are what the cross-host rows pin.
@task_item n=9 part=touches
The contracts nodes-widgets/widget-trait and nodes-widgets/widget-fonts, and the decisions custom-widget-object and custom-widget-content-box-paint: the new fixtures are custom widgets painted at the canvas content box, the glyph widget taking the document's font context through Widget::fonts; no contract item moved.
@task_item n=10 part=touches
The screenshot contracts (driver-verbs/screenshot, answer-json/screenshot, target-check/screenshot): the table reads each row from the screenshot answer's digest, whole and with an id; no item moved.
@task_item n=11 part=touches
The conventions blitz-tests-one-target-per-file, integration-test-one-behavior, scenarios-both-layout-modes, fixture-precondition-first and descriptive-test-names: the new stand target follows each; no exception added.
@task_item n=12 part=touches
The decisions stand-repeat-zero-failure-budget and stand-repeat-zero-failure: the stand-repeat leg takes the new stand target by its prefix, green three of three rounds; no leg script or budget moved.
@task_item n=13 part=touches
The decisions font-embolden-local-witness and stand-typography-dejavu-sans: the canvas glyph rows read the same digests under blitz-paint's font-embolden feature; the apple-font-embolden setting and narrowing the no-shot rule to macOS land as no rule.
@task_item n=14 part=touches
The decision layout-modes-identical: on the dev host each canvas fixture's whole and element digests read the same in both layout modes.
@task_fact n=1 slug=existing-scopes
fact: A canvas-only shot is held byte-equal across linux x86_64, windows x86_64 and macOS aarch64 by one committed digest table (three fixtures, whole and element forms, both layout modes, no pixels committed); a shot holding document text still differs between linux x86_64 and macOS aarch64 and stays out of cross-OS byte checks.
@task_item n=1 part=untouched
The session socket, its wire, the MCP role, the command line and the driver verbs, argument kinds, refusal causes, session errors and exit codes: no verb, argument, carrier or answer field moved.
@task_item n=2 part=untouched
Engine paint, layout, style, hit-testing, scrolling and the select list in blitz-dom and blitz-paint: no engine code changed.
@task_item n=3 part=untouched
Accessibility tree, snapshot, diff, UI graph, stable element ids and actionable-key rows: untouched.
@task_item n=4 part=untouched
Telemetry, spans, log lines and the no-content-in-logs rules: the test prints nothing and no span or field was added.
@task_item n=5 part=untouched
CI workflow, ci-leg legs, coverage floors, cargo-deny audit, pinned actions and the stack's dependencies: no workflow, leg script, manifest or lock line moved.
@task_item n=6 part=untouched
Names (crates, binaries, environment variables, ports, servers) and occupied listeners: none taken or freed.
@task_item n=7 part=untouched
Kept checks, recording directory, cold-agent runs, WPT runner and the browser app: unchanged.
@task_item n=8 part=untouched
The seven_guis Circle Drawer canvas entry: unclaimed and pooled; its element shot is a later join, its whole screen holds page text.
