proposals:
  - detector: D-arch-decisions
    severity: warning
    section: Established Decisions → [Dependency pinning]
    change: 'Replace "taffy is a git dependency pinned by `rev` (Cargo.toml:106)" with: taffy is a registry version, "0.15" from crates.io (resolved 0.15.0, its eight features unchanged), and since 2026-10-13-upstream-sync-polish-and-ship no git dependency is left in Cargo.toml; the rest of the decision (coupled pins, winit exact beta, commented-out patch blocks) stands.'
    sidecar: '2026-10-13-upstream-sync-polish-and-ship: [Dependency pinning] taffy moves from a git dependency pinned by rev (0.14.0) to the crates.io registry "0.15" (0.15.0) with upstream 434e0661; no git dependency remains in the manifest.'
    rationale: 'report Dependencies: "taffy 0.14.0 → 0.15.0. Its source moves from git DioxusLabs/taffy?rev=7d33901c… to registry+https://github.com/rust-lang/crates.io-index … with its eight features byte for byte" and "No git dependency is left in Cargo.toml (grep -c DioxusLabs/taffy Cargo.toml → 0)"; Counts, pattern taffy-pin, names architecture.md:73.'
    basis: 'escher-0.1.0/chunks/2026-10-13-upstream-sync-polish-and-ship/report.md:71; Cargo.toml:106'
  - detector: D-arch-decisions
    severity: warning
    section: Stack and Technologies → Layout row
    change: 'Replace "taffy from git `https://github.com/DioxusLabs/taffy` at a pinned rev" with: taffy "0.15" from the crates.io registry (resolved 0.15.0), the same eight features std, flexbox, flexbox_balance, grid, block_layout, content_size, calc, detailed_layout_info.'
    sidecar: '2026-10-13-upstream-sync-polish-and-ship: Stack Layout row reads taffy registry 0.15 in place of a git rev.'
    rationale: 'Same retired claim as [Dependency pinning]: report Dependencies (taffy git 0.14.0 → registry 0.15.0) and Counts taffy-pin, architecture.md:48 ×2.'
    basis: 'escher-0.1.0/chunks/2026-10-13-upstream-sync-polish-and-ship/report.md:71; Cargo.toml:106'
    dependent-of: D-arch-decisions
  - detector: D-arch-decisions
    severity: warning
    section: Stack and Technologies → Workspace package row
    change: 'Workspace package version "0.3.0-beta.3" (was "0.3.0-beta.2") and the nine in-repo crates pinned `=0.3.0-beta.3` (blitz, blitz-dom, blitz-html, blitz-net, blitz-paint, blitz-vibey-script, blitz-shell, blitz-traits, stylo_taffy); the rest of the row unchanged.'
    sidecar: '2026-10-13-upstream-sync-polish-and-ship: workspace version and in-repo `=` pins move 0.3.0-beta.2 → 0.3.0-beta.3 with upstream 434e0661 (escher-driver and escher-telemetry follow by cargo update --workspace).'
    rationale: 'report Dependencies: "The workspace version moves 0.3.0-beta.2 → 0.3.0-beta.3, with its nine in-repo = pins (Cargo.toml pin-set count 11, gate green)"; Counts taffy-pin names architecture.md:45 ×2.'
    basis: 'escher-0.1.0/chunks/2026-10-13-upstream-sync-polish-and-ship/report.md:72-73; Cargo.toml:36; Cargo.toml:46-54'
  - detector: D-arch-decisions
    severity: warning
    section: Inherited Defaults → Framework
    change: 'Engine crates pinned `=0.3.0-beta.3` (was `=0.3.0-beta.2`) and taffy "0.15" from the registry (was "at a pinned git rev"); the other pins in the line unchanged.'
    sidecar: '2026-10-13-upstream-sync-polish-and-ship: Inherited Defaults Framework line reads engine pins =0.3.0-beta.3 and taffy registry 0.15.'
    rationale: 'Both retired claims (beta.2 pins; taffy git rev) are restated here: report Dependencies and Counts taffy-pin, architecture.md:219 ×2.'
    basis: 'escher-0.1.0/chunks/2026-10-13-upstream-sync-polish-and-ship/report.md:71-76'
    dependent-of: D-arch-decisions
  - detector: D-arch-decisions
    severity: warning
    section: Established Decisions → [DOM semantics]
    change: 'Retire "`node_id_mapping` entries are intentionally not cleared on removal so dioxus-core can reuse detached nodes (tests/blitz-tests/tests/stale_node_mapping.rs:4-7)"; state instead: since 2026-10-13-upstream-sync-polish-and-ship (upstream #1185) the Dioxus bridge drops a removed node and its descendants and clears their `element_id_mapping` / `node_id_mapping` entries (`remove_and_drop_node`, `replace_and_drop_node`, packages/dioxus-native-dom/src/mutation_writer.rs:110-130), so a re-render-removed node no longer resolves — measured by `dioxus_document::tests::removed_nodes_are_dropped` and `element_id::tests::a_removed_element_reads_no_id` (packages/dioxus-native-dom/src/dioxus_document.rs:496-530).'
    sidecar: '2026-10-13-upstream-sync-polish-and-ship: [DOM semantics] the "mappings not cleared on removal" decision is retired — upstream #1185 drops a removed node and clears its mappings.'
    rationale: 'report Spec claims disproved: "architecture.md:91 ([DOM semantics]): node_id_mapping entries are intentionally not cleared on removal … It is false on the merged engine. Upstream #1185 remove_and_drop_node (mutation_writer.rs:110-124) drops the removed node and its descendants and clears their mappings"; Symbols, crate-private arrivals.'
    basis: 'escher-0.1.0/chunks/2026-10-13-upstream-sync-polish-and-ship/report.md:124-126; packages/dioxus-native-dom/src/mutation_writer.rs:110-124'
  - detector: D-arch-decisions
    severity: warning
    section: Standard Contracts → Dioxus DOM bridge
    change: 'Add the bridge removal contract: removing a node through the mutation writer drops it and its descendants and clears their `element_id_mapping` / `node_id_mapping` entries (`remove_and_drop_node`, `replace_and_drop_node`, crate-private, packages/dioxus-native-dom/src/mutation_writer.rs:110-130; upstream #1185, merged 2026-10-13-upstream-sync-polish-and-ship), so `element_id(removed)` reads `None` by the node no longer resolving rather than by detachment.'
    sidecar: '2026-10-13-upstream-sync-polish-and-ship: Dioxus DOM bridge contract registers the drop-on-removal behaviour of upstream #1185.'
    rationale: 'The contract is where the mechanism retired at [DOM semantics] is restated as bridge behaviour; report Expected amendments: "§Standard Contracts → Dioxus DOM bridge — … a removed node dropped — carried"; Symbols: a_removed_element_reads_no_id now asserts get_node(removed) is None, the detached-parent assertion gone.'
    basis: 'escher-0.1.0/chunks/2026-10-13-upstream-sync-polish-and-ship/report.md:37-39; escher-0.1.0/chunks/2026-10-13-upstream-sync-polish-and-ship/report.md:50'
    dependent-of: D-arch-decisions
  - detector: D-arch-decisions
    severity: warning
    section: Established Decisions → [Dioxus integration]
    change: 'Replace "`DEFAULT_CSS` always added to a `DioxusDocument` (packages/dioxus-native-dom/src/dioxus_document.rs:97-98)" with: `DioxusDocument::new` adds no stylesheet of its own since 2026-10-13-upstream-sync-polish-and-ship (upstream #1189); the default user-agent sheet reaches it once through `DocumentConfig::default()` (`vec![UserAgentStylesheet::default_sheet()]`, packages/blitz-dom/src/config.rs:73-91), and a config that sets `ua_stylesheets` is used as given (packages/blitz-dom/src/config.rs:37-41).'
    sidecar: '2026-10-13-upstream-sync-polish-and-ship: [Dioxus integration] DEFAULT_CSS is no longer added by DioxusDocument::new; the default sheet comes from DocumentConfig::default() alone.'
    rationale: 'report Spec claims disproved: "architecture.md:98: DEFAULT_CSS always added to a DioxusDocument (pattern ua-css). The call is removed (#1189). The default sheet now reaches a DioxusDocument through DocumentConfig::default() alone, once."'
    basis: 'escher-0.1.0/chunks/2026-10-13-upstream-sync-polish-and-ship/report.md:129-131; packages/blitz-dom/src/config.rs:73-91'
  - detector: D-arch-decisions
    severity: warning
    section: Established Decisions → [HTML parsing]
    change: 'Replace "`HtmlDocument` appends `DEFAULT_CSS` to the UA stylesheets if absent (packages/blitz-html/src/html_document.rs:59-63)" with: `HtmlDocument` uses the configured `ua_stylesheets` as given — the default sheet arrives through `DocumentConfig::default()` (packages/blitz-dom/src/config.rs:73-91), upstream #1189, since 2026-10-13-upstream-sync-polish-and-ship.'
    sidecar: '2026-10-13-upstream-sync-polish-and-ship: [HTML parsing] HtmlDocument no longer appends DEFAULT_CSS to a configured list.'
    rationale: 'Same retired mechanism as [Dioxus integration]; report Spec claims disproved names architecture.md:92 ("the append is removed") and Symbols, upstream removed: "HtmlDocument no longer appends DEFAULT_CSS to a configured list".'
    basis: 'escher-0.1.0/chunks/2026-10-13-upstream-sync-polish-and-ship/report.md:58-59; escher-0.1.0/chunks/2026-10-13-upstream-sync-polish-and-ship/report.md:130'
    dependent-of: D-arch-decisions
  - detector: D-arch-resources
    severity: warning
    section: Standard Contracts → Document core (blitz-dom)
    change: 'Register the user-agent stylesheet API: `DocumentConfig.ua_stylesheets` is `Vec<UserAgentStylesheet>` (was `Option<Vec<String>>`; packages/blitz-dom/src/config.rs:37-41) with a hand-written `impl Default for DocumentConfig` defaulting it to `vec![UserAgentStylesheet::default_sheet()]` (packages/blitz-dom/src/config.rs:73-91); public `blitz_dom::UserAgentStylesheet` (packages/blitz-dom/src/document.rs:80) with `parse(&str)` (:83-97), `default_sheet()` — a `LazyLock` parse of `DEFAULT_CSS` — (:99-104) and `sheet()` (:106-108), re-exported at packages/blitz-dom/src/lib.rs:96-98; `BaseDocument::add_user_agent_stylesheet(UserAgentStylesheet)` (document.rs:1244) and `remove_user_agent_stylesheet(&UserAgentStylesheet)` (document.rs:1219) replace the `&str` forms, and the field `BaseDocument::ua_stylesheets` is gone; escher uses none of them beyond the two harness `DocumentConfig { …, ..Default::default() }` sites.'
    sidecar: '2026-10-13-upstream-sync-polish-and-ship: Document core contract registers UserAgentStylesheet, the typed add/remove_user_agent_stylesheet and DocumentConfig.ua_stylesheets as Vec<UserAgentStylesheet> with a hand-written Default.'
    rationale: 'report Symbols / APIs, upstream arriving (public): UserAgentStylesheet, add/remove_user_agent_stylesheet replacing the &str forms, BaseDocument::ua_stylesheets removed, DocumentConfig.ua_stylesheets now Vec<UserAgentStylesheet>; Schema / config bullet; the contract still lists ua_stylesheets with no type and none of the new API.'
    basis: 'escher-0.1.0/chunks/2026-10-13-upstream-sync-polish-and-ship/report.md:42-45; escher-0.1.0/chunks/2026-10-13-upstream-sync-polish-and-ship/report.md:79'
  - detector: D-arch-resources
    severity: warning
    section: Standard Contracts → Events
    change: 'Add the pointer-leave event: `UiEvent::PointerLeave` (packages/blitz-traits/src/events.rs:67-68) with `DomEventKind::PointerLeave`, handled by `EventDriver::handle_pointer_leave` (packages/blitz-dom/src/events/driver.rs:65-74), which clears hover — in a hosted sub-document too — and fires the leave events (tests/blitz-tests/tests/pointer_leave.rs:28-52); arrived from upstream at 2026-10-13-upstream-sync-polish-and-ship, unused by escher.'
    sidecar: '2026-10-13-upstream-sync-polish-and-ship: Events contract registers UiEvent::PointerLeave / DomEventKind::PointerLeave and EventDriver::handle_pointer_leave.'
    rationale: 'report Symbols / APIs, upstream arriving (public): "UiEvent::PointerLeave (blitz-traits/src/events.rs:67-68) with DomEventKind::PointerLeave" and "EventDriver::handle_pointer_leave (events/driver.rs:65-74)"; Coverage: a shell-originated input event, tests integ pointer_leave.rs (7). The Events contract lists UiEvent variants and does not name it — a new event unregistered.'
    basis: 'escher-0.1.0/chunks/2026-10-13-upstream-sync-polish-and-ship/report.md:47-48; escher-0.1.0/chunks/2026-10-13-upstream-sync-polish-and-ship/report.md:152'
  - detector: D-arch-resources
    severity: warning
    section: Standard Contracts → Dioxus DOM bridge
    change: 'Replace "one of the 27 boolean attributes of the module-private `BOOLEAN_ATTRIBUTES` list — … (packages/dioxus-native-dom/src/mutation_writer.rs:11-43; …:439)" with: one of the 28 names of upstream private predicate `fn is_bool_attr` (a `matches!`, packages/dioxus-native-dom/src/mutation_writer.rs:439-475) — the former 27 plus `inert` — our `BOOLEAN_ATTRIBUTES` constant retired at 2026-10-13-upstream-sync-polish-and-ship, `mutation_writer.rs` byte-identical to upstream at 434e0661; the falsy values, both attribute paths and the `hidden` consequence unchanged, and a falsy `inert` is now cleared.'
    sidecar: '2026-10-13-upstream-sync-polish-and-ship: Dioxus DOM bridge falsy clear reads upstream is_bool_attr (28 names, inert added); BOOLEAN_ATTRIBUTES removed, mutation_writer.rs de-forked.'
    rationale: 'report Symbols, ours removed: BOOLEAN_ATTRIBUTES (27) removed, "The falsy clear now reads upstream private fn is_bool_attr (mutation_writer.rs:439-475, a matches! over 28 names)", file byte-identical to upstream; Counts falsy-attrs names architecture.md:136 ×5; Decisions: full de-fork. The contract names a symbol that no longer exists.'
    basis: 'escher-0.1.0/chunks/2026-10-13-upstream-sync-polish-and-ship/report.md:34-36; escher-0.1.0/chunks/2026-10-13-upstream-sync-polish-and-ship/report.md:92-93'
  - detector: D-arch-resources
    severity: warning
    section: Existing Scopes → blitz-tests row
    change: 'Replace "the 27-name list on both attribute paths" with "the 28-name list (upstream `is_bool_attr`, `inert` added) on both attribute paths".'
    sidecar: '2026-10-13-upstream-sync-polish-and-ship: blitz-tests scope row reads the 28-name falsy list.'
    rationale: 'Same retired claim (27 names, our list) restated: report Counts falsy-attrs, architecture.md:263 ×4; dioxus_falsy_boolean_attrs and dioxus_falsy_disabled pass on the merged engine.'
    basis: 'escher-0.1.0/chunks/2026-10-13-upstream-sync-polish-and-ship/report.md:92-93'
    dependent-of: D-arch-resources
  - detector: D-arch-resources
    severity: warning
    section: Standard Contracts → Dioxus DOM bridge
    change: 'Register the public re-export `dioxus_native_dom::synthetic_form_event` (packages/dioxus-native-dom/src/events.rs:666-674), exported beside `NodeHandle` and `synthetic_click_event` (packages/dioxus-native-dom/src/lib.rs:32); upstream, arrived at 2026-10-13-upstream-sync-polish-and-ship, unused by escher and untested in this repo.'
    sidecar: '2026-10-13-upstream-sync-polish-and-ship: Dioxus DOM bridge contract registers the public synthetic_form_event.'
    rationale: 'report Symbols, upstream arriving (public): "dioxus_native_dom::synthetic_form_event (events.rs:666-674)"; ours, re-export kept: lib.rs:32 now reads pub use events::{NodeHandle, synthetic_click_event, synthetic_form_event}; Coverage: tests ✗ in this repo. Not named in the contract.'
    basis: 'escher-0.1.0/chunks/2026-10-13-upstream-sync-polish-and-ship/report.md:40; escher-0.1.0/chunks/2026-10-13-upstream-sync-polish-and-ship/report.md:46'
  - detector: D-arch-resources
    severity: warning
    section: Inherited Defaults → Publishability
    change: 'Add blitz-vibey-script to the crates that set `publish = false` (packages/blitz-vibey-script/Cargo.toml:12, upstream at 2026-10-13-upstream-sync-polish-and-ship), and to the exceptions of "the published `packages/` library crates (not blitz-test-harness, escher-telemetry or escher-driver)".'
    sidecar: '2026-10-13-upstream-sync-polish-and-ship: blitz-vibey-script is publish = false.'
    rationale: 'report Crates / modules: "blitz-vibey-script gains publish = false"; the Publishability line lists the publish = false crates and the published packages/ set without it.'
    basis: 'escher-0.1.0/chunks/2026-10-13-upstream-sync-polish-and-ship/report.md:67; packages/blitz-vibey-script/Cargo.toml:12'
note: 'Outside both detectors and left for the orchestrator to raise: the Project Intent "Upstream sync:" line (architecture.md:205, 7832c177/9462a7e4 → 434e0661/a6c24d24) and the Front page item (architecture.md:210, README now "the eleven driver commands as tools"); the 7832c177 mentions at architecture.md:87 are dated history (refs of flagged-upstreamable diffs) and stand. No port, socket, listener, env var or workspace crate was added (report :64-66); the five new blitz-tests integration targets and the cursor-icon dev-dependency are not crates/packages. Upstream crate-private arrivals (hover_state, refresh_anonymous_block_styles, is_button_input, button_input_display, effective_display, view_box_transform, svg_viewport_affine) are classed crate-private by the report and not proposed for the contract registry.'
