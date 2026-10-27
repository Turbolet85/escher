# arch · cascade read · group 5 of 7

## Carried
- (.andromeda/architecture-detail/standard-contracts.md:196) The chunk re-asserts `stand_lint`'s pinned flight-booker `overlap` and edits `lint.rs`, the file the defect's fix lives in, so the factored findings walk must keep reading two options of one select as overlapping, the one `overlap` at boot and 0 on the other three lean tasks in both layout modes, and must not fold in the `overlaps` fix, which a `.andromeda/residuals.md` line owns.
  > The `overlap` reading of a select's options: the snapshot gives each option its select's client rect, so the lint reads two options of one select as overlapping — on the stand the flight booker reads one `overlap` at boot (`flight-type-one-way` × `flight-type-return`) and the other three lean tasks 0, which `stand_lint` pins as measured in both layout modes; the defect is open, its fix — a driver change for every document in the lint's overlap reading (`packages/escher-driver/src/lint.rs`, `overlaps`) — owned by a `.andromeda/residuals.md` line
  (.andromeda/architecture-detail/standard-contracts.md:242)
  > and an option whose parent is a `select` reads its select's client rect as its `bounds` (the private `select_parent`), so a select's options are its children in the snapshot, named by their labels
  (.andromeda/architecture.md:183)
  > a select's `input` and `change` reach the app as form data, and its options are its children in the snapshot.
  (.andromeda/architecture.md:171)
  > `Session::lint` reads the screen and changes nothing
- (.andromeda/architecture-detail/standard-contracts.md:222) The lint's findings come out in document order, which is the snapshot's pre-order, so a helper or table walk that replaces the per-node ladder must still walk `nodes()` in pre-order and read each node's `id` and `bounds` from the `SnapshotNode` as before.
  > `Snapshot { roots: Vec<SnapshotNode> }` offers `nodes()` (pre-order) and `get(&str)` (by stable element id), a `SnapshotNode` carries `id: String`, `role: accesskit::Role`, `name: String`, `state: NodeState`, `bounds: blitz_dom::BoundingRect` and `children`
  (.andromeda/architecture-detail/standard-contracts.md:223)
  > a box that is itself scrolled included since 2026-10-10-scrolling-box-bounds-and-hit: the reader positions a node through the crate-private `BaseDocument::physical_unrounded_geometry`, which subtracts the scroll offset of every box on the containing-block chain above the node and not the node's own, in both of its bodies, so such a box reads the rect it stands at, the same before and after its content is scrolled, a scrolled box inside a scrolled box moves by the outer offset alone, and a step that scrolls a box does not name it among its changed nodes — on the stand `crud-list` reads the same four figures after its content is scrolled
  (.andromeda/architecture.md:177)
  > under `accessibility` the snapshot, its text form and its diff read the screen read-only, holding no `NodeId`.
  (.andromeda/architecture.md:179)
  > No log or event carries the snapshot, its text or a diff, except a diff's three counts — added, removed and changed — on escher-driver's command span.
- (.andromeda/architecture-detail/standard-contracts.md:249) The listing test the chunk refactors asserts the MCP `tools/list` answer against these twelve verbs of `VERBS`, `lint` after `screenshot` and `help` last, so the refactored test must assert the same twelve rows and the chunk must leave the `'static` schema and every command's written answer, the `lint` verb's included, unchanged.
  > the verbs are the twelve of `VERBS` — `snapshot`, `click --id`, `type --id --text`, `press --key [--shift]`, `advance --ms`, `scroll --id`, `screenshot [--id <id>]`, `lint`, `start <task> [--record <dir>]`, `status`, `stop` and, since 2026-10-12-self-description, `help`
  (.andromeda/architecture-detail/standard-contracts.md:250)
  > a command answers one uncoloured line of JSON on stdout
  (.andromeda/architecture.md:167)
  > the command and refusal schema stated once as `'static` data — twelve verbs, eight closed causes, no refusal holding what a call supplied
  (.andromeda/architecture.md:169)
  > Every answer is written by hand in the crate-private `json` module — no serialization crate writes one, `serde_json` only reading MCP requests and kept checks — and no command's ending writes back anything the caller typed.
- (.andromeda/architecture-detail/standard-contracts.md:273) The requirement sweep test the chunk refactors reads the `stand-repeat` leg from `ci-leg.sh`'s own text, so its factored form must keep reading that script's text and asserting each 0.2.0 requirement's witness inside the leg, and `stand_requirement_sweep.rs` must keep its name so the leg's one `--test` per `stand_*.rs` still runs it.
  > `stand-repeat` (since 2026-10-14-quality-gates, not a fast leg: three rounds, each printing `stand-repeat: round R of 3` and running `cargo test -p blitz-tests --locked` with one `--test` per `tests/blitz-tests/tests/stand_*.rs` and, on the same line, ten named blitz-tests targets outside that prefix as literal `--test` words, each file keeping its name (test-plan §9 → Legs names them), pinned by `test_stand_repeat_leg_runs_three_rounds` and read from the script's own text by `stand_requirement_sweep`'s leg check (test-plan §1),
  (.andromeda/architecture.md:193)
  > `.github/scripts/ci-leg.sh {leg}` is the one leg runner of CI and the dev host
- (.andromeda/architecture-detail/standard-contracts.md:274) The chunk's done-gate is the `fast` leg, which must pass as this contract runs it, stopping at the first red, under `--locked`.
  > `fast` (fmt → clippy → test → ci-scripts, stopping at the first red: the local pre-push gate), every cargo leg `--locked` but `examples/wasm_hello` (no `Cargo.lock`)
  (.andromeda/architecture-detail/standard-contracts.md:272)
  > `doc` (`cargo doc --workspace --no-deps`)
- (.andromeda/architecture-detail/standard-contracts.md:278) The chunk re-reads the blitz-tests target pin that holds `stand_requirement_sweep`'s target, so the refactor must add no `[[test]]` table and leave the manifest's one `all` table and auto-discovery as they are.
  > the blitz-tests target contract — `[package]` does not set `autotests` to false, and the manifest holds exactly one `[[test]]` table, `all` at `tests/all.rs` with `test = false` — pinned by `test_blitz_tests_targets.py` (`BlitzTestsTargetsTest`, one test), run by the same leg, which discovers eight test files
- (.andromeda/architecture-detail/standard-contracts.md:276) The chunk moves `lint.rs::findings` into new helpers or a table in escher-driver, whose CI coverage leg holds a 97 % floor, so every new helper must stay exercised by the lint's existing tests and keep that floor.
  > the `coverage` leg calls it with `packages/escher-driver=97 packages/dioxus-native-dom=83 packages/blitz-test-harness=90`

## Bindings
- **standard-contracts (the select options' `overlap` pin) ↔ the body's Driver session**: the factored walk keeps the lint's one flight-booker `overlap` (.andromeda/architecture-detail/standard-contracts.md:196) and keeps `Session::lint` changing nothing (.andromeda/architecture.md:171).
- **standard-contracts (the snapshot's pre-order and bounds) ↔ the body's Dioxus DOM bridge**: document order and each finding's bounds come from the snapshot node walk the helpers keep (.andromeda/architecture-detail/standard-contracts.md:222), read-only and with no `NodeId` (.andromeda/architecture.md:177).
- **standard-contracts (the CLI's twelve verbs) ↔ the body's Driver session schema and `json` module**: the listing test's twelve rows are the `'static` `VERBS` (.andromeda/architecture-detail/standard-contracts.md:249; .andromeda/architecture.md:167), and the `lint` answer each carrier writes stays the hand-written line (.andromeda/architecture.md:169).
- **standard-contracts (CI contracts: `stand-repeat`, `fast`, `doc`, the target pin, the coverage floor) ↔ the test plan's §1 and §9 Legs**: the sweep's leg check reads the leg the test plan names (.andromeda/architecture-detail/standard-contracts.md:273), and the gate legs run as the body states (.andromeda/architecture.md:193).
