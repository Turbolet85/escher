# Code Audit — escher · Epoch 1 — Foundation · 2026-10-06T05:36:13Z
mode baseline · HEAD d4113768 · baseline none · span — (first record)
overshoot 0 commits — HEAD is the epoch boundary (d4113768, the 2026-10-06-cold-agent-run-pipe complete flip)

Trend judgments begin at the next boundary: this record is the baseline every later diff reads. Below are absolute-only findings. Nothing here is applied; each line is a direction for the founder's judgment.

## Baseline findings

### B1 — dependency cycles · 0
The canonical unit-cycle walk over `crate_edges` (88 usage-based edges, 21 source units, graph `fresh`) returns no cycle. Any cycle at a later boundary fires `new-cycle`.

### B2 — mutation survivors · escher-telemetry · 1
**Score:** 88.89 = caught/(caught+missed) = 8/9 · 12 mutants: 8 caught · 1 missed · 3 unviable · 0 timeout · 0 not measured on this host (x86_64-unknown-linux-gnu) · state complete 12/12

| site | mutation |
|---|---|
| packages/escher-telemetry/src/lib.rs:74:9 | replace <impl fmt::Display for InitError>::fmt -> fmt::Result with Ok(Default::default()) |

Not measured on this host (x86_64-unknown-linux-gnu): none — no project union verdict.

**Suspected shape:** the one survivor is the `Display` text of `InitError::ForeignSubscriber`. No test reads the rendered message: neither the four `telemetry_*` blitz-tests nor the crate's lib tests name `ForeignSubscriber` or its text.
**Proposal:** if the message is a contract (operators read it when init refuses), one assertion on `InitError::ForeignSubscriber.to_string()` kills it; if it is not, the survivor is accepted noise.

**Scope and method (disclose):** the test set is escher-telemetry's lib tests + blitz-tests' lib + `telemetry_panic_hook`, `telemetry_init_idempotent`, `telemetry_stdout_silent`, `telemetry_scrub` (`--cargo-test-arg`). The tool's unmutated baseline was SKIPPED (`--baseline=skip --timeout 120`). cargo-mutants runs that baseline against the mutated package alone, where the `--test=telemetry_*` filters do not exist, so it aborts before testing any mutant. The narrowed unmutated build was verified by hand and `ci-leg.sh coverage` ran the whole suite green at this sha. The 3 unviable are genuine type errors: `Ok(Default::default())` for `init`/`init_with_writer` (`InitOutcome` has no `Default`) and `Default::default()` for `decide` (`Verdict`).

**Faulted first invocation (kept as `c-mutation-escher-telemetry-faulted.json`, not scored):** the same 12 mutants, tested against all of blitz-tests at `-j 3`, came back 2 caught · 1 missed · 9 unviable, which is `unviable-dominant`. Six of the nine unviable builds failed with `Disk quota exceeded (os error 122)` on the usrquota tmpfs `/tmp`, where each cargo-mutants build copy needs about 12–13 GB. This is a host fact for later boundaries: a mutation unit that links blitz-tests fits `/tmp` only at `-j 1`.

### B3 — dead-code candidates · 108
Zero-ref rows 1379 → test-path excluded 504 → false-positive classes: trait-impl (dispatch) 556 · derive/attr-invoked 92 · entry-point 51 · format-capture 4 · runtime-invoked (macro-generated binding) 20 · test-only 44 → **108 candidates** (never "dead"). The recipe is pinned in `c-dead.json` (`recipe`); reuse it verbatim or `dead-growth` fires on filter drift.

Per crate: blitz-dom 53 · blitz-test-harness 13 · blitz-traits 11 · browser 7 · blitz-paint 6 · dioxus-native 6 · blitz-vibey-script 3 · blitz-net 2 · blitz-shell 2 · debug_timer 2 · dioxus-native-dom 2 · accesskit_xplat 1. Per kind: fn 88 · macro 1 · term 3 · type 16.

**Reading:** most candidates are public API of the upstream engine crates (blitz-dom `BaseDocument` setters and stylesheet accessors, blitz-test-harness input verbs, blitz-traits `ShellProvider` methods). An embeddable library's public surface is consumed by embedders outside this workspace, so a zero in-workspace reference does not prove it dead. The share nearest escher's roadmap is the 13 blitz-test-harness Harness verbs (`mouse_down_at`, `drag`, `tap`, `touch_*`, `ime`, `hovered`, `tick`, `time` …). That crate is upstream (#586), Epoch 1 changed 10 of its lines, and the driver epochs (Epoch 4) are their expected callers.
**Proposal:** none at baseline. If `dead-growth` fires later, the growth rows are the ones to read.

| crate | kind | symbol path | def site |
|---|---|---|---|
| browser | fn | history/HistoryNav#current_url(). | apps/browser/src/history.rs:67 |
| browser | fn | history/HistoryNav#has_back(). | apps/browser/src/history.rs:68 |
| browser | fn | history/HistoryNav#has_forward(). | apps/browser/src/history.rs:69 |
| browser | fn | history/HistoryNav#go_back(). | apps/browser/src/history.rs:70 |
| browser | fn | history/HistoryNav#go_forward(). | apps/browser/src/history.rs:71 |
| browser | fn | history/HistoryNav#navigate(). | apps/browser/src/history.rs:72 |
| browser | fn | url_suggestions/impl#[Suggestion]row_key(). | apps/browser/src/url_suggestions.rs:40 |
| accesskit_xplat | fn | impl#[Adapter]with_split_handlers(). | packages/accesskit_xplat/src/lib.rs:197 |
| blitz-dom | type | config/StyleThreading#Sequential# | packages/blitz-dom/src/config.rs:28 |
| blitz-dom | fn | document/impl#[BaseDocument]set_net_provider(). | packages/blitz-dom/src/document.rs:535 |
| blitz-dom | fn | document/impl#[BaseDocument]set_navigation_provider(). | packages/blitz-dom/src/document.rs:540 |
| blitz-dom | fn | document/impl#[BaseDocument]set_html_parser_provider(). | packages/blitz-dom/src/document.rs:550 |
| blitz-dom | fn | document/impl#[BaseDocument]set_base_url(). | packages/blitz-dom/src/document.rs:555 |
| blitz-dom | fn | document/impl#[BaseDocument]as_any_mut(). | packages/blitz-dom/src/document.rs:624 |
| blitz-dom | fn | document/impl#[BaseDocument]take_pending_resource_deallocations(). | packages/blitz-dom/src/document.rs:801 |
| blitz-dom | fn | document/impl#[BaseDocument]print_subtree(). | packages/blitz-dom/src/document.rs:1082 |
| blitz-dom | fn | document/impl#[BaseDocument]remove_user_agent_stylesheet(). | packages/blitz-dom/src/document.rs:1127 |
| blitz-dom | fn | document/impl#[BaseDocument]author_stylesheets(). | packages/blitz-dom/src/document.rs:1140 |
| blitz-dom | fn | document/impl#[BaseDocument]useragent_stylesheets(). | packages/blitz-dom/src/document.rs:1145 |
| blitz-dom | fn | document/impl#[BaseDocument]upsert_stylesheet_for_node(). | packages/blitz-dom/src/document.rs:1178 |
| blitz-dom | fn | document/impl#[BaseDocument]subdoc(). | packages/blitz-dom/src/document.rs:2007 |
| blitz-dom | type | SelectorList# | packages/blitz-dom/src/lib.rs:121 |
| blitz-dom | fn | mutator/impl#[`DocumentMutator<'_>`]remove_node_if_unparented(). | packages/blitz-dom/src/mutator.rs:622 |
| blitz-dom | fn | node/custom_widget/impl#[BaseDocument]can_create_surfaces(). | packages/blitz-dom/src/node/custom_widget.rs:15 |
| blitz-dom | fn | node/custom_widget/impl#[BaseDocument]destroy_surfaces(). | packages/blitz-dom/src/node/custom_widget.rs:34 |
| blitz-dom | fn | node/custom_widget/Widget#connected(). | packages/blitz-dom/src/node/custom_widget.rs:79 |
| blitz-dom | fn | node/custom_widget/Widget#disconnected(). | packages/blitz-dom/src/node/custom_widget.rs:81 |
| blitz-dom | type | node/custom_widget/CustomWidgetStatus#PendingRemoval# | packages/blitz-dom/src/node/custom_widget.rs:153 |
| blitz-dom | type | node/element/SpecialElementType#Stylesheet# | packages/blitz-dom/src/node/element.rs:329 |
| blitz-dom | type | node/element/SpecialElementType#Image# | packages/blitz-dom/src/node/element.rs:330 |
| blitz-dom | type | node/element/SpecialElementType#Canvas# | packages/blitz-dom/src/node/element.rs:331 |
| blitz-dom | type | node/element/SpecialElementType#TableRoot# | packages/blitz-dom/src/node/element.rs:332 |
| blitz-dom | type | node/element/SpecialElementType#TextInput# | packages/blitz-dom/src/node/element.rs:333 |
| blitz-dom | type | node/element/SpecialElementType#CheckboxInput# | packages/blitz-dom/src/node/element.rs:334 |
| blitz-dom | type | node/element/SpecialElementType#FileInput# | packages/blitz-dom/src/node/element.rs:336 |
| blitz-dom | type | node/element/SpecialElementType#None# | packages/blitz-dom/src/node/element.rs:338 |
| blitz-dom | fn | node/element/impl#[ElementData]raster_image_data_mut(). | packages/blitz-dom/src/node/element.rs:522 |
| blitz-dom | fn | node/element/impl#[ElementData]canvas_data(). | packages/blitz-dom/src/node/element.rs:529 |
| blitz-dom | fn | node/element/impl#[ElementData]file_data_mut(). | packages/blitz-dom/src/node/element.rs:621 |
| blitz-dom | type | node/element/Status#Error# | packages/blitz-dom/src/node/element.rs:850 |
| blitz-dom | type | node/node/DisplayOuter# | packages/blitz-dom/src/node/node.rs:43 |
| blitz-dom | type | node/node/DisplayOuter#Block# | packages/blitz-dom/src/node/node.rs:44 |
| blitz-dom | type | node/node/DisplayOuter#Inline# | packages/blitz-dom/src/node/node.rs:45 |
| blitz-dom | type | node/node/DisplayOuter#None# | packages/blitz-dom/src/node/node.rs:46 |
| blitz-dom | fn | node/node/impl#[Node]element_state_mut(). | packages/blitz-dom/src/node/node.rs:172 |
| blitz-dom | fn | node/node/impl#[Node]snapshot_handled_mut(). | packages/blitz-dom/src/node/node.rs:173 |
| blitz-dom | fn | node/node/impl#[Node]selector_flags_mut(). | packages/blitz-dom/src/node/node.rs:177 |
| blitz-dom | fn | node/node/impl#[Node]cache(). | packages/blitz-dom/src/node/node.rs:246 |
| blitz-dom | fn | node/node/impl#[Node]pe_by_index(). | packages/blitz-dom/src/node/node.rs:451 |
| blitz-dom | fn | node/node/impl#[Node]flush_style_attribute(). | packages/blitz-dom/src/node/node.rs:1173 |
| blitz-dom | fn | node/node/impl#[Node]hit(). | packages/blitz-dom/src/node/node.rs:1287 |
| blitz-dom | fn | node/serialize/impl#[Node]outer_html_pretty(). | packages/blitz-dom/src/node/serialize.rs:88 |
| blitz-dom | fn | node/svg/impl#[SvgImageData]resolved_width(). | packages/blitz-dom/src/node/svg.rs:143 |
| blitz-dom | fn | node/svg/impl#[SvgImageData]resolved_height(). | packages/blitz-dom/src/node/svg.rs:154 |
| blitz-dom | fn | node/text/impl#[TextLayout]content_widths(). | packages/blitz-dom/src/node/text.rs:36 |
| blitz-dom | fn | query_selector/impl#[BaseDocument]query_selector_raw(). | packages/blitz-dom/src/query_selector.rs:91 |
| blitz-dom | fn | query_selector/impl#[BaseDocument]query_selector_all_raw(). | packages/blitz-dom/src/query_selector.rs:149 |
| blitz-dom | fn | query_selector/impl#[Node]query_selector_raw(). | packages/blitz-dom/src/query_selector.rs:221 |
| blitz-dom | fn | traversal/impl#[BaseDocument]non_anon_ancestor_if_anon(). | packages/blitz-dom/src/traversal.rs:122 |
| blitz-dom | fn | tree/impl#[NodeTree]is_empty(). | packages/blitz-dom/src/tree.rs:80 |
| blitz-dom | fn | util/ToColorColor#as_color_color(). | packages/blitz-dom/src/util.rs:168 |
| blitz-net | fn | impl#[Provider]count(). | packages/blitz-net/src/lib.rs:133 |
| blitz-net | fn | ReqwestExt#apply_body(). | packages/blitz-net/src/lib.rs:416 |
| blitz-paint | fn | color/ToColorColor#as_srgb_color(). | packages/blitz-paint/src/color.rs:8 |
| blitz-paint | fn | color/ToColorColor#as_dynamic_color(). | packages/blitz-paint/src/color.rs:11 |
| blitz-paint | term | kurbo_css/css_box/BuildBezpath#TOLERANCE. | packages/blitz-paint/src/kurbo_css/css_box.rs:587 |
| blitz-paint | fn | kurbo_css/css_box/BuildBezpath#insert_arc(). | packages/blitz-paint/src/kurbo_css/css_box.rs:588 |
| blitz-paint | fn | kurbo_css/css_box/BuildBezpath#insert_point(). | packages/blitz-paint/src/kurbo_css/css_box.rs:589 |
| blitz-paint | term | layers/LayerManager#layer_depth_used. | packages/blitz-paint/src/layers.rs:15 |
| blitz-shell | fn | net/data_uri_net_provider/impl#[DataUriNetProvider]shared(). | packages/blitz-shell/src/net.rs:44 |
| blitz-shell | fn | window/impl#[`View<Rend>`]theme_override(). | packages/blitz-shell/src/window.rs:260 |
| blitz-test-harness | fn | harness/impl#[`Harness<D>`]into_inner(). | packages/blitz-test-harness/src/harness.rs:103 |
| blitz-test-harness | fn | harness/impl#[`Harness<D>`]time(). | packages/blitz-test-harness/src/harness.rs:118 |
| blitz-test-harness | fn | harness/impl#[`Harness<D>`]tick(). | packages/blitz-test-harness/src/harness.rs:129 |
| blitz-test-harness | fn | harness/impl#[`Harness<D>`]set_viewport_size(). | packages/blitz-test-harness/src/harness.rs:182 |
| blitz-test-harness | fn | input/impl#[`Harness<D>`]mouse_down_at(). | packages/blitz-test-harness/src/input.rs:111 |
| blitz-test-harness | fn | input/impl#[`Harness<D>`]mouse_up_at(). | packages/blitz-test-harness/src/input.rs:117 |
| blitz-test-harness | fn | input/impl#[`Harness<D>`]drag(). | packages/blitz-test-harness/src/input.rs:138 |
| blitz-test-harness | fn | input/impl#[`Harness<D>`]tap(). | packages/blitz-test-harness/src/input.rs:152 |
| blitz-test-harness | fn | input/impl#[`Harness<D>`]touch_down(). | packages/blitz-test-harness/src/input.rs:166 |
| blitz-test-harness | fn | input/impl#[`Harness<D>`]touch_move(). | packages/blitz-test-harness/src/input.rs:172 |
| blitz-test-harness | fn | input/impl#[`Harness<D>`]touch_up(). | packages/blitz-test-harness/src/input.rs:178 |
| blitz-test-harness | fn | input/impl#[`Harness<D>`]ime(). | packages/blitz-test-harness/src/input.rs:228 |
| blitz-test-harness | fn | inspect/impl#[`Harness<D>`]hovered(). | packages/blitz-test-harness/src/inspect.rs:113 |
| blitz-traits | term | devtools/DevtoolSettings#element_picker. | packages/blitz-traits/src/devtools.rs:21 |
| blitz-traits | fn | events/impl#[EventState]propagation_is_stopped(). | packages/blitz-traits/src/events.rs:41 |
| blitz-traits | fn | events/impl#[EventState]merge(). | packages/blitz-traits/src/events.rs:51 |
| blitz-traits | fn | events/impl#[UiEvent]discriminant(). | packages/blitz-traits/src/events.rs:74 |
| blitz-traits | fn | events/impl#[DomEventData]kind(). | packages/blitz-traits/src/events.rs:312 |
| blitz-traits | fn | shell/ShellProvider#set_window_minimized(). | packages/blitz-traits/src/shell.rs:48 |
| blitz-traits | fn | shell/ShellProvider#set_window_maximized(). | packages/blitz-traits/src/shell.rs:51 |
| blitz-traits | fn | shell/ShellProvider#is_window_maximized(). | packages/blitz-traits/src/shell.rs:54 |
| blitz-traits | fn | shell/ShellProvider#set_window_decorations(). | packages/blitz-traits/src/shell.rs:57 |
| blitz-traits | fn | shell/ShellProvider#drag_window(). | packages/blitz-traits/src/shell.rs:62 |
| blitz-traits | fn | shell/impl#[Viewport]zoom(). | packages/blitz-traits/src/shell.rs:135 |
| blitz-vibey-script | fn | document/impl#[ScriptDocument]external_script_urls(). | packages/blitz-vibey-script/src/document.rs:218 |
| blitz-vibey-script | fn | dom/hyperlink/component_getter(). | packages/blitz-vibey-script/src/dom/hyperlink.rs:106 |
| blitz-vibey-script | fn | dom/hyperlink/component_setter(). | packages/blitz-vibey-script/src/dom/hyperlink.rs:148 |
| debug_timer | fn | dummy_debug_timer/impl#[DebugTimer]init(). | packages/debug_timer/src/lib.rs:74 |
| debug_timer | macro | debug_timer_type! | packages/debug_timer/src/lib.rs:119 |
| dioxus-native-dom | fn | events/impl#[NodeHandle]doc(). | packages/dioxus-native-dom/src/events.rs:150 |
| dioxus-native-dom | fn | events/impl#[NodeHandle]node_mut(). | packages/dioxus-native-dom/src/events.rs:171 |
| dioxus-native | fn | assets/impl#[DioxusNativeNetProvider]shared(). | packages/dioxus-native/src/assets.rs:12 |
| dioxus-native | fn | assets/impl#[DioxusNativeNetProvider]inner(). | packages/dioxus-native/src/assets.rs:36 |
| dioxus-native | fn | config/impl#[Config]with_font_ctx(). | packages/dioxus-native/src/config.rs:61 |
| dioxus-native | fn | dioxus_application/impl#[DioxusNativeApplication]add_window(). | packages/dioxus-native/src/dioxus_application.rs:54 |
| dioxus-native | fn | dioxus_renderer/impl#[DioxusNativeWindowRenderer]with_features_and_limits(). | packages/dioxus-native/src/dioxus_renderer.rs:66 |
| dioxus-native | fn | use_raw_window_handle(). | packages/dioxus-native/src/lib.rs:85 |

### B4 — unused dependencies (cargo-machete) · 25
cargo-machete reads source text, not resolution: a dependency kept only to pin a feature or a version (the four `idna_adapter` rows are the likely case) or reached only through a macro reads as unused. These are candidates.

| crate | dependency |
|---|---|
| rdme | blitz-paint |
| rdme | image |
| rdme | reqwest |
| counter | blitz-dom |
| counter | blitz-paint |
| counter | idna_adapter |
| seven_guis | idna_adapter |
| todomvc | idna_adapter |
| transparent | blitz-dom |
| transparent | blitz-paint |
| transparent | idna_adapter |
| wgpu_texture | blitz-paint |
| wgpu_texture | pollster |
| blitz-html | blitz-traits |
| blitz-net | http-cache |
| blitz-paint | euclid |
| blitz-vibey-script | boa_gc |
| dioxus-native | blitz-paint |
| dioxus-native | futures-util |
| dioxus-native | keyboard-types |
| dioxus-native | rustc-hash |
| wpt | markup5ever |
| wpt | pollster |
| wpt | style |
| wpt | taffy |

**Proposal:** a per-crate check of the non-pin rows (e.g. `rdme` → `blitz-paint`/`image`/`reqwest`, `wpt` → `markup5ever`/`style`/`taffy`, `dioxus-native` → `futures-util`/`keyboard-types`/`rustc-hash`). A confirmed unused dep is a cheaper build. A pin gets a `[package.metadata.cargo-machete] ignored` entry with its reason. Any change here touches the coupled dependency pins (CLAUDE.md) and is the founder's call.

### Starting tables

**Totals:** 59272 Rust code lines · 286 files · 28 units (27 members + the root `blitz-examples` package).

**Duplication** (jscpd): 3.961 % · 2935 duplicated / 74102 lines · 264 clones over 279 files · split src 154 pairs/1595 L · test 106/1288 · mixed 4/52

| first | second | lines |
|---|---|---|
| tests/blitz-tests/tests/device_coalescing.rs:5 | tests/blitz-tests/tests/resize_restyle.rs:5 | 43 |
| packages/blitz-traits/src/events.rs:534 | packages/blitz-traits/src/events.rs:585 | 37 |
| examples/counter/src/app.rs:80 | examples/transparent/src/app.rs:181 | 35 |
| tests/blitz-tests/tests/inline_svg_restyle.rs:17 | tests/blitz-tests/tests/inline_svg_serialize.rs:9 | 32 |
| packages/blitz-dom/src/layout/inline.rs:115 | packages/blitz-dom/src/layout/inline.rs:263 | 29 |
| tests/blitz-tests/tests/interaction_state_canonicalization.rs:32 | tests/blitz-tests/tests/interaction_state_teardown.rs:47 | 28 |
| tests/blitz-tests/tests/details_element.rs:41 | tests/blitz-tests/tests/fragment_navigation.rs:43 | 27 |
| examples/custom_widget.rs:28 | examples/wgpu_texture/src/dioxus_native.rs:27 | 25 |
| tests/blitz-tests/tests/scrollbar_drag.rs:9 | tests/blitz-tests/tests/stale_interaction_state.rs:25 | 24 |
| tests/blitz-tests/tests/br_trailing_line.rs:10 | tests/blitz-tests/tests/inline_box_baseline.rs:3 | 23 |

**Complexity** (rust-code-analysis, 4559 function spaces): cognitive p50 0.0 · p90 3.0 · cyclomatic p50 1.0 · p90 6.0 · max 313.0 · **74 over the cognitive ceiling (>15)** · function sloc p50 5.0 · p90 30.0 · max 854.0

| function | site | cognitive | cyclomatic |
|---|---|---|---|
| compute_inline_layout_inner | packages/blitz-dom/src/layout/inline.rs:195 | 140.0 | 93.0 |
| synthesize_presentational_hints_for_legacy_attributes | packages/blitz-dom/src/stylo.rs:870 | 135.0 | 106.0 |
| resolved_style_value | packages/blitz-dom/src/resolved_style.rs:341 | 104.0 | 87.0 |
| handle_pointermove | packages/blitz-dom/src/events/pointer.rs:206 | 77.0 | 36.0 |
| handle_click | packages/blitz-dom/src/events/pointer.rs:614 | 69.0 | 42.0 |
| apply_keypress_event | packages/blitz-dom/src/node/text.rs:195 | 66.0 | 48.0 |
| dispatch_child_layout | packages/blitz-dom/src/layout/mod.rs:104 | 64.0 | 57.0 |
| hit_inner | packages/blitz-dom/src/node/node.rs:1295 | 63.0 | 47.0 |
| compute_replaced_layout | packages/blitz-dom/src/layout/replaced.rs:52 | 57.0 | 55.0 |
| resolve_color_stops | packages/blitz-paint/src/gradient.rs:325 | 56.0 | 29.0 |

All ten top offenders are upstream engine code (blitz-dom layout, style, events; blitz-paint gradients). Epoch 1 touched two of their files by a few lines (`5dc809a1`: `layout/replaced.rs`, `node/node.rs`, about 4 lines between them).

**Sizes** (tokei): file p50 99 · p90 546 · max 2540 · 12 files over 800 code lines

| file | code lines |
|---|---|
| packages/blitz-dom/src/document.rs | 2540 |
| packages/blitz-vibey-script/src/runtime.rs | 2200 |
| packages/blitz-dom/src/mutator.rs | 1446 |
| packages/blitz-dom/src/node/node.rs | 1360 |
| packages/blitz-vibey-script/src/dom/element.rs | 1228 |
| packages/blitz-dom/src/stylo.rs | 1126 |
| packages/blitz-dom/src/layout/construct.rs | 1027 |
| packages/blitz-paint/src/render.rs | 993 |
| packages/blitz-dom/src/node/element.rs | 850 |
| packages/stylo_taffy/src/convert.rs | 827 |

**Graph:** 0 cycles · 88 cross-unit edges · fan-out top: blitz-examples 10 · blitz-tests 8 · browser 8 · dioxus-native 6 · blitz 6. Fan-in top 5: `blitz-traits node_id/NodeId#` 544 · `blitz-dom crate/` 445 · `blitz-dom document/impl#[BaseDocument]get_node().` 222 · `blitz-dom document/BaseDocument#` 198 · `blitz-dom document/BaseDocument#nodes.` 171 (20 in `c-graph.json`).

**Coverage** (`ci-leg.sh coverage`): line 53.74 % (18850/35077) · regions 52.95 % · functions 54.99 % · branch null (the leg does not instrument branches). A falling line figure is the signal; a rising one is not celebrated.

## Informational
- Churn and hotspots are not computed in baseline mode. They start at the next boundary, and churn stays informational until the ledger holds 2+ records.
- `code-graph.py` is versionless and recorded by blob hash `d0425fb1`. Any edit to it is a trend-break on graph and dead.
- The adoption trace `tree-query-code-audit.json` is 200 KB. Most of it is the 319-row dead-code residual pull that the classing reads.

## Below threshold — no action
- None: baseline mode has no deltas.

## Skips
- churn — no-baseline (no prior record)
- hotspots — no-baseline (needs churn)
- mutation:seven_guis — declined (231 mutants would exceed the 15-min per-unit cap and score nothing (overseer, under the founder's standing delegation))
