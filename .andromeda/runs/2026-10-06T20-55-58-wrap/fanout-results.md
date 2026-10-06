# Fan-out results — 2026-10-06-id-stability-across-code-edits

Seven doc-agents, one batch; detectors 15 over 7 docs (arch 2 · security-plan 3 · design-system 1 · layout-templates 1 · test-plan 3 · obs-plan 3 · a11y-plan 2). Each list below is the agent's return, copied by script from its hand-back (0 HTML entities before and after decoding).

## architecture

Verdict: 13 proposals (D-arch-resources 13 · D-arch-decisions 0); 3 comment lines stripped — D-arch-decisions reads no drift, and a note that eight proposals are coordinates only.

```yaml
proposals:
  - detector: D-arch-resources
    severity: warning
    section: "§Standard Contracts → Dioxus DOM bridge"
    change: >-
      Register the new public API, after the `snapshot` clause: under the `accessibility` feature `DioxusDocument::unkeyed_actionable(&self) -> Vec<UnkeyedActionable>` returns every actionable element whose stable id holds a `/` (a positional path of any path tier), in document pre-order, empty when every actionable element reads its author key; read-only, no argument, logs nothing, never panics (a node that does not resolve through `BaseDocument::get_node` is skipped). `UnkeyedActionable { node: NodeId, id: String, tag: String, role: Option<accesskit::Role>, focusable: bool, interactive_role: bool, listener: bool }` (`Debug`, `Clone`, `PartialEq`) with `remedy() -> String`; `role` is `None` when the accessibility tree leaves the element out; no field or text carries the element's text content, accessible name or an attribute value. Actionable = any of three existing readers: `Node::is_focussable()`, the accessibility node's role in a closed interactive-role list, or the `data-dioxus-id` attribute (written for every listener but `onmounted`); ids come from `element_ids()` and roles from `Document::accessibility_tree`, no second computation. Re-exported as `dioxus_native_dom::UnkeyedActionable` and through dioxus-native's `pub use dioxus_native_dom::*`; no wire form, and no driver, CLI or MCP command exposes it yet. Ratified by the founder (2026-10-06). Cite packages/dioxus-native-dom/src/actionable.rs:15-44, :55-89, :91-122 and packages/dioxus-native-dom/src/lib.rs:14-15, :23-24; "as measured at escher-0.1.0/chunks/2026-10-06-id-stability-across-code-edits/report.md".
    sidecar: >-
      Dioxus DOM bridge: registers `DioxusDocument::unkeyed_actionable() -> Vec<UnkeyedActionable>` and `UnkeyedActionable` (with `remedy()`), read-only, under the `accessibility` feature; the actionable rule is three existing readers (focusability, interactive role, `data-dioxus-id`); founder-ratified.
    rationale: >-
      Report Changes → Symbols / APIs "New public API" lists the function, the type, the three-reader rule and the re-exports; Expected amendments records the sweep `unkeyed_actionable|UnkeyedActionable|actionable` over the masters at 0 hits — "the contract is unregistered today". An unregistered public API on a registered contract is D-arch-resources drift.
    basis: "packages/dioxus-native-dom/src/actionable.rs:55 (fn), :16 (struct), :38 (remedy); packages/dioxus-native-dom/src/lib.rs:14-15, 23-24; architecture.md:134 holds no `actionable` token"

  - detector: D-arch-resources
    severity: warning
    section: "§Standard Contracts → Dioxus DOM bridge"
    change: >-
      The id grammar reads FOUR tiers, not three: (1) the author key, unchanged — else (2) the NEW anchored path, for an element under an element of its own component instance that reads an author key: `{key}//{segment}`, then `/{segment}` per further DOM level (`box//p:0/b:0`), the nearest such keyed element anchoring, a segment still `{tag}[{key}]` or `{tag}:{n}` — else (3) the component path, unchanged in form — else (4) the document path, unchanged. The sentence "a template root owned by its DOM parent's owner (a list row, an `if` branch) appends to the parent's path" gains: when that parent reads its author key the base its children extend is `{key}/`, not the parent's own path; "one owned by another component restarts at that component's chain" stays, so the anchor acts inside one component only (`TaskShell/Counter/div:0` is unchanged under the keyed `main#task-body`). An `id` failing the author-key predicate (empty, `/`-bearing, a later duplicate) anchors nothing. Ids stay pairwise distinct: an author key holds no `/`, an anchored path is the only id holding `//`. Ratified by the founder (2026-10-06): both rules, the `key//segment` spelling, the one-component reach. Re-point this clause's citations by the report's line map: element_id.rs `153-241` → `159-255`, `38-52` → `44-58`, `72-127` → `78-133`, `184-229` → `198-243`, `356-391` → `370-405`; dioxus_document.rs `185-208` → `185-217`, `387-393` → `396-402`, `288-304` → `297-313`; lib.rs `23-24` → `28-29` (the snapshot re-export); stand_element_ids.rs `124-235` → `129-240`.
    sidecar: >-
      Dioxus DOM bridge: the stable-id grammar was three tiers (author key · component path · document path); now four, with the anchored path `{key}//{segment}` for an element under a keyed element of its own component instance; founder-ratified; 10 citations on the line re-pointed by the measured line map.
    rationale: >-
      Report Changes → Symbols / APIs "The id grammar changed": four tiers in order of precedence, the anchored tier NEW, mechanism at element_id.rs:181-187, one-component reach at :231-234, three-way uniqueness argument. architecture.md:134 still states "(1) the author key … else (2) the component path … else (3) the document path" and the unconditional "appends to the parent's path". Expected amendments carries this exact site (sweep `component path|\{tag\}|document path` → architecture.md:134).
    basis: "architecture.md:134 (the `(1) … (2) … (3)` clause); report.md:14-20, :46, :50, :56, :67"

  - detector: D-arch-resources
    severity: warning
    section: "§Standard Contracts → Dioxus DOM bridge"
    change: >-
      In the "on the stand" clause, "a CRUD row `TaskShell/Crud/div:0/div:1/div:0/div[0]`" becomes "a CRUD row its author key `crud-person-0`", and the chrome's author-key examples gain Home's seven cards `task-card-{slug}` (each card's five children read anchored paths, e.g. `task-card-{slug}//div:1/h2:0`); the task root `TaskShell/Counter/div:0` is unchanged.
    sidecar: >-
      Dioxus DOM bridge: the stand's CRUD row id was `TaskShell/Crud/div:0/div:1/div:0/div[0]`; now the author key `crud-person-0`; Home's cards read `task-card-{slug}`.
    rationale: >-
      Report Changes → Stand markup: crud.rs:69 gives each row `id: "crud-person-{person.id}"`, "A row's stable id is now its author key … it was `TaskShell/Crud/div:0/div:1/div:0/div[{person.id}]`"; app.rs:144 keys the Home card `task-card-{meta.slug}`. Counts / qualifiers: "The only ids that changed are CRUD's three rows, now `crud-person-0..2`". The old spelling at architecture.md:134 is a second occurrence of the retired three-tier reading.
    basis: "architecture.md:134 (`a CRUD row `TaskShell/Crud/div:0/div:1/div:0/div[0]``); examples/seven_guis/src/tasks/crud.rs:69"
    dependent-of: D-arch-resources

  - detector: D-arch-resources
    severity: warning
    section: "§Standard Contracts → Dioxus DOM bridge"
    change: >-
      In the persistence clause, "so the list diffs by key and a row's id `TaskShell/Crud/div:0/div:1/div:0/div[{person.id}]` follows its person under filter, Create and Delete" becomes: the row carries `id: "crud-person-{person.id}"` beside its `key: "{person.id}"`, so the list still diffs by the Dioxus key and a row's id is its author key `crud-person-{person.id}` (fixture `crud-person-0..2`, Create from `crud-person-3`), following its person under filter, Create and Delete; citation crud.rs `62-79` → `62-80`. The `if`-within-`for` keying trap sentence is unchanged.
    sidecar: >-
      Dioxus DOM bridge: a CRUD row's id was the Dioxus-keyed path `…/div[{person.id}]`; now its author key `crud-person-{person.id}`, the Dioxus `key:` kept for list diffing; crud.rs citation `62-79` → `62-80`.
    rationale: >-
      Same retired claim, third occurrence on the line: report Changes → Stand markup (crud.rs:69, "each person row gains `id: "crud-person-{person.id}"` beside its existing `key`") and the crud.rs line map ("one line inserted after 68 … architecture.md:134 — `62-79` → `62-80`").
    basis: "architecture.md:134 (`a row's id `TaskShell/Crud/div:0/div:1/div:0/div[{person.id}]``); report.md:28, :65"
    dependent-of: D-arch-resources

  - detector: D-arch-resources
    severity: warning
    section: "§Standard Contracts → Dioxus DOM bridge"
    change: >-
      The feature clause "gets the override and the snapshot model only by naming the feature" becomes "gets the override, the snapshot model and the actionable-key check only by naming the feature".
    sidecar: >-
      Dioxus DOM bridge: the feature clause was "the override and the snapshot model"; now "the override, the snapshot model and the actionable-key check".
    rationale: >-
      Report Changes → Symbols / APIs: `mod actionable` and `pub use actionable::UnkeyedActionable` are both `#[cfg(feature = "accessibility")]` (lib.rs:14-15, 23-24), and `cargo build -p dioxus-native-dom --no-default-features --locked` compiles the module out. The clause enumerates what the feature gates and now omits one item.
    basis: "architecture.md:134 (`gets the override and the snapshot model only by naming the feature`); packages/dioxus-native-dom/src/lib.rs:14-15, 23-24"
    dependent-of: D-arch-resources

  - detector: D-arch-resources
    severity: warning
    section: "§Existing Scopes → dioxus-native-dom"
    change: >-
      The module list reads: actionable (crate-private, compiled only under `accessibility`: the actionable-key check behind `DioxusDocument::unkeyed_actionable`, its `UnkeyedActionable` type re-exported), dioxus_document, element_id, events, mutation_writer, snapshot, write_once_attr; citation `packages/dioxus-native-dom/src/lib.rs:13-19` → `14-22`.
    sidecar: >-
      Existing Scopes: the dioxus-native-dom module list gains `actionable` (crate-private, `accessibility` only); citation `lib.rs:13-19` → `14-22`.
    rationale: >-
      Report Changes → Crates / modules: "one new module, `actionable`, in the existing crate dioxus-native-dom … The module list at `packages/dioxus-native-dom/src/lib.rs:14-22` now reads actionable (accessibility-gated), dioxus_document, element_id, events, mutation_writer, snapshot (accessibility-gated), write_once_attr"; line map "architecture.md:251 — `13-19` → `14-22`". No crate added.
    basis: "architecture.md:251; packages/dioxus-native-dom/src/lib.rs:14-22"
    dependent-of: D-arch-resources

  - detector: D-arch-resources
    severity: warning
    section: "§Cross-cutting Patterns → Config management"
    change: >-
      "which enables dioxus-native-dom's own optional `accesskit` and gates its `accessibility_tree` override and its `snapshot` module" becomes "… gates its `accessibility_tree` override and its `snapshot` and `actionable` modules".
    sidecar: >-
      Config management: was "gates its `accessibility_tree` override and its `snapshot` module"; now "and its `snapshot` and `actionable` modules".
    rationale: >-
      A restatement, outside the bridge contract, of what dioxus-native-dom's `accessibility` feature gates; the report's Crates / modules bullet adds the gated `actionable` module. No manifest changed (Dependencies: none), so the three Cargo.toml citations on the line stand.
    basis: "architecture.md:189 (`gates its `accessibility_tree` override and its `snapshot` module`); packages/dioxus-native-dom/src/lib.rs:14-15"
    dependent-of: D-arch-resources

  - detector: D-arch-resources
    severity: warning
    section: "§Conventions → Feature gating"
    change: >-
      Coordinate-only: `packages/dioxus-native-dom/src/lib.rs:5-11` → `5-12` (the feature list), `38-61` → `43-66` (the `trace!` macro), `52-55` → `57-60` (its 4-argument arm). The five listed features are unchanged.
    sidecar: >-
      Feature gating: citations re-pointed `lib.rs:5-11` → `5-12`, `38-61` → `43-66`, `52-55` → `57-60`.
    rationale: >-
      Report Changes → Line citations shift, lib.rs: "architecture.md:110 — `5-11` → `5-12` · `38-61` → `43-66` · `52-55` → `57-60`". Not an invariant violation by itself — carried so the registry's bases stay true alongside the lib.rs edit that registers the new module (prior wraps applied these in the same architecture amendment).
    basis: "architecture.md:110; packages/dioxus-native-dom/src/lib.rs:5-12 (read: feature list ends at line 12)"
    dependent-of: D-arch-resources

  - detector: D-arch-resources
    severity: warning
    section: "§Standard Contracts → Document core (blitz-dom)"
    change: >-
      Coordinate-only: the `DioxusDocument` `Document` impl citation `packages/dioxus-native-dom/src/dioxus_document.rs:233-305` → `242-314`.
    sidecar: >-
      Document core: citation re-pointed `dioxus_document.rs:233-305` → `242-314`.
    rationale: >-
      Report Changes → Line citations shift, dioxus_document.rs (the `element_id` doc comment grew 185-199 → 185-208; old 200 and below +9): "architecture.md:120 — `233-305` → `242-314`". Coordinate-only.
    basis: "architecture.md:120; report.md:51"
    dependent-of: D-arch-resources

  - detector: D-arch-resources
    severity: warning
    section: "§Standard Contracts → Headless stand (seven_guis, native only)"
    change: >-
      Coordinate-only: the `task_in_shell` citation `examples/seven_guis/src/app.rs:82` → `90` (`app.rs:7` unchanged).
    sidecar: >-
      Headless stand: citation re-pointed `seven_guis/src/app.rs:82` → `90`.
    rationale: >-
      Report Changes → Line citations shift, seven_guis app.rs (`task_in_shell` 82 → 90): "architecture.md:130 — `82` → `90` (`7` unchanged)". `TaskMeta` gained a private `slug` field only; the supporting API named on the line (`app::Task`, `task_in_shell`, private `TaskShell`) is unchanged.
    basis: "architecture.md:130; report.md:59"
    dependent-of: D-arch-resources

  - detector: D-arch-resources
    severity: warning
    section: "§Cross-cutting Patterns → Event processing"
    change: >-
      Coordinate-only: `packages/dioxus-native-dom/src/dioxus_document.rs:320-326` → `329-335` and `210-230` → `219-239` (`147-155` unchanged).
    sidecar: >-
      Event processing: citations re-pointed `dioxus_document.rs:320-326` → `329-335`, `210-230` → `219-239`.
    rationale: >-
      Report Changes → Line citations shift, dioxus_document.rs: "architecture.md:193 — `320-326` → `329-335` · `210-230` → `219-239` (`147-155` unchanged)". Coordinate-only.
    basis: "architecture.md:193; report.md:51"
    dependent-of: D-arch-resources

  - detector: D-arch-resources
    severity: warning
    section: "§Project Intent → Apps and demonstrators"
    change: >-
      Coordinate-only: the seven_guis tagline citation `examples/seven_guis/src/app.rs:119-120` → `127-128`.
    sidecar: >-
      Apps and demonstrators: citation re-pointed `seven_guis/src/app.rs:119-120` → `127-128`.
    rationale: >-
      Report Changes → Line citations shift, seven_guis app.rs: "architecture.md:209 — `119-120` → `127-128`". Coordinate-only; the `examples/todomvc/src/app.rs:1` citation on the same line is another crate's file and does not move (report sweep hazard 2).
    basis: "architecture.md:209; report.md:59, :63"
    dependent-of: D-arch-resources

  - detector: D-arch-resources
    severity: warning
    section: "§Existing Scopes → blitz-tests"
    change: >-
      Coordinate-only: `tests/blitz-tests/tests/stand_element_ids.rs:1-3` → `1-5`.
    sidecar: >-
      Existing Scopes: blitz-tests citation re-pointed `stand_element_ids.rs:1-3` → `1-5`.
    rationale: >-
      Report Changes → Line citations shift, stand_element_ids.rs (the `//!` head 1-3 → 1-5): "architecture.md:259 — `1-3` → `1-5`". The row's stand list is a non-exhaustive coverage summary (it already omits stand_id_persistence, stand_accessibility_ids and stand_snapshot), so the two new stand files are not proposed there.
    basis: "architecture.md:259; report.md:66-67"
    dependent-of: D-arch-resources

# D-arch-decisions: no drift. Report Dependencies reads "none — no manifest and no lockfile change" (asserted by the `git diff --quiet` gate over every manifest, Cargo.lock and deny.toml); `accesskit::Role` in `UnkeyedActionable` rides dioxus-native-dom's existing optional `accesskit` under the existing `accessibility` feature; no new runtime, library, feature flag, IPC method, endpoint, event, socket, port, env var, listener or process-wide state ("No IPC method, endpoint, …" in Symbols / APIs), so §Occupied Resources needs no entry. No crate added, removed or re-edged, so §Inherited Defaults needs no entry.
# Checked and left alone: architecture.md:207 (§Project Intent → Front page — README "built vs planned") — the report changes no README; architecture.md:120 "(`DioxusDocument` alone does, with stable ids)" still holds; the nine `dioxus_document.rs` citations at architecture.md:17, :98, :146, :192 sit below line 185 and do not move; `net.rs:471-489` at architecture.md:84 is a line citation, not the test count.
# Note on the eight coordinate-only proposals (the last six plus the citation halves of proposals 2, 4 and 6): they are not invariant violations in themselves; they are carried from the report's line map so the registered contracts' bases stay true, as earlier architecture amendments did. Drop them if a separate citation re-point step owns coordinates.
```

## security-plan

Verdict: 4 proposals (D-security-input 4, severity warning · D-security-auth 0 · D-security-deps 0); 13 comment lines stripped — no unvalidated boundary, the sweep it ran, why warning.

```yaml
# security-plan drift — chunk 2026-10-06-id-stability-across-code-edits
# D-security-auth: no drift — the report touches no identity / session / token / key (report.md:31, :34).
# D-security-deps: no drift — "Dependencies: none — no manifest and no lockfile change" (report.md:33).
# D-security-input: NO unvalidated boundary — the report adds no IPC / HTTP / deserialized surface (report.md:31, :89),
#   and the one changed reading of an existing input (the HTML `id` now also anchors descendants' ids) is gated by the
#   same author-key predicate (report.md:19, :90). What drifted is the TEXT of the row that states that validation:
#   four distinct clauses of one row (security-plan.md:107) are stale. Hence severity `warning`, not the detector's
#   `escalate` — that level is reserved for an unvalidated boundary, which was not found.
# Sweep for other occurrences of each retired claim over all 391 lines of security-plan.md
#   (`component path|document path|component or document|\{tag\}|Dioxus key|crud|CRUD|second, in-process|only exit|
#    element_id|author key|author_id|unkeyed|actionable|task-card`): every hit is line 107 (line 108 matches only on
#   `snapshot` / `in-process reader` for accessible names — a different claim, still true; snapshot.rs untouched).
#   So no `dependent-of` proposals. The four below edit disjoint clauses of the same row and are order-independent.
proposals:
  - detector: D-security-input
    severity: warning
    section: "§Input Validation → Markup attributes · `id` (stable element id)"
    change: >-
      Replace the clause "an empty, `/`-bearing or later-duplicate value gives no key and the element reads its
      `/`-bearing component or document path, so no `id` can make two ids equal" with: an empty, `/`-bearing or
      later-duplicate value gives no key and anchors nothing; an element with no key reads one of three `/`-bearing
      paths — the anchored path `{key}//{segment}` (then `/{segment}` per further DOM level) under the nearest element
      of its own component instance that reads an author key, else its component path, else its document path; a
      template root owned by another component instance restarts at that component's chain, so an anchor acts inside
      one component only; an author key holds no `/`, an anchored path is the only id holding `//`, and a component or
      document path has no empty segment, so no `id` can make two ids equal — a key spelled like a component name
      included (`Item//b:0` beside `Item/b:0`).
    sidecar: >-
      2026-10-06 id-stability-across-code-edits — §Input Validation `id` row: grammar restated as four tiers (author
      key · anchored path `key//segment` · component path · document path) with the three-way `//` uniqueness argument;
      an unusable `id` neither keys nor anchors.
    rationale: >-
      Report Symbols / APIs "The id grammar changed" (report.md:14-20): a NEW anchored tier sits between the author key
      and the component path, an unusable `id` "anchors nothing", and the uniqueness argument is "now three-way". The
      row's two-path wording ("component or document path") and its one-step uniqueness argument no longer describe the
      mechanism. The validation itself is present (author-key predicate decides what anchors; unit tests
      `an_unusable_id_anchors_nothing`, `a_key_spelled_like_a_component_stays_distinct` — report.md:90, :118). Listed
      as an expected amendment, carried (report.md:81).
    basis: "packages/dioxus-native-dom/src/element_id.rs:159-242 (element_ids 159-195, place_children 198-242; anchor tests 425-576)"

  - detector: D-security-input
    severity: warning
    section: "§Input Validation → Markup attributes · `id` (stable element id)"
    change: >-
      Replace "a keyed list row's segment carries its Dioxus key (`{tag}[{key}]`), and the 7GUIs CRUD row's key is its
      person's model-assigned `u64` (fixture people 0–2, Create from 3)" with: a keyed list row's segment carries its
      Dioxus key (`{tag}[{key}]`); the 7GUIs CRUD row also carries an HTML `id` `crud-person-{person.id}`, so its
      stable id is that author key, built like its Dioxus key from the person's model-assigned `u64` (fixture
      `crud-person-0..2`, Create from `crud-person-3`); Home's seven cards read the fixed-slug author keys
      `task-card-{slug}` — keeping the rest of the sentence ("never a list index, pointer, hash, clock or process-local
      value — so a row reads the same id across a re-render, a remount and a second process …") unchanged.
    sidecar: >-
      2026-10-06 id-stability-across-code-edits — §Input Validation `id` row: a CRUD row's stable id is its author key
      `crud-person-{person.id}` (was the path segment `…/div[{person.id}]`); Home's cards read `task-card-{slug}`.
    rationale: >-
      Report Symbols / APIs "Stand markup — author keys added" (report.md:27-30): crud.rs:69 adds
      `id: "crud-person-{person.id}"` beside the existing `key`, so the row's id "is now its author key … it was
      `TaskShell/Crud/div:0/div:1/div:0/div[{person.id}]`"; the Home card button gains `id: "task-card-{meta.slug}"`.
      The row still says the CRUD row's id comes from its Dioxus-key segment. The persistence property is unchanged and
      re-pinned (report.md:119, :135); the id still carries only a model id or a fixed slug (report.md:91).
    basis: "examples/seven_guis/src/tasks/crud.rs:68-69; examples/seven_guis/src/app.rs:144"

  - detector: D-security-input
    severity: warning
    section: "§Input Validation → Markup attributes · `id` (stable element id)"
    change: >-
      After the `DioxusDocument::snapshot` clause ("… so the platform adapter stays the id's only exit"), add: under
      the same feature `DioxusDocument::unkeyed_actionable` is a third in-process reader of the stable id — it takes no
      argument, returns an `UnkeyedActionable` (`node`, `id`, `tag`, `role`, `focusable`, `interactive_role`,
      `listener`) for each actionable element whose id holds a `/`, takes ids from `element_ids()` and roles from
      `accessibility_tree`, resolves each node through `BaseDocument::get_node` and skips one that does not resolve,
      logs nothing and never panics; an entry carries the id, tag and role but never the element's text content,
      accessible name or an attribute value, and — unlike the id and the snapshot — a process-local `NodeId` in its
      `node` field; it adds no crossing: it has no wire form and no driver, CLI or MCP command exposes it yet, so the
      platform adapter stays the id's only exit (packages/dioxus-native-dom/src/actionable.rs:15-44;
      packages/dioxus-native-dom/src/actionable.rs:55-89). Scope the existing "it carries no `NodeId`, `ElementId`,
      `ScopeId` or pointer" to the id and the snapshot.
    sidecar: >-
      2026-10-06 id-stability-across-code-edits — §Input Validation `id` row: records
      `DioxusDocument::unkeyed_actionable` / `UnkeyedActionable` as a third in-process reader of the stable id (no
      argument, no log, no wire form, no content; entry holds a process-local `NodeId`); the platform adapter stays the
      id's only exit.
    rationale: >-
      Report Symbols / APIs "New public API" (report.md:21-26) and Coverage of new surfaces (report.md:89): a public,
      `accessibility`-gated read API, "not an external-input surface", validation n/a, PII n/a (unit test
      `the_report_names_structure_never_content`), "no driver, CLI or MCP command exposes it yet". The row enumerates
      the id's in-process readers and names `snapshot` as "a second" and last; it is now incomplete. Note the struct's
      `pub node: NodeId` (report.md:23): the row's blanket "it carries no `NodeId` …" would read as false if left
      adjacent to the new reader unscoped. Listed as an expected amendment, carried (report.md:81).
    basis: "packages/dioxus-native-dom/src/actionable.rs:16 (struct), :38 (remedy), :55 (unkeyed_actionable); packages/dioxus-native-dom/src/lib.rs:14-15, :23-24"

  - detector: D-security-input
    severity: warning
    section: "§Input Validation → Markup attributes · `id` (stable element id)"
    change: >-
      Re-point the row's line citations — `packages/dioxus-native-dom/src/element_id.rs:153-241` → `159-255`;
      `element_id.rs:296-320` → `310-334`; `element_id.rs:356-410` → `370-423`;
      `tests/blitz-tests/tests/stand_element_ids.rs:237-270` → `242-275`;
      `examples/seven_guis/src/tasks/crud.rs:62-79` → `62-80` — leaving `stand_id_persistence.rs:1-3`,
      `snapshot.rs:72-90`, `snapshot.rs:105-151` and `stand_snapshot.rs:1-5` as they are.
    sidecar: >-
      2026-10-06 id-stability-across-code-edits — §Input Validation `id` row: five line citations re-pointed after the
      chunk's edits to element_id.rs, stand_element_ids.rs and crud.rs (coordinates only).
    rationale: >-
      Report "Line citations shift in nine files" names this row three times: security-plan.md:107 for element_id.rs
      (report.md:47), crud.rs (report.md:65) and stand_element_ids.rs (report.md:67). `356-410` must not be mapped by
      offset — its end sat on the old file's last line; the two tests it named now end at 423 (report.md:107, hazard 3).
      Re-read at HEAD: `element_ids` starts 159, `element_id` 245, `a_duplicate_html_id_reads_a_path` 310,
      `a_removed_element_reads_no_id` 370, `non_elements_read_no_id` 408, first anchor test 425;
      `text_and_removed_nodes_read_no_id` 242. The row's other two citations into edited files are unaffected
      (security-plan.md:121 `dioxus_document.rs:32-39` sits above line 185; security-plan.md:90 `construct.rs:470-490`
      is a different file — report.md:37, :54).
    basis: "packages/dioxus-native-dom/src/element_id.rs:159,245,310,370,408,425; tests/blitz-tests/tests/stand_element_ids.rs:242; examples/seven_guis/src/tasks/crud.rs:62-80"
```

## design-system

Verdict: 0 proposals; 9 comment lines stripped — D-design-tokens no drift; it names the 7 app.rs citation shifts as outside its detector (raw twin kept).

## layout-templates

Verdict: 3 proposals (D-layout-surface 3); 5 comment lines stripped — no new surface; the three are stale statements in the existing seven_guis bullet.

```yaml
# Detector fit: the report adds NO new UI surface or region (report.md:30, :89, :91), so D-layout-surface's
# literal trigger does not fire. The three proposals below are stale statements inside the EXISTING
# seven_guis wireframe entry (layout-templates.md:10), filed under the only layout-templates detector because
# the report's Expected amendments mark this site "carried" (report.md:82). All three edit the same bullet,
# so apply them together. Line numbers come from the report's line maps; I did not re-read the source files.
proposals:
  - detector: D-layout-surface
    severity: warning
    section: "§Surface: desktop-native → Primary screens (the `seven_guis Home and TaskShell` bullet)"
    change: "Replace `and each CRUD row a Dioxus key `{person.id}`, its person's model-assigned id (fixture people 0–2, Create from 3), so a row's id `…/div[{person.id}]` follows its person under filter, Create and Delete` with: each CRUD row carries the author `id` `crud-person-{person.id}` beside its Dioxus key `{person.id}` (its person's model-assigned id), so a row's stable element id is its author key `crud-person-{person.id}` (fixture rows `crud-person-0..2`, Create from `crud-person-3`) and follows its person under filter, Create and Delete (examples/seven_guis/src/tasks/crud.rs:69)."
    sidecar: "2026-10-06 id-stability-across-code-edits: a CRUD row's stable id is now its author key `crud-person-{person.id}`, no longer the Dioxus-keyed path `…/div[{person.id}]`."
    rationale: "Report Changes → Symbols / APIs → Stand markup (report.md:28): each person row gains `id: \"crud-person-{person.id}\"` beside its existing `key`; its stable id is now that author key and was `TaskShell/Crud/div:0/div:1/div:0/div[{person.id}]`. The bullet still says the row's id is the `…/div[{person.id}]` path. Expected amendments name this site as carried (report.md:82). Sweep of layout-templates for `div[`, `TaskShell/Crud`, `Dioxus key` and for any other wording of 'a row's id is a positional path': 1 hit, line 10 only — no duplicate occurrence."
    basis: "/home/turbolet/dev/projects/escher/.andromeda/layout-templates.md:10 (claim) · report.md:28 (change)"
  - detector: D-layout-surface
    severity: warning
    section: "§Surface: desktop-native → Primary screens (the `seven_guis Home and TaskShell` bullet)"
    change: "Add to the Home description: each of Home's seven task-card buttons carries the author `id` `task-card-{slug}`, its stable element id — slugs `counter` · `temp-converter` · `flight-booker` · `timer` · `crud` · `circle-drawer` · `cells` (examples/seven_guis/src/app.rs:144); like the task ids, these add no class, style, wrapper or order, and no Home or task CSS selects by them."
    sidecar: "2026-10-06 id-stability-across-code-edits: Home's seven task cards are author-keyed `task-card-{slug}`; attributes only, no layout change."
    rationale: "Report Changes → Symbols / APIs → Stand markup (report.md:29-30): `TaskMeta` gains `slug`, the seven `TASKS` entries carry the seven slugs, and the Home card `button` gains `id: \"task-card-{meta.slug}\"`; no class, style, wrapper, order, tabindex, role or handler changed and no CSS selects by the new ids. The bullet's author-id list covers only the lean-task controls; sweep `task-card|crud-person` over layout-templates reads 0 hits (report.md:82, confirmed on read). An addition to the existing Home entry, not a new surface."
    basis: "/home/turbolet/dev/projects/escher/.andromeda/layout-templates.md:10 · report.md:29-30"
    dependent-of: D-layout-surface
  - detector: D-layout-surface
    severity: warning
    section: "§Surface: desktop-native → Primary screens (the `seven_guis Home and TaskShell` bullet)"
    change: "Re-point the bullet's line citations: `examples/seven_guis/src/app.rs:122-151` → `130-160` · `app.rs:153-172` → `162-181` · `app.rs:174-328` → `183-337` · `app.rs:82` → `90` · `examples/seven_guis/src/tasks/crud.rs:85` → `86` · `crud.rs:91` → `92` · `crud.rs:46-125` → `46-126` (`crud.rs:44` unchanged)."
    sidecar: "2026-10-06 id-stability-across-code-edits: coordinates only — seven_guis `app.rs` citations +9 (Home, TaskShell, CSS constants, `task_in_shell`) and `crud.rs` citations +1 below line 68."
    rationale: "Report Changes → Counts / qualifiers moved → Line citations shift (report.md:58, :61, :64-65): `app.rs` is +9 from old line 136 on (`Home` 122-151 → 130-160, `TaskShell` 153-172 → 162-181, CSS constants 174-328 → 183-337, `task_in_shell` 82 → 90); `crud.rs` has one line inserted after 68, so 85 → 86, 91 → 92, 46-125 → 46-126, and 44 is unchanged. The report names layout-templates.md:10 for both maps. The `dioxus_document.rs` citations at layout-templates.md:35-37 are unchanged (report.md:54) — no proposal there. No other seven_guis `app.rs` or `crud.rs` citation exists in the doc (line 91 cites `index.html`, not edited)."
    basis: "/home/turbolet/dev/projects/escher/.andromeda/layout-templates.md:10 · report.md:61 and :65"
    dependent-of: D-layout-surface
```

## test-plan

Verdict: 10 proposals (D-tests-coverage 10 · D-tests-framework 0 · D-tests-obs-harness 0); 3 comment lines stripped — the invariant holds, the plan's own coverage record drifted.

```yaml
# D-tests-framework: no drift — every test command in the report is cargo's built-in harness, `scripts/agent-run.sh` or `.github/scripts/ci-leg.sh` (report.md:130-150); Dependencies: none (report.md:33).
# D-tests-obs-harness: no drift — "Harness / gate surface: none changed", `scripts/agent-run.sh` byte-identical (report.md:71); obs-plan states no stand `ok` count (0 hits), so the §3 Proof append below is not a one-sided harness change.
# D-tests-coverage: the invariant itself holds — every new path carries unit + integration tests (report.md:89-91). What drifted is test-plan's own coverage record (counts, file census, stand-file list, baseline chain), which the report names as expected amendments (report.md:84). Proposals 6-10 are coordinates only (line citations), from the report's line map (report.md:48, :52); drop them if a separate citation remap already covers test-plan.
proposals:
  - detector: D-tests-coverage
    severity: warning
    section: "## 1. Test Scope Summary — Coverage scope → dioxus-native-dom (test-plan.md:23)"
    change: >-
      Say "29 unit tests in five files" instead of "18 unit tests in four files"; cite
      `dioxus_document.rs:429-465` instead of `420-456`; say "thirteen on the stable element id — seven of them on the
      anchored tier (a keyed parent anchors its unkeyed children, the nearest keyed ancestor anchors, an unusable id
      anchors nothing, a component root under a keyed element restarts at its chain, a key spelled like a component
      stays distinct, a Dioxus-keyed row under a keyed list reads its key segment, an edit outside the anchor keeps the
      anchored id) (packages/dioxus-native-dom/src/element_id.rs:257-577)" instead of "six on the stable element id
      (…element_id.rs:243-410)"; add "four on the `unkeyed_actionable` check — an unkeyed actionable element reported
      with its remedy, a keyed or inert element not reported, a hidden or disabled control still held, the report
      naming structure never content — compiled only under the crate's default `accessibility` feature
      (packages/dioxus-native-dom/src/actionable.rs:124-270)"; the nine snapshot tests and events.rs citation unchanged.
    sidecar: >-
      2026-10-06-id-stability-across-code-edits — §1 dioxus-native-dom unit census 18 in four files → 29 in five
      (element_id 6 → 13, new actionable.rs 4); element_id.rs and dioxus_document.rs test citations re-anchored.
    rationale: >-
      Report "Counts / qualifiers moved": dioxus-native-dom lib unit tests 18 in four files → 29 in five files —
      element_id 6 → 13, actionable 0 → 4 (new file), snapshot 9, dioxus_document 1, events 2 (report.md:39); line maps
      `243-410` → `257-577` and `420-456` → `429-465` (report.md:48, :52). The new `actionable` module is
      `#[cfg(feature = "accessibility")]` (report.md:21, :32).
    basis: >-
      report.md:39; packages/dioxus-native-dom/src/element_id.rs:257-577 (`mod tests`, 13 `#[test]`);
      packages/dioxus-native-dom/src/actionable.rs:124-270 (`mod tests`, 4 `#[test]` at 141, 192, 223, 248);
      packages/dioxus-native-dom/src/dioxus_document.rs:429-465

  - detector: D-tests-coverage
    severity: warning
    section: "## 1. Test Scope Summary — Coverage scope → dioxus-native and stylo_taffy (test-plan.md:24)"
    change: >-
      The census parenthesis should read "matches only in dioxus-native-dom: dioxus_document.rs and events.rs at that
      search, element_id.rs, snapshot.rs and actionable.rs since".
    sidecar: >-
      2026-10-06-id-stability-across-code-edits — §1 `#[test]` census for slice s11 gains actionable.rs (five
      dioxus-native-dom files).
    rationale: >-
      Same retired claim as the primary (four test-bearing files): the report names test-plan.md:24 as stating the
      `#[test]` census of dioxus_document.rs, events.rs, element_id.rs and snapshot.rs — "now also actionable.rs"
      (report.md:39).
    basis: "report.md:39; packages/dioxus-native-dom/src/actionable.rs:124-125"
    dependent-of: D-tests-coverage

  - detector: D-tests-coverage
    severity: warning
    section: "## 1. Test Scope Summary — Coverage scope → tests/blitz-tests (test-plan.md:25)"
    change: >-
      In the seven_guis headless-stand clause, after the fresh-process / `ids_in_a_child_process` item, add: "the ids
      held across code edits — a before/after component pair per edit in both layout modes: each lean task under an
      edited shell, keyed elements under every edit, anchored elements under an edit outside their anchor, a behaviour
      edit, and the edits inside an anchor measured on positional ids only — and every actionable element of the four
      lean tasks and Home reading an author key (`DioxusDocument::unkeyed_actionable` empty) in both layout modes and
      across state, the three non-lean tasks measured and pinned (Temperature Converter 2 · Circle Drawer 3 · Cells
      676)"; and add `tests/blitz-tests/tests/stand_id_edits.rs:1; tests/blitz-tests/tests/stand_actionable_keys.rs:1`
      to that clause's citation list (eleven stand files, was nine).
    sidecar: >-
      2026-10-06-id-stability-across-code-edits — §1 stand coverage gains stand_id_edits (5 checks) and
      stand_actionable_keys (3 checks); stand check files 9 → 11.
    rationale: >-
      Report "Files — new": `tests/blitz-tests/tests/stand_id_edits.rs` and `stand_actionable_keys.rs` (report.md:9);
      "Stand check files: 9 → 11 … Stated at test-plan.md:25" (report.md:40); behaviours from Outcome (report.md:113-117)
      and the pinned non-lean counts (report.md:41). test-plan holds 0 hits for `stand_id_edits` / `actionable`.
    basis: >-
      report.md:40; tests/blitz-tests/tests/stand_id_edits.rs:1-6 (5 `#[test]`: 120, 373, 402, 425, 454);
      tests/blitz-tests/tests/stand_actionable_keys.rs:1-4 (3 `#[test]`: 86, 108, 146)

  - detector: D-tests-coverage
    severity: warning
    section: "## 3. Test Harness Contract — Agent-run contract → Proof (test-plan.md:111)"
    change: >-
      Append one link to the Proof chain: "; re-counted at 2026-10-06-id-stability-across-code-edits: `run stand` 41
      `ok` stand events (+5 `stand_id_edits`, +3 `stand_actionable_keys`, both picked up by the `stand_` prefix with no
      script change; `run.end` passed 41 · failed 0 · ignored 1) — as measured at
      escher-0.1.0/chunks/2026-10-06-id-stability-across-code-edits/report.md". Earlier links stay as written.
    sidecar: >-
      2026-10-06-id-stability-across-code-edits — §3 agent-run Proof chain re-counted: `run stand` 33 → 41 `ok`.
    rationale: >-
      Report "Stand `ok` events (`bash scripts/agent-run.sh run stand`): 33 → 41 … Stated at test-plan.md:111 (the §3
      Proof chain, last link 33)" (report.md:38); gate `agent-run.sh logs | python3 …` read `last line 41`
      (report.md:145). The script is byte-identical (report.md:71), so only the measured count moves; obs-plan carries
      no stand count, so §3 ↔ obs-plan §3 stay in agreement.
    basis: "report.md:38, :145; test-plan.md:111 (last link `33 \\`ok\\``)"

  - detector: D-tests-coverage
    severity: warning
    section: "## 9. CI Integration — Pipeline facts → Local baseline → `cargo test --workspace` (test-plan.md:314)"
    change: >-
      Append one link to the baseline chain: "; re-counted at 2026-10-06-id-stability-across-code-edits: 127 result
      lines, 490 passed · 0 failed · 5 ignored (+7 `dioxus-native-dom` `element_id` unit tests and +4 `actionable` unit
      tests in the crate's existing lib result line, +5 `stand_id_edits` and +3 `stand_actionable_keys` stand checks in
      two new result lines), timings not re-measured — as measured at that chunk's `ci-leg.sh fast` runs at /implement
      and on its pre-CI commit 75f12a09
      (escher-0.1.0/chunks/2026-10-06-id-stability-across-code-edits/report.md)". Earlier links stay as written.
    sidecar: >-
      2026-10-06-id-stability-across-code-edits — §9 local workspace baseline re-counted: 471 · 0 · 5 over 125 result
      lines → 490 · 0 · 5 over 127.
    rationale: >-
      Report "Workspace test baseline (`cargo test --workspace`, the `test` leg): 471 passed · 0 failed · 5 ignored
      over 125 result lines → 490 passed · 0 failed · 5 ignored over 127 result lines … Stated at test-plan.md:314"
      (report.md:37); `ci-leg.sh fast` green at 490 · 0 · 5 over 127 (report.md:148). Sweep `\b471\b` in test-plan: 1
      hit (line 314); `Ran 64 tests` and the a11y 6 + 6 + 3 are unchanged (report.md:43), so §4 line 138 and §9 line
      316 need no edit.
    basis: "report.md:37, :148; test-plan.md:314 (last link `125 result lines, 471 passed`)"

  - detector: D-tests-coverage
    severity: warning
    section: "## 2. Test Strategy — Test levels observed → Inline unit tests (test-plan.md:43)"
    change: >-
      Coordinates only: cite `packages/dioxus-native-dom/src/dioxus_document.rs:420-421` instead of `411-412`.
    sidecar: >-
      2026-10-06-id-stability-across-code-edits — §2 citation re-anchored: dioxus_document.rs 411-412 → 420-421 (+9,
      the `element_id` doc comment grew).
    rationale: >-
      Report line map: dioxus_document.rs old 200 and below → +9; "test-plan.md … :43 `411-412` → `420-421`"
      (report.md:49, :52). Same retired coordinate set as the primary's dioxus_document.rs citation.
    basis: "report.md:52; packages/dioxus-native-dom/src/dioxus_document.rs:420-421 (`#[cfg(test)]` / `mod tests`)"
    dependent-of: D-tests-coverage

  - detector: D-tests-coverage
    severity: warning
    section: "## 2. Test Strategy — Test directory + naming conventions → Test function naming (test-plan.md:64)"
    change: >-
      Coordinates only: cite `packages/dioxus-native-dom/src/dioxus_document.rs:429-432` instead of `420-423`
      (`keyed_nodes_do_not_crash`). The Doc examples line below it (`43-65`) is unchanged.
    sidecar: >-
      2026-10-06-id-stability-across-code-edits — §2 citation re-anchored: dioxus_document.rs 420-423 → 429-432.
    rationale: >-
      Report line map: "test-plan.md … :64 `420-423` → `429-432`"; test-plan.md:65 listed unchanged (below line 185
      rule, report.md:52, :54).
    basis: "report.md:52; packages/dioxus-native-dom/src/dioxus_document.rs:429-432"
    dependent-of: D-tests-coverage

  - detector: D-tests-coverage
    severity: warning
    section: "## 3. Test Harness Contract — Crate-local test helpers → dioxus-native-dom (test-plan.md:96)"
    change: >-
      Coordinates only, four citations: `dioxus_document.rs:415` → `424`; `dioxus_document.rs:439-449` → `448-458`;
      `element_id.rs:250-254` → `264-268`; `element_id.rs:356-391` → `370-405`. The Cargo.toml and snapshot.rs
      citations are unchanged.
    sidecar: >-
      2026-10-06-id-stability-across-code-edits — §3 helper citations re-anchored: dioxus_document.rs 415 → 424 and
      439-449 → 448-458; element_id.rs 250-254 → 264-268 and 356-391 → 370-405.
    rationale: >-
      Report line maps: "test-plan.md:96 — `250-254` → `264-268` · `356-391` → `370-405`" (report.md:48) and ":96
      `415` → `424` · `439-449` → `448-458`" (report.md:52).
    basis: >-
      report.md:48, :52; packages/dioxus-native-dom/src/element_id.rs:264-268 (`fn build`), :370-405
      (`a_removed_element_reads_no_id`); packages/dioxus-native-dom/src/dioxus_document.rs:424 (`use dioxus::prelude::*`)
    dependent-of: D-tests-coverage

  - detector: D-tests-coverage
    severity: warning
    section: "## 5. Integration Test Strategy — Boundaries covered → Dioxus VirtualDom ↔ DioxusDocument (test-plan.md:171)"
    change: >-
      Coordinates only: cite `packages/dioxus-native-dom/src/dioxus_document.rs:448-464` instead of `439-455`.
    sidecar: >-
      2026-10-06-id-stability-across-code-edits — §5 citation re-anchored: dioxus_document.rs 439-455 → 448-464.
    rationale: "Report line map: \"test-plan.md … :171 `439-455` → `448-464`\" (report.md:52)."
    basis: "report.md:52"
    dependent-of: D-tests-coverage

  - detector: D-tests-coverage
    severity: warning
    section: "## 7. Test Data & Fixtures — Builders and options (test-plan.md:241)"
    change: >-
      Coordinates only: cite `packages/dioxus-native-dom/src/dioxus_document.rs:433-448` instead of `424-439` (the
      keyed-nodes test's shared `Rc<RefCell<HashMap<usize, usize>>>` props).
    sidecar: >-
      2026-10-06-id-stability-across-code-edits — §7 citation re-anchored: dioxus_document.rs 424-439 → 433-448.
    rationale: "Report line map: \"test-plan.md … :241 `424-439` → `433-448`\" (report.md:52)."
    basis: "report.md:52"
    dependent-of: D-tests-coverage
```

## obs-plan

Verdict: 0 proposals; 5 comment lines stripped — no drift on three detectors; obs-plan.md:68 re-checked and standing (raw twin kept).

## a11y-plan

Verdict: 0 proposals; 39 comment lines stripped — neither detector fires; it lists 11 stale citations and two content items as outside both (raw twin kept).

## Validate — dispositions

Thirty proposals from four docs; three docs returned none. Decode probe: 0 HTML entities in every return, before and after. No proposal rests on a fact the report does not carry: where a `basis` names a source line (security 3 · 4, architecture 1, test-plan 1 · 2 · 3 · 6 · 7 · 8), the line sits inside a range the report's Symbols / APIs bullet or its line maps state, and every value in the `change` is the report's.

Checks, in order:
1. **Playbook.** Every applied proposal matches *Accurate this-chunk addition* (routine): its named symbols, ids, counts and coordinates are this chunk's Changes, landing inside an existing section. The grammar change amends a locked contract; that is not a reversal left to this wrap — the founder ratified both rules, the `key//segment` spelling and the one-component reach at /andromeda-phase (the founder, 2026-10-06; chunk inputs I1 · I2), and the public check as a contract addition (the founder, 2026-10-06; inputs I3), and the plan's Expected amendments name each change. No *Boundary widening*: `unkeyed_actionable` is an in-process reader with no wire form, the HTML `id` is the same admitted input under the same predicate, and nothing new crosses a boundary. 0 escalations.
2. **Cross-contradiction.** None. Architecture 2 · 3 · 4 · 5 and security 1 · 2 · 3 · 4 and layout 1 · 2 · 3 each edit one line's disjoint clauses in the same direction.
3. **Intent-consistency.** The report matches the working-route entry and the plan's acceptance; its seven deviations are justified in place; the scope record holds no line (`gate.py scope` clean).
4. **Absence needs evidence.** The 0-hit claims (`unkeyed_actionable|UnkeyedActionable|actionable`, `task-card|crud-person`, `stand_id_edits|stand_actionable_keys`) are the report's sweeps over the seven masters, the registries and the distillations; the citation sweep found 96 citations and dispositions each (61 change · 35 unchanged). The cascade sweep re-reads the long lines by offset.
5. **Expected-amendments reconciliation.** Proposed: architecture, security-plan, layout-templates, test-plan. Raised by the orchestrator, routine (the report substantiates each): a11y-plan (O1 · O2 · O3) and design-system (O4). obs-plan: not carried (0 moved citations, no value). `matrix#v010-01 notes`: P7.3.
6. **Disproved claims.** The report lists none.

### architecture — 13 proposals, all `apply` (check 1)
- 1 register `unkeyed_actionable` / `UnkeyedActionable` in §Standard Contracts → Dioxus DOM bridge — apply.
- 2 the grammar reads four tiers; ten citations on the line re-pointed — apply.
- 3 the stand clause: a CRUD row reads `crud-person-0`; Home's cards `task-card-{slug}` — apply (dependent of 2).
- 4 the persistence clause: a row's id is its author key; `crud.rs:62-79` → `62-80` — apply (dependent of 2).
- 5 the feature clause names the actionable-key check — apply (dependent of 1).
- 6 §Existing Scopes module list gains `actionable`; `lib.rs:13-19` → `14-22` — apply (dependent of 1).
- 7 §Cross-cutting Patterns → Config management names the `actionable` module — apply (dependent of 1). A site the report's own sweep did not list; the claim is the report's (Crates / modules).
- 8 · 9 · 10 · 11 · 12 · 13 coordinates only (§Conventions → Feature gating, Document core, Headless stand, Event processing, Apps and demonstrators, §Existing Scopes → blitz-tests) — apply; each value is the report's line map.

### security-plan — 4 proposals, all `apply` (check 1)
- 1 the `id` row states four tiers and the `//` uniqueness argument — apply.
- 2 a CRUD row's id is its author key; Home's cards — apply.
- 3 `unkeyed_actionable` as a third in-process reader; the "carries no `NodeId`" clause scoped to the id and the snapshot, since `UnkeyedActionable.node` is a `NodeId` — apply. The detector judged its own severity `warning`, the detector's `escalate` being for an unvalidated boundary; none exists (check 1, no widening).
- 4 five citations re-pointed — apply.

### layout-templates — 3 proposals, all `apply` (check 1)
- 1 a CRUD row's id is its author key — apply.
- 2 Home's seven `task-card-{slug}` keys — apply (dependent of 1).
- 3 seven citations re-pointed — apply (dependent of 1).

### test-plan — 10 proposals, all `apply` (check 1)
- 1 §1 unit census 18 in four files → 29 in five — apply.
- 2 §1 `#[test]` census gains `actionable.rs` — apply (dependent of 1).
- 3 §1 stand coverage gains the two files — apply.
- 4 §3 Proof chain, one link: 41 — apply.
- 5 §9 baseline chain, one link: 490 · 0 · 5 over 127 — apply.
- 6 · 7 · 8 · 9 · 10 coordinates only — apply.

### Raised by the orchestrator (check 5) — all `apply`, routine
- O1 a11y-plan — 11 citations on 8 lines re-pointed by the report's line maps (§2 Feature exposure :55 · §5 :174 · :175 · :178 · §7 :269 · :281 · §8 :313 · :319).
- O2 a11y-plan §2 Feature exposure (:55) — the feature also gates the actionable-key check.
- O3 a11y-plan §7 — the actionable rule and its library check, with the measured counts.
- O4 design-system — 7 `examples/seven_guis/src/app.rs` citations on 6 lines, each +9; no value changes.

### design-system · obs-plan · a11y-plan
Each returned `proposals: []` beside commentary; the returns are kept as raw twins (`.raw-fanout-{doc}.md`). Their commentary names the coordinate shifts and, for a11y-plan, the two content items — the same facts O1–O4 carry.
