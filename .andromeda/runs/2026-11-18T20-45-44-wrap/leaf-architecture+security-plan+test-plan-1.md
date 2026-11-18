@leaf_group group=architecture+security-plan+test-plan-1
@leaf_read leaf=.claude/docs/services/escher-driver.md lines=63
last: - The command line, through the real `escher-session` binary
@leaf_edit n=1 leaf=.claude/docs/services/escher-driver.md line=11 class=omission
leaf-text: Through `seven_guis → escher-driver` both seven_guis binaries
source: dec-xxvmhvjw «The driver library depends on no app, emits no event, installs no global subscriber, setting only escher-telemetry's scoped capture default around an acting step, and reads no env var.»
replace-with: The sixth, `escher-telemetry`: the driver sets only its scoped log capture as the default around an acting step, installing no global subscriber. Through `seven_guis → escher-driver` both seven_guis binaries
@leaf_edit n=2 leaf=.claude/docs/services/escher-driver.md line=19 class=omission
leaf-text: `diff` is dioxus-native-dom's `SnapshotDiff`, unchanged.
source: ctr-5uc2ynp6 «A read answering a LayoutExplanation of String ids, closed words and numbers with no NodeId, in process or by the layout verb inside its call's command span»
replace-with: a `layout` call answers the `LayoutExplanation` `Session::layout` reads (since 2026-11-17-layout-on-the-carriers); `diff` is dioxus-native-dom's `SnapshotDiff`, unchanged.
@leaf_edit n=3 leaf=.claude/docs/services/escher-driver.md line=24 class=omission
leaf-text: panics, then errors new since the boundary marks, then records, 16 kept
source: ctr-lhhrhpqh «A caught panic the capture's hook did not see still answers a `panic` item in `caused` whose message is the fixed text `<payload unread>`, so the panic count never loses an item.»
replace-with: panics, then errors new since the boundary marks, then records, 16 kept, a caught panic the capture's hook did not see still answering a `panic` item whose message is the fixed text `<payload unread>`
@leaf_edit n=4 leaf=.claude/docs/services/escher-driver.md line=24 class=stale
leaf-text: thirteen `FieldKind`s
source: ctr-bynrjicd «The verbs explored are the actions table's target verbs less any whose result holds an image or a layout reading field (png, rect, placement, shifts or size kind), selected by the field's kind»
replace-with: and `layout`'s readings of the rect, placement, shifts and size kinds among the `FieldKind`s
@leaf_edit n=5 leaf=.claude/docs/services/escher-driver.md line=25 class=omission
leaf-text: · `Actions` (validated from `("actions", [])`, since 2026-11-08-what-can-i-do-here)
source: ctr-4nnclsgf «Takes one required id, refused malformed without it or with a second argument and not-found or stale for an unknown id»
replace-with: · `Actions` (validated from `("actions", [])`, since 2026-11-08-what-can-i-do-here) · the `layout` call's command (validated with its one required `id`, since 2026-11-17-layout-on-the-carriers)
@leaf_edit n=6 leaf=.claude/docs/services/escher-driver.md line=62 class=omission
leaf-text: where `caused` carries its error text and its chain.
source: dec-hmr2374t «A check asserts engine-sourced caused items only on a build with the engine's tracing compiled in, read through the engine's own constant, and runs on both builds.»
replace-with: where `caused` carries its error text and its chain; engine-sourced `caused` items are asserted only on a build with the engine's tracing compiled in, read through the engine's own constant, the check (`stand_act_error_text`) running on both builds.
