@task_prose part=chunk
Measures what a headless shot of a canvas (a custom widget painted from Rust through the Widget trait's paint) reads on each CI host — linux x86_64, macOS aarch64 and windows x86_64 — in both layout modes, as the PNG's bytes and as the decoded RGBA raster, so an encoder difference is told apart from a rasterizer difference.
Puts the measured readings to the founder as one fork: canvas-shot byte equality across hosts required, amending the no-screenshot-in-cross-os-byte-check rule, or held within a tolerance he sets; the chunk stops there until his word.
After his word, adds a stand check that holds a canvas shot on every CI OS to that word (equal to one committed table, or within the tolerance), read where the CI matrix's cargo test of all tests and the linux test leg already run every blitz-tests target.
Re-measures the no-shot rule's stated basis (a shot's PNG bytes differ between linux x86_64 and macOS aarch64) at HEAD for a canvas shot; the canvas is a stand fixture booted alone, not Circle Drawer.
@task_item n=1 part=needs
The headless screenshot path the measurement reads: headless screenshots render on the Vello CPU renderer, composited over opaque white after paint, and the harness's screenshot and screenshot_rect readers are what the probe and the later stand check call.
@task_item n=2 part=needs
The shot encoding the measurement splits from the raster: png encodes a shot (base64 by hand) — the probe compares the encoded PNG bytes and the decoded RGBA raster separately to tell the encoder from the rasterizer.
@task_item n=3 part=needs
Canvas content crosses only as pixels: the custom widget is painted at its content box through the Widget trait (paint, fonts) with the harness turning on blitz-paint's custom-widget feature; the chunk reads those pixels and adds no other crossing for the canvas.
@task_item n=4 part=needs
The no-screenshot-in-cross-os-byte-check rule is the rule the founder's fork meets: the chunk measures against its stated basis and writes the fork (required, which amends it, or toleranced) without amending it by its own hand.
@task_item n=5 part=needs
Stand readings identical on every CI OS: the new stand check holds the canvas shot on every CI OS to the founder's word — byte-equal to one committed table if he requires equality, or within his tolerance otherwise.
@task_item n=6 part=needs
Both layout modes give identical layouts: the measurement and the later check read the canvas shot in both layout modes.
@task_item n=7 part=needs
The canvas fixture is a stand fixture booted alone, as the existing canvas shot, placement and text stand checks boot theirs: the stand boots offline, pins its environment and its typography (DejaVu Sans, the bundled font the stand shares).
@task_item n=8 part=needs
The measurement reaches macOS and Windows only through CI: CI runs on a push to main, v0.* or build/** or a pull request; the matrix jobs Test (macos) and Test (windows) run cargo test of all tests after the fast legs, and per-ref CI concurrency cancels an in-progress run of the same ref, so a probe push to the build branch while a run is open cancels it — the cost is about 41 minutes wall-clock a run, no billed minutes on the public repository.
@task_item n=9 part=needs
CI fast/slow split and Linux tested by the fast leg: the kept check is read by the existing matrix test job and the linux test leg that already run every blitz-tests target; it adds no CI job.
@task_item n=10 part=needs
No new driver verb, argument, carrier, result field, refusal cause, session error, exit, log field or span: a shot's pixels cross only as they already do, in band.
@task_item n=11 part=needs
The committed stand-Counter checks stay free of a screenshot step unless the founder's word amends that rule too; the kept check keeps no pixels.
@task_item n=12 part=needs
Raw run output is never printed into evidence: no raw log carrying a temp or home path enters the evidence the measurement writes; readings are stated as measured, with their reach.
@task_item n=13 part=needs
Each new control the chunk adds is proved with its red reading: the stand check is shown failing on a shot that departs from the committed table or tolerance before it is shown passing.
@task_standing n=1
convention: Integration tests live in blitz-tests, one test target per file, one behavior per integration test file.
home: conventions
@task_standing n=2
convention: Descriptive test names; a test asserts its fixture's condition first.
home: conventions
@task_standing n=3
convention: Stand-check helpers live in non-target modules and shared blitz-tests fixtures are stated once.
home: conventions
@task_standing n=4
convention: Scenarios run in both layout modes.
home: conventions
@task_standing n=5
convention: Module, file and public item docs; expect and unwrap need a justification; constants carry their rationale.
home: conventions
@task_item n=1 part=not_needed
Select list, select readers, form controls and form submission rows: the canvas fixture holds no select or form control.
@task_item n=2 part=not_needed
Accessibility tree, AccessKit adapter and ARIA state rows: the chunk reads pixels, not the accessibility tree.
@task_item n=3 part=not_needed
Browser app, browser history, persistence, about pages and toolbar rows: the chunk touches no browser chrome.
@task_item n=4 part=not_needed
WPT runner, reftest, WPT diff and WPT CI artifact rows: the measurement is a stand reading, not a WPT run.
@task_item n=5 part=not_needed
Script engine, Boa, script globals and script fetcher rows, and network fetching, HTTP cache and net provider rows: the stand boots offline with no page script.
@task_item n=6 part=not_needed
Cold-agent run, agent-run script, cold-agent verdict and model provider rows: the chunk runs no agent.
@task_item n=7 part=not_needed
Upstream sync, merge-base and cherry-pick rows: the chunk takes nothing from another tree.
@task_item n=8 part=not_needed
Circle Drawer on a canvas and its seven_guis screen: the next entry builds it; this chunk carries only the cross-host leg.
@task_item n=9 part=not_needed
Scroll, hit testing, event dispatch, keyboard activation and pointer default-action rows: the canvas is shot, not acted on.
