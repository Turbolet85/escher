# arch extract

## Relevance
relevant: the chunk has to carry dioxus-native-dom's stable id into blitz-dom's AccessKit tree build. That crosses the crate boundary, a locked contract and the default-feature set.

## Constraints
- **Id source is fixed.** Per architecture §Standard Contracts → Dioxus DOM bridge, the stable id is computed on demand by `DioxusDocument::element_id` / `element_ids` in dioxus-native-dom. It is written to neither the DOM nor the vdom, no `NodeId`, `ElementId`, `ScopeId` or pointer enters it, and `element_id` returns `None` for a non-element, stale or detached node. The carrier must take its value from this source alone.
  - Writing the id into the DOM as an `id` attribute, which is the scope's third option, contradicts this contract. If P4 chooses it, it is a contract amendment, not a silent change.
- **Crate boundary.** blitz-dom is the headless core that wrappers such as `DioxusDocument` drive (architecture §Design Philosophy → Radically modular, embeddable engine). Shared interop types live in blitz-traits "without circular or unnecessary dependencies" (same bullet). Per §Existing Scopes, `element_id` is a crate-private module of dioxus-native-dom.
  - blitz-dom must not depend on dioxus-native-dom.
  - Any crossing is either a blitz-dom/blitz-traits-side hook that the Dioxus document installs, or a pass over the built `TreeUpdate` inside dioxus-native-dom.
- **Injected services with no-op defaults.** Embedder services reach the DOM only through `DocumentConfig` provider traits that fall back to `Dummy*` no-ops (architecture §Design Philosophy → Embedder services are injected; §Inherited Defaults → API style; §Cross-cutting Patterns → Config management). No-op implementations are prefixed `Dummy` (§Conventions → Visibility and naming). A new id hook follows this shape, and a document without it builds the tree exactly as before.
- **Tree contract and node keying.** `build_accessibility_tree` returns an accesskit `TreeUpdate` of `(NodeId, accesskit::Node)` pairs (architecture §Standard Contracts → Mutation, query and CSSOM). `NodeId` is a versioned slot id that packs index and version (§Established Decisions → [Node identity]; §Design Philosophy → One incremental pipeline, safe identities). The AccessKit `NodeId` keying keeps its meaning, and the stable id travels in a separate property.
- **Pinned dependency.** The accessibility layer is accesskit "0.25", with the accesskit_xplat platform adapters pinned per target (architecture §Stack and Technologies → Accessibility). Dependencies are declared once in `[workspace.dependencies]` (§Conventions → Manifests), and builds pass `--locked`. The carrier must exist in the pinned version: whether accesskit 0.25's `Node` exposes `author_id`, and under what name, is research's question. Bumping accesskit to get it would also move the accesskit_xplat pins.
- **Feature gating.** `accessibility` is a default feature of blitz-dom, blitz-shell, blitz, dioxus-native and dioxus-native-dom (architecture §Established Decisions → [Default features]). Optional functionality is gated by cargo features (§Conventions → Feature gating), and each Dioxus feature forwards to the same-named blitz feature (§Cross-cutting Patterns → Config management). The new hook or pass and any harness accessibility read compile out cleanly without `accessibility`.
  - Any tracing follows the per-call-site `#[cfg(feature = "tracing")]` pattern, or dioxus-native-dom's crate-local `trace!` (§Conventions → Feature gating).
- **Refresh path and identical layout modes.** The accessibility tree is rebuilt on poll when the document changes (architecture §Cross-cutting Patterns → Invalidation and state integrity). Incremental and non-incremental runs must agree (§Design Philosophy → One incremental pipeline, safe identities). Whether the rebuilt or incremental tree update carries the id on changed nodes is research's question.

## Patterns to follow
- **One id source.** Use `element_id(NodeId)` / `element_ids()` as the only id source. A consumer reads the result through the public `DioxusDocument` API, not the crate-private `element_id` module (architecture §Standard Contracts → Dioxus DOM bridge; §Existing Scopes → dioxus-native-dom).
- **Optional embedder hook.** If the crossing is a hook, model it on the existing provider traits: a trait object on `DocumentConfig` with a `Dummy*` default, which `BaseDocument::new` falls back to (architecture §Design Philosophy → Embedder services are injected; §Standard Contracts → Provider traits (blitz-traits)).
- **Stand checks.** Boot through `stand::boot(LeanTask, stand::options(incremental))` → `Harness<DioxusDocument>`, with the pinned viewport, the bundled font and offline (architecture §Standard Contracts → Headless stand).
  - Each integration-test file covers one behaviour and opens with a `//!` doc.
  - Run both modes `for incremental in [false, true]`, use `#[track_caller]` helpers, and first assert that the fixture produces the condition under test (§Conventions → Tests).
- **Cite the spec.** Web semantics are written against named specs, as with the HTML-AAM role mapping and the WAI-ARIA exclusion in accessibility.rs (architecture §Design Philosophy → Behave like a browser, and cite the spec). An engine-side accessible-name computation cites its spec, such as accname or HTML-AAM.
- **Documentation and lints.** New modules and files open with `//!` docs (architecture §Conventions → Documentation), and crate internals stay `pub(crate)` (§Conventions → Visibility and naming).

## Anti-patterns to avoid
- **Wrong dependency direction or internals.** Do not reach from blitz-dom into dioxus-native-dom, which would make the dependency circular or inverted, and do not make the `element_id` module public in order to share it (architecture §Design Philosophy → Radically modular, embeddable engine; §Existing Scopes).
- **Breaking id rules or node keying.** Do not build the carried id from a `NodeId`, `ElementId` or `ScopeId`, and do not change what the AccessKit `NodeId` means (architecture §Standard Contracts → Dioxus DOM bridge; §Established Decisions → [Node identity]).
- **Back-door DOM writes.** Do not write to the DOM through `DocumentMutator::doc` instead of extending the mutator (architecture §Conventions → DOM API patterns). Do not write the id into the DOM without amending the Dioxus DOM bridge contract.

## Contract bindings
- **arch ↔ a11y.** architecture §Standard Contracts → Dioxus DOM bridge supplies the id. a11y-plan §2 names `packages/blitz-dom/src/accessibility.rs` as the tree builder and HTML-AAM as the role mapping. The carrier joins the two.
- **arch ↔ tests.** The new stand check binds to architecture §Standard Contracts → Headless stand and → Test harness.
  - The harness lists no accessibility-tree read today. If one is added, it extends the harness's public contract, recorded at wrap.
  - The `a11y` CI leg runs only `accessibility_hidden`, `accessibility_roles` and `focusability_updates`, and `test_ci_workflows.py` pins that list (architecture §Standard Contracts → CI contracts). Whether the new `stand_*` file joins that leg or rides the workspace `test` leg is a tests-domain decision.
- **arch ↔ obs.** The per-call-site tracing gate (architecture §Conventions → Feature gating) meets obs-plan's ban on new user-content log fields. Both the id and the accessible name carry author content.
- **arch ↔ wrap.** Any new public surface is a §Standard Contracts amendment at wrap. That covers a `DocumentConfig` hook, a blitz-traits trait, a `DioxusDocument` method or a harness read. A new crate, env var or port is a §Occupied Resources registration; none is expected.

## Acceptance criteria contributions
- On each lean task, in both layout modes, every element-bearing AccessKit node's carried id equals `DioxusDocument::element_id(node_id)`, and the carried ids are pairwise distinct. Nodes stay keyed by the same `NodeId`-derived AccessKit ids as before. (per architecture §Standard Contracts → Dioxus DOM bridge / → Mutation, query and CSSOM)
- blitz-dom's manifest gains no dependency on any dioxus crate, the id crossing defaults to a no-op when no Dioxus document installs it, and no new crate, env var or port is added. (per architecture §Design Philosophy → Embedder services are injected; §Occupied Resources)
- The touched crates build with `accessibility` off (for example `cargo build -p blitz-dom --no-default-features --locked`) as well as with default features. (per architecture §Established Decisions → [Default features]; §Conventions → Feature gating)
- Every new public item carries rustdoc and passes `bash .github/scripts/ci-leg.sh doc`, and each is recorded in §Standard Contracts at wrap. (per architecture §Conventions → Formatting and lints; §Inherited Defaults → Code quality)
