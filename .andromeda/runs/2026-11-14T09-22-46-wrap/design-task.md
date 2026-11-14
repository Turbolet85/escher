@task_prose part=session
The session added two stand checks and changed no production code, manifest, CI step, committed check or dependency.
One check reads the stand's bundled typography across operating systems: the six served tasks (counter, flight-booker, timer, crud, cells, load), both layout modes, at boot and after one click, type or advance step, compared by the driver's answer digests over snapshot (every part), lint and actions; it was green on linux x86_64, windows x86_64 and macOS aarch64, and it takes no screenshot.
The other check reads the generic families on the bundled set: serif, monospace, system-ui, cursive, fantasy and an unavailable named family all lay out at the sans-serif text extent, in both layout modes.
Measured: the bundled DejaVu Sans is registered for four generics only (sans-serif, serif, monospace, system-ui); cursive and fantasy are registered for nothing yet read the same, because with one font loaded every family falls back to that font. So the claim that the font is registered for every generic family is false; the reason for the reading is the fallback.
Nothing was deferred: cursive and fantasy read the bundled font, so no family limit is stated. The fonts-unavailable screenshot refusal is kept unchanged.
@task_file n=1 path=tests/blitz-tests/tests/stand_cross_os_readings.rs
what: a stand check that holds each of the six served tasks as the session host does and compares the driver's answer digests for snapshot, lint and actions against one table read the same on linux, windows and macOS — the cross-OS reading of the stand's bundled typography
@task_file n=2 path=tests/blitz-tests/tests/stand_bundled_font_families.rs
what: a stand check that the generic font families (serif, monospace, system-ui, cursive, fantasy) and an unavailable named family lay out at the sans-serif extent on the stand's one bundled font, both layout modes
@task_item n=1 part=touches
Typography decision that unavailable font families fall back to the default font: it is the site of the families reading; it now has a guard (stand_bundled_font_families), covers cursive and fantasy too, and must give the fallback to the only loaded font as the reason, not per-family registration.
@task_item n=2 part=touches
Typography decision that no generic family lays out as sans-serif: the stand now measures every generic family, cursive and fantasy included, laying out at the sans-serif extent on the bundled set. Check whether the row's scope (system fonts versus the stand's bundled set) still holds as written.
@task_item n=3 part=touches
Typography decisions that the test engine and the tests are built with system fonts: the stand's checks run with system fonts off and one bundled font, now read across three operating systems. Check whether either row needs the stand named as an exception.
@task_item n=4 part=touches
WASM bundled font context decision and the dioxus-native configuration contract item for the font context on WASM: both describe the single-font context that is served where one font is bundled. That context registers the font for four generics only and relies on fallback for cursive and fantasy. Any wording of registration for every generic family is false.
@task_item n=5 part=touches
dioxus-native configuration contract item FontContext: the document's font context is the one every glyph run is shaped from, so canvas text is a future (0.4.0) reader of the same bundled set.
@task_item n=6 part=touches
Driver screenshot and driver screenshot refusals contract items fonts-unavailable: kept unchanged and still green. They may take the new families reading as a neighbour, but their behaviour did not move.
@task_item n=7 part=touches
escher-session tasks contract items (counter, flight-booker, timer, crud, cells, load): each served task is now held in a cross-OS reading by its answer digests, with Cells read on every snapshot part.
@task_item n=8 part=touches
Driver verbs and the virtual clock contract items click, type and advance: these are the acting steps of the cross-OS reading (press is not used). Their contract is unchanged, but they now carry a new guard over cross-OS sameness.
@task_item n=9 part=touches
escher-session commands snapshot with part, the actions line content and the UI graph or lint reading: their answers are now digested and compared across the three CI operating systems.
@task_item n=10 part=touches
Decision that a screenshot never paints with text skipped: the new checks hold no screenshot. The cross-OS reading compares answer digests, not images, so the rule is not exercised and not contradicted.
@task_item n=11 part=touches
Decision that mobile targets are built, not tested: the new reading covers desktop linux x86_64, windows x86_64 and macOS aarch64 only, so the decision is unchanged.
@task_item n=12 part=touches
Surface web-spa overview (WASM pages rendering into a canvas): canvas text is named as a 0.4.0 reader of the same bundled font set. The canvas renderer itself stays out of this release.
@task_fact n=1 slug=typography
fact: On the stand's bundled set, every generic family (serif, monospace, system-ui, cursive, fantasy) and any unavailable named family lays out in DejaVu Sans at the sans-serif extent, because DejaVu Sans is the only font loaded and every family falls back to it. The font is registered for sans-serif, serif, monospace and system-ui only. Guarded by stand_bundled_font_families.
@task_fact n=2 slug=typography
fact: The stand's bundled typography reads the same on linux x86_64, windows x86_64 and macOS aarch64 for all six served tasks, in both layout modes, at boot and after one step. It is compared by the driver's answer digests over snapshot, lint and actions, with no screenshot, and guarded by stand_cross_os_readings.
@task_fact n=3 slug=typography
fact: cursive and fantasy carry no stated limit on the bundled set: both read the sans-serif extent, so no family is deferred.
@task_item n=1 part=untouched
Color palette rows (colour scheme recascade, engine colours, select list colours, base colour, colour token absence): no colour changed.
@task_item n=2 part=untouched
Spacing, depth strategy and hit-testing rows: no layout spacing, paint order or hit-test path changed.
@task_item n=3 part=untouched
Iconography and cursor rows, and the shell provider contract items (decorations, maximize, minimize, window drag): untouched.
@task_item n=4 part=untouched
Motion rows on scroll timing, overlay scrollbars, smooth scroll and the select list's motion: no clock or animation behaviour changed. The verbs are only used as acting steps.
@task_item n=5 part=untouched
Select list rows (overlay, open gestures, list events, accessibility of a select, select list row bounds): no select behaviour changed.
@task_item n=6 part=untouched
escher-session exit statuses and exit codes, stderr errors, mcp role result shapes, stdout lines, help and coverage check file bounds: no output shape or status changed.
@task_item n=7 part=untouched
WPT runner output and result colour rows, the JS window object items and MountedData calls during a render: not reached.
@task_item n=8 part=untouched
The absence of a project type scale or font tokens still stands. The session adds no type scale, and the typography overview stays NOT YET MEASURED.
@task_item n=9 part=untouched
Font element face and size hints, the lang attribute setting the text locale, root-relative units, and document.fonts in the script runtime: no font matching or unit resolution code changed.
