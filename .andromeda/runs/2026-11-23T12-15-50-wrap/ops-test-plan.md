@row_add n=1 row=decision/app-widget-paints-at-content-box area=test-strategy fact=F1 detector=D-tests-coverage
name: App widget painted at its content box
rule: A widget an app paints from Rust is handed its content box times scale to paint at, and its scene paints at its content origin inside the element's transform, in both layout modes.
guard: stand_canvas_placement
basis: fct-m4p637pr, fct-wvvcibgm, fct-6pt5ja5d
because: The chunk moves a widget's paint size and origin to its content box and witnesses it by a new stand check, which test-plan §2 names as a witness per the plan's expected amendment.
@row_add n=2 row=decision/app-widget-pointer-frame-content-box area=test-strategy fact=F2 detector=D-tests-coverage
name: App widget hears pointer events from its content box
rule: A widget an app paints from Rust hears pointer events in the frame of its content box, page and client coordinates alike, in both layout modes.
guard: stand_canvas_pointer_frame
basis: fct-m4p637pr, fct-wvvcibgm, fct-6pt5ja5d
because: The chunk forwards pointer events to a widget in its content-box frame and witnesses it by a new stand check, which test-plan §2 names as a witness.
@row_add n=3 row=decision/canvas-text-from-document-fonts area=test-strategy fact=F3 detector=D-tests-coverage
name: Canvas text shaped from the document's fonts
rule: A widget an app paints from Rust is handed the document's own font context, and its text region is non-blank and byte-equal to the bundled-font route, in both layout modes and under font-embolden.
guard: stand_canvas_text
basis: fct-m4p637pr, fct-wvvcibgm, fct-6pt5ja5d
because: The chunk hands a widget the document's font context before paint and witnesses it by a new stand check, which test-plan §2 names as a witness.
@row_add n=4 row=decision/shared-build-dir-stale-fresh area=ci-integration fact=F4 detector=none
name: Stale Fresh reading under a shared build directory
rule: When another cargo shares the build directory, cargo can read a crate Fresh against edited sources and neither a re-run nor a touch clears it; a per-crate clean does, and a gate reading taken before that clean reads the pre-edit code.
basis: fct-wrcglkpd, fct-ermftsqd
because: The chunk measured a limit of cargo on the shared build directory that binds how later gate readings are taken, and no ci-integration row states it.
