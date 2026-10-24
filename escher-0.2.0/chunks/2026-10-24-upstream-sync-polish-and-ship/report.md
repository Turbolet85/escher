# Report — 2026-10-24-upstream-sync-polish-and-ship

**Chunk:** Upstream sync ahead of polish and ship. upstream/main `f9cbb2756` was measured: sixteen commits past `17de049b1`, among them git Stylo 0.23-pre, two git Taffy bumps, Dioxus 0.8.0-alpha.3 and engine code, with four conflicts. The take is a fork of the pin. The audit red on tokio RUSTSEC-2026-0386 is folded in. CI and the audit are green, the per-OS tallies re-read and the sink allowlist re-checked.
**Date:** 2026-10-24
**Commits:** since `last_wrap` 2026-10-20T12:23:10Z (`git log --since`): `34f6f051b` chore(2026-10-24-upstream-sync-polish-and-ship): operator pre-CI commit, for the run this chunk's verdict reads · `a356f9086` · `94d756139` · `c1896916d` · `200643656` · `caf8e6303` · `d1ddbb19b` chore(u55): the door — {design-system, obs-plan, a11y-plan, security-plan, test-plan, architecture} split into its body and its category files · `be671bbc0` chore(handoff) · `f1f0c2a40` chore(epoch4) · `cbe90e224` feat(2026-10-20-large-screen-in-parts). This chunk's own commit is `34f6f051b` alone; its base is `a356f9086` (Setup 4: the parent of the oldest pre-CI commit).

## Changes (structured — detectors read this)
- **Files:** 27 code paths, exactly research's two lists (`gate.py scope`: `clean — changed 27 · listed 27 · recorded 0`; base `a356f908`).
  - New, upstream's text as written: `packages/blitz-dom/src/layout/inline/build.rs` (303 lines, from `28f848d3b`), `packages/blitz-dom/src/layout/inline/mod.rs` (4 lines), `examples/multi_doc.rs` (116 lines, from `fb297a38c`), `tests/blitz-tests/tests/hit_test_dropped_nodes.rs` (82 lines, from `f102663b8`).
  - Renamed: `packages/blitz-dom/src/layout/inline.rs` → `packages/blitz-dom/src/layout/inline/layout.rs` (1175 lines), by `28f848d3b`, then `21edc92d9`'s hunk.
  - Modified, upstream's hunks: blitz-dom `layout/{construct,mod,paint_tree,replaced}.rs`, `node/{node,element,mod}.rs`, `mutator.rs`, `stylo.rs`, `config.rs`, `resolve.rs`; `packages/dioxus-native/src/lib.rs`; `apps/browser/{Cargo.toml,src/document_loader.rs,src/toolbar.rs}`; `wpt/runner/{Cargo.toml,src/main.rs,src/test_runners/attr_test.rs}`; `tests/blitz-tests/tests/all.rs`; `flake.nix`.
  - Lock: `Cargo.lock`.
  - Chunk evidence: `evidence/{before,take,lock,readings,sink,counts,ci,operator-pass}.md`.
  - The ten upstream commits, taken by `git cherry-pick --no-commit` in this order (each a `(cherry picked from commit …)` trailer on `34f6f051b`): `28f848d3b` (#1233, inline layout module split) · `9eeebb302` (#1239, mimalloc in the browser and WPT runner) · `e4ca5b9b5` (#1240, class atoms on `ElementData`) · `46d7cf993` (#1238, out-of-flow-ness on `Node`) · `fb297a38c` (#1243, Parallel style traversals serialised; `examples/multi_doc.rs`) · `50bf2f235` (#1246, a replaced element's natural height through `aspect-ratio`) · `f8723f495` (#1247, WPT runner) · `21edc92d9` (#1258) · `f102663b8` (#1244, dropped nodes skipped in hit testing) · `f9cbb2756` (#1264, the flake's toolchain). The merge base `23b9b26a9` is unmoved. On the founder's word «Десять коммитов (Рекомендую)» (inputs#I4).
- **Symbols / APIs** (coordinates from `## New text, by line`):
  - `Node::out_of_flow: AtomicBool` — a new pub field (`packages/blitz-dom/src/node/node.rs` added 125-128), written by the style traversal from the node's new primary style through the new `pub(crate) fn Node::record_out_of_flow` (added 1271-1279). Its callers are `stylo.rs` added 1342-1344 and 1459 and `construct.rs` added 720 and 775. Beside them is the new `pub(crate) fn style_is_out_of_flow` (`node.rs` 2069-2076: `Absolute | Fixed` and not `display: none`). `Node::is_out_of_flow` keeps its signature and its style reading, now through `style_is_out_of_flow`. Its other callers (`resolve.rs`, `writing_mode.rs`, `layout/mod.rs` `dispatch_child_layout`) still read the style. Only `LayoutPassState::is_out_of_flow` (`layout/mod.rs` added 856-858) now reads the recorded flag (research.md §Graph impact).
  - `ElementData::classes: ThinVec<Atom>` — a new pub field (`packages/blitz-dom/src/node/element.rs` added 56-61), filled from the `class` attribute by `ElementData::new` (added 422-426, 431) and `Clone` (313). `DocumentMutator::set_attribute`'s `class` arm (`mutator.rs` added 375-378, after the `id` arm, ahead of the `value` arm) and `remove_attribute`'s `classes.clear()` (added 499-502) keep it in sync. The new `pub(crate) fn parse_classes` (`element.rs` 45-47) is re-exported `pub(crate)` from `node/mod.rs` (added 22). Stylo's `has_class` and `each_class` now read `ElementData::classes` (`stylo.rs` added 586-590, 697-699, 710). `ElementData` is referenced outside blitz-dom by type only, never by a struct literal (research.md §Graph impact, nine sites), so no caller changes.
  - `static PARALLEL_TRAVERSAL_LOCK: Mutex<()>` — new process-wide state in `packages/blitz-dom/src/stylo.rs` (added 29-35), taken around every `StyleThreading::Parallel` traversal (added 157-163). Poison is recovered by `unwrap_or_else(|e| e.into_inner())`, so there is no panic path. Two documents resolving `Parallel` concurrently are now serialised and no longer panic with `already mutably borrowed` (blitz issue 430).
  - `StyleThreading` docs (`packages/blitz-dom/src/config.rs` added 13-17, 23-24, 54): the enum doc now says `Parallel` traversals are serialised process-wide, and the `style_threading` field doc says the default is `Sequential` (the enum's `#[default]` already was). Doc comments only; the `clock` field is untouched.
  - `dioxus_native::launch` (`launch_cfg_with_props`) now builds its document with `style_threading: StyleThreading::Parallel` (`packages/dioxus-native/src/lib.rs` added 63, 222). This is the windowed path (`seven_guis_native` through `dioxus_native::launch`). The browser's `document_loader.rs` (added 62) and `toolbar.rs` (added 203) do the same. No headless document — harness, stand or driver — sets `style_threading` (research.md §Sweeps: 7 `git grep` hits, all in blitz-dom).
  - `build_inline_layout_into` moved from `layout/construct.rs` to `layout/inline/build.rs` (23-303), re-exported `pub(crate)` by `layout/inline/mod.rs`. `resolve.rs` imports it from `inline` (added 29, 32). `construct.rs`'s import list is resolved to upstream's plus our `RangeInputData` (the take's one non-additive edit, `evidence/take.md`).
  - Hit testing (`Node::hit_inner`, `node.rs` added 1414-1417 doc, 1572-1574, 1578, 1587-1589, 1621-1623, 1627, 1646-1650) skips nodes dropped since the last resolve: `self.with(id)` → `self.tree().get(id)` with `continue`, in the positive-z hoisted loop, the paint-children loop, the negative-z hoisted loop and the text-cluster lookup. `paint_tree.rs` (added 146-147, 156, 161, 164-172) does the same. `hit_inner` keeps its signature; its callers `hit` and `hit_with_scrollbar` are unedited.
  - `compute_replaced_layout` (`packages/blitz-dom/src/layout/replaced.rs` added 144-145, 201-211) coerces a replaced element's natural height through its `aspect-ratio`. It keeps `is_usable_ratio` (finite and > 0) and `.max(0.0)`, and keeps its signature. Its callers `layout` and `dispatch_child_layout` see the new height.
  - `examples/multi_doc.rs` (new, root example): three env variables read through two `std::env::var` call sites — `MULTI_DOC_N` (55-58, parsed with a default fallback), `MULTI_DOC_SEQ` (36-40) and `MULTI_DOC_FLAT` (22), both through `env_flag` (17-19). It also reads `std::env::args` (48-54), has one `println!` (111-115), two `Instant::now()` (61, 64) and a `panic!` on an unknown mode (108).
  - WPT runner: `#[global_allocator] static GLOBAL: mimalloc::MiMalloc` (`wpt/runner/src/main.rs` added 1-3); `attr_test.rs` gains the reads at added 3 · 97 · 135-136 · 141-147 · 164-165, including a `get_node(node_id).unwrap()` on an upstream-only path.
  - Three engine changes for every document, ratified by the founder's word (inputs#I4; the playbook's «An engine behaviour change for every document»): a replaced element's natural height follows its `aspect-ratio`; a hit test skips nodes dropped since the last resolve; the windowed `dioxus_native::launch` resolves styles `Parallel` under the process-wide lock.
- **Crates / modules:** module `blitz-dom::layout::inline` is now a directory (`build.rs` · `layout.rs` · `mod.rs`). The root examples gain `multi_doc`. No crate is added or removed: the root manifest is byte-identical (gate entry 6).
- **Dependencies:** no package enters or leaves `Cargo.lock` (987 → 987, `evidence/lock.md`). Moves:
  - **tokio** 1.53.2 → 1.54.1 in the lock alone (`cargo update -p tokio`), past RUSTSEC-2026-0386. The manifest's `tokio = "1.42"` is unchanged.
  - Twelve edges re-pointed by cargo to versions already locked: eleven `windows-sys`, plus `cssparser-macros 0.7.1`'s `syn` 3.0.6 → 2.0.119.
  - **mimalloc** 0.1.52 / **libmimalloc-sys** 0.1.49, already locked, now the default global allocator of `apps/browser` (`default` gains `"mimalloc"`) and a direct dependency of the WPT runner (`mimalloc = "0.1.48"`, one added lock edge). Hand review re-read: MIT; repository `purpleprotocol/mimalloc_rust`; libmimalloc-sys builds vendored C through `cc` (`build.rs`, `links = "mimalloc"`); neither a proc-macro; reaching only upstream-only binaries.
  - The pins stay: Stylo 0.22.0, taffy 0.15 from the registry, Dioxus 0.7.3, no git dependency (gate entries 6, 7).
- **Schema / config:** no config key added. `DocumentConfig.style_threading`'s doc now names `Sequential` as the default (doc only; the default is unchanged). No scrub set, redaction shape or violation schema moved: `packages/escher-telemetry/`, `packages/escher-driver/` and `packages/dioxus-native-dom/` are unchanged (gate entry 6).
- **Spec-master edits:** none.
- **Retired:** none. `layout/inline.rs` moved to `layout/inline/layout.rs`, a path change with no surface removed. The `StyleThreading` doc's warning that two `Parallel` documents panic is retired with the lock.
- **Counts / qualifiers moved:**
  - Local workspace tally `lines 205 passed 1098 failed 0 ignored 12` → `lines 206 passed 1099 failed 0 ignored 12` (gate entry 20 over `target/ci-logs/test.log`). +1 line and +1 passed, `hit_test_dropped_nodes`. Pattern `tally-local`: 1 standing (test-plan.md:396, the «Local baseline» label, no figure on it — no-change); the figures stand in `.andromeda/test-plan-detail/ci-integration.md` (`git grep -c -E '205 · 1098|1098 passed|186 · 1095|186 · 1051|1095 passed|1051 passed' -- '.andromeda/*-detail/*'`: 2 lines — changed).
  - Per-OS CI tallies vs CI#39410865604:
    - Test [default features] 205 · 1098 · 0 · 12 → 206 · 1099 · 0 · 12;
    - Test (macos) 186 · 1095 · 0 · 12 → 187 · 1096 · 0 · 12;
    - Coverage report 186 · 1095 · 0 · 12 → 187 · 1096 · 0 · 12;
    - Test (windows) 186 · 1051 · 0 · 9 → 187 · 1052 · 0 · 9;
    - Accessibility (a11y) tests 6 · 43 · 0 · 0 and Stand checks repeated 168 · 684 · 0 · 21, unmoved.

    Every move is +1 line and +1 passed, `hit_test_dropped_nodes` (not unix-gated). On CI#40092092468 (gate entry 39, `evidence/ci.md`). Pattern `tally-os`: 1 standing (test-plan.md:391, the «Per-OS test tallies» label — no-change); the rows stand in `ci-integration.md` (the same `git grep`, 2 lines — changed).
  - `run all` run.end passed 616 → 617 (+1 `hit_test_dropped_nodes`); `run stand` 195 · 0 · 7 over 48 files, unmoved (gate entries 29, 31). Pattern `run-all`: 7 standing (test-plan.md:170, 172, 173, 174, 175, 189 — 6 no-change: dated re-count history and the selection's name; test-plan.md:176 — changed: the latest link of the re-count chain, this chunk's 195 · 0 · 7 over 48 files and `run all` 617 · 0 · 12 to append). `616` stands in no detail file (`git grep -c -E '616 passed|passed 616|616 · 0'`: 0).
  - `tests/blitz-tests/tests/*.rs` 144 → 145 (gate entry 22). Pattern `files-dir`: 1 standing (test-plan.md:108, the «Directory pattern», no count on it — no-change). 0 detail-file hits (`git grep -c -E '144 (\.rs|files)|144 →'`: 0).
  - `hit_test_dropped_nodes` — a new blitz-tests target, 1 test. Pattern `new-target`: control unfired over the seven masters (the sweep's refusal line `control: title, owned, new-target, replaced, classes, multi-doc, mimalloc, tokio never fired over the seven masters' pre-pass text (34f6f051)`); 0 detail-file hits.
  - `stand_*.rs` 48, the a11y leg 6 · 43, seven_guis 12 lines · 36 passed, blitz-dom lib 106, the CARRY integration set 19 targets / 106, CI scripts `Ran 100 tests`: each unmoved (`evidence/counts.md`).
  - Sweep reach: 12 patterns · 17 moved facts, 17 with a pattern (the report's own count: one fact per pattern of the 20 in `report-patterns.toml` but `title` and `owned` (the owner patterns) and `kbd-compat` (a re-read, unmoved)) · owner pattern control unfired · by line 0 lines · 0 offsets. Two calls: the first `cascade.py sweep --pre-pass` over all 20 patterns exited 3 on the refusal line above; the re-fire over the 12 that fired printed `total (12 patterns) · 88 rows over 25 files` and `by line · 0 lines · 0 offsets`. **Its reach:** since the U55 door (`d1ddbb19b` … `a356f9086`) each master is a body plus `.andromeda/{doc}-detail/` category files, and the sweep reads the bodies and `.andromeda/registries/**` alone. So the eight unfired patterns' sites stand in the category files, located by `git grep` (each count below names it).
- **Dev-tool versions:** none — cargo-deny re-read at 0.20.2 (dev host, `cargo deny --version` at /implement Setup). The flake's `rust-bin` toolchain pin 1.90.0 → 1.91.0 (`flake.nix` added 31, `f9cbb2756`) is a Nix-shell pin, no host tool read here. tokio is a lockfile crate, not this line's subject.
- **Harness / gate surface:** none — no script, CI file or `scripts/` file changed (gate entry 6, the preservation guard over `.github scripts`).
- **Cross-project / external claims:**
  - upstream DioxusLabs/blitz `main` read at `f9cbb275628b8f3017df8f76150f5f32df218620`, 16 commits past `17de049b1` (inputs#I2). The ten taken shas are byte-checked by gate entries 1–4. Left out: `06ecec886` (git Stylo 0.23-pre) with `3954ecb85` and `1c5265a8b`; `b8f4ba6fd` and `9babd796a` (git Taffy → `802e497b…`), with CARRY 1's four `1a11761bc` · `4a5ab9cc9` · `510b592ab` · `1e5479ae6`; `286874cc1` (Dioxus 0.8.0-alpha.3). The reasons are measured in research.md §The trial takes.
  - RustSec RUSTSEC-2026-0386 (tokio `AsyncFd::new` I/O safety; fix ≥ 1.53.3) — cleared, audit `advisories ok, licenses ok` (gate entry 25).
  - **CI:** CI#40092092468, push, on `34f6f051b81460d82837f9433dbe1faebaf49e56` (the pre-CI commit) — `verdict: green · checks 17/17 · wall 1210 s` (gate entry 38, `ci.py conclusion --sha HEAD --wait 1800`). The verdict was taken on that sha; this wrap's commit adds to it. The base's CI#40081725681 on `a356f9086` was red by Dependency audit alone.
  - inputs (`inputs.py verify`, fired before this report: `inputs: 7 entries — unchanged 1 · drifted 0 · vanished 0 · broken 0 · altered 0 · unreachable 0 · n/a 6 · uncited 3 · unparsed 0`; the three uncited are I5, I6, I7, cited here):
    - I1 · message (the /andromeda-phase invocation, the founder's word relayed by overseer1) · copy message · n/a — inputs#I1, the run mode: a U55 live-proof run, the forced sample.
    - I2 · message (the orchestrator's git, GitHub Actions and crates.io measurement, 09:33Z) · copy message · n/a — inputs#I2.
    - I3 · message (P3's external reads, 11:06Z) · copy message · n/a — inputs#I3 (its "twelve `windows-sys` edges" is eleven, corrected in research.md and `evidence/lock.md`).
    - I4 · `../additional/escher-overseer/relays/upstream-sync-epoch5-founder-2026-10-24.md` · copy no-repo · unchanged — inputs#I4, the founder's «Десять коммитов (Рекомендую)».
    - I5 · message (the operator, in the session, 2026-10-24T11:36Z) · copy message · n/a — inputs#I5, the operator's «yes — apply» on `.andromeda/runs/2026-10-24T09-31-30-phase/p5-card-1.md`. The card's `## Beyond the plan` rows, verbatim, approved by the operator's own word (inputs#I5: "It approves the plan and both `## Beyond the plan` rows"):

      | # | Proposal | Owner / place |
      |---|---|---|
      | 1 | **The upstream delta this take leaves out**, which moves to escher's next version:<br>• `06ecec886` (git Stylo 0.23-pre) with `3954ecb85` and `1c5265a8b`, plus our `contain: paint` hit stop's getter under Stylo 0.23;<br>• `b8f4ba6fd` and `9babd796a`, with CARRY 1's four Taffy-main commits (git Taffy `802e497b…`);<br>• `286874cc1` (Dioxus 0.8.0-alpha.3), with the port of escher's stable-id derivation onto its vdom (10 + 15 errors measured), and CARRY 2's `keyboard_compat.rs`, retired upstream there.<br>Measured at this chunk's `research.md` §The trial takes. | A new `.andromeda/residuals.md` line, origin `2026-10-24-upstream-sync-polish-and-ship`, `(target: next)`, appended by the wrap's route-resolve. No later upstream-sync entry stands in escher-0.2.0's route. |
      | 2 | **CARRY 3, 4, 5:** the merge surfaces of 2026-10-18-engine-debts, 2026-10-18-one-virtual-clock and 2026-10-20-select-by-id are re-read at the next sync, with nothing sent upstream. | A new `.andromeda/residuals.md` line, origin `2026-10-24-upstream-sync-polish-and-ship`, `(target: next)`, beside row 1 |
    - I6 · message (the operator, a message in this session, 2026-10-24 11:55Z) · copy message · n/a — inputs#I6, the operator-pass directive: entries 36–39 fired by hand in plan order, entry 38 with `--wait 1800` in place of the plan's 2400, recorded in `evidence/operator-pass.md`.
    - I7 · message (the operator, the wrap invocation, 2026-10-24 12:21Z) · copy message · n/a — inputs#I7: this wrap runs P1 only and stops; at P2, after the clear, the operator's word will be «forced sample» (a U55 live-proof chunk).
- **Reverted / negative API facts:** none.
- **Insufficient fixes (written, kept, not the remedy):** none.
- **Spec claims disproved by measurement:**
  - The plan's CARRY 4 forecast (plan.md Implementation Steps 5: "CARRY 4, per file: only `config.rs`'s doc comments are reached") — measured false. `packages/blitz-dom/src/resolve.rs`, in CARRY 4's file list, is reached too, by `28f848d3b`'s import list alone (+2 −2, `build_inline_layout_into` from `inline`), with no time read touched (`git diff --cached --stat a356f9086 -- …`, `evidence/readings.md`). No master site: a plan forecast, not a master's claim. `established-decisions.md:76` lists `resolve.rs` among the clock surface's files and says nothing of reach.
  - inputs#I3 item 1's "twelve `windows-sys` edges" — eleven, plus one `syn` (corrected at P3 in research.md; re-measured by parsing both locks, `evidence/lock.md`). No master site.
- **Expected amendments (from plan):**
  - architecture §Project Intent (`Upstream sync:`, project-intent.md:9 and the body) — carried: Changes → Files (the ten commits, the merge base unmoved) and Cross-project (upstream read at `f9cbb2756`, six left out with reasons, plus CARRY 1's four). Pattern `upstream`: 1 standing (architecture.md:286 ×3 — changed: "upstream `main` last read at `17de049b1`, three of its commits taken …, the four Taffy-main commits waiting" now reads `f9cbb2756`, ten taken, six Taffy-main waiting); `git grep -c -E 'Upstream sync:|17de049b1'`: project-intent.md 1, security-plan-detail/dependency-security.md 1 — changed.
  - architecture §Established Decisions → [Style threading] (body :44) — carried: Changes → Symbols (`PARALLEL_TRAVERSAL_LOCK`, the field doc `Sequential`, `dioxus_native::launch` `Parallel`). Pattern `threading`: 1 standing (architecture.md:44 ×4 — changed); `git grep`: occupied-resources.md 1, standard-contracts.md 1 — read at P2.
  - architecture §Established Decisions, replaced elements (established-decisions.md:84); §Design Philosophy :53 (`Node::out_of_flow`, `ElementData::classes`); hit testing skipping dropped nodes — carried: Changes → Symbols, each. Patterns:
    - `replaced`: control unfired in the bodies; `git grep -c -E 'aspect-ratio|natural height|compute_replaced_layout'` reads standard-contracts.md 1 (the plan's established-decisions.md:84 is worded otherwise — the detectors read it).
    - `out-of-flow`: 1 standing (architecture.md:249 — no-change: the out-of-flow pass inside the cache wrapper, a true claim sharing the token); `git grep` established-decisions.md 1, stack-and-technologies.md 1.
    - `classes`: unfired, 0 detail hits.
    - `new-target`: unfired, 0 detail hits.
  - architecture §Occupied Resources → Environment variables (occupied-resources.md:55); security-plan §Secret Management, Environment values read (security-plan.md:294) — `MULTI_DOC_N` · `MULTI_DOC_SEQ` · `MULTI_DOC_FLAT` — carried: Changes → Symbols (`examples/multi_doc.rs`). Pattern `multi-doc`: unfired, 0 detail hits — a new registration at both sites.
  - architecture §Occupied Resources → Process-wide state and threads (occupied-resources.md:38) — `PARALLEL_TRAVERSAL_LOCK`, mimalloc the global allocator of `apps/browser` (default) and the WPT runner — and §Stack and Technologies (stack-and-technologies.md:60, :69; body :21: mimalloc by default; the flake's rust-bin 1.91.0, retiring the recorded flake drift) — carried: Changes → Symbols and Dependencies, Dev-tool versions (the flake pin). Patterns:
    - `mimalloc`: unfired, `git grep` stack-and-technologies.md 1;
    - `threading`: above;
    - `flake`: 9 standing — architecture.md:37 ×2, :224 ×3, :296, :310; security-plan.md:236, :251, :264; design-system.md:161 ×2; obs-plan.md:66. All no-change: `flake.nix:N` citations and file lists, with the line count unmoved by a one-line change at 31. The citation sweep owns the digits;
    - `rust-bin|flake drift|1\.90\.0` by `git grep`: stack-and-technologies.md 1 — changed (1.90.0 → 1.91.0). The leaf `.claude/docs/stack.md:8` ("rust-bin stable 1.90.0") is the cascade's.
  - architecture §Existing Scopes (existing-scopes.md:19, :46, :123) — `inline` now a directory; root examples gain `multi_doc` — carried: Changes → Crates / modules. Pattern `inline`: 2 standing (architecture.md:276 ×2 `layout/inline.rs:236`…; security-plan.md:363 `layout/inline.rs:732` — changed: the path moved to `inline/layout.rs` or `inline/build.rs`); `git grep -c -E 'layout/inline|inline\.rs'`: conventions.md 2, established-decisions.md 2, existing-scopes.md 2, input-validation.md 1 — changed (path re-points).
  - security-plan §Dependency Security (Pinning; Audit tool; the 2026-10-24 sync record in dependency-security.md) — carried: Changes → Dependencies and Cross-project. Pattern `tokio`: unfired in the bodies; `git grep` 0 for `RUSTSEC-2026-0386|tokio 1\.53|tokio 1\.54` in detail — a new record; `Taffy-main` in dependency-security.md 2 — changed (four → six).
  - security-plan §Input Validation and §Error Handling (the layout guards' cites re-pointed; `replaced.rs`'s degenerate-ratio guard kept, input-validation.md:245, :248; `attr_test.rs`'s `get_node(node_id).unwrap()`; `multi_doc.rs`'s `panic!`) — carried: Changes → Symbols. Pattern `validation`: 3 standing — architecture.md:268 (`attr_test.rs:38-40` citation — the citation sweep's digits); security-plan.md:121 and :141 (degenerate aspect ratios — no-change: `is_usable_ratio` kept).
  - obs-plan §3 (Logging stack, the silence census) and §6 (examples emitter); test-plan §2 (144 → 145), §9 (Local baseline and Per-OS tallies, the measured rows), §10 (the `keyboard_compat` exclusion standing, CARRY 2) — carried: Changes → Counts / qualifiers moved and Coverage of new surfaces. Patterns:
    - `tally-local`, `tally-os`, `files-dir`, `run-all`: above;
    - `kbd-compat`: 2 standing (test-plan.md:398, :408 — no-change: the exclusion stands, `keyboard_compat.rs` kept, gate entry 15).
  - every master — `file:line` citations into files the take moved, re-pointed by its line map — no master site of its own: P2's citation sweep (`cites.py apply`) owns it, over `construct.rs`, `inline.rs` → `inline/{build,layout}.rs`, `node/{node,element}.rs`, `mutator.rs`, `stylo.rs`, `config.rs`, `layout/{mod,paint_tree,replaced}.rs`, `resolve.rs`, dioxus-native `lib.rs`, `wpt/runner/src/{main.rs,test_runners/attr_test.rs}`.
  - the sampler's Lost items, taken as carried (sampler) — carried as plan.md §Implementation notes dispositions each, held by the gates above (all green) or the citation re-point; no further master site.
  - **The CARRY 3–5 owners** (not on the plan's list; motivating fact: this chunk closes the entry that owns them). Owner pattern `title` («Upstream sync ahead of polish and ship»): control unfired in the bodies; `git grep -c -F` reads `.andromeda/architecture-detail/established-decisions.md` 3. Lines :75 (engine-debts), :76 (one-virtual-clock) and :78 (select-by-id) each say "owned by a CARRY on «Upstream sync ahead of polish and ship»". Changed: the owner moves to the new `.andromeda/residuals.md` line of the card's row 2 (target next, inputs#I5). Pattern `carry`: 11 standing — architecture.md:82 (the blitz-paint merge surface, no-change), :286 (changed, above); test-plan.md:124, :126, :132, :174, :175, :176 (dated history naming earlier chunks, no-change; :176 counted under `run-all`); obs-plan.md:86 ×2, :94; a11y-plan.md:54 ×2 (history, no-change).
- **Coverage of new surfaces:**
  - `examples/multi_doc.rs` (a root example, three env reads) → validation `MULTI_DOC_N` `parse().ok()` with a default ✓ · instrumentation n/a (its `println!` is example output on stdout) · PII n/a · tests ✗ (an upstream example; built by `cargo build --workspace` within `fast`'s clippy) · a11y n/a · tokens n/a
  - `PARALLEL_TRAVERSAL_LOCK` (process-wide state) → validation n/a · instrumentation n/a (no log site, `evidence/sink.md`) · PII n/a · tests: the windowed smoke (gate entry 35) the one local reading of a `Parallel` launch; no headless document sets `Parallel` · a11y n/a · tokens n/a
  - `Node::out_of_flow` / `ElementData::classes` (engine state) → validation n/a · instrumentation n/a · PII n/a · tests: `incremental_oracle`, `paint_tree_incremental`, `paint_order`, the blitz-dom lib tests and every stand check in both layout modes (gate entries 17, 18, 29) · a11y n/a · tokens n/a
  - hit test of dropped nodes → tests `hit_test_dropped_nodes` (integ; red with `f102663b8`'s engine hunks reversed, green restored — `evidence/counts.md`) · instrumentation n/a · PII n/a · a11y n/a · tokens n/a
  - mimalloc global allocator in `apps/browser` and the WPT runner → upstream-only binaries; validation n/a · tests n/a (builds in `fast`)

## Deviations from intent
- **A red-before-green control for `hit_test_dropped_nodes`** — entry 16's baseline names "the implement red-before-green run". `f102663b8`'s hunks to `node/node.rs` and `layout/paint_tree.rs` were reversed in the working tree (`git apply -R`), and the test read red: exit 101, panic at `node.rs:1572:26`. The saved bytes were then written back (sha256 equal, worktree equal to the index), and the test read green again. No file is left changed (`evidence/counts.md`).
- **CARRY 4's reach corrected** — `resolve.rs` reached by an import line, beside `config.rs`'s doc comments (Spec claims disproved, above).
- **The trial worktree** — the plan's implementation notes say `target/tmp/trial-wt` stays registered for the operator to remove; it was already removed (inputs#I5; `git worktree list` at /implement Setup: the main tree alone).
- **Entry 38's wait** — fired `--wait 1800` on the operator's word (inputs#I6) in place of the plan's `--wait 2400`. The run settled at 1207 s, inside both.
- **The pre-CI commit's content** — `git add -A`, 551 files: the take, the lock, the take-up documents, the phase and implement run dirs and the chunk folder, as the earlier pre-CI commits carried (`191e66fdb`). `evidence/ci.md` and `evidence/operator-pass.md`, written after it, are uncommitted.
- scope record: none — `gate.py scope` clean, 0 recorded (`scope: clean — changed 27 · listed 27 · recorded 0 (companion 0 · mechanical 0 · in-intent 0 · widening 0) · absorbed 0 · excluded 532`).

## Decisions & corrections
- **The founder's take-shape word** «Десять коммитов (Рекомендую)» (inputs#I4): the cherry-pick take of ten plus tokio 1.54.1, pins unchanged; ratified, not PROVISIONAL. It ratifies the three engine changes for every document (Changes → Symbols).
- **The operator's «yes — apply»** on the P5 card (inputs#I5): the plan and both Beyond-the-plan rows approved (two `.andromeda/residuals.md` lines, target next — P5's route-resolve appends them). "rr-cache is left as is; at implement, read each auto-applied resolution against the plan before staging." Met: git printed `Staged 'packages/blitz-dom/src/layout/construct.rs' using previous resolution.`, and the file was read after the pick and confirmed (`evidence/take.md`).
- **The operator-pass directive** (inputs#I6): entries 36–39 by hand in plan order, entry 38 `--wait 1800`, recorded in `evidence/operator-pass.md`, stop on a red. No entry read red.
- **The two-window wrap** (inputs#I7): this wrap stops after P1. At P2, after the clear, the operator's word will be «forced sample» (U55 live proof). P2 acts on it at its resume.
- **Sweep hazard — the pre-pass sweep reads master bodies only.** Since the U55 door split each master into a body and `{doc}-detail/` category files, `cascade.py sweep --pre-pass` reads the bodies and the registries. 8 of this wrap's 20 patterns (`title`, `owned`, `new-target`, `replaced`, `classes`, `multi-doc`, `mimalloc`, `tokio`) never fired over the bodies, and the call exited 3. Meanwhile `git grep` found sites of four of them in the category files: `replaced` in standard-contracts.md, `mimalloc` and `out-of-flow` in stack-and-technologies.md, and the owner title ×3 in established-decisions.md. A wrap reading the sweep's zero as an absence would have missed the three CARRY-owner lines. Evidence: the refusal line in Changes → Counts and the `git grep` counts beside each pattern.
- **Guard refusal** (/implement): bash-guard's cd arm refused `cd {skill references dir} && grep …`; re-issued once with an absolute path (`.andromeda/runs/2026-10-24T11-39-00-implement/outcome-2026-10-24-upstream-sync-polish-and-ship.md`).
- **Hook refusal** (/implement): the stamp-ahead PostToolUse hook refused an estimated `11:45Z` stamp in `evidence/readings.md` while the clock read 11:41:17Z. It was corrected to the read clock; later stamps were read from `date -u` or dropped (the same outcome file).
- **Standing learning met:** the operator's word to read each rerere auto-resolution before staging (inputs#I5) was kept. The Tier 1 grep hazard (a bounded repetition refused by the embedded ugrep) was avoided: `command grep` / `git grep` throughout.

## Outcome
- Acceptance criteria, re-asserted against the diff:
  - **The take is exact** — met.
    - The exact-take entry reads no output (entry 1).
    - The four new files are byte-identical to the upstream commits that add them (entry 2).
    - `inline/layout.rs` is the base `inline.rs` moved by exactly `28f848d3b` and `21edc92d9`'s lines, and the old path is gone (entry 3).
    - `construct.rs` differs from upstream's two commits by the six import-line lines (entry 4, `last line 6`); the resolved line reads once (entry 5).
    - The resolution is named in `evidence/take.md`; no further gate-driven edit. The ten trailers ride `34f6f051b`.
  - **The pins stay** — met.
    - The preservation guard (entry 6): the root manifest and `deny.toml` byte-identical; no git dependency (entry 7).
    - The lock moves by +15 −14 (entry 8) with no package `name` line moved (entry 9).
    - Every tokio is ≥ 1.53.3 (entry 10).
    - `evidence/lock.md` records the moves, the mimalloc hand review and the commits left out with their reasons.
  - **The audit is green** — met: `advisories ok, licenses ok` (entry 25), no ignore added for RUSTSEC-2026-0386. Every standing ignore is still encountered (no `advisory-not-detected` warning). Two warnings: `yanked` clipboard-win 5.4.1 and `license-not-encountered` NCSA at `deny.toml:46`.
  - **Upstream's test lands as a target** — met.
    - `hit_test_dropped_nodes` 1 passed as its own target (entry 16).
    - Every file but `all.rs` ran as its own target, no `Running tests/all.rs` (entry 21).
    - `BlitzTestsTargetsTest` green inside `fast` (entry 19); 145 files (entry 22).
  - **The local gates pass** — met: `fast` (entry 19) and `doc` (entry 24) green; tally `lines 206 passed 1099 failed 0 ignored 12` (entry 20), recorded in `evidence/counts.md`.
  - **The a11y and stand readings hold, both layout modes** — met.
    - a11y `lines 6 passed 43 failed 0` (entry 27).
    - `run stand` passed 195 · failed 0 · ignored 7 over 48 stand files (entries 29, 30).
    - No stand figure moved: Tab tables, contrast rows, lint, bounds and the committed Counter checks all read inside it.
  - **CARRY 3–5 readings pass in both layout modes** — met: the integration entry (19 result lines, 106 passed, entry 18) and blitz-dom lib 106 passed (entry 17). `evidence/readings.md` re-reads CARRY 1–5 per row, each CARRY 3 F-row untouched. Nothing sent upstream.
  - **The sink allowlist re-check holds** — met.
    - The fixed-sha census reads its one commented `dbg!` (entry 11).
    - 0 wall-clock reads outside `clock.rs` (entry 12).
    - `host_log` and `mcp_log` pass in the per-package build (entry 32) and the workspace build (inside entry 19).
    - `hit_test_dropped_nodes` reads 0 env · 0 print · 0 subscriber; `packages/escher-telemetry/` is unchanged (entry 6).
    - The example's three env variables (entry 13) and its `println!` are listed for registration (`evidence/sink.md`).
  - **Fork CI is green on the pushed take** — met: `verdict: green` on CI#40092092468 (`34f6f051b`), 17/17, Dependency audit green. The per-OS tallies were re-read job by job, each move explained by name (`evidence/ci.md`).
  - **No 0.2.0 capability is claimed** — met: `matrix.py show --chunk 2026-10-24-upstream-sync-polish-and-ship`: `claimed by 2026-10-24-upstream-sync-polish-and-ship: 0` · `pool: unclaimed 2 [v020-10 v020-11]`.
- Gates (the plan's `[[gate]]` entries by `run`, in order; /implement's block firing `entries 39 · green 34 · red 0 · recorded 1 · timeout 0 · not-run 4`, one iteration, trail `.andromeda/runs/2026-10-24T11-39-00-implement/gate-2026-10-24-upstream-sync-polish-and-ship.json`):
  - `diff <(git diff … a356f908… -- packages tests wpt apps examples flake.nix …) <(git show … {the ten} …)` — green (exit 0, no output)
  - `git diff --quiet 28f848d3b… -- inline/{build,mod}.rs && … fb297a38c… -- examples/multi_doc.rs && … f102663b8… -- hit_test_dropped_nodes.rs` — green (exit 0)
  - `diff <(diff <(git show a356f908…:…/layout/inline.rs) …/inline/layout.rs …) … && test ! -e …/inline.rs` — green (exit 0, no output)
  - `diff <(git diff … -- …/construct.rs …) <(git show … 28f848d3b… 46d7cf993… -- …/construct.rs …) | grep -c -E '^[<>]'` — green (exit 1, last line 6)
  - `grep -c -F 'node::{NodeFlags, NodeKind, RangeInputData, SpecialElementData, TextInputData, TextLayout},' …/construct.rs` — green (last line 1)
  - `git diff --quiet a356f908… -- Cargo.toml deny.toml packages examples tests apps wpt .github scripts flake.nix {the take excluded}` — green (exit 0)
  - `grep -c -E 'DioxusLabs/taffy|DioxusLabs/stylo|git = ' Cargo.toml` — green (exit 1, last line 0)
  - `git diff --numstat a356f908… -- Cargo.lock | awk …` — green (last line +15 -14)
  - `git diff a356f908… -- Cargo.lock | grep -c -E '^[+-]name = '` — green (exit 1, last line 0)
  - `python3 -c "import tomllib; … tokio …"` — green (last line tokio fixed)
  - `git show --format= {the ten} -- 'packages/*.rs' | grep -c -E '^\+[^+].*(tracing::|…|SystemTime::now)'` — green (last line 1)
  - `grep -rlE 'Instant::now\(\)|SystemTime::now\(\)' packages/blitz-dom/src | grep -vc 'clock\.rs$'` — green (exit 1, last line 0)
  - `grep -o -E 'MULTI_DOC_[A-Z]+' examples/multi_doc.rs | sort -u | wc -l` — green (last line 3)
  - `git ls-tree -r --name-only 286874cc1… -- packages/dioxus-native-dom/src/keyboard_compat.rs | wc -l` — green (last line 0)
  - `test -f packages/dioxus-native-dom/src/keyboard_compat.rs && grep -c -F -e "--ignore-filename-regex 'keyboard_compat\.rs\$'" .github/scripts/ci-leg.sh` — green (last line 2)
  - `cargo test -p blitz-tests --locked --test hit_test_dropped_nodes 2>&1` — green (exit 0, `test result: ok. 1 passed; 0 failed`)
  - `cargo test -p blitz-dom --locked --lib 2>&1` — green (exit 0, lacks FAILED; 106 passed)
  - `cargo test -p blitz-tests --locked --test stale_node_mapping … --test dioxus_falsy_boolean_attrs 2>&1` — green (exit 0, lacks FAILED)
  - `bash .github/scripts/ci-leg.sh fast` — green (exit 0; artifact `target/ci-logs/test.log` fresh)
  - `grep -E '^test result:' target/ci-logs/test.log | awk …` — green (`lines 206 passed 1099 failed 0 ignored 12`)
  - `for f in tests/blitz-tests/tests/*.rs; … | grep -c absent` — green (exit 1, last line 0)
  - `ls tests/blitz-tests/tests/*.rs | wc -l` — green (last line 145)
  - `grep -E '^Ran [0-9]+ tests|^OK$|^FAILED' target/ci-logs/ci-scripts.log` — recorded: `Ran 100 tests in 8.704s` · `OK` (unmoved, as forecast)
  - `bash .github/scripts/ci-leg.sh doc` — green (exit 0)
  - `bash .github/scripts/ci-leg.sh audit` — green (exit 0, `advisories ok`, `licenses ok`)
  - `bash .github/scripts/ci-leg.sh a11y` — green (exit 0, lacks FAILED; artifact fresh)
  - `grep -E '^test result:' target/ci-logs/a11y.log | awk …` — green (`lines 6 passed 43 failed 0`)
  - `bash scripts/agent-run.sh boot` — green (exit 0)
  - `bash scripts/agent-run.sh run stand` — green (`"outcome": "passed"`; run.end passed 195 · failed 0 · ignored 7)
  - `ls tests/blitz-tests/tests/stand_*.rs | wc -l` — green (last line 48)
  - `bash scripts/agent-run.sh run all` — green (`"outcome": "passed"`; run.end passed 617 · failed 0 · ignored 12)
  - `cargo test -p seven_guis --locked 2>&1` — green (exit 0, lacks FAILED; 12 result lines · 36 passed)
  - `cargo tree --locked -p seven_guis -e normal --prefix none | grep -c -E '^accesskit_(xplat|winit) '` — green (exit 1, last line 0)
  - `cargo build -p seven_guis --bin seven_guis_native --locked` — green (exit 0)
  - `RUST_LOG=info timeout 10 target/debug/seven_guis_native; test $? -eq 124` — green (exit 0, `service.name=seven_guis`; WAYLAND_DISPLAY set) — the smoke where the boot path changed (`fb297a38c`'s `Parallel` windowed launch)
  - `python -X utf8 ~/.claude/skills/andromeda-tools/scripts/gate.py hygiene` — leg operator, fired as written by the operator pass (inputs#I6): `hygiene: clean — read 518 (runs 505 · evidence 6 · inputs 7)` (`evidence/operator-pass.md`)
  - `bash .github/scripts/ci-leg.sh fast && git diff --quiet && git diff --cached --quiet && git push origin build/escher-0.2.0` — leg operator, `gate.py run --operator 37`: green · exit 0 · `history moved: refs/remotes/origin/build/escher-0.2.0 a356f908→34f6f051` (the intended push), its `fast` tally 206 · 1099 · 0 · 12 (`evidence/operator-pass.md`)
  - `python -X utf8 ~/.claude/skills/andromeda-tools/scripts/ci.py conclusion --sha HEAD --wait 2400` — leg operator, fired with `--wait 1800` (inputs#I6): `34f6f051b814 verdict: green · checks 17/17 · wall 1210 s · runs CI#40092092468 completed/success` (`evidence/operator-pass.md`, `evidence/ci.md`)
  - `gh run view <id> -R Turbolet85/escher --log | sed … | awk …` — leg operator, `gate.py run --operator 39 --id 40092092468`: recorded · exit 0 · `history: unmoved`. Its read: the six per-OS rows (Changes → Counts), each move `hit_test_dropped_nodes`, dispositioned in `evidence/ci.md`. The CI outcome it reads is green (CI#40092092468), this chunk's.
- Watches: none folded (scope.md: no `BLOCKED-ON`, `PREREQ`, `CONTEXT` or `WATCH` on the entry).
- Outcome basis: the operator pass ran.
  - Setup 4's commit list from the base `a356f9086`: `34f6f051b` (the pre-CI commit) alone, no fix commit.
  - The final HEAD's CI run CI#40092092468 on `34f6f051b` is recorded in `chunks/2026-10-24-upstream-sync-polish-and-ship/evidence/{ci,operator-pass}.md` (written after the commit, uncommitted; this wrap's commit carries them).
  - /implement's run is held whole in this window, and its persisted report is `.andromeda/runs/2026-10-24T11-39-00-implement/outcome-2026-10-24-upstream-sync-polish-and-ship.md` (the one `*-implement` run dir of this chunk), outcome green + smoke.
  - The operator directive between implement and this report: inputs#I6 — it changed entry 38's wait alone.
  - Post-implement artifacts: `evidence/ci.md`, `evidence/operator-pass.md`.
- Process hygiene: /implement's P4 census, `gate.py census --comm seven_guis_native --comm escher-session --comm cargo --comm rustc --args agent-run.sh` at 11:52:35Z, read `census: none running`. Every process its entries started (cargo/rustc, `agent-run.sh` boot · run stand · run all, `seven_guis_native` under `timeout 10`, the control's two cargo runs) is terminated. The operator pass's processes (entry 37's cargo legs and push, `ci.py`'s poll, entry 39's `gh`) ended with their calls. Re-measured by this wrap with the same arms (run dir `.andromeda/runs/2026-10-24T12-21-29-wrap/`): `census: none running`.
## Addendum — resumed 2026-10-24
- inputs#I8 · message: the operator, the wrap resume invocation, 2026-10-24 12:29Z · copy message · n/a — the operator's word at the P2 resume: «forced sample» (`detail.py sample --side wrap --force`, the full read beside the cascade); every subagent transcript kept; spawns paced to ≤ 20 running; a quote one unbroken run; a stop at any hard cascade failure (a tool error, a refused spawn, a strong reader reading a `-detail/` dir); the founder away, so a fork of his is NOT ANSWERED and waits in one file.
  - bears on **The two-window wrap**: adds to it (the «forced sample» word I7 forecast is given)
  - bears on **Cross-project / external claims**: adds to it (an eighth input, a message)
- inputs.py verify, re-fired: `inputs: 8 entries — unchanged 1 · drifted 0 · vanished 0 · broken 0 · altered 0 · unreachable 0 · n/a 7 · uncited 1 · unparsed 0`; moved since the report: uncited 3 → 1 (I5–I7 cited by the report; I8, this resume's, cited by this addendum)
## Addendum — resumed 2026-10-24 (after the refused spawn)
- inputs#I9 · message: the operator, on overseer1's ruling, the reply to the refused-spawn halt · copy message · n/a — resume the classifier pair under (a) with at most 16 subagents running at once (the slot-release lag unmeasured), the strong readers likewise ≤ 16; a third refusal at ≤ 16 a stop; each master's sampler line, the tally rows and any no-fit question reported when P2 validates.
  - bears on **The two-window wrap**: adds to it (the P2 pacing word, superseding I8's «≤ 20 running» for the classifiers and strong readers)
  - bears on **Cross-project / external claims**: adds to it (a ninth input, a message)
## Addendum — the P2 Validate halt, 2026-10-24
- inputs#I10 · message: the operator, the P2 Validate question dialog, 2026-10-24 13:28Z · copy message · n/a — category-file citations owned forward: this wrap re-points only the citations whose path is gone (`layout/inline.rs`, 6 category sites and 3 body sites) and those inside text it amends; the digit drift over the rest — 402 citations in `.andromeda/*-detail/` naming a file this take changed — is the structural blind spot overseer1 logged as F334 (`cites.py`, `cascade.py` and `planlint.py` read no `*-detail/` file; the fix in the cascade hotfix awaiting deploy), owned by a `.andromeda/residuals.md` line, target next.
  - bears on **Sweep hazard — the pre-pass sweep reads master bodies only**: adds to it (the citation sweep is blind the same way; the operator's ruling on the drift)
  - bears on **Cross-project / external claims**: adds to it (a tenth input, a message)
## Addendum — the F334 order, 2026-10-24
- inputs#I11 · message: the operator, relaying overseer1, a message to the wrap mid-P2, 2026-10-24 13:56Z · copy message · n/a — the F334 residuals line (the 402 category-file citations, target next) writes the order into its text: this wrap commits; then the founder deploys cites v1.5; then a one-time `cites.py apply --base a356f9086` (dry run first) re-points the category-file backlog, leaving 14 sites listed for a hand read; only then the next chunk. No wrap runs cites v1.5 before that run, or it re-points the stale set's lines wrongly.
  - bears on **Sweep hazard — the pre-pass sweep reads master bodies only**: adds to it (the order in which the citation backlog is repaired)
  - bears on **Cross-project / external claims**: adds to it (an eleventh input, a message)
## New text, by line
Generated by `cites.py added` (cites v1.4); pasted by `splice.py`. No line of this section is typed or edited.
The diff: a356f908 (the parent of the oldest pre-CI commit 34f6f051) → the work tree.
A row is a block this chunk added: `{first}-{last}`, `@{head}` its head line where not the first, «the head line».

### Cargo.lock — added 15 line(s) in 15 range(s)
added: 275 · 286 · 2138 · 2789 · 3083 · 5440 · 7031 · 7606 · 8087 · 8122 · 8357 · 8359 · 8668 · 9783 · 10407
### apps/browser/Cargo.toml — added 1 line(s) in 1 range(s)
added: 14
### apps/browser/src/document_loader.rs — added 2 line(s) in 2 range(s)
added: 3 · 62
### apps/browser/src/toolbar.rs — added 2 line(s) in 2 range(s)
added: 3 · 203
### examples/multi_doc.rs — new file · 116 line(s)
- 17-19 «fn env_flag(name: &str) -> bool {»
- 21-33 «fn html(n: usize) -> String {»
  - 24-30 «for i in 0..n {»
- 35-45 «fn config() -> DocumentConfig {»
  - 36-40 «let style_threading = if env_flag("MULTI_DOC_SEQ") {»
  - 41-44 «DocumentConfig {»
- 47-116 «fn main() {»
  - 48-50 «let mode = std::env::args()»
  - 51-54 «let iters: usize = std::env::args()»
  - 55-58 «let n: usize = std::env::var("MULTI_DOC_N")»
  - 63-67 «let mut timed_resolve = |doc: &mut BaseDocument| {»
  - 69-109 «match mode.as_str() {»
  - 111-115 «println!(»
### flake.nix — added 1 line(s) in 1 range(s)
added: 31
### packages/blitz-dom/src/config.rs — added 8 line(s) in 3 range(s)
added: 13-17 · 23-24 · 54
### packages/blitz-dom/src/layout/construct.rs — added 5 line(s) in 5 range(s)
added: 6 · 23 · 29 · 720 · 775
### packages/blitz-dom/src/layout/inline.rs — deleted
### packages/blitz-dom/src/layout/inline/build.rs — new file · 303 line(s)
- 3-5 «use parley::{»
- 6-12 «use style::{»
  - 8-11 «values::{»
- 14-21 «use crate::{»
- 23-303 «pub(crate) fn build_inline_layout_into(»
  - 33-37 «let root_node_style = root_node.primary_styles().or_else(|| {»
  - 39-45 «let parley_style = root_node_style»
  - 49-54 «if let Some(style) = root_node_style.as_deref() {»
  - 56-59 «let text_transform = root_node_style»
  - 61-67 @62 «if let Some(ListItemLayout {»
  - 68-83 «{»
  - 88-96 «if let Some(before_id) = root_node.before() {»
  - 97-105 «for child_id in root_node.children.iter().copied() {»
  - 106-114 «if let Some(after_id) = root_node.after() {»
  - 119-302 «fn build_inline_layout_recursive(»
### packages/blitz-dom/src/layout/inline/layout.rs — new file · 1175 line(s)
- 4-7 «use style::values::{»
- 8-14 «use taffy::{»
- 27-36 @30 «fn subtract_margins(mut child_inputs: LayoutInput, margin: taffy::Rect<f32>) -> LayoutInput {»
  - 31-34 «child_inputs.available_space = child_inputs»
- 38-82 @41 «fn inline_box_inputs(»
  - 50-80 «match width_style.tag() {»
- 84-1170 «impl LayoutPassState<'_> {»
  - 85-195 «pub(crate) fn compute_inline_layout(»
  - 197-228 @199 «fn inline_span_cb_flags(&self, root_id: NodeId, box_id: NodeId) -> StyleFlags {»
  - 230-1169 «fn compute_inline_layout_inner(»
- 1172-1175 @1173 «fn f32_max(a: f32, b: f32) -> f32 {»
### packages/blitz-dom/src/layout/inline/mod.rs — new file · 4 line(s)
### packages/blitz-dom/src/layout/mod.rs — added 3 line(s) in 1 range(s)
added: 856-858
### packages/blitz-dom/src/layout/paint_tree.rs — added 13 line(s) in 4 range(s)
added: 146-147 · 156 · 161 · 164-172
### packages/blitz-dom/src/layout/replaced.rs — added 13 line(s) in 2 range(s)
added: 144-145 · 201-211
  - 201-211 @203 «let inherent_size = match style_aspect_ratio {»
### packages/blitz-dom/src/mutator.rs — added 8 line(s) in 2 range(s)
added: 375-378 · 499-502
### packages/blitz-dom/src/node/element.rs — added 17 line(s) in 5 range(s)
added: 45-48 · 56-61 · 313 · 422-426 · 431
- 45-47 «pub(crate) fn parse_classes(value: &str) -> ThinVec<Atom> {»
### packages/blitz-dom/src/node/mod.rs — added 1 line(s) in 1 range(s)
added: 22
### packages/blitz-dom/src/node/node.rs — added 43 line(s) in 11 range(s)
added: 125-128 · 425 · 1271-1279 · 1414-1417 · 1572-1574 · 1578 · 1587-1589 · 1621-1623 · 1627 · 1646-1650 · 2069-2077
- 2069-2076 @2072 «pub(crate) fn style_is_out_of_flow(style: &ComputedValues) -> bool {»
  - 2074-2075 «matches!(box_style.position, Position::Absolute | Position::Fixed)»
### packages/blitz-dom/src/resolve.rs — added 2 line(s) in 2 range(s)
added: 29 · 32
### packages/blitz-dom/src/stylo.rs — added 27 line(s) in 7 range(s)
added: 29-35 · 157-163 · 586-590 · 697-699 · 710 · 1342-1344 · 1459
### packages/dioxus-native/src/lib.rs — added 2 line(s) in 2 range(s)
added: 63 · 222
### tests/blitz-tests/tests/all.rs — added 1 line(s) in 1 range(s)
added: 31
### tests/blitz-tests/tests/hit_test_dropped_nodes.rs — new file · 82 line(s)
- 11-22 «fn make_doc(html: &str) -> HtmlDocument {»
  - 12-19 «let mut doc = HtmlDocument::from_html(»
- 24-26 «fn node_id(doc: &HtmlDocument, selector: &str) -> NodeId {»
- 28-30 «fn hit(doc: &BaseDocument, x: f32, y: f32) -> Option<NodeId> {»
- 32-82 @33 «fn hit_test_skips_dropped_nodes() {»
  - 34-47 «let mut doc = make_doc(»
### wpt/runner/Cargo.toml — added 1 line(s) in 1 range(s)
added: 15
### wpt/runner/src/main.rs — added 3 line(s) in 1 range(s)
added: 1-3
### wpt/runner/src/test_runners/attr_test.rs — added 13 line(s) in 5 range(s)
added: 3 · 97 · 135-136 · 141-147 · 164-165
  - 141-147 @143 «let (offset_width, offset_height) = doc»
