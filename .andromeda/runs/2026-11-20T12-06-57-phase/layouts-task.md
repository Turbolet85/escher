@task_prose part=chunk
Ends the four surviving mutants the 0.3.0 boundary audit read as missed in the browser's capture module: render_scene's returned (u32, u32) size replaced by (0, 0), (0, 1), (1, 0) and (1, 1).
Each is either caught by a browser test or kept by the founder's word; the path between a direct test that pins the returned size, removing the unused return value, or keeping the four is a fork for the founder.
Re-derives the mutants' coordinate at HEAD: render_scene is declared at line 81 and the coordinate stands at line 86, column 5.
Re-measures that no caller reads the returned size: both capture paths discard it, so a test can catch the four only by calling render_scene directly.
Any cargo-mutants run, even of the four alone, is an operator entry or a fork for the founder, never a gate that runs on its own.
@task_file n=1 path=apps/browser/src/capture.rs mark=edits
what: the browser app's capture module — the screenshot capture and anyrender scene capture paths and the scene render they share — edits: its own cfg(test) module (a direct test of render_scene's returned size), or render_scene's return value, as the founder's fork rules
@task_item n=1 part=needs
render_scene's returned size: the value the four mutants replace, which a new test must assert for an input whose viewport is wider and taller than 1 px, and for a second input of a different size so the value is shown to follow the input.
@task_item n=2 part=needs
The browser crate's own unit test run under its default features: the new test lives in the capture module's cfg(test) module and must build a document with a set viewport and a paint scene there, with no new feature, fixture crate or harness.
@task_item n=3 part=needs
The founder's fork between a test that pins an unused value, removing the return value (an edit of upstream browser code that adds to the upstream divergence), and keeping the four on his word.
@task_item n=4 part=needs
The mutation tier: a cargo-mutants run of the browser crate starts only on the founder's word, runs on the dev host only and never in CI, and is planned as an operator entry or a fork, never as a local gate.
@task_item n=5 part=needs
What the browser paints, captures and saves stays as it is: the change adds no new carrier, verb, log field, span, env read or listener, and the two capture paths keep resolving their size as they do now.
@task_item n=1 part=not_needed
The stand's stable element ids and the stand's layout decisions (keyed controls, accent colour, keyboard regions, document skeleton, accessibility attributes): the chunk touches no stand page or control.
@task_item n=2 part=not_needed
The window chrome controls of the shell provider: the chunk changes no window behaviour.
@task_item n=3 part=not_needed
The command-line contracts of the session driver, the cold-agent and agent-run pipes, multi_doc and paint_bench (their arguments, exit codes, stdout, recordings and MCP server): the chunk adds no verb, argument, output or exit path to any command.
@task_item n=4 part=not_needed
The session driver's served tasks and the cold-agent decisions (prompts, pipe, driver use, caller input echo, session recordings): no served task or agent run is changed.
@task_item n=5 part=not_needed
The absence of arbitrary index.html templates: the chunk adds no document template.
