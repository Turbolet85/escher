# Apply — architecture · part 4 of 4

## Verdicts
- fanout-architecture--cascade-g9.md 1 · applied · .andromeda/architecture-detail/occupied-resources.md:8#1, .andromeda/architecture-detail/occupied-resources.md:8#2
- fanout-architecture--cascade-g9.md 2 · applied · .andromeda/architecture-detail/occupied-resources.md:17+#1
- fanout-architecture--cascade-g9.md 4 · applied · .andromeda/architecture-detail/occupied-resources.md:78+#1
- fanout-architecture--cascade-g9.md 5 · applied · .andromeda/architecture-detail/occupied-resources.md:110#1
- fanout-architecture--cascade-g9.md 6 · applied · .andromeda/architecture-detail/cross-cutting-patterns.md:35+#1
- fanout-architecture--cascade-g9.md 7 · applied · .andromeda/architecture-detail/cross-cutting-patterns.md:44+#1
- cascade-compare-architecture.md 1 · applied · .andromeda/architecture-detail/cross-cutting-patterns.md:27+#1
- fanout-architecture--cascade-g9.md 8 · applied · .andromeda/architecture-detail/project-intent.md:9#1, .andromeda/architecture-detail/project-intent.md:9#2, .andromeda/architecture-detail/project-intent.md:9+#1
- fanout-architecture--cascade-g9.md 10 · applied · .andromeda/architecture-detail/project-intent.md:11+#1
- fanout-architecture.md 9 · refused · .andromeda/architecture-detail/inherited-defaults.md holds no §Inherited Defaults → API style row and no «no network listener»: its three rows are Publishability (line 8), Code quality (line 14) and Optional capabilities (line 20), so the place the proposal names is not in this file

## Edits
- line: .andromeda/architecture-detail/occupied-resources.md:8
  old: «no network port — no TCP or UDP port;»
  new: «no served network port — no TCP or UDP port a binary serves;»
- line: .andromeda/architecture-detail/occupied-resources.md:8
  old: «(beside it, since 2026-10-14-quality-gates, one test-only bind:»
  new: «(beside it, three test-only binds: blitz-net's two below and this one:»
- after: .andromeda/architecture-detail/occupied-resources.md:17
  anchor: «- A shot of an open select list also carries that list's option labels and group `label`s, a page's text as painted pixels, on both carriers, and a row's own `bounds` cross in the snapshot and the diff (`differs` naming `bounds`, a closed set inside the ratified crossing); no key, node key, field, carrier, listener or port is added; the pixels are a boundary widening ratified (as measured at escher-0.3.0/chunks/2026-10-31-select-popup/report.md).»
  add: «- `blitz-net`'s `#[cfg(test)] mod tests` holds two test-only TCP binds on loopback, `std::net::TcpListener::bind("127.0.0.1:0")` once each in `delivers_network_and_bad_url_errors` and `http_error_body_is_not_delivered_as_css`, served by no binary and reached by no caller, so no crossing and no served port (packages/blitz-net/src/lib.rs:526; packages/blitz-net/src/lib.rs:534; as measured at escher-0.3.0/chunks/2026-11-01-upstream-sync-ui-graph/evidence/sink.md)»
- after: .andromeda/architecture-detail/occupied-resources.md:78
  anchor: «- `static PARALLEL_TRAVERSAL_LOCK: Mutex<()>` in blitz-dom's `stylo.rs`, process-wide: taken around every `StyleThreading::Parallel` traversal, so two documents resolving `Parallel` at once on Stylo's global rayon pool are serialised and no longer panic `already mutably borrowed` (blitz issue 430); its poison recovered by `unwrap_or_else(|e| e.into_inner())`, with no panic path and no log site; `Sequential` stays the default, and `Parallel` is set by the windowed `dioxus_native::launch` (`seven_guis_native`) and the browser's document loader and toolbar, by no headless document — harness, stand or driver (packages/blitz-dom/src/stylo.rs:29-35; packages/blitz-dom/src/stylo.rs:157-163; packages/dioxus-native/src/lib.rs:63; packages/dioxus-native/src/lib.rs:222; apps/browser/src/document_loader.rs:62; apps/browser/src/toolbar.rs:203).»
  add: «- The `Once` `INIT` in blitz-dom's `pub(crate) fn init_stylo_prefs`, process-wide: the first call, from `UserAgentStylesheet::parse` or `BaseDocument::new`, sets Stylo's prefs under their Stylo 0.22 names before the user-agent sheet is parsed, and a later call does nothing; no log site (packages/blitz-dom/src/document.rs:87-102; as measured at escher-0.3.0/chunks/2026-11-01-upstream-sync-ui-graph/report.md)»
- line: .andromeda/architecture-detail/occupied-resources.md:110
  old: «A `jlumbroso/free-disk-space` step also stands in the `matrix_test` job, skipped on every matrix row: its `if:` names `ubuntu-24.04` and `ubuntu-24.04-arm` and the android row runs `ubuntu-latest`, read `completed · skipped` on `Test (android)` in CI#41190771843, so the `Stand checks repeated` step is this workflow's one measured run of the action (.github/workflows/ci.yml:392-394; as measured at escher-0.3.0/chunks/2026-10-31-engine-debts/evidence/ci.md)»
  new: «The `matrix_test` job holds no `jlumbroso/free-disk-space` step, `Test (android)` reading no such step in CI#41333172440, so the `Stand checks repeated` step is this workflow's one run of the action, pinned by `test_only_stand_repeat_frees_disk` (.github/scripts/test_ci_workflows.py:110-115; as measured at escher-0.3.0/chunks/2026-11-01-upstream-sync-ui-graph/evidence/ci.md)»
- after: .andromeda/architecture-detail/cross-cutting-patterns.md:27
  anchor: «- A text input that holds focus before its editor exists has the shell provider's `set_ime_enabled(true)` called when `create_text_editor` constructs that editor (packages/blitz-dom/src/layout/construct.rs:878; packages/blitz-dom/src/layout/construct.rs:885-890); the route a windowed shell's redraw request takes, through its event loop, is §Standard Contracts → Paint and shell's»
  add: «- A stylesheet that fails, comes back empty or is not CSS stops holding the redraw skip for pending critical resources, on every document: `ResourceHandler<StylesheetHandler>::error` responds `Err(error)`, so `pending_critical_resources` loses the request and it stops blocking render (packages/blitz-dom/src/net.rs:144-146); `NetHandler::error`, the trait method it answers, is §Standard Contracts' and the render-blocking rule §Established Decisions'»
- after: .andromeda/architecture-detail/cross-cutting-patterns.md:35
  anchor: «<!-- /detail -->»
  add: «»
  add: «<!-- detail: values · crosscut-event-processing -->»
  add: «»
  add: «- A queued `mounted` event whose element was removed since it was queued is skipped: `flush_queued_mounted_events` reads the element through `try_element_to_node_id` (packages/dioxus-native-dom/src/dioxus_document.rs:235-239)»
  add: «»
  add: «<!-- /detail -->»
- after: .andromeda/architecture-detail/cross-cutting-patterns.md:44
  anchor: «<!-- /detail -->»
  add: «»
  add: «<!-- detail: values · crosscut-invalidation-and-state-integrity -->»
  add: «»
  add: «- `set_attribute`, `clear_attribute`, removal and insertion restyle through `DocumentMutator::restyle_for_attribute_change` and `restyle_for_child_list_change`, no blanket `RestyleHint::restyle_subtree()`; the changed-set insert in `remove_and_drop_all_children` stays first, and `append_text_to_node` also damages the parent (packages/blitz-dom/src/mutator.rs:689-735; packages/blitz-dom/src/mutator.rs:737-807; packages/blitz-dom/src/mutator.rs:257-259; as measured at escher-0.3.0/chunks/2026-11-01-upstream-sync-ui-graph/report.md)»
  add: «»
  add: «<!-- /detail -->»
- line: .andromeda/architecture-detail/project-intent.md:9
  old: «last read at `ebacd68bd`, twenty-four of its commits taken»
  new: «last read at `e42ee1f25`, 57 commits past the merge base, thirty-six of them taken»
- line: .andromeda/architecture-detail/project-intent.md:9
  old: «(#1264, the flake's toolchain), then eleven together (`e795a088c`): `d63ed2c8c` (browser tab lenses kept across a webview props replacement) · `6587e7e79` (the windowed shell's redraw requests through its event loop) · `e6f4615ec` (blitz-vibey-script's runtime and element bindings in child modules) · `a7ea9631b` · `b5b163450` (publish-browser) · `7fc9ee3af` (IME enabled at a focused input's editor construction) · `2963b65a2` (`::before`/`::after` voting in flow classification) · `cee6b8731` (a `file:` script load only for a `file:` document) · `53e9d81c6` (`:open`) · `6fee0db25` (the WPT pin) · `efdc5472a` (CSS properties as `CSSStyleDeclaration` accessors); sixteen left out, each with its measured reason, among them git Stylo 0.23-pre (`06ecec886`, with `3954ecb85` and `1c5265a8b`), the six Taffy-main commits (`b8f4ba6fd` and `9babd796a`, a git Taffy, with `1a11761bc` · `4a5ab9cc9` · `510b592ab` · `1e5479ae6`) and Dioxus 0.8.0-alpha.3 (`286874cc1`), which wait for escher's next version, owned by a `.andromeda/residuals.md` line (target: next), so Stylo stays "0.22.0", Taffy the registry "0.15" and Dioxus "0.7.3", with no git dependency (as measured at escher-0.3.0/chunks/2026-10-27-upstream-sync-engine-debts/evidence/lock.md)»
  new: «(#1264, the flake's toolchain),»
- after: .andromeda/architecture-detail/project-intent.md:9
  anchor: «Upstream sync: DioxusLabs/blitz `main` last merged at 23b9b26a9b6af5947b8f003d2c2cc768bff10b8a (merge commit 3eda07bc, parents `143b6013` · `23b9b26a9`, 2026-10-17) — the next sync's merge base, unmoved; upstream `main` last read at `ebacd68bd`, twenty-four of its commits taken by no-commit cherry-picks of upstream's text, each with its `(cherry picked from commit …)` trailer: `fb8d6c15e` (the default cursor over no box, upstream #1220) alone (`275950dc0`), then `38ca4e777` (`:focus-within`, upstream #1226) and `17de049b1` (the WPT pin, upstream #1228) together (`79603d1ef`), then ten together (`34f6f051b`): `28f848d3b` (#1233, the inline layout module split) · `9eeebb302` (#1239, mimalloc in the browser and the WPT runner) · `e4ca5b9b5` (#1240, class atoms on `ElementData`) · `46d7cf993` (#1238, out-of-flow-ness on `Node`) · `fb297a38c` (#1243, `Parallel` style traversals serialised; `examples/multi_doc.rs`) · `50bf2f235` (#1246, a replaced element's natural height through `aspect-ratio`) · `f8723f495` (#1247, WPT runner) · `21edc92d9` (#1258) · `f102663b8` (#1244, dropped nodes skipped in hit testing) · `f9cbb2756` (#1264, the flake's toolchain), then eleven together (`e795a088c`): `d63ed2c8c` (browser tab lenses kept across a webview props replacement) · `6587e7e79` (the windowed shell's redraw requests through its event loop) · `e6f4615ec` (blitz-vibey-script's runtime and element bindings in child modules) · `a7ea9631b` · `b5b163450` (publish-browser) · `7fc9ee3af` (IME enabled at a focused input's editor construction) · `2963b65a2` (`::before`/`::after` voting in flow classification) · `cee6b8731` (a `file:` script load only for a `file:` document) · `53e9d81c6` (`:open`) · `6fee0db25` (the WPT pin) · `efdc5472a` (CSS properties as `CSSStyleDeclaration` accessors); sixteen left out, each with its measured reason, among them git Stylo 0.23-pre (`06ecec886`, with `3954ecb85` and `1c5265a8b`), the six Taffy-main commits (`b8f4ba6fd` and `9babd796a`, a git Taffy, with `1a11761bc` · `4a5ab9cc9` · `510b592ab` · `1e5479ae6`) and Dioxus 0.8.0-alpha.3 (`286874cc1`), which wait for escher's next version, owned by a `.andromeda/residuals.md` line (target: next), so Stylo stays "0.22.0", Taffy the registry "0.15" and Dioxus "0.7.3", with no git dependency (as measured at escher-0.3.0/chunks/2026-10-27-upstream-sync-engine-debts/evidence/lock.md)»
  add: « then eleven together (`e795a088c`): `d63ed2c8c` (browser tab lenses kept across a webview props replacement) · `6587e7e79` (the windowed shell's redraw requests through its event loop) · `e6f4615ec` (blitz-vibey-script's runtime and element bindings in child modules) · `a7ea9631b` · `b5b163450` (publish-browser) · `7fc9ee3af` (IME enabled at a focused input's editor construction) · `2963b65a2` (`::before`/`::after` voting in flow classification) · `cee6b8731` (a `file:` script load only for a `file:` document) · `53e9d81c6` (`:open`) · `6fee0db25` (the WPT pin) · `efdc5472a` (CSS properties as `CSSStyleDeclaration` accessors), then twelve together (`45c333f16`): `fe33b778b` (#1276, restyle narrowing) · `8b1a0a3ce` (#1312, table damage) · `dee854c76` (#1317, `<picture>`) · `f6edb9701` (#1318, failed stylesheets) · `6fca6308d` (#1316, blend and isolation) · `7a3dd8918` (#1310, prefs before the UA sheet) · `91ef65e21` (#1314, `URLSearchParams`) · `a26c6ddeb` (#1322, `CustomWidget` attributes) · `f76777766` (#1319, inline decorations) · `6ef4b532d` (#1305, mounted events) · `ae774d7f4` (#1339, computed sizes) · `e42ee1f25` (#1342, culling bounds, a port);»
  add: « twenty-one left out, each with its measured reason: `22f0cd000` · `ac6f9f582` · `e5929f9e2` · `0674e8efa` · `784a567e4` and sixteen others, among them git Stylo 0.23-pre (`06ecec886`, with `3954ecb85` and `1c5265a8b`), the six Taffy-main commits (`b8f4ba6fd` and `9babd796a`, a git Taffy, with `1a11761bc` · `4a5ab9cc9` · `510b592ab` · `1e5479ae6`) and Dioxus 0.8.0-alpha.3 (`286874cc1`), which wait for escher's next version, owned by a `.andromeda/residuals.md` line (target: next), so Stylo stays "0.22.0", Taffy the registry "0.15" and Dioxus "0.7.3", with no git dependency (as measured at escher-0.3.0/chunks/2026-10-27-upstream-sync-engine-debts/evidence/lock.md and escher-0.3.0/chunks/2026-11-01-upstream-sync-ui-graph/evidence/lock.md)»
- after: .andromeda/architecture-detail/project-intent.md:11
  anchor: «<!-- /detail -->»
  add: «»
  add: «<!-- detail: values · intent -->»
  add: «»
  add: «- Upstream's own tree at `e42ee1f25` panics on a key press with focus on a control whose anonymous block was dropped, at `packages/blitz-dom/src/node/node.rs:1740:27`, focused by `Tab` (as measured at escher-0.3.0/chunks/2026-11-01-upstream-sync-ui-graph/evidence/panic-reading.md)»
  add: «»
  add: «<!-- /detail -->»

## Patterns
[[pattern]]
id = "arch4-01"
fixed = 'no network port — no TCP or UDP port'

[[pattern]]
id = "arch4-02"
fixed = 'one test-only bind'

[[pattern]]
id = "arch4-03"
fixed = 'step also stands in the `matrix_test` job'

[[pattern]]
id = "arch4-04"
fixed = 'twenty-four of its commits taken'

[[pattern]]
id = "arch4-05"
fixed = 'sixteen left out, each with its measured reason'
