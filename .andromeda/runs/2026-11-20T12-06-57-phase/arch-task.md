@task_prose part=chunk
Ends the four browser mutants the 0.3.0 boundary audit read as missed: render_scene's returned (u32, u32) replaced by (0, 0), (0, 1), (1, 0) and (1, 1), at the coordinate re-derived at HEAD as capture.rs:86:5.
Either a browser unit test calls render_scene directly and asserts the size it returns for two inputs of different sizes, both wider and taller than 1 px, or the unused return value is removed, or the four are kept on the founder's word; the choice between the three is a fork put to the founder.
Re-measured at HEAD: no caller of render_scene reads its returned size, so no browser path's behaviour depends on it.
Any cargo-mutants run, even of these four alone, is planned as an operator entry or a fork for the founder, never as a gate that fires by itself.
@task_file n=1 path=apps/browser/src/capture.rs mark=edits
what: the browser app's screenshot capture: rendering the current document into a paint scene and saving the shot — edits: its own cfg(test) unit-test module, or render_scene's return value, as the fork rules
@task_item n=1 part=needs
render_scene's returned size, a (u32, u32) pair: the value the new test asserts, and the value the founder may rule removed, leaving render_scene returning nothing.
@task_item n=2 part=needs
The viewport and its scale, which render_scene reads from the document to size its output: a direct test sets a viewport on the document and expects the returned size to follow it.
@task_item n=3 part=needs
BaseDocument as the input a direct test builds, with a set viewport, inside the browser crate's own test module under the crate's default features.
@task_item n=4 part=needs
The paint scene render_scene paints into: a direct test passes some implementation of the paint scene trait, built inside the browser crate.
@task_item n=5 part=needs
The host-run mutation tier: cargo-mutants runs on the dev host only, never in CI, through the nextest mutants profile; the browser's mutation target is the run that read the four, and any re-run of it starts only on the founder's word, as an operator entry or a fork.
@task_item n=6 part=needs
No change to what the browser paints or saves, and no new carrier, verb, log field, span, environment read or listener.
@task_item n=7 part=needs
The browser is an upstream app the fork does not develop: removing the unused return value edits upstream code and widens the upstream divergence, which is why that way is a fork for the founder, not the plan's own pick.
@task_item n=8 part=needs
Where a new unit test lives: in the source file's own cfg(test) module at the end of the file, reaching render_scene as a private item of its module.
@task_item n=9 part=needs
The browser crate's test run stays green, with the new test in it and every existing browser test unchanged.
@task_standing n=1
convention: each new test has a descriptive name that says what it asserts
home: conventions
@task_standing n=2
convention: unit tests sit in a cfg(test) module at the end of the source file they test
home: conventions
@task_standing n=3
convention: each expect or unwrap in a new test carries its justification
home: conventions
@task_standing n=4
convention: a new test asserts its fixture's precondition before the behaviour it checks
home: conventions
@task_standing n=5
convention: new code passes the formatting and clippy gates, any lint allow kept local to its item
home: conventions · infrastructure-patterns
@task_item n=1 part=not_needed
The driver's carriers (verbs, the session socket and its wire, the MCP role, the command line, kept checks and recordings): the chunk adds nothing that crosses them.
@task_item n=2 part=not_needed
The accessibility tree and its ARIA readings: nothing in the chunk builds or reads it.
@task_item n=3 part=not_needed
The select list, form controls, event dispatch and their default actions: the chunk touches no interaction.
@task_item n=4 part=not_needed
The WPT runner, its outputs, statuses and CI artifacts: not reached.
@task_item n=5 part=not_needed
Browser history, its persistence store, about pages and the toolbar: the chunk stays inside screenshot capture.
@task_item n=6 part=not_needed
Layout reasons, lint findings and snapshot contracts: no reader of layout or screen state changes.
@task_item n=7 part=not_needed
The cold-agent and agent-run tooling and the seven_guis stand: not reached.
@task_item n=8 part=not_needed
Upstream sync by merge or cherry-pick: the chunk takes nothing from another tree.
@task_item n=9 part=not_needed
The name registers of binaries, crates and environment names: the chunk adds no name.
