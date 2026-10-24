# arch extract

## Relevance
relevant: an upstream sync touches the workspace's pins, the engine crates' merge surfaces, the Dioxus bridge, blitz-tests' target layout and the plan's own `Upstream sync:` record.

## Constraints
- The coupled pins move as one set: the parser crates track stylo's web_atoms, skrifa tracks parley/vello, svgtypes tracks usvg. Taffy and parley are registry versions, so the manifest holds no git dependency. A take that carries upstream's git Taffy (`802e497b…`), git Stylo (`d6502e00…`) or Dioxus `0.8.0-alpha.3` contradicts this decision as recorded. That makes it the P4 fork, not an implementation detail (per arch §Established Decisions → Dependency pinning).
- The inherited framework pins are Stylo "0.22.0", taffy "0.15" from the registry, parley "0.12", Dioxus "0.7.3", winit "=0.31.0-beta.3" and rust-version "1.91.0". Any take that moves one of them also moves the inherited-default record (per arch §Inherited Defaults).
- MSRV is checked by a build-only job, and the plan makes an effort not to raise MSRV. Whether a Stylo 0.23-pre or Dioxus 0.8 take builds on 1.91 is the MSRV job's reading, not something to assume (per arch §Established Decisions → MSRV; per arch §Design Philosophy).
- Both layout modes must produce identical layouts. A dropped node's id stops resolving and never aliases a reused slot. Every public API takes `NodeId`. Upstream's hit-test-of-dropped-nodes (`f102663b8`) and stale-node-mapping (`286874cc1`) changes land on this invariant (per arch §Design Philosophy).
- The document clock invariant is required on the take's tree: `DocumentConfig.clock`, the root re-exports `Clock` · `WallClock` · `VirtualClock`, and no `Instant::now()` or `SystemTime::now()` in blitz-dom outside `clock.rs`. The changed-set contract (`has_changes` / `take_changed_nodes`, ids read through `get_node`) holds through any `mutator.rs` / `scrolling.rs` conflict resolution (per arch §Standard Contracts → Document core).
- The engine changes made for every document are each flagged upstreamable with no upstream pull request open: the scrolled-box client rect and clipped-box hit, the accessibility state writes, range input, keyboard activation, the seven engine-debts fixes, the document clock and the select model. Their merge surfaces are owned by CARRYs on this entry. A resolution keeps their behaviour, and nothing is sent upstream (per arch §Established Decisions → Scrolling and input).
- The plan names no new env var, crate, binary or listener. The env var registry is `RUST_LOG`, `WPT_DIR`, `PAINT_TREE_BENCH_HTML`, test-only `TMPDIR` and the CI script's GitHub variables. Research must decide whether the env reads in upstream's root example `examples/multi_doc.rs` (`MULTI_DOC_N` and a parameter-named read) count as a registrable env read (per arch §Occupied Resources → Environment variables; per arch §Occupied Resources → Names).

## Patterns to follow
- Earlier syncs took part of upstream by no-commit cherry-picks of upstream's text, each with its `(cherry picked from commit …)` trailer, leaving the merge base unmoved. The four Taffy-main commits stayed out because they put a git Taffy in the root manifest. That shape is the precedent for a measured partial take (per arch §Project Intent).
- An upstream change to `README.md` resolves to escher's version at every sync (per arch §Project Intent).
- blitz-tests keeps one test target per file directly under `tests/`, with no `autotests` setting. Upstream's `tests/all.rs` is the one `[[test]]` table, held at `test = false` and pinned by `BlitzTestsTargetsTest`. New upstream test files become targets by auto-discovery, and `all.rs` takes upstream's `mod` lines. The plan already cites `tests/blitz-tests/tests/stale_node_mapping.rs:2`, so whether that file is already on our branch is a question for research (per arch §Conventions → Tests).
- Dependencies are declared once in `[workspace.dependencies]` and used as `{ workspace = true }`. In-repo crates are declared with `default-features = false`, except the `seven_guis` path entry (per arch §Conventions → Manifests).
- Every gate runs as `bash .github/scripts/ci-leg.sh {leg}` on CI and on the dev host. The audit leg is cargo-deny over the root `deny.toml`. The coverage leg's `--ignore-filename-regex 'keyboard_compat\.rs$'` exclusion is tied to upstream's temporary `keyboard_compat.rs` bridge, so it drops only with a take that removes that file (per arch §Standard Contracts → CI contracts; per arch §Infrastructure Patterns → Build system).

## Anti-patterns to avoid
- Never bump one side of a coupled pin alone, and never patch a pin set that fails to resolve with a one-sided fix (per arch §Established Decisions → Dependency pinning).
- Never add a new env var, crate, port or listener silently. Each one is an Occupied Resources registration first (per arch §Occupied Resources → Network ports and listeners).
- When resolving a `mutator.rs` conflict, do not reach through `DocumentMutator::doc`. Mutations stay inside the mutator, which flushes on `Drop` (per arch §Conventions → DOM API patterns).

## Contract bindings
- arch ↔ security: the dependency audit (cargo-deny, `deny.toml`, the tokio RUSTSEC-2026-0386 fix by a semver-compatible update) and hand review of the new `mimalloc` dependency in `apps/browser` and the WPT runner. Arch's side is the Stack and Infrastructure record of the audit gate (per arch §Infrastructure Patterns → Build system; per arch §Stack and Technologies).
- arch ↔ obs: the sink allowlist that the entry re-checks is arch's telemetry bootstrap contract. Targets outside the engine and escher prefix sets are dropped whole, and engine targets print only `SAFE_FIELDS` (per arch §Standard Contracts → Telemetry bootstrap; per arch §Cross-cutting Patterns → Logging and timing).
- arch ↔ tests: blitz-tests' target layout is pinned by `.github/scripts/test_blitz_tests_targets.py`. The per-OS tallies count those targets (per arch §Conventions → Tests).
- arch ↔ a11y: the accessibility builder's naming and state-write contract, which upstream's `06ecec886` reaches in `accessibility.rs` (per arch §Standard Contracts → Mutation, query and CSSOM).
- arch ↔ dioxus bridge: stable element ids are computed on demand and written to neither DOM nor vdom. The snapshot, its text and its diff read the screen without writing to it. That contract must survive a take of `286874cc1` (per arch §Standard Contracts → Dioxus DOM bridge).

## Acceptance criteria contributions
- (arch) The root manifest holds no git dependency, and any pin that moved, moved together with its coupled partners. If the take carries a git dependency, it lands only on a recorded founder ruling, with §Established Decisions → Dependency pinning and §Inherited Defaults amended to match (per arch §Established Decisions → Dependency pinning).
- (arch) On the take's tree, blitz-dom has no `Instant::now()` or `SystemTime::now()` outside `clock.rs`, and `has_changes` / `take_changed_nodes` keep their contract (per arch §Standard Contracts → Document core).
- (arch) No new workspace member, binary, listener or escher-crate env read is added. Each upstream env read is either registered or recorded as not registrable (per arch §Occupied Resources → Environment variables).
- (arch) At the wrap, the plan's `Upstream sync:` line states the measured merge base or cherry-picked shas and the upstream sha last read (per arch §Project Intent).
