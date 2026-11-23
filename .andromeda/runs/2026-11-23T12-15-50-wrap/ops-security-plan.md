@row_add n=1 row=decision/custom-widget-content-box-frame area=input-validation fact=F1 detector=D-security-input
name: Custom widget painted and pointed in its content box
rule: Behind the custom-widget feature, a custom widget's paint size is its content box times scale, floored at 0, its scene sits at the content origin inside the element's transform, unclipped, and its pointer events arrive in its content-box frame while a sub-document's stay in its border-box frame.
why: Ratified; answers upstream issue #1578, and paint size, placement and pointer frame are layout values, not input.
guard: stand_canvas_placement, stand_canvas_pointer_frame
basis: fct-wvvcibgm, fct-m4p637pr, fct-ogavdygd
because: The chunk moves a custom widget's paint size, scene placement and pointer frame to its content box, a decision the plan sets beside the widget's attribute replay in Input Validation.
@row_add n=2 row=decision/custom-widget-document-font-context area=input-validation fact=F2 detector=D-security-input
name: Custom widget handed the document's font context
rule: Behind the custom-widget feature, a custom widget is handed the document's own shared, mutable font context before each paint, and the base document exposes that context publicly, so app widget code holds a live handle to the engine's font context; no input crosses and the default ignores it.
why: Ratified; no input crosses, the handle being the engine's own.
guard: stand_canvas_text
basis: fct-wvvcibgm, fct-m4p637pr, fct-ogavdygd
because: The chunk adds a defaulted widget method and a public accessor that give app code a mutable handle to the engine's font context, a new boundary no standing row names.
@row_add n=3 row=decision/cookie-advisory-lock-pin area=dependency-security fact=F3 detector=D-security-deps
name: cookie pinned at the advisory's patched release
rule: The cookie crate is pinned at 0.18.3 in the lockfile, fixing RUSTSEC-2026-0594, as a lock-only move with no advisory ignore and deny.toml unchanged.
why: A reviewed pin: the patched release for the advisory, taken instead of an ignore.
guard: the audit leg
basis: fct-wgpw7fkq, fct-dj547r75, fct-sbp6ljmj
because: The chunk fixes an advisory the audit leg read red by moving cookie in the lock alone, a reviewed pin no standing row records.
@row_add n=4 row=decision/lock-only-patch-bump-by-hand-edit area=dependency-security fact=F4 detector=D-security-deps
name: Lock-only patch bump by hand-editing the lock
rule: A lock-only patch bump on this lockfile is made by hand-editing the crate's version and checksum lines, never by cargo update -p, and the syn edges of bindgen and cssparser-macros are kept at 3.0.6.
why: cargo update -p re-resolves unrelated edges on this lock, moving those syn edges to 2.0.119.
basis: fct-cbi3t2u2, fct-coyvnl7v, fct-wgpw7fkq
because: The chunk measured that a targeted cargo update moves unrelated edges and set the hand-edit as the way a lock-only patch bump is made.
@row_none n=1 fact=F5 type=5
