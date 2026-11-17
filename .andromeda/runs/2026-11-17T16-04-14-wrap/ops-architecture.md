@row_set n=1 row=decision/anyrender-rendering-abstraction cell=rule seen=1 detector=D-arch-decisions
value: Painting goes through the anyrender abstraction, with its vello, vello-cpu, vello-gpu, skia and svg backends and its serializer.
basis: fct-xfukctcw
because: The AnyRender 0.15 set replaced the anyrender_vello_hybrid backend with anyrender_vello_gpu, so the row's backend list must name vello-gpu, not vello-hybrid.
@row_set n=2 row=decision/window-renderer-chosen-at-compile-time cell=rule seen=1 fact=F5 detector=D-arch-decisions
value: dioxus-native chooses its window renderer at compile time among vello, vello-cpu-base, skia, skia-raster-base and vello-gpu, the GPU window renderer being anyrender_vello_gpu, while blitz uses the vello window renderer.
basis: fct-y4zyhjnd
because: dioxus-native's public feature vello-hybrid was renamed vello-gpu over anyrender_vello_gpu, so no hybrid renderer name remains for the row to list.
@row_set n=3 row=decision/in-repo-default-features-off cell=except seen=1 detector=D-arch-decisions
value: The `seven_guis` path entry keeps its default `vello-gpu` renderer feature, because `dioxus-native` raises `compile_error!` with no renderer.
basis: fct-y4zyhjnd
because: The apps' and examples' renderer feature hybrid was renamed vello-gpu, so the exception's feature name no longer stands.
@row_set n=4 row=decision/sub-resource-requests-carry-initiator cell=rule seen=2 fact=F1 detector=none
value: Every engine sub-resource request (linked stylesheet, its reload, @import, @font-face, CSS image, img, iframe, SVG image or feImage href) carries its loading document's URL as initiator, a use href never fetched; a request with no initiator is a top-level or embedder load, or a page navigation.
basis: fct-y4zyhjnd
because: An SVG image or feImage href is now loaded through the stamping request constructor as a sub-resource of its document, while a use href is not fetched, so the list of stamped kinds must name it.
@row_retire n=1 row=decision/skrifa-matches-parley-vello seen=1 heir=decision/skrifa-matches-parley detector=D-arch-decisions
why: The renderer set now brings a second skrifa version beside parley's, so skrifa no longer matches vello's version.
basis: fct-xfukctcw
because: skrifa 0.49 and read-fonts 0.46 joined beside 0.44 and 0.41 with the AnyRender 0.15 move, and the rule is amended in its standing form.
@row_retire n=2 row=decision/font-deps-match-renderers seen=1 heir=decision/skrifa-matches-parley detector=D-arch-decisions
why: The renderer set now brings a second skrifa version beside parley's, so skrifa no longer matches the renderers' version.
basis: fct-xfukctcw
because: skrifa 0.49 and read-fonts 0.46 joined beside 0.44 and 0.41 with the AnyRender 0.15 move, and the rule is amended in its standing form.
@row_retire n=3 row=decision/only-stand-repeat-frees-disk seen=1 heir=decision/free-disk-on-test-and-stand-repeat detector=none
why: The default-features test job now frees disk too, so stand-repeat is no longer the only job that does.
basis: fct-w2ipnyht
because: test-features-default gained the same SHA-pinned free-disk step and its guard was renamed, so the row and its guard name are false.
@row_retire n=4 row=decision/free-disk-space-only-in-stand-repeat seen=1 heir=decision/free-disk-on-test-and-stand-repeat detector=none
why: The default-features test job now runs the free-disk-space action too, so stand-repeat is no longer its one run.
basis: fct-w2ipnyht
because: test-features-default gained the same SHA-pinned free-disk step and its guard was renamed, so the row and its guard name are false.
@row_add n=1 row=decision/skrifa-matches-parley area=established-decisions detector=D-arch-decisions
name: skrifa follows parley
rule: The workspace's skrifa version matches the one parley uses; the renderer set may bring a second skrifa and read-fonts version, which stands beside it in the graph.
why: ratified as the standing form of the font-crate rule
basis: fct-xfukctcw
because: The AnyRender 0.15 move brought skrifa 0.49 beside parley's 0.44, and the amended rule must replace the two retired rows.
@row_add n=2 row=decision/scroll-offsets-clamped-after-layout area=established-decisions fact=F2 detector=none
name: Scroll offsets clamped after layout
rule: After layout rounding every scroll container's offset is clamped to its scroll range, the viewport and a running scroll animation are clamped too, and a removed subtree's scroll state is reset, so a list whose content shrinks after a delete reads offset 0.
why: an upstream behaviour change taken, ratified for its take
guard: stand_act_scroll, percentage_height_scroller_clamps_after_viewport_resize
basis: fct-y4zyhjnd
because: The take clamps scroll offsets in the final-layout write and resets a removed subtree's scroll state, a behaviour change for every document no row states.
@row_add n=3 row=decision/svg-embedded-images-by-context area=established-decisions fact=F3 detector=none
name: SVG embedded images resolved only inline
rule: Inline and top-level SVG resolve their embedded image elements against the document URL and the image cache; an SVG loaded as an image resolves no embedded images.
basis: fct-y4zyhjnd
because: SVG parsing now takes the document URL and image cache for an inline or top-level svg and none for an SVG image, a scope no row states.
@row_add n=4 row=decision/free-disk-on-test-and-stand-repeat area=infrastructure-patterns fact=F4 detector=none
name: Disk freed only on the test and stand-repeat jobs
rule: The default-features test job and the stand-repeat job are the only jobs that free disk, with the same SHA-pinned action; the test job reads disk before and after its test leg into the step log and uploads nothing.
why: the readings measure the test job's disk use rather than cure it
guard: test_free_disk_only_on_test_and_stand_repeat, test_test_job_reads_its_disk
basis: fct-w2ipnyht
because: test-features-default gained the free-disk step and two disk readings pinned by two guards, replacing the stand-repeat-only rule.
