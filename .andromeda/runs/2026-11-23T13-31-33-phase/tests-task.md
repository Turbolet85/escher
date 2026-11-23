@task_prose part=chunk
Measures what a headless shot of a canvas (a widget painted from Rust) reads on each CI host — linux x86_64, macOS aarch64, windows x86_64 — in both layout modes, as the PNG's bytes and as the decoded RGBA raster, so an encoder difference is told apart from a rasterizer difference.
The measurement reaches macOS and Windows only through the CI matrix's test jobs, its cost read off CI run wall-clock.
Puts the readings to the founder as one fork: canvas-shot byte equality across hosts required (amending the no-shot cross-OS rule) or held within a tolerance he sets.
After his word, adds a stand check holding a canvas shot on every CI OS to that word (equal to one committed table, or within the tolerance), run where the matrix's workspace tests and the linux test leg already run every blitz-tests target.
Re-measures whether the no-shot rule's stated basis (shot PNG bytes differ between linux and macOS) still holds at HEAD for a canvas shot.
@task_item n=1 part=needs
The no-shot cross-OS rule the fork may amend: cross-OS byte comparisons and cross-OS byte-compared checks hold no screenshot step, and its stated basis that a shot's PNG bytes differ between hosts.
@task_item n=2 part=needs
Shot PNG determinism on one host, the premise a cross-host reading builds on.
@task_item n=3 part=needs
The headless screenshot from the harness and a widget painted from Rust showing in shots: what the canvas shot reads and the harness screenshot calls it is taken through.
@task_item n=4 part=needs
PNG decoding in checks: the shot read both as its encoded bytes and as its decoded RGBA raster.
@task_item n=5 part=needs
Pixel tests on a CPU buffer: the canvas rasterized on the CPU path whose SIMD paths may differ between x86_64 and aarch64.
@task_item n=6 part=needs
Frames compared whole, and a stated tolerance read inclusively, where the founder's word is a tolerance.
@task_item n=7 part=needs
Stand checks run in both layout modes, on the bundled font with no skip; the measurement and the kept check cover both modes.
@task_item n=8 part=needs
The native platform CI matrix: the macOS and Windows test jobs run the workspace tests after the fast legs gate them; the only way a reading reaches those hosts.
@task_item n=9 part=needs
CI trigger branches and in-progress cancellation of a run on the same ref: a probe pushed to the build branch while a run is open cancels it; a hosted-runner cancel answered by a hand re-run.
@task_item n=10 part=needs
The canvas stand fixture booted alone, as the earlier canvas stand checks boot theirs, through the seven_guis headless stand's boot-alone entry.
@task_item n=11 part=needs
The kept check keeps no pixels and adds no driver verb, argument, carrier, result field, refusal cause, session error, exit, log field or span: a shot's pixels cross only as they already do.
@task_item n=12 part=needs
Committed recorded checks stay free of a screenshot step unless the founder's word amends that rule too.
@task_item n=13 part=needs
Each new control proved red before green (red reading recorded), and expected values pinned as literals where the word is equality to one committed table.
@task_item n=14 part=needs
Zero-failure budget for stand checks, repeated three rounds: the new cross-host check must hold every round on every CI OS.
@task_item n=15 part=needs
One test target per file and one blitz-tests target per file, with shared stand tables kept in one shared module that is no target.
@task_item n=16 part=needs
Live-run evidence kept free of private paths: no raw log carrying a temp or home path reaches the evidence written for the fork.
@task_standing n=1
convention: each new test target is one file, one behaviour, run in both layout modes on the bundled font
home: test-strategy · 2-test-strategy
@task_standing n=2
convention: each new test is controlled red before green, its failure printing no argument value
home: unit-test-strategy · 4-unit-test-strategy
@task_standing n=3
convention: each new stand check holds under the zero-failure stand-repeat budget
home: quality-gates-coverage-targets · 10-quality-gates-coverage-targets
@task_standing n=4
convention: each new test's data is stated in code, offline, with a hand-written stand-in and no mocking library
home: test-data-fixtures · 7-test-data-fixtures
@task_item n=1 part=not_needed
The WPT conformance runner, its CLI, outputs, status set and pinned commit: the chunk measures a canvas shot, not web conformance.
@task_item n=2 part=not_needed
The cold-agent pipe, its events, exit codes, verdict, prompt and isolation: no cold-agent run is part of this measurement.
@task_item n=3 part=not_needed
The agent-run events, verbs, selections and exit codes: the chunk adds no agent-run verb or event.
@task_item n=4 part=not_needed
The session wire, session host lifecycle, socket and session state directory: the shot is taken in a stand check, not over a held session.
@task_item n=5 part=not_needed
The escher-session command line, MCP role, tools list and protocol rows: no command, tool or role changes.
@task_item n=6 part=not_needed
Handler coverage and its coverage line: canvas handlers stay uncounted and no coverage reading is taken.
@task_item n=7 part=not_needed
Lint defects, text contrast and the WCAG contrast formula: the chunk reads pixels of a shot, not contrast or lint findings.
@task_item n=8 part=not_needed
The virtual clock, advance, timer ticks and settle pass limits: the canvas fixture's shot reads no time.
@task_item n=9 part=not_needed
Circle Drawer's own checks and its claim of v040-09: the next entry builds them.
