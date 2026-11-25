@task_prose part=chunk
Lets an app declare virtual nodes inside a canvas widget (an element the app paints from Rust): each with an author key, a role from a closed set, a non-empty name and a region inside the canvas's content box.
A declaration that breaks a rule is not taken and nothing panics; each taken node gets the reserved id form owner, separator, key; the declared nodes are readable in process.
Writes every operator fork (the declaration's form, the names' crossing, and any further fork found) into the chunk's founder-forks evidence and stops at P4 with no default taken.
Claims no capability; v040-11 stays pooled; two carried items ride the chunk to its wrap.
@task_file n=1 path=packages/dioxus-native-dom/src/element_id.rs
what: the stable element id component — the reserved virtual node id form, its key separator and the function that builds a virtual node's id from the canvas element's stable id and the key
@task_file n=2 path=packages/blitz-dom/src/node/custom_widget.rs
what: the engine's custom widget trait — the canvas element whose content the app paints from Rust, with its paint size, its pointer frame and a commented per-widget accessibility tree hook
@task_item n=1 part=needs
A canvas widget can declare virtual nodes; the declaration's author key is non-empty and holds no slash, its role is one of a closed set, its name is non-empty, its region lies inside the canvas's content box.
@task_item n=2 part=needs
A declaration that breaks any of those rules is not taken: no node stands for it and no panic path exists on that untrusted input (the security rule against panics on untrusted input binds the validation).
@task_item n=3 part=needs
Each taken node carries the reserved virtual node id form: the owning canvas element's stable id, then the reserved separator, then the key — the spelling the stable element id contract reserves for virtual nodes; the canvas's own stable id is the owner part.
@task_item n=4 part=needs
Two nodes in one canvas with the same key are refused, so that one id names one node only.
@task_item n=5 part=needs
The region is held in the canvas's content-box frame, the same frame the widget paints in and hears pointer events in (security rule on custom widget paint size and pointer frame).
@task_item n=6 part=needs
The nodes a canvas declares now — validated, with their ids — can be read in process.
@task_item n=7 part=needs
The chunk adds no driver verb, argument kind, cause, session error, exit status, carrier, log field or span, and no new reading on the session socket, the command line, the MCP server or a kept check; whatever answers those give today stays as it is.
@task_item n=8 part=needs
The names' crossing fork: a virtual node's name is text the app author wrote, and by the founder's rule authored text on a ratified carrier is a widening; whether a name or key reaches any carrier is left to the founder, and a plan that crosses nothing leaves it for the later entry.
@task_item n=9 part=needs
The declaration's form fork: a list at each paint, or callbacks with invalidation; with the further forks found (the role set, whether the declaration lives on the engine widget trait or the Dioxus renderer, the region's frame and its rule), each written with what crosses, the rule it meets, each option's measured cost, the dependences, whose it is and the most conservative option that still builds the entry.
@task_item n=10 part=needs
CARRY: two gaps in the security summary (OS drag-and-drop as an untrusted input source; the dropped-file reader with no path restriction) are written only at this chunk's wrap by the leaf pass, never by hand.
@task_item n=11 part=needs
CARRY: the test-plan rule that no committed check takes a screenshot step still says screenshot repeatability is asserted on one host only, while the cross-OS canvas stand check holds canvas-only shot digests equal on three hosts; its one-host clause is carried by this chunk's wrap report, with no plan step editing code for it.
@task_item n=12 part=needs
Operator conditions: no raw log with a temp or home path goes into evidence; a probe worktree goes outside the build output directory and no plan step deletes it; a guard's refusal of a write is a stop; a control enters the plan with its red reading.
@task_item n=13 part=needs
The CI verdict of the head commit is not yet available and is not read as green; the chunk's base read green on all its checks.
@task_item n=14 part=needs
The new validation and id assignment owe tests proving each refusal (empty key, key with a slash, unknown role, empty name, region outside the content box, duplicate key) takes no node and does not panic, and that a taken node's id has the reserved form.
@task_item n=1 part=not_needed
The window chrome provider controls (drag, maximise, minimise, close, decorations): the chunk touches no window control.
@task_item n=2 part=not_needed
The stand's stable element ids for the task controls and task cards (counter, temperature, flight booker, timer, CRUD, cells, order, load) other than the circle canvas: the chunk adds or renames no stand control.
@task_item n=3 part=not_needed
The agent-run and cold-agent test pipes, their command lines and the cold-agent decisions: the chunk runs no agent.
@task_item n=4 part=not_needed
The multi_doc and paint_bench command lines: no benchmark is added or changed.
@task_item n=5 part=not_needed
The session's graph, actions, coverage and layout explained answers, and the off-screen refusal: these reads and refusals for virtual nodes belong to the later entries «Virtual nodes as elements» and «Virtual nodes in every read».
@task_item n=6 part=not_needed
The Circle Drawer circles painted on a canvas and the draw commands grouped by node: the stand's circles as nodes belong to «Circle Drawer's circles as virtual nodes».
@task_item n=7 part=not_needed
The CRUD list keyboard region and keyboard reach into a canvas: «Canvas keyboard path» owns keyboard reach.
@task_item n=8 part=not_needed
The document skeleton, arbitrary index.html templates, the headless stand viewport, the stand accent colour, the load page's network provider, the order page and the flight type select: the chunk changes no page layout or stand boot.
@task_item n=9 part=not_needed
Into-view scrolling, visible region through clipping boxes and open list rows' bounds: the chunk measures no element's visible region.
@task_item n=10 part=not_needed
The session recording as a replayable check, its check files and record directory: the chunk records no session.
