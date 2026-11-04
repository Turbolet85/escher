@extract_section name=relevance
relevant — the probe reads the Dioxus DOM bridge (`packages/dioxus-native-dom/`), the pinned Dioxus 0.7.3 runtime, the headless stand and the harness, and its MountedData fork is an engine change for every document
@extract_section name=constraints
- Dioxus stays at the workspace's "0.7.3" pin, Dioxus 0.8.0-alpha.3 waiting for escher's next version; no patch, vendored copy or pin move of a Dioxus crate (per architecture §Inherited Defaults; per architecture §Project Intent, its upstream-sync line).
- Every bridge read the probe makes stays read-only: architecture §Standard Contracts → Dioxus DOM bridge requires `DioxusDocument::element_id` / `element_ids` to compute ids on demand, written to neither DOM nor vdom, and the snapshot to hold no `NodeId`; whether the component-path tier already names the component (link 1) is research's question.
- DOM events reach the vdom through `data-dioxus-id` and Dioxus skips event kinds with zero handlers (per architecture §Standard Contracts → Dioxus DOM bridge; per architecture §Cross-cutting Patterns → Event processing); whether a node's `click` listener is readable from the listener map today (link 2) is research's question, measured on the stand.
- A MountedData fix is a direct engine change for every document; architecture §Established Decisions records each such change as ratified, flagged upstreamable and with its merge surface a line owned by Upstream sync, so the fix goes to the founder as a fork and the probe itself lands no such change.
- Nothing new lands in occupied resources — no env var, listener, port, workspace crate or binary name (per architecture §Occupied Resources → Environment variables; per architecture §Occupied Resources → Network ports and listeners; per architecture §Occupied Resources → Names).
- The stand boots offline in a pinned 800x600 Light viewport, installs no telemetry and reads no env var (per architecture §Standard Contracts → Headless stand); a probe run on Counter or CRUD keeps to that boot.
- Both layout modes produce identical layouts and ids resolve through versioned `NodeId`s (per architecture §Design Philosophy, its "One incremental pipeline, safe identities" principle); a probe reading that touches layout runs in both modes.
@extract_section name=patterns
- The four-tier stable id (author key, anchored path, component path, document path) on `DioxusDocument` is the bridge's existing node-identity reader (per architecture §Standard Contracts → Dioxus DOM bridge).
- Shared state is `Rc<RefCell<…>>` and Dioxus services are root contexts (per architecture §Cross-cutting Patterns → Service wiring); this is the borrow shape the MountedData hazard's `RefCell already borrowed` panic is measured against in `poll` / `render_immediate`, which P3 re-verifies at HEAD.
- The harness pumps and never settles, holds one shared `VirtualClock` moved only by `tick` / `advance_time`, and `settle` never waits on a load (per architecture §Standard Contracts → Test harness); a headless probe drives the stand through it.
- Mutations go through `DocumentMutator`, which flushes deferred work on `Drop` (per architecture §Conventions → DOM API patterns); a probe that writes state does so through the app's handlers, not by mutating the DOM directly.
- Stand apps hold reactive state with `#[derive(Store)]` and signal and store hooks under `#[component]` components (per architecture §Conventions → Apps and examples); links (3) and (4) are read against those hooks.
@extract_section name=anti_patterns
- No fork-level change: no Dioxus crate edit or pin move and no engine change for every document landed by the probe (per architecture §Inherited Defaults; per architecture §Established Decisions).
- No log, event or field carries a snapshot, an id, a name or a value the probe reads (per architecture §Standard Contracts → Dioxus DOM bridge, its no-log line; per architecture §Cross-cutting Patterns → Logging and timing).
- No raw indexing of `NodeTree` by an id; read ids through `get_node`, since stale-id indexing panics (per architecture §Standard Contracts → Document core; per architecture §Standard Contracts → Scrolling, selection, tree).
@extract_section name=bindings
- arch ↔ tests: a probe check lives in `tests/blitz-tests/`, one test target per file opening with a `//!` doc and run in both layout modes, its fixture first asserting the condition (per architecture §Conventions → Tests); the harness contract binds the probe's runs.
- arch ↔ obs: the probe adds no span or log field, and the stand installs no telemetry (per architecture §Standard Contracts → Telemetry bootstrap; per architecture §Standard Contracts → Headless stand).
- arch ↔ security: values the probe reads stay masked and never cross; the driver session's no-`NodeId`, no-value rule stays as it is (per architecture §Standard Contracts → Driver session).
- arch ↔ all domains: the work stays in the locked crate names `dioxus-native-dom`, `seven_guis` and `blitz-tests` (per architecture §Existing Scopes).
@extract_section name=acceptance
- `Cargo.toml` still pins Dioxus "0.7.3", and the chunk's diff touches no Dioxus crate source, patch section or vendored copy (per architecture §Inherited Defaults).
- The chunk's diff adds no env var read, listener, workspace crate or binary (per architecture §Occupied Resources → Names).
- Any probe check is its own test target under `tests/blitz-tests/tests/`, with a `//!` doc naming its behavior, and runs in both layout modes where layout touches the reading (per architecture §Conventions → Tests).
- The chunk lands no change under `packages/blitz-dom/` or to the bridge's event path for every document unless the founder's fork chooses a fix here; if one lands, it is recorded as a direct engine change with its merge surface (per architecture §Established Decisions).
