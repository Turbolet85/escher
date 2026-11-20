@task_prose part=session
The session added test code alone: two unit tests of the browser's screenshot capture, which check that the scene render returns the viewport size at scale 1.0 (800 by 600) and the physical size at a HiDPI scale of 2.0 (1280 by 720).
The tests sit out of line in a sibling file with the _tests suffix, declared as a cfg(test) child module by path; no public, crate-visible or IPC surface was added or changed, and no dependency, schema or config moved.
It measured: the four surviving mutants of the scene render's returned size, from the 0.3.0 Epoch 6 mutation audit, now read caught under hand controls and under a cargo-mutants run (4 caught), fired on the founder's word for those four alone.
One mutant outside that word, the scene-capture function that returns nothing, read missed; it is carried as a residual and the next version's incubator line, with no work on it here.
After implement only the plan's mutation-run filter changed (a gate edit, no product file); nothing was retired.
@task_file n=1 path=apps/browser/src/capture.rs
what: edited — the browser's screenshot capture, which renders the document's scene and returns its size, the viewport size scaled to physical pixels; only a test module declaration was appended, no line of its behaviour changed
@task_file n=2 path=apps/browser/src/capture_tests.rs
what: new — the unit tests of the browser's screenshot capture: the scene render returns the viewport size at scale 1.0 and the physical size at a HiDPI scale
@task_item n=1 part=touches
The rule that the logical viewport is the physical size over the total scale: the new tests now pin the capture's returned size at scale 1.0 and at a HiDPI scale of 2.0, confirming the rule rather than changing it.
@task_item n=2 part=touches
The screenshot verb of escher-session and the driver's screenshot: its capture's returned size is now under unit test; no output, refusal or argument of the verb changed.
@task_item n=3 part=touches
The rule that a screenshot never paints with text skipped: it governs the same capture component, which the session tested but did not change.
@task_item n=4 part=touches
The JS window's devicePixelRatio and innerWidth / innerHeight contracts: the tests exercise the same scale-to-physical relationship from the capture side, with no change to what the window reports.
@task_item n=5 part=touches
Rows on the screenshot of an open select list's rows: they read through the same capture, which changed no behaviour.
@task_item n=6 part=touches
Any row holding a mutation-survivor count or a mutation-run rule for the browser: the four scene-render survivors now read caught; a row naming them or counting survivors would be stale.
@task_item n=7 part=touches
The convention that unit tests sit inline in cfg(test) modules, with its exception for an out-of-line sibling file with the _tests suffix: the session follows the exception, adding no new one.
@task_item part=untouched n=1
Colour rows: engine colours, select list colours, colour scheme recascade, colour tokens and the WPT runner's result colours.
@task_item part=untouched n=2
Motion and the virtual clock: animation, scroll behaviour, overlay scrollbar timings and the driver's advance, click, press and type verbs.
@task_item part=untouched n=3
The select list's open, close, highlight and choose events, its placement, hit-testing and accessibility snapshot.
@task_item part=untouched n=4
escher-session's exit statuses, stderr errors, mcp role results, tasks, actions, coverage and graph commands.
@task_item part=untouched n=5
Iconography and cursors, the shell provider's window controls, and the web-spa canvas surface.
@task_item part=untouched n=6
Font family fallback, the lang attribute, presentational hints, spacing and stacking-context paint order.
