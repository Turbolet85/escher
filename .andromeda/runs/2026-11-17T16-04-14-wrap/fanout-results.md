# Fan-out results — 2026-11-17-upstream-sync-ahead-of-polish-and-ship

architecture: add 4 · set 4 · retire 4 · none 0 · ops-architecture.md
security-plan: add 2 · set 2 · retire 1 · none 0 · ops-security-plan.md
design-system: add 3 · set 0 · retire 0 · none 1 · ops-design-system.md
layout-templates: add 1 · set 1 · retire 0 · none 2 · ops-layout-templates.md
test-plan: add 3 · set 1 · retire 1 · none 1 · ops-test-plan.md
obs-plan: add 1 · set 0 · retire 0 · none 2 · ops-obs-plan.md
a11y-plan: add 0 · set 0 · retire 0 · none 1 · ops-a11y-plan.md

Nothing was removed by a fix: no file took a scripted fix.

Check 2 homes (one fact, one home): the scroll clamp at layout — architecture decision/scroll-offsets-clamped-after-layout; the free-disk rule — test-plan decision/free-disk-on-test-and-stand-repeat, its disk readings test-plan decision/test-job-reads-its-disk; the SVG embedded-image scope — architecture decision/svg-embedded-images-by-context; the renderer feature name — architecture decision/window-renderer-chosen-at-compile-time.

Check 5 (the plan's Expected amendments, each disposed): architecture `upstream-sync-merge-base` — its row states the mechanism (the last merged commit is the base, cherry-picks leave it unmoved) and names no pin, true on this take, no operation; architecture §Stack versions — an inventory (type 5), no row, the backend rows set below; the four engine behaviour changes — the scroll clamp and the SVG fetch as rows below, the AnyRender paint by the backend rows and the shear direction a paint mechanism with no row (type 4), each one's ratification the founder's own word (inputs#I5 §1–4) carried in the landed entries' Why; security-plan SVG widening — the answer card (Q1); security-plan dependency set — set below; the four zip ignores and `res-h7acz56c` — no master row names them (the site sweep's `advisories` master zero), the residual the route's; test-plan free-disk and disk readings — below; the per-OS tally record — an inventory (type 5), no row, carried in the report's Outcome, final; layout-templates CRUD clamped reading — no row names it, the clamp's home architecture; obs-plan silence line — obs-plan's `@row_none` type 8 (an inventory of moved log lines); the citations — no master site (rows hold no citation).

Check 6 (Spec claims disproved, each disposed): «112 carried» → 97 — no master row states the count (`left-out` sweep: the one row names no count), carried by the report; «SVG `<image>`/`<use>`» → `<image>`/`<feImage>` — every SVG operation below names image and feImage, a use href never fetched, ratified as measured (inputs#I8 §2); the clamp's call in `construct.rs` — no master row names a file; `cargo update -w` order — no master site (sweep master zero), a plan-step fact; research's three tracing lines moving → two move, one rewritten — obs-plan's `@row_none` (inventory), the count unchanged.

ops-architecture.md
decision/anyrender-rendering-abstraction · apply — check 1: the backend rename vello-hybrid → vello-gpu is a mechanical consequence of the AnyRender 0.15 take the founder ruled («Взять AnyRender 0.15 в этом sync», inputs#I5 §1); the report's Dependencies carry it
decision/window-renderer-chosen-at-compile-time · apply — check 1: the public feature rename vello-hybrid → vello-gpu, the report's Symbols / APIs; inside the founder's AnyRender ruling (inputs#I5 §1)
decision/in-repo-default-features-off · apply — check 1: the feature name in the exception follows the rename hybrid → vello-gpu, the report's Symbols / APIs
decision/sub-resource-requests-carry-initiator · escalate → Q1 — check 1: the SVG image/feImage href load is a boundary widening by upstream merge (rul-72xhfjoz, never routine); the founder's own word covers it (inputs#I5 §2, inputs#I8 §2) · resolved: apply (Q1) — relayed verbatim by the overseer, 2026-11-17, inputs#I8: «Да, в силе (Recommended)» (with inputs#I5 §2 «Взять (Recommended)»); the card confirmed by the operator, 2026-11-17: «Confirm all three (Recommended)»
decision/skrifa-matches-parley-vello · apply — check 1: the founder's own word amends the rule in its standing form, «Да, оба правила меняем (Recommended)» (relayed verbatim by the overseer, 2026-11-17, inputs#I8 §1); the report's Dependencies carry skrifa 0.49 beside 0.44
decision/font-deps-match-renderers · apply — check 1: the same ruling (inputs#I8 §1), one fact kept under the heir decision/skrifa-matches-parley
decision/only-stand-repeat-frees-disk · reject — check 2: the free-disk fact's home is test-plan decision/free-disk-on-test-and-stand-repeat; the row is retired without a cross-master heir in ops-main-architecture.md
decision/free-disk-space-only-in-stand-repeat · reject — check 2: the free-disk fact's home is test-plan decision/free-disk-on-test-and-stand-repeat; the row is retired without a cross-master heir in ops-main-architecture.md
decision/skrifa-matches-parley · apply — check 1: the amended rule's standing form, on the founder's own word (relayed verbatim by the overseer, 2026-11-17, inputs#I8 §1)
decision/scroll-offsets-clamped-after-layout · apply — check 1: rul-47a6zcoz (routine), an engine behaviour change for every document on the founder's own word «Взять, утверждаю (Recommended)» (inputs#I5 §4); the clamp's one home (check 2)
decision/svg-embedded-images-by-context · reject — check 7: not_master type 4 — how the SVG parse passes the image cache by caller, no reason stated; the code holds it; escalated (Q1), not asked
decision/free-disk-on-test-and-stand-repeat · reject — check 2: the fact's home is test-plan decision/free-disk-on-test-and-stand-repeat

ops-main-architecture.md
decision/only-stand-repeat-frees-disk · apply — check 2: the row is false (the free-disk step stands on two jobs, the report's Harness / gate surface), retired with its fact's home in test-plan
decision/free-disk-space-only-in-stand-repeat · apply — check 2: the row is false (the report's Harness / gate surface), retired with its fact's home in test-plan

ops-security-plan.md
decision/upstream-sync-dependency-set · apply — check 1: the founder's own word amends the rule in its standing form, «Да, оба правила меняем (Recommended)» (relayed verbatim by the overseer, 2026-11-17, inputs#I8 §1); the report's Dependencies carry the seven entering packages
decision/programmatic-scroll-clamped · reject — check 2: the layout-time clamp's home is architecture decision/scroll-offsets-clamped-after-layout; the row's programmatic clamp stays true as it stands
decision/free-disk-only-on-stand-repeat · reject — check 2: the free-disk fact's home is test-plan decision/free-disk-on-test-and-stand-repeat; the row is retired without a cross-master heir in ops-main-security-plan.md
decision/free-disk-on-test-and-stand-repeat · reject — check 2: the fact's home is test-plan decision/free-disk-on-test-and-stand-repeat
decision/svg-image-href-fetch · escalate → Q1 — check 1: a boundary widening by upstream merge (rul-72xhfjoz, never routine); covered by the founder's own word, inputs#I5 §2 and inputs#I8 §2 · resolved: apply (Q1) — relayed verbatim by the overseer, 2026-11-17, inputs#I8: «Да, в силе (Recommended)» (with inputs#I5 §2 «Взять (Recommended)»); the card confirmed by the operator, 2026-11-17: «Confirm all three (Recommended)»

ops-main-security-plan.md
decision/free-disk-only-on-stand-repeat · apply — check 2: the row is false (the report's Harness / gate surface), retired with its fact's home in test-plan

ops-test-plan.md
decision/file-urls-only-for-file-documents · escalate → Q1 — check 1: names the SVG image/feImage href among the refused sub-resource loads, the widening's subject (rul-72xhfjoz); covered by inputs#I5 §2 and inputs#I8 §2 · resolved: apply (Q1) — relayed verbatim by the overseer, 2026-11-17, inputs#I8: «Да, в силе (Recommended)» (with inputs#I5 §2 «Взять (Recommended)»); the card confirmed by the operator, 2026-11-17: «Confirm all three (Recommended)»
decision/only-stand-repeat-frees-disk · apply — check 1: the row is false on the report's Harness / gate surface; the plan's Expected amendment names the change (test-plan §9, the guards renamed and added)
decision/free-disk-on-test-and-stand-repeat · apply — check 2: the free-disk fact's one home; the plan's Expected amendment names it
decision/test-job-reads-its-disk · reject — check 7: not_master type 6 — two diagnostic df steps retell the workflow's configuration and bind no future work; ci.yml and its guard test_test_job_reads_its_disk hold them
decision/unacquired-ci-job-rerun · reject — check 3: the operator's ruling governed this pass's two cancelled jobs and was not stated as a standing rule; a master row would reach past his word (a P3 curation candidate)

ops-design-system.md
decision/scroll-offset-clamped-to-range · reject — check 2: the scroll clamp's home is architecture decision/scroll-offsets-clamped-after-layout
contract/dioxus-native-renderer-features/vello-gpu · reject — check 2: the renderer feature name's home is architecture decision/window-renderer-chosen-at-compile-time
decision/svg-external-images-through-image-cache · reject — check 2: the fact's home is architecture decision/svg-embedded-images-by-context (its `area=new` question not put: the add does not stand)

ops-layout-templates.md
contract/paint-bench-cli/backend · apply — check 1: the backend name follows upstream's rename hybrid → gpu, the report's Deviations from intent (commands.md's paint_bench list)
decision/scroll-offset-clamped-to-range · reject — check 2: the scroll clamp's home is architecture decision/scroll-offsets-clamped-after-layout

ops-obs-plan.md
decision/ci-test-job-disk-readings · reject — check 2: the disk readings' home is test-plan decision/test-job-reads-its-disk
