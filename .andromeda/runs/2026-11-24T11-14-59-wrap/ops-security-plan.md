@row_set n=1 row=decision/engine-dependency-pins cell=rule seen=4 detector=D-security-deps
value: The pins stay: Stylo 0.24, Taffy 0.16.0 from crates.io, Dioxus 0.7.3 and winit =0.31.0-beta.4.
basis: fct-xlmy5ueo
because: Taffy moved from the git pin of upstream DioxusLabs/taffy at a3997d55 to 0.16.0 from crates.io, so the pin the row names no longer stands.
@row_set n=2 row=decision/no-git-dependencies cell=except seen=3 detector=D-security-deps
value: 
basis: fct-x3oubhd5
because: The workspace's one git dependency, Taffy's git pin, is retired and the lock holds no git source, so the Taffy exception no longer stands.
@row_set n=3 row=decision/no-git-dependency cell=except seen=3 detector=D-security-deps
value: 
basis: fct-x3oubhd5
because: The workspace's one git dependency, Taffy's git pin, is retired and the lock holds no git source, so the Taffy exception no longer stands.
@row_set n=4 row=decision/upstream-sync-dependency-set cell=rule seen=4 detector=D-security-deps
value: Each upstream sync takes upstream's dependency set whole or, by cherry-pick, a coupled registry set upstream moved, packages entering with it, or a lock-only semver-compatible update; no git content; its lock read name by name, the audit leg green after it.
basis: fct-x3oubhd5
because: Taffy's git pin is retired and the lock holds no git source, so the clause admitting git content by Taffy's own pin no longer stands.
@row_add n=1 row=decision/os-drop-input area=input-validation fact=F1 detector=D-security-input
name: OS drag-and-drop input
rule: OS drag-and-drop data enters the engine from the shell's drop source as dragover, dragleave and drop events dispatched to the node under the drag, passing no parse of ours, and a dropped URI is accepted only as a file:// URI and turned into a path.
why: Upstream's design, ratified.
basis: fct-pr3l6fxd
because: The merge adds the OS drop source as a new external-input surface, ratified as taken, which no row of Input Validation states.
@row_add n=2 row=decision/dropped-file-read-unrestricted area=input-validation fact=F2 detector=D-security-input
name: Dropped-file reader without path restriction
rule: The Dioxus native bridge's dropped-file reader reads a dropped file's metadata and bytes through std::fs with no path restriction.
why: Upstream's design, ratified.
basis: fct-xuofjrvy
because: The merge adds a reader of dropped files with no path restriction, ratified as upstream's design, which no row states.
@row_add n=3 row=decision/root-drop-examples area=input-validation fact=F3 detector=none
name: Root drag-and-drop examples
rule: The root examples dnd and file_drop are taken; file_drop's image and HTML-file previews by file: URL are refused, while its srcdoc preview of dropped HTML data and its text preview read no URL and stand.
why: Ratified.
basis: fct-qbl64vse
because: The merge registers two root examples reading dropped data, and the measured limit of which previews are refused is stated in no row.
@row_add n=4 row=decision/custom-widget-drop-by-drop-effect area=input-validation fact=F4 detector=none
name: Custom widget accepts a drop by its drop effect
rule: A custom widget accepts a drop only by setting the dragover event's drop effect, and a drop effect is taken only where the drag source allows it.
guard: widget_accepts_a_drop_by_setting_the_drop_effect, drop_effect_must_be_allowed_by_the_drag_source
basis: fct-xuofjrvy
because: The custom widget contract gains a drop acceptance rule with the merge, which no row states.
@row_add n=5 row=decision/prevent-default-non-cancelable-ignored area=input-validation fact=F5 detector=none
name: preventDefault ignored on a non-cancelable event
rule: preventDefault on a non-cancelable event is ignored, so such an event's default action is never cancelled.
why: Upstream's behaviour, ratified.
guard: prevent_default_does_not_cancel_pointerleave
basis: fct-xuofjrvy
because: The event driver's default-prevented check now ignores preventDefault on a non-cancelable event, ratified, which no row states.
@row_add n=6 row=decision/os-drag-and-drop-untrusted area=threat-model-summary fact=F6 detector=none
name: OS drag-and-drop is an untrusted input source
rule: The local desktop's drag-and-drop is an untrusted input source to the engine and to a Dioxus app's handlers, carrying strings and file paths chosen by the user's OS session.
basis: fct-pr3l6fxd
because: The merge opens drag-and-drop as a new input source to the engine and to a Dioxus app's handlers, a threat no row of the summary names.
@row_none n=1 fact=F7 type=7
