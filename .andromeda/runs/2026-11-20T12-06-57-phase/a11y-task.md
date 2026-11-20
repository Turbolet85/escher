@task_prose part=chunk
Ends the four surviving mutants the 0.3.0 boundary audit read at the browser's capture module: render_scene's returned (u32, u32) size replaced by (0, 0), (0, 1), (1, 0) and (1, 1), with no browser test telling them apart.
Re-derives the coordinate at HEAD: render_scene is declared at line 81 and 86:5 is the first statement of its body, so the coordinate stands as 86:5.
Re-measures that no caller reads the returned size: both callers, the screenshot capture and the anyrender scene capture, discard it.
Either adds a direct unit test in the capture module's own cfg(test) module that asserts the returned size for two different viewport sizes, or removes the unused return value, or keeps the four on the founder's word; the choice is a fork for the founder.
Any cargo-mutants run, even of these four alone, is planned as an operator entry or a fork for the founder, never as a gate that fires by itself; the mutation tier runs on the dev host only, never in CI.
@task_file n=1 path=apps/browser/src/capture.rs mark=edits
what: the browser app's capture module — the screenshot capture, the anyrender scene capture and the scene render that paints a document at its viewport scale and returns the size it rendered — edits: its own cfg(test) module (a new test of render_scene's returned size), or render_scene's return value, as the fork rules
@task_item n=1 part=needs
render_scene's returned size: for a document whose viewport is wider and taller than 1 px, the size it returns is the viewport's real size, not any of (0, 0), (0, 1), (1, 0), (1, 1); a second input of a different size shows the value follows the input.
@task_item n=2 part=needs
The new test calls render_scene directly, building what it takes (a base document with a set viewport, a paint scene) inside the browser crate's own cfg(test) module, since no caller reads the returned value.
@task_item n=3 part=needs
The new test builds and runs under the browser crate's default features, where the browser's accessibility feature is off by default; it must not need that feature turned on.
@task_item n=4 part=needs
The viewport the test sets carries a scale (the scene render reads the viewport's scale, 1.0 unzoomed); the asserted size follows the viewport and its scale as set.
@task_item n=5 part=needs
The screenshot capture changes nothing: the chunk changes nothing the browser paints or saves, and capture_screenshot keeps resolving its size itself before it calls the scene render, whichever way the fork rules.
@task_item n=6 part=needs
The chunk adds no new carrier, verb, log field, span, environment read or listener.
@task_item n=7 part=needs
The cargo-mutants mutation run of the browser crate is the founder's to start: an operator entry or a fork, on the dev host only, never in CI and never a local gate that fires by itself.
@task_item n=1 part=not_needed
Keyboard navigation — focus, Tab order, keyboard activation, range input and select list keys, the seven_guis grid and list: the chunk presses no key and moves no focus.
@task_item n=2 part=not_needed
ARIA patterns and roles and the accessibility tree's states and ARIA property reads: the chunk builds no accessibility tree node, role, name or state.
@task_item n=3 part=not_needed
Visual design verification — contrast thresholds, the contrast lint and its finding, stand accent colour, scrollbar thumbs, select list colours and motion, hit-testing: the chunk measures no colour pair and changes no paint.
@task_item n=4 part=not_needed
Cognitive accessibility — invalid state cues and the stand's shared tab order table: the chunk touches no form state and no stand.
@task_item n=5 part=not_needed
The accessibility platform adapter, AccessKit and accesskit_xplat reach, the session library's accessibility feature and the stand binaries: the chunk adds no adapter and turns no accessibility feature on.
@task_item n=6 part=not_needed
Dioxus document handler records and UI graph state registration: the chunk touches no Dioxus render or graph state.
