# Report — 2026-10-10-upstream-sync-agent-surfaces

**Chunk:** Upstream sync ahead of agent surfaces — upstream/main 7832c177 merged (61 commits), our changes kept mostly additive, our tests and CI prove our logic survived
**Date:** 2026-10-10
**Commits:** since the last wrap (`state.yaml` `last_wrap` 2026-10-10T00:34:22Z; `git log --first-parent 57f0495d..HEAD`): `09f479b8 chore(setup-project): upgrade escher — U03` (the chunk start, not this chunk's) · `9462a7e4 chore(2026-10-10-upstream-sync-agent-surfaces): operator pre-CI commit, for the run this chunk's verdict reads` — the merge commit, parents `09f479b8` then `7832c177ff272128154bac58afe56c2b9164b417`, bringing upstream's 61 commits. Uncommitted at this report: `evidence/ci.md`, the later sections of `evidence/operator-pass.md`, friction-ledger appends and their tool trails, this wrap's run dir, the `inputs#I3` snapshot.

## Changes (structured — detectors read this)
- **Files:** 74 changed outside the excluded sets, all 74 inside research's two lists (`gate.py scope`, base `09f479b8`: `scope: clean — changed 74 · listed 74 · recorded 0 · absorbed 0 · excluded 56`). By hand, 6:
  - `Cargo.toml` — conflict resolved, both sides kept (added lines 106 · 116 · 201-207).
  - `tests/blitz-tests/Cargo.toml` — conflict resolved; upstream's `autotests = false` removed; upstream's `[[test]]` table for `all` kept with `test = false` under a nine-line comment (added 17 · 44-57).
  - `deny.toml` — add/add conflict resolved: our header, `[graph]`, `[advisories]`; upstream's `[licenses]` (added 2 · 22-42).
  - `.github/workflows/ci.yml` — every conflict region ours; one line removed against the chunk start (`components: rustfmt` of the matrix job's toolchain step), nothing added.
  - `packages/blitz-dom/src/scrolling.rs` — the one call of `visible_region` moved (added 776-780).
  - `.github/scripts/test_blitz_tests_targets.py` — new, ours, 30 lines.
  - The other 68 arrive by the merge alone: 14 new files and 54 modified, each named in the last section. `Cargo.lock` is the merge's text, not rewritten by cargo (`cargo metadata --locked` exit 0; `git status` read `M ` staged, no work-tree change on top).
- **Symbols / APIs:**
  - Removed by upstream: `Node::unrounded_absolute_position` (was `pub`, `packages/blitz-dom/src/node/node.rs`). `git grep -c unrounded_absolute_position -- packages` → exit 1, no output: no definition and no caller left. Its five call sites at the chunk start: three in `document.rs` and its own recursion rewritten or removed by upstream, and ours in `visible_region`, moved by this chunk.
  - Added by upstream, crate-private: `BaseDocument::physical_unrounded_geometry(node_id) -> (Point<f32>, taffy::Layout)`, two bodies — `packages/blitz-dom/src/document.rs` 2254-2270 (head 2257) under `not(feature = "writing-mode")`, `packages/blitz-dom/src/layout/writing_mode.rs` 371-401 (head 375) under the feature. Both sum `location − scroll_offset` over the containing-block chain, the node's own scroll offset included.
  - Changed by this chunk, signature kept: `BaseDocument::visible_region(&self, NodeId) -> Option<BoundingRect>` (`packages/blitz-dom/src/scrolling.rs`, new text 776-780) reads each clipping box through `self.physical_unrounded_geometry(box_id)`, adds the box's own scroll offset back on each axis, and takes border, size and scrollbar from the layout that call returns. Its two callers are unchanged and remain: `execute::in_view` (`packages/escher-driver/src/execute.rs`, not in the diff) and `reader_holds_the_target` (`tests/blitz-tests/tests/scroll_into_view_nested.rs`, not in the diff).
  - `BaseDocument::get_client_bounding_rect`, `offset_rect`, `inline_fragment_rects`: signatures unchanged, bodies rewritten by upstream onto the new reader (`document.rs` added 2280-2282 · 2287-2288 · 2306-2313 · 2339-2340 · 2379). Every snapshot `bounds` is read through code this merge changes; the snapshot checks pass unedited.
  - Taffy's tree traits moved off `BaseDocument` (upstream #1099): `packages/blitz-dom/src/layout/mod.rs` now holds `impl … for LayoutPassState<'_>` for `TraversePartialTree`, `TraverseTree`, `LayoutPartialTree`, `LayoutContainingBlock`, `CacheTree`, `LayoutBlockContainer`, `LayoutFlexboxContainer`, `LayoutGridContainer`, `RoundTree` (`grep -n '^impl.* for LayoutPassState' layout/mod.rs`: lines 608, 640, 642, 686, 732, 763, 795, 815, 846, plus `Deref` 144 and `DerefMut` 151); the chunk start held 10 `impl … for BaseDocument` there and the merged file 0. `LayoutPassState<'doc>` is `pub(crate)` (82-103, head 84; `new` 123-141). `PrintTree` is now implemented for a `TaffyDebugTree` (`grep -n PrintTree layout/mod.rs`: line 911; the type's own new impls 870-907).
  - Added by upstream, public: `Node::layout_style_in` (`node/node.rs` 1140-1150, head 1143); `stylo_taffy::WritingModeExt` (`packages/stylo_taffy/src/writing_mode.rs` 26-84, head 27); `stylo_taffy::convert::inline_containing_block_claims` (`convert.rs` 373-394, head 375); `TaffyStyloStyle::new_in` and `set_percent_basis` (`wrapper.rs` 76-90 and 414-423); `full_width` and `full_size_kana` in `packages/blitz-dom/src/layout/text_transform.rs` (247-289, 291-321).
  - Added by upstream on the script-to-DOM crossing (blitz-vibey-script): `document.getSelection` (`src/dom/document.rs` 75-81; `src/dom/selection.rs`, new, `get_selection` 29-57, `set_base_and_extent` 69-119, `remove_all_ranges` 121-128, `to_string` 130-145) and the `innerText` / `outerText` getters (`src/dom/element.rs` 764-774; `src/inner_text.rs`, new, `inner_text` 12-48). Research read the Selection API as writing through `BaseDocument::set_text_selection` with 0 `mutate()` calls and `inner_text.rs` as read-only. escher uses none of them.
  - Engine behaviour changed by upstream: a focusable element's `autofocus` attribute enables autofocus when present with any value but `"false"` (`packages/blitz-dom/src/mutator.rs` 1024-1026 · 1028, #1085; upstream's diff of the line: `if value == "true"` → `if value != "false"`); a custom widget's element returns early from the built-in attribute handling and skips the built-in tag post-processing (`mutator.rs` 388-393 · 993-998 · 1003-1006, #1081); the default stylesheet's link rule selects `a[href]`, colour and decoration unchanged (`packages/blitz-dom/assets/default.css` 42-43, #1134); `file:` URLs are read through `Url::to_file_path` off wasm, a non-path URL returning an `InvalidInput` error (`packages/blitz-net/src/lib.rs` 160-169, #1110); an `@font-face` source with no format hint whose URL has no extension (a `data:` URL) is no longer skipped — it is fetched and its format sniffed from the bytes (`packages/blitz-dom/src/net.rs` 458-462, #1109; upstream's diff replaces an early `?` return on `rsplit_once('.')` with a guarded branch).
  - No listener, bind site, runtime `env::var` read or `tracing::` call site is added by upstream's delta (the gate block's fixed-sha entry: 0). Added by it: one `println!` (the WPT runner, upstream-only), one `unsafe` (a stylo calc pointer read in `stylo_taffy`), one compile-time `env!("CARGO_MANIFEST_DIR")` in the unbuilt `tests/all.rs`.
- **Crates / modules:** no workspace member added or removed. New modules, all upstream's: `blitz-dom` `layout/text_transform.rs` (979 lines) and `layout/writing_mode.rs` (435); `stylo_taffy` `writing_mode.rs` (283); `blitz-vibey-script` `dom/selection.rs` (417) and `inner_text.rs` (280); the WPT runner's `test_variants.rs` (291). `accesskit_xplat` reads version 0.2.1 (was 0.2.0).
- **Dependencies:** taken as upstream's set, whole (basis `evidence/lock.md`: the merged `name`+`version`+`source` set equals upstream's plus every entry of the two names ours added since the base, difference empty both ways; 894 names / 992 entries against 890 / 992 at the chunk start).
  - Pin form changed: `parley` is a registry version, `parley = { version = "0.12", … }` (`Cargo.toml` 116), no longer a git `rev`; with it `fontique`, `parley_data`, `parley_emoji`, `parley_engine` 0.11.0 (git `e41dfea5`) → 0.12.0 (registry) and `parlance` 0.1.0 → 0.1.1. `taffy` stays a git dependency, rev `4142c9d8` → `7d33901cc383489d9945ae796a1ceed0a935ef66` (`Cargo.toml` 106).
  - Bumped: `accesskit` 0.25.0 → 0.25.1 (the manifest still reads `"0.25"`), `accesskit_unix` 0.23.0 → 0.24.0 and `accesskit_android` 0.8.0 → 0.9.0 (`packages/accesskit_xplat/Cargo.toml`, added 27 · 30), `accesskit_consumer` · `accesskit_macos` · `accesskit_windows` · `accesskit_atspi_common` by a patch or minor in the lock, `comrak` 0.55.0 → 0.56.0.
  - Added to the root manifest (202-206, under the comment at 201): `icu_casemap`, `icu_locale_core`, `icu_properties`, `icu_segmenter` at "2.3" and `writeable` "0.6". Names entering the lock, 6: `core_detect` 1.0.0, `multiversion_no_op` 1.0.0, `icu_casemap` 2.3.0, `icu_casemap_data` 2.3.0, `memchr-n` 0.1.9, `fearless_simd_macros` 0.1.0 — each reviewed by hand in `evidence/lock.md` (licence, repository, build script, proc-macro flag, direct dependent); all crates.io with a checksum. Leaving, 2: `jetscii`, `tinyvec_macros`.
  - Unmoved: `skrifa` 0.44.0 (one version), `vello` 0.11.0, `vello_cpu` 0.3.0, `harfrust` 0.12.0, `winit` 0.31.0-beta.3, `stylo` 0.22.0, `libc` 0.2.189, `ttf-parser` 0.25.1 (the ignored advisory is still reached), `rustls` 0.23.45, `tracing` 0.1.44, `tracing-subscriber` 0.3.23, `tracing-log` 0.2.0. In all 101 names read a different version set. No `cargo update` was run.
- **Schema / config:**
  - blitz-dom features (`packages/blitz-dom/Cargo.toml`, added 20-21 · 32-34 · 37-38 · 78-84): new `text-transform-icu = ["dep:icu_casemap", "dep:writeable"]`, in blitz-dom's own default list, and new `writing-mode = ["stylo_taffy/writing-mode"]`, in no default list. `blitz`, `dioxus-native`, `dioxus-native-dom` gain forwarding `text-transform-icu` (in their defaults) and `writing-mode` (not in them) features.
  - Under `writing-mode` the document sets the stylo pref `layout.writing-mode.enabled` (`document.rs` added 416-417).
  - `deny.toml`: a `[licenses]` table (22-42; `allow` 27-42, 13 expressions, `include-dev`, `include-build`, `confidence-threshold = 0.8`), byte-identical to upstream's from its header line to the end of the file. `[graph]` and `[advisories]` are the chunk start's: six targets, `all-features = true`, the one per-ID ignore `RUSTSEC-2026-0192`. No ignore added or dropped.
  - `tests/blitz-tests/Cargo.toml`: the `blitz-dom` dev-dependency line gains `autofocus` and `text-transform-icu` (17); a `[[test]]` table `name = "all"`, `path = "tests/all.rs"`, `test = false` (54-57) under the comment 45-53; `[package]` sets no `autotests`.
  - Root `Cargo.toml`: `[profile.dev]` / `debug = "line-tables-only"` now sits after the ICU4X block (the stanza itself unchanged).
- **Spec-master edits:** none — no master, sidecar or registry file is in the diff.
- **Counts / qualifiers moved:** (bases in `evidence/counts.md` and `evidence/moved-readings.md`; each "stated in" is a grep hit of this report's searches)
  - Workspace test leg, `awk` over `target/ci-logs/test.log`: 154 → 158 result lines, 657 → 719 passed, 0 failed, 10 ignored unmoved. Stated in test-plan (`657 passed|154 result|154 ` : 1 hit, line 326).
  - CI-scripts tally: `Ran 70 tests` → `Ran 78 tests` (4 new in `test_wpt_area_changes.py`, 4 → 7 in `test_wpt_diff_to_pr.py`, 1 ours); the leg's test files 4 → 6. Stated in test-plan (`Ran 70|70 tests|25 \+ 14`: 2 hits, lines 8 and 144).
  - blitz-tests: 98 → 102 `.rs` files under `tests/` — 101 test targets and the unbuilt `all.rs`; integration-test executables in the workspace build set 103 → 107.
  - `bash scripts/agent-run.sh run all`: 396 passed, 0 failed, 10 ignored (its `run.end`); `run stand` unmoved at 110 passed, 0 failed, 5 ignored over 30 files. The `run stand` figure is stated in test-plan (`110 passed|110 `: 1 hit, line 115, "passed 110 · failed 0 · ignored 5"); the `run all` total is stated in no master (`run all`: test-plan 1 hit, the same line, a different reading — 15 `ok` over the three a11y-leg files).
  - blitz-dom features by runner: workspace build 14 → 16 (`writing-mode`, `text-transform-icu` new), per-package build 8 → 10 (`autofocus`, `text-transform-icu` new); `tracing` on in the first and off in the second, as before. Stated in test-plan (`Engine features by runner`: line 320) and obs-plan (line 42).
  - a11y leg: 3 result lines, 15 passed (6 + 6 + 3) — unmoved. Stated in a11y-plan (`6 \+ 6 \+ 3`: line 328).
  - Our merge surface against the merge base at HEAD (`git diff --numstat 23354585 HEAD`, hunks by `-U0`): `packages/blitz-dom/src/scrolling.rs` 159 added / 6 removed in 6 hunks (the entry's CARRY states 157 / 6 — the two lines are this chunk's edit); `packages/blitz-test-harness/src/input.rs` 21 / 1 in 3; `packages/dioxus-native-dom/src/dioxus_document.rs` 58 / 0 in 4 — the last two unmoved. The same figures read against the upstream pin, so upstream changed none of the three.
  - Fork CI wall-clock on the merge commit: 1100 s, a cold run, against 641 s on the chunk start. Stated in test-plan (`641 s|1255 s|695 s`: line 313).
  - The upstream sync line: merge base `23354585` → `7832c177`, merge commit `f00b0216` → `9462a7e47923ee512ef71a8b0eb86613315aa69d`. Stated in architecture (`Upstream sync:`: line 205).
- **Dev-tool versions:** none — cargo re-read at 1.99.0 and rustc at 1.99.0 on the dev host (2026-10-10T01:18Z); Python 3.14.7 on the dev host. The new CI-script test imports `tomllib` (Python 3.11 or later): the fork's `Test CI scripts` job ran it on `ubuntu-latest` (`Ran 78 tests`, `OK`); the runner's Python version was not read. `icu_casemap` and the other entering names are lockfile-resolved crates, not this line's subject.
- **Harness / gate surface:**
  - `.github/workflows/ci.yml`: the matrix job's toolchain step no longer installs `rustfmt` (one line removed; the `fmt` job keeps its own). No job added, removed or renamed; `test_ci_workflows.py` is unedited and passes.
  - The `ci-scripts` leg (`python3 -m unittest discover -s .github/scripts`) now discovers six test files: ours `test_agent_run.py`, `test_ci_workflows.py`, `test_cold_agent.py` and the new `test_blitz_tests_targets.py` (`BlitzTestsTargetsTest`, one test, lines 17-26: `[package]` does not set `autotests` to false; exactly one `[[test]]` table, `all` at `tests/all.rs` with `test = false`); upstream's `test_wpt_diff_to_pr.py` and the new `test_wpt_area_changes.py`. Controlled: the new test read FAILED on three scratch manifests (upstream's shape; `autotests = false` alone; the table without `test = false`) and OK on the resolved one.
  - blitz-tests keeps one test target per file; upstream's single-binary target `all` stands in the manifest unbuilt. `scripts/agent-run.sh`, `.github/scripts/ci-leg.sh`, `apt-install.sh` and their three pinning tests are byte-identical to the chunk start (the gate block's preservation entry, exit 0).
  - The three upstream-only workflows change by merge (bare `apt-get` installs replace the apt-cache action); every job keeps its `github.repository == 'DioxusLabs/blitz'` guard — 1, 2 and 1 — with 0 conflict markers.
  - `deny.toml` carries a licence table no leg runs; `bash .github/scripts/ci-leg.sh audit` still runs `check advisories` only.
  - No status, verdict or event shape changed.
- **Cross-project / external claims:**
  - DioxusLabs/blitz (the remote `upstream`): `main` read `7832c177ff272128154bac58afe56c2b9164b417` by `git ls-remote upstream refs/heads/main` at 2026-10-10T01:20Z, the pin, 61 commits past the merge base `2335458530518cdf167c55ce635fae99323e0789`. Upstream PR numbers named in this report are read from commit subjects of `23354585..7832c177`.
  - Turbolet85/escher, fork CI: **CI#38013740580** on `9462a7e47923ee512ef71a8b0eb86613315aa69d`, attempt 1, `completed` / `success`, checks 16/16, wall 1100 s (`ci.py conclusion --sha HEAD --wait 3600`; `gh api …/runs/38013740580` at 01:54Z). The sha is the record: this wrap's own commit adds to that tree. Read from its log (3,215,667 bytes, kept outside the tree): the per-job counts under Outcome. Earlier runs as the scope recorded them: CI#38010081458 on `09f479b8` green 16/16, 641 s.
  - crates.io, by way of the registry source cargo downloaded on the dev host: the licence, repository, build-script and proc-macro facts of the six entering crates (`evidence/lock.md`); each crate's source beyond its manifest was not read.
  - Inputs (`inputs.py verify`, 3 entries — n/a 3, drifted 0, vanished 0, broken 0, unparsed 0): `I1 · message: orchestrator git measurement, /andromeda-phase take-up · copy · n/a — a message has no live source`, cited by scope, research and plan · `I2 · message: the operator, in the implement session, 2026-10-10 · copy · n/a` — the word that opened the operator pass, cited here as inputs#I2 (`verify` printed it UNCITED before this report existed) · `I3 · message: the operator, as the arguments of /andromeda-wrap-session, 2026-10-10 · copy · n/a` — snapped at this wrap, cited here as inputs#I3.
- **Reverted / negative API facts:** five things the textual merge took, or upstream's manifest carried, that the resolved tree does not hold — none ever committed on the fork:
  - upstream's `licenses` CI job (`EmbarkStudios/cargo-deny-action@v2`, #1089) — taken by the merge outside every conflict region, reverted: the operator's decision of 2026-10-10 (no licence gate yet), and as written it is refused by three invariants of `test_ci_workflows.py`.
  - `env: CARGO_PROFILE_DEV_DEBUG: "line-tables-only"` (#1125) — taken, reverted: the manifest stanza is the one statement of the level.
  - the Windows row's `os: warp-windows-2025-x64-8x` (#1128) — taken, reverted to `windows-latest`: a runner label the fork has no registration for.
  - upstream's `linux` matrix row, its bare `apt-get` lines and its unpinned actions in `ci.yml` — upstream's side of the conflict regions, not taken.
  - `autotests = false` (#1123) — taken by the merge outside the conflict region, removed; with it upstream's `all` target would have been the crate's one test target.
- **Insufficient fixes (written, kept, not the remedy):** none.
- **Spec claims disproved by measurement:**
  - research.md, the `Cargo.lock` bullet: "the merged `name`+`version`+`source` set equals upstream's set plus the 3 entries ours added since the base". The set equality holds; the count is 2 — `escher-driver` and `escher-telemetry`, 990 + 2 = 992 (`tomllib` over the four lock blobs, `evidence/lock.md`). No decision rested on the 3.
  - plan.md, the last `[[gate]]` entry (`gh run view <id> … --log | grep -c -E 'Running tests/(stand_act_keys|stand_session_lifecycle|scroll_into_view_nested)\.rs'`), and the last acceptance that names its count: the pattern cannot match a CI log. Fired as written it read `0` at exit 1 on a green 16/16 run; the log spells cargo's colour codes as text between `Running` and the path (`Running^[[0m tests/…`, all 559 `Running` lines) and the Windows job prints a backslash path. Read again with the codes dropped and either separator: 12. Disposed by the operator (inputs#I3): "the count of 12 from the cleaned log is the reading, the 0 is the pattern defect."
  - The scope's hypothesis that Parley 0.12 and the Taffy bumps would move a stand bound or a snapshot's byte size ("not measured") is measured: none moved (`evidence/moved-readings.md`, no row of the class `our check reads differently`).
  - Master sentences the merge made false are carried as expected amendments below, not here.
- **Expected amendments (from plan):** searched with one script over the seven masters and every file under `.andromeda/registries/`; "hits" are matching lines per file.
  - architecture §Project Intent, the `Upstream sync:` line — carried: Counts / qualifiers moved (the sync line). `Upstream sync:`: architecture 1 (line 205); no other file.
  - architecture §Established Decisions [Dependency pinning] and §Stack and Technologies — carried: Dependencies. `git .?rev|`rev`|pinned by`: architecture 5 (73, 78, 141, 150, 219), security-plan 4 (72, 229, 234, 358), test-plan 1 (108, a `git rev-parse`, another subject); line 73 reads "taffy and parley are git dependencies pinned by `rev`" and 219 "taffy and parley at pinned git revs". `accesskit[_a-z]* 0\.[0-9]+`: architecture 1 (53, naming accesskit_unix 0.23.0 and accesskit_android 0.8.0), a11y-plan 1 (6). `comrak`: architecture 3, security-plan 1, layout-templates 1. `ICU4X|icu_`: 0 hits in any file — the ICU4X entries are stated nowhere yet. `e41dfea5|4142c9d8|7d33901c`: 0 hits — no master spells a rev.
  - architecture §Conventions → Tests and §Standard Contracts → CI contracts — carried: Harness / gate surface and Schema / config. `autotests|tests/all\.rs|single.binary|one binary`: 0 hits; `one file per`: 0 hits in the masters (the phrase is CLAUDE.md's and the rules'); `test_ci_workflows|test_agent_run|test_cold_agent|test_wpt_diff_to_pr`: architecture 4 (66, 115, 141, 180), security-plan 3, test-plan 10.
  - architecture §Standard Contracts → Layout and §Established Decisions [Unsupported features] — carried: Symbols / APIs (the trait impls on `LayoutPassState`) and Schema / config (`writing-mode`). `LayoutPassState|TraversePartialTree|LayoutPartialTree|Taffy tree`: architecture 1 (125, "`BaseDocument` implements Taffy's `TraversePartialTree` …"). `text-indent`: architecture 1 (86, "`text-indent` `hanging`/`each-line` do not work because their parsing is cfg'd out in Stylo"); what upstream's text-indent commit changed about that sentence was not read by this chunk — the detector reads `packages/blitz-dom/src/layout/inline.rs` (added ranges in the last section). `writing-mode|writing_mode|vertical`: architecture 0, design-system 4, test-plan 2.
  - test-plan §1, §2 and §4 — carried: Counts / qualifiers moved (CI scripts 70 → 78, six files; 102 files, 101 targets) and Harness / gate surface (the unbuilt `all` target). `Ran 70|70 tests|25 \+ 14`: test-plan 2 (8, 144); the pinning-test pattern above: test-plan 10.
  - test-plan §9 — carried: Counts / qualifiers moved (158 lines, 719 passed; features 16 and 10; wall 1100 s). `657 passed|154 result|154 `: test-plan 1 (326); `Engine features by runner`: test-plan 1 (320), obs-plan 1 (42); `641 s|1255 s|695 s`: test-plan 1 (313).
  - security-plan §Dependency Security and §Input Validation — carried: Dependencies (Parley's pin form, the six names), Schema / config (the licence table) and Symbols / APIs (the `file:` read, the hintless font source, the `autofocus` reading). `git .?rev|…`: security-plan line 229 ("Git dependencies are pinned by commit rev (Cargo.toml:106; Cargo.toml:116)" — line 116 is now a registry version). `\[licenses\]|licen[cs]e`: security-plan 1 (252, "NOT YET MEASURED — … license compliance"), architecture 4. `deny\.toml`: security-plan 2 (226, 259), architecture 2. `to_file_path|`file:``: security-plan 3 (9, 13, 84 — 84 reads "Read with `std::fs::read(request.url.path())`"), architecture 1, test-plan 1. `font-face|format hint|hint`: security-plan 2 (94, 96). `boolean`: security-plan 2 (114, 135), architecture 3, test-plan 2, a11y-plan 1. The licence reading itself: `cargo deny --locked check licenses` → `licenses ok`, exit 0, two unmatched allowances (`evidence/licenses.md`).
  - a11y-plan §5 and §1 — carried: Symbols / APIs (`autofocus`) and Dependencies (the accesskit family; no adapter in the stand's graph, the gate block's probe reading 0). `autofocus`: a11y-plan 6 (147, 172, 177, 179, 184, 189 — 172 reads "`autofocus` reflection writes the value "true" because blitz-dom's autofocus handling expects it"; the reflection still writes `"true"`, `packages/blitz-vibey-script/src/dom/element.rs` 487), architecture 1, security-plan 1. `Cargo\.lock|lockfile`: a11y-plan 1 (14). `accesskit_winit|accesskit_xplat|platform adapter|adapter`: a11y-plan 19, architecture 15.
  - design-system §Color Palette (Core Colors) — carried: Symbols / APIs (`a[href]`). `0, 0, 238|default\.css`: design-system 8 (25, 72, 116, 117, 118, 178, 210, 244), a11y-plan 4 (127, 229, 249, 286), architecture 1 (107). `a\[href\]`: 0 hits anywhere.
  - obs-plan §2 and §6, layout-templates §Surface: desktop-native → IA notes and §Surface: cli → Output structure — not carried as a claim change: no reading of theirs moved. The `tracing` half of the by-runner split reads as before (obs-plan line 42); `visible_region` keeps its signature (`visible_region`: layout-templates 1, line 45; architecture 2); no log field, sink allowlist or scrub set is in the diff. Their cited lines move with the merge and are the citation sweep's.
  - All seven masters, `file:line` citations into merged files — not a Changes fact: the citation sweep's, at P2. The merge moves lines in 54 modified files; e.g. `[profile.dev]` is now at `Cargo.toml` 208-209 where architecture cites 201-202, and `.github/workflows/ci.yml` loses one line at 354.
- **Coverage of new surfaces** (ours first; upstream's as arrived, flagged for what this chunk measured):
  - `.github/scripts/test_blitz_tests_targets.py` (ours, a CI-script check) → validation n/a · instrumentation n/a · PII n/a · tests unit (itself: 1 test in the `ci-scripts` leg, red on three control manifests) · a11y n/a · tokens n/a
  - `BaseDocument::visible_region` (ours, body changed) → validation n/a (no input surface: a `NodeId`, `None` when it does not resolve) · instrumentation n/a (no call site added; the driver's one span is unchanged) · PII n/a · tests integ (`scroll_into_view_nested` 7 passed; `stand_act_scroll` 4, `stand_act_obstructed` 3; under both bodies of the reader — the workspace build with `writing-mode`, the per-package build without) · a11y n/a · tokens n/a
  - script Selection API (upstream: `document.getSelection`, `setBaseAndExtent`, `removeAllRanges`, `toString`) → validation ✓ (argument count and offsets checked, a JS error thrown; upstream's test `invalid_offsets_throw_without_changing_selection`) · instrumentation n/a (0 `tracing::` sites added) · PII n/a (no log site) · tests integ (`packages/blitz-vibey-script/tests/selection.rs`, 15 passed) · a11y n/a · tokens n/a
  - script `innerText` / `outerText` getters (upstream) → validation n/a (read-only) · instrumentation n/a · PII n/a (returned to the script, not logged) · tests ✗ in the crate's own tests (`grep -rn -E 'innerText|outerText' packages/blitz-vibey-script/tests`: 0 hits; whether a WPT case covers them was not read) · a11y n/a · tokens n/a
  - `file:` URL read through `Url::to_file_path` (upstream) → validation ✓ for the path form (a typed `InvalidInput` error, no panic); still no path restriction · instrumentation n/a · PII n/a · tests ✗ (none named for it in the delta) · a11y n/a · tokens n/a
  - `autofocus` by presence (upstream) → validation n/a · instrumentation n/a · PII n/a · tests integ (`tests/blitz-tests/tests/autofocus_attribute.rs`, 5 passed) · a11y focus✓ for the engine (no stand source carries `autofocus`, so no stand boot focus moves) · tokens n/a
  - default link rule on `a[href]` (upstream) → validation n/a · instrumentation n/a · PII n/a · tests ✗ (none named) · a11y n/a (colour unchanged; no stand source holds an `a`) · tokens n/a (the UA stylesheet's own value)
  - vertical writing modes behind `writing-mode`, and the `text-transform` values (upstream) → validation n/a · instrumentation n/a · PII n/a · tests unit + integ (`layout/text_transform.rs` 22, `tests/text_transform.rs` 3, `inline_fragment_rects` +1 over three writing modes) · a11y n/a · tokens n/a

## Deviations from intent
- **One non-additive edit to upstream-owned code, as the plan named it:** `BaseDocument::visible_region`'s one call (`packages/blitz-dom/src/scrolling.rs`). Three lines became five; the signature and every other line of the function are unchanged. No step-12 fix followed: the gate block read green on its first run, so no other upstream-owned line was edited by hand.
- **The four resolved paths were marked resolved with `git add` by /implement.** The plan says how each file is resolved and that the merge stays staged; it does not say who marks the paths. Done so that no unmerged path was left for the commit; reported at P4.
- **The operator pass was driven by the session, on the operator's word (inputs#I2), the pre-CI commit included.** The plan reserves those four entries and the commit to the operator; the word delegated each by name, in order. The commit is the merge commit, subject and body as the plan's implementation notes give them. Nothing was pushed on top of it.
- **The last operator entry was read twice.** Once as written (0, exit 1), once beside it with a pattern that can match the log (12). The plan is not edited; the operator accepted the second reading (inputs#I3).
- **The windowed boot smoke ran as a gate entry and was not re-driven at P3** — the entry is a self-contained bounded boot, and a second firing would have opened a second window for no new reading.
- **`evidence/operator-pass.md` is an eighth evidence record** beside the seven the plan lists; it holds each operator entry's whole record, and `ci.md` its readings.
- Scope record: none — `gate.py scope` clean at implement P4 and at this wrap (changed 74 · listed 74 · recorded 0).

## Decisions & corrections
- The operator, 2026-10-10, at the plan (by question dialog): every blitz-tests file stays its own test target, upstream's `all` target kept with `test = false`; the standing difference is kept visible (a comment at the table, a standing check, a CARRY for the next sync) and the `test = false` reading is a gate entry. And: upstream's licence table is kept, no gate yet, one reading into evidence.
- The operator, 2026-10-10, at the P5 review: the moved-numbers table names each number's cause as one of two classes.
- The operator, 2026-10-10, in the implement session (inputs#I2): the operator pass is run by the session, each entry by hand in plan order; push nothing on top until the run settles; stop on a red; no wrap.
- The operator, 2026-10-10, opening this wrap (inputs#I3): P1 only, then stop; entry 38 accepted as recorded; the unattributed per-OS differences go into the report as an open finding for route-resolve.
- Sweep hazards met this chunk:
  - A grep for cargo's `Running tests/{name}.rs` over `gh run view --log` reads 0 on a log that holds the line: CI's cargo colours the word, and the log carries the colour codes between `Running` and the path. A count over a CI log needs a known positive from a CI log first — here the bare target name, which read 4 lines.
  - The same log spells the escape as the two characters `^[`, not as the ESC byte: a pattern written for `\x1b[…m` reads 0 too (this session's first tolerant read did, on 559 coloured lines).
  - The Windows job prints `tests\{name}.rs`; a `/`-only pattern misses that job even with the colour handled.
  - A merge commit's file count: `git diff-tree -r -m` lists the files against both parents (2,556 here); the chunk's change is `git diff --name-only HEAD^1 HEAD` (128).
  - "Every region takes our side" was resolved by taking our whole file (`git checkout --ours -- {file}`) and re-applying the one kept upstream line by hand, then proven by the one-line diff against the chunk start — not region by region.
- Corrections made in the session before they left it: three claims in `evidence/lock.md` were written ahead of their reading and corrected when read (a `libc` row shown as entering — it is unmoved; "80 registry names" — 101 names differ in all; `icu_casemap` attributed to #1093 / #1095 — it arrives with #947). The lock comparison was re-derived from the four blobs before the record was kept.
- One guard refusal: a no-op `cat` heredoc left at the head of a compound command was refused by the Bash guard; the append was re-issued once through the Edit tool.

## Outcome
**Acceptance criteria, each against the diff and the committed merge:**
1. (arch) Met — the merge commit `9462a7e4` has two parents, `09f479b8` first and `7832c177ff272128154bac58afe56c2b9164b417` second (`git log -1 --format=%P`); the merge-witness entry green; no conflict marker (the marker entry: exit 1, no output).
2. (arch) Met — the pin-set entry `last line 3`, the manifest entry `last line 7`, `cargo metadata --locked` exit 0, one `skrifa`.
3. (security) Met — `audit` reads `advisories ok`; `deny.toml` holds per-ID ignores only, beside upstream's table (`last line 5`); the merge added and dropped no ignore; the six entering names are reviewed in `evidence/lock.md`.
4. (security, tests) Met — `test_ci_workflows.py` unedited (the preservation entry) and green inside `fast`; the workflow differs from its chunk-start text by one line.
5. (security, arch) Met — the fixed-sha census reads 0 listener or bind sites, 0 runtime env reads, 0 `tracing` call sites; the preservation entry exits 0 over the leg and install scripts, their pinning tests, both contract scripts, the stand, the driver, the telemetry crate and the bridge sources.
6. (tests) Met — `fast` exit 0 on the uncommitted merge and again on the committed merge (the operator pass); the tally reads `failed 0`; `evidence/counts.md` holds both per-target tables and attributes all eight differing rows to named upstream files and PRs (62 = 62).
7. (tests) Met — the by-name entry `last line 0`; the three `test = false` readings 0, `all-target 0` (107 executables) and 0. Beyond the plan: fork CI's log holds 0 `Running` lines for `tests/all.rs` and 0 occurrences of `all_test_files_are_included`, the Coverage report job included.
8. (tests) Met — `BlitzTestsTargetsTest` green and named in the verbose listing (the unit entry); inside the `ci-scripts` leg locally and in fork CI (`Ran 78 tests`, `OK` — the leg is not verbose there, so the name is not in that log).
9. (tests, layouts) Met — `run stand` and `run all` each `"outcome": "passed"` (110 / 0 / 5 over 30 files; 396 / 0 / 10); the stand selection lists 30; `test_agent_run.py` unedited.
10. (a11y) Met — the a11y leg exit 0, `lines 3 passed 15 failed 0`; every stand check passes in both layout modes inside `fast` (0 failed; `stand_accessibility_ids` 5, `stand_snapshot` 8 among the rows); the adapter probe reads 0.
11. (arch, a11y) Met — 15 changed-set marks in the merged `mutator.rs`; blitz-dom's unit tests 87 passed, 0 failed (the changed-set tests of `document.rs` among them); `incremental_oracle` 7 passed.
12. (obs, security) Met — `stand_act_spans` (2 passed, 2 ignored), the five `telemetry_*` files, `host_log` and `host_binary` are rows of the post-merge table, 0 failed; `packages/escher-telemetry` unchanged.
13. (design, layouts) Met — no stand source, id, style or markup in the diff; `evidence/moved-readings.md` lists every moved number with its cause and says none is of the class `our check reads differently`; `scroll_into_view_nested` 7 passed and `stand_settle` 8 passed, their files not in the diff.
14. (tests) Met — `ci-leg.sh doc` exit 0.
15. (arch) Met — Deviations names the one edit to upstream-owned code; there was no step-12 fix.
16. (tests) Met, on the operator's disposition — fork CI reads `verdict: green`, 16/16, on a run left to settle (CI#38013740580, `evidence/ci.md`); the count of our three named targets is 12 across the run's jobs by the second read, the entry's own 0 being its pattern's defect (inputs#I3).
17. Met — no verification-matrix capability is claimed (`matrix.py show --chunk`: claimed 0).

**Gates — the plan's `[[gate]]` entries in order, by `run`.** /implement's gate block on the uncommitted merge (one call, no re-run): 30 green, 0 red, 4 recorded, 4 operator legs not run. This wrap's light gate has not run (P1 only).
- `git merge-base --is-ancestor 7832c177… HEAD || test "$(git rev-parse -q --verify MERGE_HEAD)" = 7832c177…` — green, exit 0
- `git grep -n -I -E '^(<<<<<<< |>>>>>>> )' …` — green, exit 1, no output
- `grep -c -E '^    "packages/escher-(telemetry|driver)",$|…' Cargo.toml` — green, last line 7
- `grep -c -E 'rev = "7d33901c…"|^parley = \{ version = "0\.12"|^icu_casemap = ' Cargo.toml` — green, last line 3
- `grep -c '^name = "skrifa"$' Cargo.lock` — green, last line 1
- `cargo metadata --locked --format-version 1 > /dev/null` — green, exit 0
- `git diff -U0 09f479b8… -- .github/workflows/ci.yml | grep -c -E "^[-+][^-+]"` — green, last line 1
- `git diff --quiet 09f479b8… -- .github/scripts/ci-leg.sh … packages/dioxus-native-dom/src` — green, exit 0
- `grep -c -E '^\[graph\]$|^\[advisories\]$|^\[licenses\]$|RUSTSEC-2026-0192|"wasm32-unknown-unknown",' deny.toml` — green, last line 5
- `grep -c 'changed_nodes.insert' packages/blitz-dom/src/mutator.rs` — green, last line 15
- `git grep -c unrounded_absolute_position -- packages` — green, exit 1, no output
- `git diff 23354585… 7832c177… -- packages apps examples wpt tests | grep -c -E '^\+[^+].*(TcpListener|…|tracing::)'` — green, exit 1, last line 0
- `python3 -m unittest discover -s .github/scripts -v -k BlitzTestsTargetsTest` — green, exit 0, names the test case
- `bash .github/scripts/ci-leg.sh fast` — green, exit 0, artifact `target/ci-logs/test.log` fresh
- `grep -E '^test result:' target/ci-logs/test.log | awk …` — green: `lines 158 passed 719 failed 0 ignored 10`
- `for f in tests/blitz-tests/tests/*.rs; do … done | grep -c absent` — green, exit 1, last line 0
- `grep -c -E 'Running tests/all\.rs' target/ci-logs/test.log` — green, exit 1, last line 0
- `cargo test --workspace --locked --no-run 2>&1 | awk …` — green: `executables 107`, `all-target 0`
- `grep -E '^Ran [0-9]+ tests|^OK$|^FAILED' target/ci-logs/ci-scripts.log` — recorded: `Ran 78 tests`, `OK`
- `bash .github/scripts/ci-leg.sh doc` — green, exit 0
- `bash .github/scripts/ci-leg.sh audit` — green, `advisories ok`
- `bash .github/scripts/ci-leg.sh a11y` — green, exit 0, lacks `FAILED`, artifact `target/ci-logs/a11y.log` fresh
- `grep -E '^test result:' target/ci-logs/a11y.log | awk …` — green: `lines 3 passed 15 failed 0`
- `grep -c -E 'Running tests/all\.rs' target/ci-logs/a11y.log` — green, exit 1, last line 0
- `bash scripts/agent-run.sh boot` — green, exit 0
- `bash scripts/agent-run.sh run stand` — green, `"outcome": "passed"` (110 passed, 0 failed, 5 ignored)
- `ls tests/blitz-tests/tests/stand_*.rs | wc -l` — green, last line 30
- `bash scripts/agent-run.sh run all` — green, `"outcome": "passed"` (396 passed, 0 failed, 10 ignored)
- `cargo tree --locked -p seven_guis -e normal --prefix none | grep -c -E '^accesskit_(xplat|winit) '` — green, exit 1, last line 0
- `cargo tree --locked --workspace -e features -i blitz-dom --depth 1` — recorded: 16 features, `writing-mode` and `tracing` among them (`evidence/features.md`)
- `cargo tree --locked -p blitz-tests -e features -i blitz-dom --depth 1` — recorded: 10 features, neither `writing-mode` nor `tracing`
- `cargo deny --locked check licenses` — recorded: exit 0, `licenses ok`, two `license-not-encountered` warnings (`NCSA`, `Unicode-DFS-2016`); a reading, no gate (`evidence/licenses.md`)
- `cargo build -p seven_guis --bin seven_guis_native --locked` — green, exit 0
- `RUST_LOG=info timeout 10 target/debug/seven_guis_native; test $? -eq 124` — green, exit 0, contains `service.name=seven_guis` — the smoke: the windowed stand stayed up for its bound on the merged layout, text and paint
- `python -X utf8 ~/.claude/skills/andromeda-tools/scripts/gate.py hygiene` — `leg = 'operator'`, fired by hand 2026-10-10T01:33:03Z: exit 0, `hygiene: clean` — green (and a second clean read with the record in the tree)
- `bash .github/scripts/ci-leg.sh fast && git diff --quiet && git diff --cached --quiet && git push origin build/escher-0.1.0` — `leg = 'operator'`, fired by hand 01:33:34Z on the committed merge: exit 0 — green; pushed `09f479b8..9462a7e4`; the fast leg read 158 lines, 719 passed, 0 failed, 10 ignored and `Ran 78 tests`
- `python -X utf8 ~/.claude/skills/andromeda-tools/scripts/ci.py conclusion --sha HEAD --wait 3600` — `leg = 'operator'`, fired by hand 01:35:16Z: exit 0, `verdict: green · checks 16/16 · wall 1100 s` — green; CI#38013740580 on `9462a7e4`, attempt 1, the operator pass's and this chunk's
- `gh run view <id> -R Turbolet85/escher --log | grep -c -E 'Running tests/(…)\.rs'` — `leg = 'operator'`, report-only, fired by hand 01:54:01Z with the id 38013740580: recorded, `0` at exit 1 — the pattern's defect, not an absence. The outcome it was written to carry, read beside it and accepted by the operator (inputs#I3): 12 — each of the three targets once in `Test [default features]`, `Coverage report`, `Test (macos)` and `Test (windows)`, 0 failed in each.

**Open findings for route-resolve** (each needs an owner; none is this chunk's red):
- **Per-OS test tallies not attributed (the operator's direction, inputs#I3).** In CI#38013740580 the linux test job prints 158 result lines, 719 passed, 0 failed, 10 ignored — the local figures. `Coverage report`, `Test (macos)` and `Test (windows)` each print 139 result lines against 158; `Test (macos)` and `Coverage report` read 716 passed and 10 ignored, `Test (windows)` 712 passed and 8 ignored. Why 139 and not 158, and which tests account for 716 − 712 and 10 − 8 on Windows, were not read. Known: `stand_session_lifecycle` is `cfg(unix)`-gated and reads 0 tests on Windows. Not known: whether the same differences stood before the merge. The ios and android jobs print no `Running` line and no result line; what their `test` step ran was not read.
- **A CI-log count needs a pattern that can match a CI log.** The plan's last entry is blind to colour codes and to the Windows separator; any later plan entry that greps `gh run view --log` for a cargo line inherits it.
- **Our standing merge surface in `tests/blitz-tests/Cargo.toml`** (the plan's route note): the two-line difference from upstream, guarded by `BlitzTestsTargetsTest`, re-measured by `git diff 7832c177 -- tests/blitz-tests/Cargo.toml` — for "Upstream sync ahead of polish and ship", beside the two CARRYs this entry carried. Of those two: `scrolling.rs` now reads 159 added / 6 removed in 6 hunks and its reader calls a crate-private upstream function with two cfg bodies; `dioxus_document.rs` and `input.rs` are unmoved.
- **The licence-gate question** (the plan's route note, for "Quality gates"): the one reading is green over the six-target graph; upstream reads the table with no target filter, which was not taken.
- **Upstream's CI lints blitz-dom with a named feature set** (`cargo clippy -p blitz-dom --no-default-features --features svg,woff,accessibility,system-fonts,file-input,custom-widget --all-targets -- -D warnings`, upstream's side of the clippy region), which the fork does not run — for "Quality gates", with the Tier 1 fact that a per-crate clippy reads red in blitz-dom with `file-input` off.
- **The moved bounds reader** (the plan's route note, for "Driver CLI"): `physical_unrounded_geometry` replaces `unrounded_absolute_position` and still subtracts a node's own scroll offset; that entry's cited lines move.
- Upstream's Selection API, `innerText` / `outerText` getters and `writing-mode` feature exist and escher uses none — a fact for the agent-surface entries, no work.

**Watches:**
- A fork CI job hanging in its package-install step — 1 green run this chunk [CI#38013740580, 16/16, no job hung; 0 lines of the install script's failure line `apt-install: attempt` in its log]. Neither the script's bound nor the android step's timeout was seen firing. The matrix job's install step is our side of the conflict, unchanged.

**Outcome basis:** the operator pass ran, so the verdicts above rest on its final state: the one commit `9462a7e4` (Setup's list from the parent `09f479b8`) and the final HEAD's run CI#38013740580, recorded in `evidence/operator-pass.md` and `evidence/ci.md`. /implement's P4 report, given in this same conversation, is the basis for what only it holds (the gate block's verdicts on the uncommitted merge, the pre-merge reading, the controls). Two operator directives stand between implement and this report: inputs#I2 (the pass delegated to the session) and inputs#I3 (entry 38 accepted; the per-OS finding routed; this wrap stops after P1). Post-implement artifacts: `evidence/operator-pass.md`, `evidence/ci.md`.

**Process hygiene** (implement's census, re-measured for this report by `ps -eo pid,comm` at 2026-10-10T02:03Z: no `seven_guis`, `escher-session`, `cargo`, `rustc` or code-graph process):
| process | started by | final state |
|---|---|---|
| `seven_guis_native`, the windowed smoke | implement's gate block | terminated — by its own `timeout 10` |
| session hosts of `stand_session_*` and seven_guis' `host_*` checks | the test legs (implement's block; the operator pass's `fast`) | terminated |
| cargo / rustc | the legs and probes | terminated |
| the code-graph refresh | this wrap's Setup, in the background | terminated — exit 0, `tree-refresh[rust]: 7021 nodes / 38763 edges - 58s` |
## New text, by line
Generated by `cites.py added` (cites v1.4); pasted by `splice.py`. No line of this section is typed or edited.
The diff: 09f479b8 (the parent of the oldest pre-CI commit 9462a7e4) → the work tree.
A row is a block this chunk added: `{first}-{last}`, `@{head}` its head line where not the first, «the head line».

### .github/scripts/test_blitz_tests_targets.py — new file · 30 line(s)
- 12-14 «def load_manifest():»
  - 13-14 «with open(MANIFEST, "rb") as f:»
- 17-26 «class BlitzTestsTargetsTest(unittest.TestCase):»
  - 18-26 «def test_every_file_is_a_target_and_all_is_unbuilt(self):»
- 29-30 «if __name__ == "__main__":»
### .github/scripts/test_wpt_area_changes.py — new file · 66 line(s)
- 8-15 «BEFORE = {»
- 17-23 «AFTER = {»
- 26-62 «class CompareTest(unittest.TestCase):»
  - 27-36 «def test_areas_use_only_url_path(self):»
  - 38-48 «def test_variants_remain_distinct_in_area_totals(self):»
  - 50-59 «def test_counts_changed_areas(self):»
  - 61-62 «def test_no_changes(self):»
- 65-66 «if __name__ == "__main__":»
### .github/scripts/test_wpt_diff_to_pr.py — added 87 line(s) in 5 range(s)
added: 6 · 50-102 · 117-126 · 129-133 · 144-161
- 50-101 @51 «VERBOSE_ENTRIES = [»
  - 52-65 «{»
  - 66-78 «{»
  - 79-91 «{»
  - 92-100 «{»
  - 117-125 «def test_subtests_changing_in_both_directions(self):»
  - 129-132 «def test_headline_counts_each_direction(self):»
  - 144-160 «def test_area_lines(self):»
### .github/scripts/wpt_area_changes.py — new file · 92 line(s)
- 21-34 «def load_units(path):»
  - 22-24 @23 «with open(path, encoding="utf-8") as file:»
  - 27-33 «for result in results:»
- 37-39 «def areas_of(test):»
- 42-76 «def compare(before, after):»
  - 45-70 «for test in before.keys() | after.keys():»
  - 72-76 «return [»
- 79-88 «def main():»
- 91-92 «if __name__ == "__main__":»
### .github/scripts/wpt_diff_to_pr.py — added 102 line(s) in 15 range(s)
added: 18 · 22-30 · 54-63 · 71-81 · 88 · 95-100 · 104-105 · 116 · 120 · 137 · 146 · 154-192 · 217-228 · 279 · 286-291
- 22-28 «def subtest_passes(subtest):»
  - 23-25 @24 «if subtest["kind"] == "added":»
  - 26-27 «if subtest["kind"] == "removed":»
  - 71-80 @72 «def is_relevant(self):»
  - 95-99 @96 «def delta_text(self):»
- 154-189 «def format_area_lines(areas):»
  - 157-158 «def percent(passing, total):»
  - 161-180 «for area in areas:»
  - 184-189 «return [»
  - 217-227 «if areas:»
  - 287-289 «if args.areas and os.path.exists(args.areas):»
### .github/workflows/ci.yml — nothing added (lines removed only)
### .github/workflows/publish-browser.yml — added 2 line(s) in 2 range(s)
added: 148 · 150
### .github/workflows/wpt-post-results.yml — added 1 line(s) in 1 range(s)
added: 49
### .github/workflows/wpt.yml — added 6 line(s) in 5 range(s)
added: 11 · 43 · 73-74 · 79 · 88
### CONTRIBUTING.MD — added 6 line(s) in 1 range(s)
added: 8-13
### Cargo.lock — added 504 line(s) in 417 range(s)
added: 23 · 25 · 32 · 34 · 44 · 46 · 51 · 58 · 60 · 68 · 70 · 75 · 82 · 84 · 102 · 104 · 116 · 201 · 210 · 342 · 439
       441 · 454 · 581 · 612 · 617 · 619 · 623 · 638 · 658 · 750 · 796 · 857 · 859 · 906 · 914-917 · 923 · 944 · 1104
       1125 · 1129 · 1156 · 1165 · 1187 · 1206 · 1236 · 1248 · 1279 · 1294 · 1304 · 1472 · 1474 · 1478 · 1541 · 1543
       1545 · 1547 · 1558 · 1574 · 1576 · 1612 · 1614 · 1622 · 1624 · 1665 · 1667 · 1675 · 1677 · 1687 · 1689 · 1694
       1699 · 1701 · 1718 · 1806 · 1808 · 1816 · 1818 · 1823-1825 · 2010-2015 · 2062 · 2064 · 2071 · 2073 · 2080 · 2082
       2090 · 2092 · 2099 · 2101 · 2128 · 2135 · 2137 · 2259 · 2312 · 2323 · 2325 · 2329 · 2418 · 2501 · 2800 · 2803
       2814 · 2853 · 2868 · 2936 · 2938 · 2963 · 2965 · 2968-2972 · 3190-3206 · 3219 · 3221 · 3225 · 3227 · 3288 · 3290
       3319-3321 · 3326 · 3366 · 3492 · 3560 · 3703 · 3711 · 3761 · 3771 · 3782 · 3797 · 3799 · 3832 · 4047 · 4049
       4068 · 4070 · 4099 · 4101 · 4103 · 4109 · 4143-4164 · 4173 · 4366 · 4368 · 4389 · 4422 · 4424 · 4473 · 4475
       4488 · 4490 · 4493 · 4498 · 4500 · 4536 · 4594 · 4596 · 4618 · 4629 · 4681 · 4737 · 4739 · 4850 · 4852 · 4854
       4857 · 5016-5017 · 5083-5092 · 5099 · 5205 · 5207 · 5239-5244 · 5253 · 5267 · 5279 · 5293 · 5306 · 5332 · 5368
       5414 · 5432 · 5600 · 5602 · 5610 · 5612 · 5623 · 5626 · 5639 · 5641 · 5654-5655 · 5665 · 5667 · 5677 · 5687
       5690 · 5699 · 5702 · 5714 · 5725 · 5735 · 5745-5746 · 5757 · 5774 · 5777 · 5786 · 5789 · 5799-5800 · 5810 · 5812
       5822 · 5824 · 5834 · 5836 · 5847-5848 · 5861 · 5863 · 5882 · 5910 · 5965 · 5967 · 6009 · 6062-6064 · 6071-6073
       6085-6087 · 6095-6097 · 6101-6103 · 6184 · 6204 · 6281 · 6328 · 6345 · 6404 · 6406 · 6445 · 6510 · 6576 · 6578
       6646 · 6689 · 6747 · 6757 · 6767 · 6772 · 6774 · 6776 · 6781 · 6783 · 6786 · 6797 · 6847 · 6849 · 6851 · 6901
       6916 · 6970 · 7015 · 7024 · 7026 · 7028 · 7144 · 7167 · 7253 · 7277 · 7282 · 7284 · 7443 · 7445 · 7455 · 7482
       7558 · 7560 · 7571 · 7577-7578 · 7623 · 7630 · 7661 · 7733 · 7775 · 7852 · 7888 · 7901 · 7929 · 7985 · 7987
       8022 · 8041 · 8086 · 8163 · 8165 · 8178 · 8180 · 8182 · 8198 · 8200 · 8204 · 8311 · 8313 · 8357 · 8359 · 8378
       8393 · 8395 · 8414 · 8416 · 8428 · 8430 · 8435 · 8449 · 8451 · 8470 · 8472 · 8475 · 8482 · 8484 · 8497 · 8499
       8522 · 8652 · 8668 · 8715 · 8717 · 8727 · 8729 · 8849 · 8851 · 8894 · 8907 · 8913 · 8957 · 8986 · 9020 · 9022
       9066 · 9068 · 9079 · 9081 · 9084 · 9090 · 9092 · 9100 · 9102 · 9107 · 9113 · 9115 · 9171 · 9183-9184 · 9195
       9206 · 9217 · 9229 · 9242 · 9255 · 9268 · 9300 · 9302 · 9324-9325 · 9339 · 9363 · 9388 · 9418 · 9450 · 9465
       9484 · 9500 · 9582 · 9599 · 9615 · 9636 · 9653 · 9671 · 9708 · 9722 · 9776 · 10157 · 10163 · 10185 · 10200
       10204 · 10226 · 10242 · 10258 · 10275 · 10279 · 10296 · 10305 · 10325 · 10347 · 10366 · 10374 · 10416 · 10427
       10439 · 10445 · 10470 · 10520 · 10538 · 10553 · 10596 · 10598 · 10630 · 10632 · 10636-10637 · 10662 · 10708
       10748 · 10750 · 10757 · 10759 · 10777 · 10779 · 10783-10784 · 10789 · 10791 · 10825 · 10841 · 10847 · 10849
       10908 · 10921
- 3201-3205 «dependencies = [»
- 4148-4157 «dependencies = [»
- 5088-5091 «dependencies = [»
### Cargo.toml — added 9 line(s) in 3 range(s)
added: 106 · 116 · 201-207
### apps/browser/Cargo.toml — added 2 line(s) in 2 range(s)
added: 44 · 56
### apps/readme/Cargo.toml — added 1 line(s) in 1 range(s)
added: 60
### deny.toml — added 22 line(s) in 2 range(s)
added: 2 · 22-42
- 27-42 «allow = [»
### docs/licensing.md — new file · 104 line(s)
- 66-68 «- Style/Servo stack: `app_units`, `cssparser`, `cssparser-macros`, `dtoa-short`,»
- 82-84 «- Generate third-party notices for the **actual** shipped dependency graph,»
- 86-88 «- For shipped MPL components, provide their exact corresponding source (and any»
- 89-91 «- Audit bundled assets and native libraries separately. Cargo metadata does not»
- 92-99 «- The bundled Mozilla bullet font is outside the Cargo license check, as are the»
### examples/screenshot.rs — added 1 line(s) in 1 range(s)
added: 41
### examples/wasm_hello/Cargo.toml — added 1 line(s) in 1 range(s)
added: 5
### packages/accesskit_xplat/Cargo.toml — added 3 line(s) in 3 range(s)
added: 3 · 27 · 30
### packages/blitz-dom/Cargo.toml — added 14 line(s) in 4 range(s)
added: 20-21 · 32-34 · 37-38 · 78-84
### packages/blitz-dom/assets/default.css — added 2 line(s) in 1 range(s)
added: 42-43
### packages/blitz-dom/src/debug.rs — added 53 line(s) in 3 range(s)
added: 5 · 9-10 · 157-206
- 158-206 @159 «mod tests {»
  - 165-205 @166 «fn prints_tree_from_shared_document_reference() {»
### packages/blitz-dom/src/document.rs — added 42 line(s) in 8 range(s)
added: 416-417 · 1405-1410 · 2254-2271 · 2280-2282 · 2287-2288 · 2306-2313 · 2339-2340 · 2379
  - 2254-2270 @2257 «pub(crate) fn physical_unrounded_geometry(»
### packages/blitz-dom/src/layout/construct.rs — added 57 line(s) in 22 range(s)
added: 15 · 39 · 504 · 1070-1072 · 1097-1099 · 1102-1108 · 1111-1117 · 1120-1126 · 1134 · 1137 · 1145-1146 · 1194-1200
       1208-1210 · 1235 · 1251 · 1254 · 1261 · 1264 · 1270 · 1273 · 1282-1284 · 1306-1308
### packages/blitz-dom/src/layout/inline.rs — added 83 line(s) in 21 range(s)
added: 21-22 · 24 · 84 · 197-229 · 377-385 · 399 · 473-477 · 479-480 · 504-505 · 539-540 · 645-652 · 714 · 763 · 908
       914-917 · 919 · 1036 · 1065-1066 · 1068 · 1071 · 1102-1105
  - 197-228 @199 «fn inline_span_cb_flags(&self, root_id: NodeId, box_id: NodeId) -> StyleFlags {»
### packages/blitz-dom/src/layout/mod.rs — added 223 line(s) in 28 range(s)
added: 11 · 32-34 · 82-157 · 166 · 175-191 · 200-218 · 225 · 311-313 · 321-322 · 474-489 · 495-496 · 499 · 590-596
       608-612 · 640 · 642 · 651-663 · 686 · 693 · 732 · 763 · 779 · 795 · 811 · 815 · 831 · 846 · 868-911
- 82-103 @84 «pub(crate) struct LayoutPassState<'doc> {»
- 105-142 «impl<'doc> LayoutPassState<'doc> {»
  - 106-121 @109 «fn orthogonal_percent_basis_of(&self, dom_id: crate::NodeId) -> Option<f32> {»
  - 123-141 «pub(crate) fn new(doc: &'doc mut BaseDocument) -> Self {»
- 144-149 «impl Deref for LayoutPassState<'_> {»
  - 146-148 «fn deref(&self) -> &BaseDocument {»
- 151-155 «impl DerefMut for LayoutPassState<'_> {»
  - 152-154 «fn deref_mut(&mut self) -> &mut BaseDocument {»
  - 200-217 @203 «pub(crate) fn child_layout_style<'a>(»
  - 609-612 «type ChildIter<'a>»
- 870-874 «impl TaffyDebugTree<'_> {»
  - 871-873 «fn node_from_id(&self, node_id: NodeId) -> &Node {»
- 876-907 «impl TraversePartialTree for TaffyDebugTree<'_> {»
  - 877-880 «type ChildIter<'a>»
  - 882-887 «fn child_ids(&self, node_id: NodeId) -> Self::ChildIter<'_> {»
  - 889-896 «fn child_count(&self, node_id: NodeId) -> usize {»
  - 898-906 «fn get_child_id(&self, node_id: NodeId, index: usize) -> NodeId {»
### packages/blitz-dom/src/layout/replaced.rs — added 2 line(s) in 2 range(s)
added: 204 · 212
### packages/blitz-dom/src/layout/table.rs — added 42 line(s) in 19 range(s)
added: 3-4 · 28 · 33-34 · 140-161 · 212 · 221 · 246 · 279 · 443 · 460 · 468 · 517 · 567 · 608 · 635 · 770 · 791 · 793
       825
  - 143-150 @144 «{»
  - 151-154 @152 «{»
- 157-160 @158 «fn table_taffy_style(style: &ComputedValues, table_wm: WritingMode) -> taffy::Style<Atom> {»
### packages/blitz-dom/src/layout/text_transform.rs — new file · 979 line(s)
- 26-27 «const WORD_SEGMENTER: WordSegmenterBorrowed<'static> =»
- 28-29 «const GENERAL_CATEGORY: CodePointMapDataBorrowed<'static, GeneralCategory> =»
- 31-32 «const WIDTH_TRANSFORMS: TextTransform =»
- 37-44 @39 «pub(crate) struct CaseTransform {»
- 46-77 «impl CaseTransform {»
  - 47-51 «pub(crate) const NONE: Self = Self {»
  - 53-70 «pub(crate) fn from_style(style: &ComputedValues) -> Self {»
  - 72-76 @74 «fn has_turkic_casing(&self) -> bool {»
- 79-99 @83 «fn casing_language(lang: LanguageIdentifier) -> LanguageIdentifier {»
  - 84-86 «let Some(script) = lang.script else {»
  - 87-93 @88 «let language_script = match lang.language.as_str() {»
  - 94-98 «if script.as_str() == language_script {»
- 101-115 @108 «pub(crate) struct TextTransformer {»
- 117-202 «impl TextTransformer {»
  - 118-121 @119 «pub(crate) fn word_break<B: Brush>(&mut self, builder: &TreeBuilder<'_, B>) {»
  - 123-135 @125 «pub(crate) fn transform<'a, B: Brush>(»
  - 137-201 «fn transform_nonempty<'a, B: Brush>(»
- 204-236 @206 «fn math_auto(text: &str, output: &mut OutputSink<'_>) {»
  - 208-211 «let Some(c) = chars.next().filter(|_| chars.next().is_none()) else {»
  - 212-233 «let codepoint = match c {»
- 238-245 @239 «fn is_collapsible(white_space_collapse: WhiteSpaceCollapse, c: char) -> bool {»
  - 240-244 «match white_space_collapse {»
- 247-289 @250 «pub fn full_width(c: char) -> char {»
  - 251-261 «const HALFWIDTH_KATAKANA: [char; 63] = [»
  - 262-287 «let codepoint = match c {»
- 291-321 @293 «pub fn full_size_kana(c: char) -> char {»
  - 294-298 «const SMALL_KATAKANA_EXTENSIONS: [char; 16] = [»
  - 299-319 «let codepoint = match c {»
- 323-338 @325 «fn map_ascii<'a>(»
  - 331-333 «let Some(first) = text.bytes().position(|b| needs_mapping(&b)) else {»
- 340-351 @342 «struct OutputSink<'a> {»
- 353-415 «impl<'a> OutputSink<'a> {»
  - 354-362 «fn new(original: &'a str, buffer: &'a mut String) -> Self {»
  - 364-370 «fn push_str(&mut self, s: &str) {»
  - 372-387 @377 «fn push_char(&mut self, mut c: char) {»
  - 389-401 @390 «fn push_unmapped(&mut self, s: &str) {»
  - 403-407 @404 «fn write(&mut self, writeable: impl Writeable) {»
  - 409-414 «fn finish(self) -> &'a str {»
- 417-423 @418 «impl std::fmt::Write for OutputSink<'_> {»
  - 419-422 «fn write_str(&mut self, s: &str) -> std::fmt::Result {»
- 425-467 @428 «fn capitalize(»
  - 436-443 «let combined = if context.is_empty() {»
  - 446-466 «for segment_end in WORD_SEGMENTER.segment_str(combined).skip(1) {»
- 469-472 @470 «fn uppercase(text: &str, lang: &LanguageIdentifier, output: &mut OutputSink<'_>) {»
- 474-477 @475 «fn lowercase(text: &str, lang: &LanguageIdentifier, output: &mut OutputSink<'_>) {»
- 479-487 @482 «fn titlecase_segment(text: &str, lang: &LanguageIdentifier, output: &mut OutputSink<'_>) {»
- 489-494 @490 «fn uppercase(text: &str, _lang: &LanguageIdentifier, output: &mut OutputSink<'_>) {»
  - 491-493 «for c in text.chars() {»
- 496-506 @497 «fn lowercase(text: &str, _lang: &LanguageIdentifier, output: &mut OutputSink<'_>) {»
  - 498-502 @499 «if text.contains('Σ') {»
  - 503-505 «for c in text.chars() {»
- 508-516 @510 «fn titlecase_segment(text: &str, _lang: &LanguageIdentifier, output: &mut OutputSink<'_>) {»
  - 512-514 «if let Some(first) = chars.next() {»
- 518-523 @519 «fn push_chars(output: &mut OutputSink<'_>, chars: impl Iterator<Item = char>) {»
  - 520-522 «for c in chars {»
- 525-530 @526 «fn is_typographic_letter_unit(c: char) -> bool {»
  - 528-529 «GeneralCategoryGroup::Letter.contains(category)»
- 532-537 «fn ceil_char_boundary(s: &str, mut index: usize) -> usize {»
  - 533-535 «while !s.is_char_boundary(index) {»
- 539-979 @540 «mod tests {»
  - 544-556 «fn transform(kind: TextTransform, lang: &str, texts: &[&str]) -> Vec<String> {»
  - 558-563 «fn with_builder<R>(f: impl FnOnce(&mut TreeBuilder<'_, ()>, &mut TextTransformer) -> R) -> R {»
  - 565-574 «fn push(»
  - 576-578 «fn capitalize(texts: &[&str]) -> Vec<String> {»
  - 580-592 @581 «fn capitalize_words() {»
  - 594-600 @595 «fn capitalize_skips_leading_punctuation() {»
  - 602-608 @604 «fn capitalize_uses_titlecase() {»
  - 610-622 @611 «fn capitalize_across_text_nodes() {»
  - 624-636 @625 «fn capitalize_mid_word_text_node() {»
  - 638-650 @639 «fn capitalize_after_word_break() {»
  - 652-661 @653 «fn capitalize_after_collapsed_whitespace() {»
  - 663-668 @664 «fn capitalize_long_words() {»
  - 670-677 @672 «fn capitalize_language_sensitive() {»
  - 679-702 @680 «fn unchanged_text_is_borrowed() {»
  - 704-721 @706 «fn ascii_fast_path_matches_icu() {»
  - 723-737 @724 «fn uppercase_and_lowercase() {»
  - 739-764 @740 «fn math_auto_italic_mappings() {»
  - 766-777 @767 «fn math_auto_operates_on_each_text_node() {»
  - 779-796 @780 «fn math_auto_unchanged_text_is_borrowed() {»
  - 798-814 «fn transform_with(»
  - 816-818 «fn full_width_with(white_space_collapse: WhiteSpaceCollapse, text: &str) -> String {»
  - 820-852 @821 «fn full_width_mappings() {»
  - 854-865 @855 «fn full_width_preserved_spaces() {»
  - 867-872 @868 «fn full_width_leaves_collapsible_whitespace() {»
  - 874-885 @875 «fn full_size_kana_mappings() {»
  - 887-937 @888 «fn combined_transforms() {»
  - 939-963 @940 «fn width_transforms_unchanged_text_is_borrowed() {»
  - 965-978 @967 «fn uppercase_and_lowercase_language_sensitive() {»
### packages/blitz-dom/src/layout/writing_mode.rs — new file · 435 line(s)
- 29-32 «use taffy::{»
- 34-56 «impl BaseDocument {»
  - 35-55 @37 «pub(crate) fn root_layout_wm(&self, root_id: crate::NodeId) -> WritingMode {»
- 58-292 «impl LayoutPassState<'_> {»
  - 59-68 @62 «pub(crate) fn layout_wm_of(&self, node_id: crate::NodeId) -> WritingMode {»
  - 70-103 @72 «pub(crate) fn compute_child_layout_in_own_wm(»
  - 105-197 «fn compute_orthogonal_child_layout(»
  - 199-211 @203 «pub(crate) fn physicalise_and_round_layout(&mut self, root: NodeId) {»
  - 213-291 «fn physicalise_and_round_inner(»
- 294-324 @298 «fn transpose_static_positions(»
  - 303-305 «if output.oof_candidates.is_empty() {»
  - 306-310 «let vertical_wm = if child_wm.is_vertical() {»
  - 313-323 «for candidate in output.oof_candidates.as_mut_slice() {»
- 326-343 «fn mirror_static_position(position: AxisStaticPosition, extent: f32) -> AxisStaticPosition {»
  - 327-331 «let flip = |edge: AxisStaticEdge| match edge {»
  - 332-342 «AxisStaticPosition {»
- 345-368 @347 «fn physical_layout(logical: Layout, placer_wm: WritingMode, placer_size: Size<f32>) -> Layout {»
  - 348-350 «if !placer_wm.is_vertical() {»
  - 351-367 «Layout {»
- 370-435 «impl BaseDocument {»
  - 371-401 @375 «pub(crate) fn physical_unrounded_geometry(»
  - 403-434 @404 «fn placer_geometry(»
### packages/blitz-dom/src/lib.rs — added 1 line(s) in 1 range(s)
added: 88
### packages/blitz-dom/src/mutator.rs — added 20 line(s) in 5 range(s)
added: 388-393 · 993-998 · 1003-1006 · 1024-1026 · 1028
### packages/blitz-dom/src/net.rs — added 5 line(s) in 1 range(s)
added: 458-462
### packages/blitz-dom/src/node/element.rs — added 6 line(s) in 2 range(s)
added: 132-135 · 146-147
### packages/blitz-dom/src/node/node.rs — added 56 line(s) in 3 range(s)
added: 20-21 · 1123-1125 · 1128-1178
  - 1140-1150 @1143 «pub fn layout_style_in(»
  - 1152-1158 @1155 «pub(crate) fn has_containment(&self) -> bool {»
  - 1160-1171 @1163 «pub(crate) fn is_column_flex_container(&self) -> bool {»
### packages/blitz-dom/src/node/text.rs — added 4 line(s) in 1 range(s)
added: 32-35
### packages/blitz-dom/src/resolve.rs — added 16 line(s) in 2 range(s)
added: 26 · 445-459
### packages/blitz-dom/src/resolved_style.rs — added 106 line(s) in 16 range(s)
added: 5-7 · 16-18 · 340-341 · 347-350 · 384-398 · 401-413 · 425 · 427 · 453 · 458 · 489 · 502-540 · 610 · 631 · 650
       727-745
- 727-744 @729 «fn serialize_sides(sides: &[String]) -> String {»
  - 730-742 «let len = match sides {»
### packages/blitz-dom/src/scrolling.rs — added 5 line(s) in 1 range(s)
added: 776-780
### packages/blitz-net/src/lib.rs — added 10 line(s) in 1 range(s)
added: 160-169
### packages/blitz-paint/src/text.rs — added 10 line(s) in 5 range(s)
added: 137-138 · 194-196 · 466-467 · 470-471 · 601
### packages/blitz-shell/src/window.rs — added 1 line(s) in 1 range(s)
added: 682
### packages/blitz-vibey-script/Cargo.toml — added 3 line(s) in 3 range(s)
added: 33 · 36 · 48
### packages/blitz-vibey-script/src/dom/document.rs — added 7 line(s) in 1 range(s)
added: 75-81
  - 75-81 «define_method(»
### packages/blitz-vibey-script/src/dom/element.rs — added 16 line(s) in 4 range(s)
added: 97 · 101 · 105-106 · 764-775
- 764-774 «fn get_inner_text(this: &JsValue, _: &[JsValue], context: &mut Context) -> JsResult<JsValue> {»
  - 769-772 «let text = doc»
### packages/blitz-vibey-script/src/dom/mod.rs — added 1 line(s) in 1 range(s)
added: 14
### packages/blitz-vibey-script/src/dom/selection.rs — new file · 417 line(s)
- 6-8 «use boa_engine::{»
- 9-12 «use icu_properties::{»
- 21-27 @22 «struct Selection {»
- 29-57 «pub(crate) fn get_selection(»
  - 35-37 «if let Some(selection) = &ctx.state.borrow().selection {»
  - 38-44 «let selection = JsObject::from_proto_and_data(»
  - 45-51 «define_method(»
- 59-67 «fn selection_object(this: &JsValue) -> JsResult<JsObject> {»
  - 60-66 «this.as_object()»
- 69-119 «fn set_base_and_extent(»
  - 75-79 «if args.len() < 4 {»
  - 80-81 «let anchor = node_id_of_value(&args[0])»
  - 82-83 «let focus = node_id_of_value(&args[2])»
  - 84-87 «let points = [»
  - 90-106 «for &(id, offset) in &points {»
  - 107-112 «if points.iter().any(|&(id, _)| {»
- 121-128 «fn remove_all_ranges(this: &JsValue, _: &[JsValue], context: &mut Context) -> JsResult<JsValue> {»
- 130-145 «fn to_string(this: &JsValue, _: &[JsValue], context: &mut Context) -> JsResult<JsValue> {»
  - 135-138 @136 «if range_endpoints(&doc) != data.rendered_range.get() {»
  - 140-143 «if let Some(points) = data.points.get() {»
- 147-171 «fn range_endpoints(doc: &BaseDocument) -> Option<[Range; 2]> {»
  - 149-166 «let stable_range = |&(id, start, end): &(NodeId, usize, usize)| {»
  - 167-170 «Some([»
- 173-183 «fn apply_points(doc: &mut BaseDocument, points: [Point; 2]) {»
  - 174-182 «match (»
- 185-222 «fn rendered_point(doc: &BaseDocument, point: Point) -> Option<Point> {»
  - 187-189 «if !node.flags.is_in_document() {»
  - 190-209 «if let Some(root) = inline_root(doc, node.id) {»
  - 213-221 «node.children[offset..]»
- 224-235 «fn inline_root(doc: &BaseDocument, mut id: NodeId) -> Option<NodeId> {»
  - 225-234 «loop {»
- 237-257 «fn subtree_point(doc: &BaseDocument, id: NodeId, end: bool) -> Option<Point> {»
  - 239-246 «if let NodeData::Text(data) = &node.data {»
  - 247-256 «if end {»
- 259-267 «struct OffsetMapper<'a> {»
- 269-405 «impl OffsetMapper<'_> {»
  - 270-341 «fn visit(&mut self, id: NodeId) {»
  - 343-404 «fn advance(&mut self, c: char, transform: TextTransform, whitespace: WhiteSpaceCollapse) {»
- 407-417 «fn matching_prefix(text: &str, mapped: impl Iterator<Item = char>) -> Option<usize> {»
  - 410-415 «for c in mapped {»
### packages/blitz-vibey-script/src/inner_text.rs — new file · 280 line(s)
- 12-48 @16 «pub(crate) fn inner_text(element: &Node) -> String {»
  - 17-19 «if !is_rendered(element) {»
  - 26-39 «if let Some(mut root) = element»
  - 41-45 «let mut collector = InnerTextCollector {»
- 50-52 «fn display(node: &Node) -> Option<Display> {»
- 54-68 @56 «fn is_rendered(element: &Node) -> bool {»
  - 57-58 «let has_box = display(element)»
  - 60-67 «element.flags.is_in_document()»
- 70-80 «fn is_replaced_element(name: &markup5ever::LocalName) -> bool {»
  - 71-79 «matches!(»
- 82-88 @83 «fn is_generated_pseudo(node: &Node) -> bool {»
  - 84-87 «node.parent.is_some_and(|parent| {»
- 90-99 «fn is_inclusive_descendant_of(node: &Node, ancestor: NodeId) -> bool {»
  - 92-97 «while let Some(n) = current {»
- 101-106 @102 «struct InnerTextCollector {»
- 108-238 «impl InnerTextCollector {»
  - 109-120 «fn push_str(&mut self, s: &str) {»
  - 122-124 «fn require_line_breaks(&mut self, count: usize) {»
  - 126-173 @128 «fn visit(&mut self, node: &Node, filter: Option<NodeId>) {»
  - 175-184 «fn visit_child(&mut self, child: &Node, filter: Option<NodeId>) {»
  - 186-230 «fn visit_inline_layout(&mut self, root: &Node, layout: &TextLayout, filter: Option<NodeId>) {»
  - 232-237 «fn visit_inline_box(&mut self, root: &Node, id: u64, filter: Option<NodeId>) {»
- 240-248 «fn is_internal_table_part(display: DisplayInside) -> bool {»
  - 241-247 «matches!(»
- 250-258 «fn is_last_table_cell(cell: &Node) -> bool {»
  - 251-253 «let Some(row) = cell.parent.map(|id| cell.with(id)) else {»
  - 255-257 «row.children[position.map_or(0, |p| p + 1)..]»
- 260-280 @261 «fn inside_marker_len(root: &Node, text: &str) -> usize {»
  - 262-264 «let Some(list_item) = root»
  - 265-267 «else {»
  - 268-270 «if !matches!(list_item.position, ListItemLayoutPosition::Inside) {»
  - 271-274 «let len = match &list_item.marker {»
  - 275-279 «if text.is_char_boundary(len.min(text.len())) {»
### packages/blitz-vibey-script/src/lib.rs — added 1 line(s) in 1 range(s)
added: 37
### packages/blitz-vibey-script/src/runtime.rs — added 6 line(s) in 1 range(s)
added: 1402-1407
### packages/blitz-vibey-script/src/state.rs — added 2 line(s) in 1 range(s)
added: 73-74
### packages/blitz-vibey-script/tests/dom.rs — added 38 line(s) in 1 range(s)
added: 787-824
- 788-824 @791 «fn computed_style_used_values() {»
  - 792-818 «let doc = doc_from_html(»
  - 819-823 «assert_eq!(»
### packages/blitz-vibey-script/tests/selection.rs — new file · 266 line(s)
- 4-9 «fn select(html: &str, script: &str) -> Vec<String> {»
- 11-26 @12 «fn selection_identity_and_empty_state() {»
  - 13-21 «assert_eq!(»
  - 22-25 «"#»
- 28-42 @29 «fn selects_math_auto_transformed_character() {»
  - 30-37 «assert_eq!(»
  - 38-41 «"#»
- 44-61 @45 «fn selection_uses_utf16_offsets_in_both_directions() {»
  - 46-56 «assert_eq!(»
  - 57-60 «"#»
- 63-80 @64 «fn maps_multiple_text_nodes_and_element_child_offsets() {»
  - 65-75 «assert_eq!(»
  - 76-79 «"#»
- 82-97 @83 «fn maps_collapsed_whitespace_and_case_expansions() {»
  - 84-92 «assert_eq!(»
  - 93-96 «"#»
- 99-116 @100 «fn selection_is_recomputed_after_style_changes() {»
  - 101-111 «assert_eq!(»
  - 112-115 «"#»
- 118-137 @119 «fn invalid_offsets_throw_without_changing_selection() {»
  - 120-132 «assert_eq!(»
  - 133-136 «"#»
- 139-154 @140 «fn selects_across_inline_roots_and_anonymous_blocks() {»
  - 141-149 «assert_eq!(»
  - 150-153 «"#»
- 156-171 @157 «fn hidden_content_and_comments_do_not_shift_offsets() {»
  - 158-166 «assert_eq!(»
  - 167-170 «"#»
- 173-186 @174 «fn preserves_whitespace_and_forced_line_breaks() {»
  - 175-181 «assert_eq!(»
  - 182-185 «"#»
- 188-203 @189 «fn element_boundaries_map_across_block_children() {»
  - 190-198 «assert_eq!(»
  - 199-202 «"#»
- 205-220 @206 «fn preserves_pre_line_segment_breaks() {»
  - 207-215 «assert_eq!(»
  - 216-219 «"#»
- 222-238 @223 «fn native_selection_changes_replace_script_selection() {»
  - 225-227 «doc.eval(»
  - 228-232 «{»
- 240-255 @241 «fn removed_combining_marks_do_not_consume_the_next_character() {»
  - 242-250 «assert_eq!(»
  - 251-254 «"#»
- 257-266 @258 «fn retains_dom_endpoints_when_anonymous_inline_roots_are_rebuilt() {»
  - 259-260 «let mut doc =»
### packages/blitz/Cargo.toml — added 3 line(s) in 3 range(s)
added: 14 · 17 · 20
### packages/dioxus-native-dom/Cargo.toml — added 3 line(s) in 2 range(s)
added: 13 · 24-25
### packages/dioxus-native/Cargo.toml — added 3 line(s) in 2 range(s)
added: 13 · 26-27
### packages/stylo_taffy/Cargo.toml — added 3 line(s) in 1 range(s)
added: 27-29
### packages/stylo_taffy/src/convert.rs — added 64 line(s) in 9 range(s)
added: 6 · 12 · 273-279 · 373-395 · 438-441 · 532-542 · 544-546 · 549-550 · 938-949
- 373-394 @375 «pub fn inline_containing_block_claims(»
  - 382-387 «let fixed = !effects.filter.0.is_empty()»
  - 388-393 «taffy::ContainingBlockClaims {»
### packages/stylo_taffy/src/lib.rs — added 6 line(s) in 1 range(s)
added: 12-17
### packages/stylo_taffy/src/wrapper.rs — added 350 line(s) in 55 range(s)
added: 2 · 6 · 10 · 26-31 · 42-55 · 59-60 · 62-138 · 145 · 153 · 190 · 196 · 199-200 · 221-224 · 229 · 235 · 238 · 244
       247 · 253 · 256 · 261-262 · 268 · 273 · 279-293 · 298 · 304 · 321 · 330-331 · 350-447 · 461-464 · 469 · 471
       483-484 · 490-491 · 538-548 · 572-575 · 669-717 · 751 · 772 · 793 · 798 · 825 · 837 · 873-883 · 893-894 · 903
       908 · 913-916 · 921 · 923 · 931-934 · 939 · 941 · 952 · 957
  - 76-90 @79 «pub fn new_in(style: T, flags: StyleFlags, layout_wm: WritingMode) -> Self {»
  - 92-103 @94 «fn layout_wm(&self) -> WritingMode {»
  - 105-115 @106 «fn align_axis_is_inline(&self) -> bool {»
  - 117-127 @118 «fn percent_basis(&self) -> Option<f32> {»
  - 129-133 @131 «fn rect<U>(&self, physical: taffy::Rect<U>) -> taffy::Rect<U> {»
  - 356-360 @358 «fn inline_item_alignment(&self, input: stylo::AlignFlags) -> Option<taffy::AlignItems> {»
  - 362-373 @365 «fn content_left_is_end(&self, main_is_inline: bool) -> Option<bool> {»
  - 375-388 @378 «fn self_alignment(»
  - 390-403 @393 «fn oof_self_alignment(»
  - 405-412 @408 «fn oof_inline_item_alignment(&self, input: stylo::AlignFlags) -> Option<taffy::AlignItems> {»
  - 414-423 @418 «pub fn set_percent_basis(&mut self, basis: Option<f32>) {»
- 669-716 @672 «impl<T: Deref<Target = ComputedValues>> TaffyStyloStyle<T> {»
  - 673-677 @674 «fn template_rows_source(&self) -> &stylo::GenericGridTemplateComponent<LengthPercentage, i32> {»
  - 679-685 @680 «fn template_columns_source(»
  - 687-691 @688 «fn auto_rows_source(&self) -> &stylo::ImplicitGridTracks {»
  - 693-697 @694 «fn auto_columns_source(&self) -> &stylo::ImplicitGridTracks {»
  - 699-706 @700 «fn grid_row_placement(&self) -> taffy::Line<taffy::GridPlacement<Atom>> {»
  - 708-715 @709 «fn grid_column_placement(&self) -> taffy::Line<taffy::GridPlacement<Atom>> {»
### packages/stylo_taffy/src/writing_mode.rs — new file · 283 line(s)
- 26-84 @27 «pub trait WritingModeExt: Copy {»
- 86-94 @88 «fn taffy_left_side(wm: WritingMode) -> PhysicalSide {»
  - 89-93 «if wm.swaps_axes() {»
- 96-283 «impl WritingModeExt for WritingMode {»
  - 97-100 @98 «fn swaps_axes(self) -> bool {»
  - 102-105 @103 «fn line_left_is_bottom(self) -> bool {»
  - 107-124 @108 «fn direction_of(self, own: WritingMode) -> taffy::Direction {»
  - 126-129 @127 «fn direction(self) -> taffy::Direction {»
  - 131-134 @132 «fn inline_left_is_end(self) -> bool {»
  - 136-139 @137 «fn block_left_is_end(self) -> bool {»
  - 141-152 @142 «fn block_start_is_reversed(self, own: WritingMode) -> bool {»
  - 154-170 @155 «fn logical_rect<T>(self, physical: taffy::Rect<T>) -> taffy::Rect<T> {»
  - 172-188 @173 «fn physical_rect<T>(self, logical: taffy::Rect<T>) -> taffy::Rect<T> {»
  - 190-197 @191 «fn logical_size<T>(self, physical: taffy::Size<T>) -> taffy::Size<T> {»
  - 199-206 @200 «fn logical_point<T>(self, physical: taffy::Point<T>) -> taffy::Point<T> {»
  - 208-215 @209 «fn logical_aspect_ratio(self, physical: Option<f32>) -> Option<f32> {»
  - 217-229 @219 «fn logical_float(self, float: taffy::Float) -> taffy::Float {»
  - 231-243 @233 «fn logical_clear(self, clear: taffy::Clear) -> taffy::Clear {»
  - 245-257 @247 «fn logical_text_align(self, align: taffy::TextAlign) -> taffy::TextAlign {»
  - 259-282 «fn transpose_style<S: taffy::CheapCloneStr>(self, style: &mut taffy::Style<S>) {»
### tests/blitz-tests/Cargo.toml — added 15 line(s) in 2 range(s)
added: 17 · 44-57
### tests/blitz-tests/tests/all.rs — new file · 84 line(s)
- 67-84 @68 «fn all_test_files_are_included() {»
  - 71-83 «for entry in std::fs::read_dir(dir).unwrap() {»
### tests/blitz-tests/tests/anonymous_block_percentage_height.rs — new file · 39 line(s)
- 6-39 @7 «fn anonymous_block_preserves_percentage_height_containing_block() {»
  - 8-38 «for container_height in ["200px", "auto"] {»
### tests/blitz-tests/tests/autofocus_attribute.rs — new file · 50 line(s)
- 9-25 «fn focussed_id(attr: &str) -> Option<String> {»
  - 10-12 «let html = format!(»
  - 13-19 «let doc = HtmlDocument::from_html(»
  - 21-24 «doc.get_node(node_id)?»
- 27-30 @28 «fn bare_autofocus_focusses_element() {»
- 32-35 @33 «fn empty_autofocus_focusses_element() {»
- 37-40 @38 «fn true_autofocus_focusses_element() {»
- 42-45 @43 «fn false_autofocus_is_ignored() {»
- 47-50 @48 «fn no_autofocus_leaves_focus_unset() {»
### tests/blitz-tests/tests/inline_fragment_rects.rs — added 27 line(s) in 1 range(s)
added: 124-150
- 125-150 @126 «fn offset_sizes_use_unrounded_border_box() {»
  - 127-149 «for writing_mode in ["horizontal-tb", "vertical-lr", "vertical-rl"] {»
### tests/blitz-tests/tests/text_transform.rs — new file · 47 line(s)
- 3-16 «fn layout_text(html: &str) -> String {»
  - 7-15 «doc.get_node(id)»
- 18-28 @19 «fn math_auto_is_inherited_and_can_be_overridden() {»
  - 20-27 «assert_eq!(»
- 30-39 @31 «fn math_auto_counts_characters_before_whitespace_collapsing() {»
  - 32-38 «assert_eq!(»
- 41-47 @42 «fn math_auto_transforms_adjacent_single_character_text_nodes() {»
  - 43-46 «assert_eq!(»
### wpt/WPT_COMMIT — added 1 line(s) in 1 range(s)
added: 1
### wpt/runner/Cargo.toml — added 6 line(s) in 3 range(s)
added: 15 · 28-29 · 53-55
### wpt/runner/src/main.rs — added 91 line(s) in 16 range(s)
added: 43 · 46 · 239-240 · 269-276 · 285 · 289 · 299-300 · 305 · 476-480 · 483 · 485-486 · 541 · 618 · 627 · 688
       853-914
  - 476-479 «let suites = env::args()»
- 854-914 @855 «mod tests {»
  - 858-913 @859 «fn discovers_variants_and_accepts_explicit_urls() {»
### wpt/runner/src/test_runners/js_wrapper.rs — added 8 line(s) in 1 range(s)
added: 27-34
- 27-33 «pub fn js_test_variants(js_source: &str) -> Vec<String> {»
  - 28-32 «parse_metas(js_source)»
### wpt/runner/src/test_runners/mod.rs — added 9 line(s) in 9 range(s)
added: 15 · 28 · 176 · 259 · 265 · 294 · 318 · 326 · 369
### wpt/runner/src/test_runners/ref_test.rs — added 57 line(s) in 9 range(s)
added: 23 · 36 · 116 · 125-126 · 129-132 · 172-175 · 207-210 · 220-224 · 355-389
  - 172-175 «let ref_out_path = ctx.out_dir.join(format!(»
- 220-223 «fn resolve_reference_url(test_url: &Url, ref_file: &str) -> Result<Url, url::ParseError> {»
- 356-389 @357 «mod tests {»
  - 360-377 @361 «fn references_receive_query_and_fragment_variants() {»
  - 379-388 @380 «fn explicit_reference_queries_are_preserved() {»
### wpt/runner/src/test_variants.rs — new file · 291 line(s)
- 7-9 «use html5ever::tokenizer::{»
- 13-16 «pub struct Test {»
- 18-21 «pub fn split_test_url(url: &str) -> (&str, &str) {»
- 23-27 «pub fn is_xml(path: &str) -> bool {»
  - 24-26 «[".xht", ".xhtm", ".xhtml", ".xml", ".svg"]»
- 29-38 «pub fn test_url(path: &str, variant: &str) -> String {»
  - 30-36 «let path = if let Some(stem) = path.strip_suffix(".any.js") {»
- 40-49 @41 «pub fn artifact_name(url: &str) -> String {»
  - 43-45 «if suffix.is_empty() {»
- 51-96 «pub fn test_variants(path: &str, source: &str) -> Vec<String> {»
  - 52-81 «let mut variants = if path.ends_with(".js") {»
  - 83-91 «for variant in &variants {»
  - 92-94 «if variants.is_empty() {»
- 98-118 «pub fn expand_test(wpt_dir: &Path, path: PathBuf, suffix: &str) -> Vec<Test> {»
  - 99-103 «let relative_path = path»
  - 104-110 «let variants = if suffix.is_empty() {»
  - 111-117 «variants»
- 123-157 «impl TokenSink for VariantSink {»
  - 126-156 «fn process_token(&self, token: Token, _: u64) -> TokenSinkResult<()> {»
- 159-291 @160 «mod tests {»
  - 163-177 @164 «fn html_variants_replace_default_and_decode_entities() {»
  - 179-193 @180 «fn ignores_metadata_in_comments_and_scripts() {»
  - 195-209 @196 «fn xml_variants_require_html_namespace() {»
  - 211-223 @212 «fn xhtml_variants_with_doctype_and_prefixed_elements() {»
  - 225-234 @226 «fn js_variants_only_from_initial_metadata_block() {»
  - 236-240 @237 «fn undeclared_variants_run_once() {»
  - 242-254 @243 «fn wrapper_urls_preserve_suffix() {»
  - 256-274 @257 «fn script_location_uses_wrapper_url_and_variant() {»
  - 276-284 @277 «fn artifact_suffix_cannot_change_directories() {»
  - 286-290 @288 «fn rejects_empty_query() {»
