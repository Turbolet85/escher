@task_prose part=session
The session added two test-only stand checks to blitz-tests and changed no production file, manifest, dependency, workflow, CI step or committed check file.
The first reads the driver's answer digests for the six served tasks (counter, flight-booker, timer, crud, cells, load) in both layout modes, at boot and after one step by click, type or advance, over snapshot (every part), lint and actions, against a committed 48-row table of digests; CI read the same table on linux x86_64, windows x86_64 and macOS aarch64.
Before it only the two committed Counter checks were read cross-OS; the new check holds no shot and prints nothing.
The second reads the text extent of seven font families (sans-serif, serif, monospace, system-ui, an unavailable named family, cursive, fantasy) on the stand's one bundled font, DejaVu Sans, with system fonts off: all read the sans-serif extent, by fallback to the only loaded font, not by registration.
The fonts-unavailable refusal is kept unchanged; nothing was deferred or retired.
@task_file n=1 path=tests/blitz-tests/tests/stand_cross_os_readings.rs
what: a stand check of the test harness (blitz-tests, not published) holding the cross-OS reading of the six served tasks as driver answer digests (snapshot, lint, actions) in a committed table, acting by click, type and advance, no shot, no output
@task_file n=2 path=tests/blitz-tests/tests/stand_bundled_font_families.rs
what: a stand check of the test harness (blitz-tests, not published) reading the text extent reader over a fixture page for each generic font family on the stand's bundled font, system fonts off
@task_item n=1 part=touches
decision/committed-counter-checks and decision/kept-check-actions-answer: the cross-OS reading, before held by the committed Counter checks alone, now also covers all six served tasks through a stand check holding answer digests, not a kept check file; the rows stay true but a picker may set the new reach beside them.
@task_item n=2 part=touches
Kept check rules (decision/kept-check-owner-only, decision/kept-checks-owner-only, contract/kept-check-file/contents, contract/kept-check-file/medium): the new committed digest table is no kept check file and holds 16-hex digests only, no answer text, screen content or shot.
@task_item n=3 part=touches
decision/text-extent-read-only-none-when-unresolved: the text extent reader gains a test caller asserting a non-zero extent for every family on the bundled font.
@task_item n=4 part=touches
contract/refusal-causes/fonts-unavailable: kept unchanged and still green; the stand's bundled typography with system fonts off is now read on every CI OS.
@task_item n=5 part=touches
decision/no-agent-urls-through-font-path and decision/hintless-font-face-source-sniffed: the stand's fonts come from the one bundled DejaVu Sans with system fonts off; no font source is fetched by the new checks.
@task_item n=6 part=touches
Screenshot and shot rows (decision/screenshot-in-band, decision/screen-pixels-in-band, contract/screenshot-answer/png, contract/mcp-tool-result/screenshot): the cross-OS check compares by answer digests and holds no shot (census 0).
@task_item n=7 part=touches
Log and output rows (decision/no-screen-content-in-logs, decision/driver-outcomes-never-logged, decision/driver-installs-no-subscriber): both new checks print nothing; a scratch print used to read the digests was never committed.
@task_item n=8 part=touches
Acting verbs and time (decision/time-moves-only-on-advance, decision/advance-moves-virtual-clock-exactly, decision/replay-through-session-run): the cross-OS check steps by click, type and advance through the session's run path and reads the same digests on three OSes.
@task_item n=9 part=touches
CI rows (decision/ci-matrix-runs-every-job, decision/free-disk-only-on-stand-repeat, decision/locked-builds): the test leg, the windows and macOS legs and the repeated stand job take both new stand_ targets by the workspace build and the stand_ glob, run locked, with no workflow change; CI 17 of 17 green.
@task_item n=10 part=touches
decision/blitz-tests-not-published and decision/test-harness-no-tracing-no-style: two new auto-discovered integration targets in the unpublished test harness, no manifest edit and no new dependency.
@task_fact n=1 slug=data-protection
fact: The cross-OS reading of the six served tasks is committed as driver answer digests only, in a stand test's table read on linux, windows and macOS, never as a kept check file or a shot.
@task_item n=1 part=untouched
Authentication and authorization, CI tokens, publish guards and secret management: no secret, token, environment variable or workflow changed.
@task_item n=2 part=untouched
API security and outbound requests: no listener, socket bound, MCP transport, fetch scheme, request limit or CORS and same-origin absence changed.
@task_item n=3 part=untouched
Driver session socket and MCP wire contracts, refusal cause set, session errors and exit codes: no reply, request line, bound, cause or code changed.
@task_item n=4 part=untouched
Stable element id, snapshot text state words, masking of control values and the actions and graph answer contracts: read by digest, none of their shape or rule changed.
@task_item n=5 part=untouched
Dependency security: no dependency, pin, advisory ignore, licence gate or audit tool changed.
@task_item n=6 part=untouched
Select list, range input, accessibility, recording, cold-agent, WPT and browser storage rows: the session reaches none of them.
@task_item n=7 part=untouched
Rows holding the token cursive inside recursive (state-directory non-recursive delete): false hits, nothing about fonts.
