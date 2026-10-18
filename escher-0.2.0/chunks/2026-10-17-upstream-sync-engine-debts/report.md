# Report — 2026-10-17-upstream-sync-engine-debts

**Chunk:** Upstream sync ahead of engine debts — upstream/main merged at a measured pin, our changes mostly additive; CI and audit green, per-OS tallies re-read, sink allowlist re-checked
**Date:** 2026-10-17
**Commits (since last_wrap 2026-10-17T08:30:00Z, from the base `143b6013`):**
- `3eda07bc` chore(2026-10-17-upstream-sync-engine-debts): operator pre-CI commit, for the run this chunk verdict reads — the two-parent merge commit (`143b6013` · `23b9b26a9`), tree `45fcd733`; reworded from `984fabd4`, same tree (`evidence/operator-pass.md`, *The reword*)
- `ae8c8daf` fix(deps): ignore the zbus-family unmaintained advisories by ID
- Upstream's 10, merged by `3eda07bc`: `23b9b26a9` Delete the bump crate · `92b11f1bd` Bump version for 0.3.0-beta.4 release · `85340347f` Revert to Taffy 0.15 · `dd6f46b55` aria-description and aria-describedby (#1013) · `8114cd505` ARIA level, disabled, selected, expanded (#1014) · `fabd70e78` aria-label and aria-labelledby (#859) · `3f483ff2a` keyboard-types 0.8 (compat code in dioxus-native-dom) · `e72650a6e` vibey-script Element.attributes, getAttributeNames, clientTop/clientLeft, offsetParent, window.event (#1211) · `6032d0b9d` Bump Taffy to main (#1212) · `8d35e562c` baseline-source for atomic inlines (#1198)

## Changes (structured — detectors read this)

- **Files** (`gate.py scope`, base `143b6013`: changed 36 · listed 35 · recorded 1; excluded 106 bookkeeping and version-workspace files):
  - By the merge, upstream's: `Cargo.toml`, `justfile`, `apps/bump/Cargo.toml` (D), `apps/bump/src/main.rs` (D),
    `packages/blitz-dom/src/{layout/inline.rs,node/text.rs,util.rs}`, `packages/blitz-shell/src/convert_events.rs`,
    `packages/blitz-vibey-script/{src/dom/element.rs,src/runtime.rs,tests/preact.rs}`,
    `packages/dioxus-native-dom/src/{events.rs,keyboard_compat.rs (A),lib.rs}`,
    `tests/blitz-tests/tests/{accessibility_description.rs (A),accessibility_label.rs (A),all.rs}`.
  - Conflicts resolved: `packages/blitz-dom/src/accessibility.rs` (both sides kept), `Cargo.lock`.
  - Ours: `packages/blitz-test-harness/src/lib.rs`, `packages/blitz-dom/src/events/keyboard.rs` (our arms; upstream's
    Tab line by the merge), `packages/escher-driver/src/execute.rs`, `packages/dioxus-native-dom/src/snapshot.rs`, the ten
    blitz-tests checks (`range_input_interaction`, `stand_accessibility_ids`, `stand_contrast`, `stand_diff`,
    `stand_keyboard`, `stand_session_fresh`, `stand_session_state`, `stand_settle`, `stand_snapshot_state`,
    `stand_snapshot_text`), `.github/scripts/ci-leg.sh`, `.github/scripts/test_ci_workflows.py`.
  - The operator pass: `deny.toml` (M, a recorded widening — Deviations).
  - Ledger: `.andromeda/residuals.md` (M, one `open` line appended at this wrap before the new-text listing, line 30,
    inputs#I3).
  - Chunk folder: `scope.md`, `research.md`, `plan.md`, `scope-record.md` (this wrap), `inputs/` (I1–I3), `evidence/`
    (`before.md`, `merge.md`, `lock.md`, `counts.md`, `readings.md`, `sink.md`, `ci.md`, `operator-pass.md`).
- **Symbols / APIs** (coordinates from §New text, by line):
  - `packages/blitz-dom/src/accessibility.rs` — `build_accessibility_tree` keeps its signature and its three callers
    (`Document::accessibility_tree`, `BlitzApplication::handle_blitz_shell_event`, `DioxusDocument::accessibility_tree`).
    The builder now also holds upstream's reads, as upstream wrote them (`evidence/merge.md`):
    - `aria-labelledby`: after our `<label>` → input loop, it clears `labelled_by`, pushes the referenced nodes and
      sets the computed label (`set_label`), so it outranks a `<label>` (accname 2B over 2D);
    - `aria-describedby`: the referenced nodes pushed as `described_by`;
    - `aria-description`: its own text into AccessKit's description field;
    - `aria-expanded` (boolean), `aria-disabled` (`"true"` → `set_disabled`), `aria-level` with the tag and role
      defaults (a number), and `set_html_tag` last.
    - Our trimmed non-empty `aria-label` read and our closed-set `aria-selected` · `aria-pressed` · `aria-invalid`
      reads are kept. Upstream's own `aria-selected` (`parse::<bool>`) and unfiltered `aria-label` reads are dropped,
      so there is still one read per attribute (entries 14 · 15: 1 each).
    - Upstream's private helpers, new text: `referenced_dom_ids_to_node_ids` 118-125, `write_text_alternative` 244-252,
      `write_flat_text_content` 254-264, `FlatText` 267-295, `default_level_for_tag` 362-374, `default_level_for_role`
      376-383.
  - `packages/blitz-test-harness/src/lib.rs:32` — the re-export is now `pub use keyboard_types::{Key, Modifiers,
    NamedKey};`. escher-driver keeps naming keyboard types only through the harness, with no `keyboard-types`
    dependency of its own (entry 11 over its manifest).
  - `packages/escher-driver/src/execute.rs` — `keyboard_key` (added 490-494 · 496-501) maps each of the 11 named keys to
    `KeyboardKey::Named(NamedKey::…)`, and Space stays `Character(" ")`. Its unit test table is at 545-549 · 551-556.
    `ACTION_MODIFIER` under `cfg(target_os = "macos")` (:462) is now `Modifiers::META`. Its callers are unchanged:
    `execute/press()` and the unit test. The driver's twelve key-name words, `command.rs`, `schema.rs`, the wire and
    `help` are unchanged (entry 11).
  - `packages/blitz-dom/src/events/keyboard.rs` — our Enter arm (:111) and range arms (151-154) are spelled
    `Key::Named(NamedKey::…)`; upstream's Tab line (:27) and its `NamedKey` import (:12) arrived by the merge.
  - `packages/dioxus-native-dom/src/keyboard_compat.rs` (new, 1-564, upstream's) — a crate-private module
    (`mod keyboard_compat;`, `lib.rs:20`) converting keyboard-types 0.8 into Dioxus's 0.7 types: `convert_modifiers`
    10-12, `convert_location` 14-21, `convert_key` 23-28, `convert_named_key` 30-342, `convert_code` 344-564, all
    `pub(crate)` but `convert_named_key`. Its caller is `events.rs` (upstream's 10 added lines).
  - `packages/blitz-vibey-script/src/dom/element.rs` (upstream's #1211) — new script bindings `getAttributeNames`
    436-444, `attributes` 446-467 (both over `read_attrs` 421-434), `offsetParent` 1110-1123, `clientTop` 1128-1130,
    `clientLeft` 1132-1136; `runtime.rs` gains `window.event` (added 1750-1758 · 1822-1826 · 1860).
  - `packages/blitz-dom/src/layout/inline.rs` (#1198) — honours `baseline-source` when aligning atomic inlines (added 5
    · 411-415 · 423-428 · 436-438 · 450-462).
  - `packages/dioxus-native-dom/src/snapshot.rs:635-638` — one unit test's key spelled `Key::Named(NamedKey::ArrowRight)`.
  - No port, listener, socket, env var, verb, field, cause, span field or log site is added (entries 11 · 19).
- **Crates / modules:** `apps/bump` (crate `bump`) leaves the workspace and its `members` (entries 6 · 7).
  dioxus-native-dom gains the private module `keyboard_compat` (upstream's). No crate is added.
- **Dependencies** (`evidence/lock.md`; entry 10: the lock set equals upstream's at the pin, our `escher-*` entries set
  aside):
  - keyboard-types `"0.7"` → `"0.8"` (lock 0.8.3) — upstream's; Dioxus 0.7.3 still carries its own keyboard-types 0.7,
    so two versions now stand in the graph.
  - test-that and test-that-macro 0.5.2 → 0.6.0 (dev-dependencies of blitz-tests). Reviewed by hand: Apache-2.0, the same
    repository, no build script, proc-macro flag unchanged.
  - The workspace version 0.3.0-beta.3 → 0.3.0-beta.4, with its nine in-repo `=` pins (entry 4: 11 lines) and our two
    `escher-*` lock entries (entry 9).
  - Leaving with bump: `toml_datetime` 0.6.11, `toml_edit` 0.22.27, `toml_write` 0.1.2.
  - Taffy stays the registry `"0.15"`; there is no git dependency in the root manifest (entry 5).
- **Schema / config:**
  - `.github/scripts/ci-leg.sh` (added 41-42 · 44-45) — both `cargo llvm-cov` lines (the `--lcov` export and the
    `report`) take `--ignore-filename-regex 'keyboard_compat\.rs$'`, with a comment naming the file as upstream's
    temporary keyboard-types 0.7/0.8 bridge, excluded until upstream retires it. `coverage_floor.py` and the three floors
    are unchanged.
  - `.github/scripts/test_ci_workflows.py:327` — `test_coverage_leg_reaches_the_floor_on_each_crate` also asserts
    `--ignore-filename-regex` among the leg's words.
  - `deny.toml:26-28` — `[advisories] ignore` grows from 5 entries to 8: RUSTSEC-2026-0370 (zvariant 5.15.0),
    RUSTSEC-2026-0371 (zbus_names 4.3.4) and RUSTSEC-2026-0372 (zvariant_derive 5.15.0), each per ID with its reason:
    unmaintained, folded into zbus 6.0.0 (z-galaxy/zbus#2043); reached only through zbus 5.19 + atspi 0.29 under
    accesskit_unix 0.24.0 (the latest, requiring zbus ^5.19) ← accesskit_xplat ← blitz-shell, so no fix is reachable;
    owned by the `.andromeda/residuals.md` line; re-checked by the audit leg on every push. On the operator's word
    (Deviations).
- **Spec-master edits:** none.
- **Counts / qualifiers moved** (every site from the pre-pass sweep, trail
  `cascade-2026-10-17-upstream-sync-engine-debts-pre-pass.json`, baseline `ae8c8daf`; each row read, dispositioned per
  row or per class):
  - **The upstream sync pin** `434e0661` → `23b9b26a9`, merge `3eda07bc`. Pattern `pin`, 11 standing: 2 changed —
    `architecture.md:206` ×3 (§Project Intent, the `Upstream sync:` line) and `security-plan.md:242` ×2 (§Dependency
    Security, the Pinning sync record); 9 no-change:
    - `architecture.md:87` ×4, `:136`, `security-plan.md:145`, `test-plan.md:24`, `a11y-plan.md:186` — dated
      measurements at `434e0661` (`mutation_writer.rs` byte-identical to upstream's there). They still hold as dated:
      entry 11 kept `mutation_writer.rs` unchanged, and the plan reads it byte-identical to upstream at the pin too.
      Re-pointing them to `23b9b26a9` is P2's option, not a defect;
    - `test-plan.md:26`, `obs-plan.md:70` — dated statements of the 2026-10-13 merge;
    - `test-plan.md:331` — the §9 dated tally history (the line gains an entry, under `tally`);
    - `architecture.md:211` — a generic "an Upstream sync".
    - Leaves: `.claude/docs/workflow.md:15` ×2 (changed), `obs-summary.md:49` and `tests-summary.md:30` (dated, no-change).
  - **The workspace version** 0.3.0-beta.3 → 0.3.0-beta.4. Pattern `wsver`, 3 standing, all changed:
    `architecture.md:45` ×2 (§Stack, the workspace package and the in-repo `=` pins), `architecture.md:220` (the engine
    crates pinned), `security-plan.md:242` ×2 (Pinning: the record gains this move). Leaf `stack.md:7`.
  - **keyboard-types** `"0.7"` → `"0.8"`, the harness re-export gaining `NamedKey`, and the new `keyboard_compat`
    module. Pattern `kbt`, 5 standing:
    - 3 changed: `architecture.md:52` (§Stack, `keyboard-types "0.7"`); `architecture.md:130` ×2 @c199,225 and
      `test-plan.md:88` ×2 @c1807,1833 ("re-exports of `keyboard_types::Key` and `keyboard_types::Modifiers`, the two
      types `press_with` takes" — now three, with `NamedKey`);
    - 2 no-change: `a11y-plan.md:172` (winit key events converted to keyboard-types — still true; its citation is
      the citation sweep's) and `a11y-plan.md:193` (the browser app's `NamedKey::BrowserBack`).
    - 0 rows name `keyboard_compat`, so its text is new: the coverage exclusion (test-plan §10) and the module in arch
      §Existing Scopes' dioxus-native-dom row.
    - Leaves: `services/blitz-test-harness.md:11,14`, `services/blitz-traits.md:11`, `stack.md:21`.
  - **The re-export** `Key` · `Modifiers` → `Key` · `Modifiers` · `NamedKey`. Pattern `reexp`, 3 standing, all changed:
    `architecture.md:101` @c3292 (the driver's key types come through the harness's re-exports), `architecture.md:130`
    ×2, `test-plan.md:88`. Leaves: `rules/verification-harness.md:31`, `services/blitz-test-harness.md:14`,
    `services/escher-driver.md:11`.
  - **test-that** 0.5.2 → 0.6.0. Pattern `testthat`, 2 standing: `architecture.md:59` changed; `architecture.md:64`
    no-change (the dev-dependency names, no version). Leaves: `stack.md:44` changed, `tests-summary.md:12` no-change.
  - **The bump crate leaves.** Pattern `bump`, 18 standing: 17 changed, 1 no-change.
    - Changed in architecture: `:44` ×2 (the edition-2024 list), `:63` ×3 (§Stack, the Release tooling row), `:76` ×4
      ([Release versioning]: "bump versions blitz packages together", citing `apps/bump`), `:138` ×2 (bump's
      arguments), `:154` ×2 (§Occupied Resources, the Names row), `:171` (the directory tree's `bump/`), `:180` ×2,
      `:213` ×2, `:216` ×2, `:225` ×3 (each describes or cites `apps/bump`), `:262` ×4 (the crate table's bump row).
    - Changed elsewhere: `security-plan.md:73` ×2 (§Input Validation, the "CLI arguments (bump)" row) and `:330` ×2
      (§Error Handling, bump's `bail!`); `design-system.md:385` ×2 and `:396` ×4 (§Surface: cli); `layout-templates.md:71`
      ×4 (§Surface: cli); `test-plan.md:10` (the "apps (rdme, bump, examples, accesskit_xplat)" inventory line).
    - No-change: `security-plan.md:237` @c1257 ("a lone bump of the coupled winit pin" — the verb).
    - Leaves: `CLAUDE.md:15` (changed), `CLAUDE.md:51` (the verb, no-change), `commands.md:71` (the `just bump` recipe,
      which leaves with the justfile), `design-summary.md:28`, `security-summary.md:18`, `workflow.md:38` (changed);
      `gotchas.md:7` and `stack.md:59` (the verb, no-change).
  - **The three toml crates leave with bump.** Pattern `tomlc`, 2 standing, both changed, on the bump rows:
    `architecture.md:63`, `:180`.
  - **The builder's six new reads.** Pattern `aria6`, 8 standing:
    - 4 changed:
      - `architecture.md:122` ×2 @c1928,2185 (§Standard Contracts → Mutation, query and CSSOM: "naming a node from
        three sources" — `aria-labelledby` is a fourth, and it outranks a `<label>`);
      - `security-plan.md:125` ×2 @c565,2456 (§Input Validation, the `aria-label` · `<label for>` row);
      - `a11y-plan.md:281` @c198 (§7 names: the name sources gain `aria-labelledby`);
      - `architecture.md:136` @c12639 (a snapshot node's `name` is its non-empty `label`, else its `labelled_by`
        names). The letter holds, but the computed `aria-labelledby` label now reaches `label`, so a name source is
        added.
    - 4 no-change: `a11y-plan.md:117`, `:122`, `:125` (the example apps' markup inventory) and `:279` (the label
      association's order).
    - 0 rows state `aria-describedby`, `aria-description`, `aria-level` or `aria-disabled` as builder reads, so those
      four are new text in arch §Standard Contracts, security-plan §Input Validation and a11y-plan §2.
    - Leaves: `rules/a11y.md:26`, `services/dioxus-native-dom.md:17`.
  - **The three closed-set reads, unchanged** (control for `aria6`). Pattern `aria3`, 13 standing: 12 no-change
    (`architecture.md:87`, `:133`, `:136`; `security-plan.md:124`, `:126` ×2; `layout-templates.md:10`; `test-plan.md:24`,
    `:26`; `a11y-plan.md:46`, `:102`, `:282`, `:283` — each states the three reads, which the merge keeps as they were)
    and 1 changed, `architecture.md:122` (the same line as `aria6`). Its 10 leaf rows are no-change.
  - **Files and targets** 125/124 → 127/126 (`evidence/counts.md`; entries 24 · 25 · 27). Pattern `files`, 5 standing:
    - 1 changed: `test-plan.md:66` @c2359 ("124 `.rs` files, 123 test targets"). It was already one short before this
      chunk — see Spec claims disproved;
    - 4 no-change: `architecture.md:115`, `:141`, `test-plan.md:8`, `:146` (`BlitzTestsTargetsTest` pins the manifest
      shape, which is unchanged — entry 20).
  - **The inventory gains `accessibility_description` and `accessibility_label`.** Pattern `newt`, 11 standing:
    - 2 changed: `test-plan.md:26` (the blitz-tests inventory) and `test-plan.md:66` @c4381 (the files' history
      clause);
    - 9 no-change: `architecture.md:122`, `:264`, `security-plan.md:126`, `test-plan.md:116`, `:331`, `obs-plan.md:70`
      ×2, `a11y-plan.md:46`, `:68`, `:102` — each names `accessibility_states`, which is unchanged. The two new files stay
      outside the a11y leg's six (plan Leans).
  - **The workspace tally** 186 · 958 · 0 · 12 → 187 · 971 · 0 · 12 (`fast`, entry 23). Per target: `accessibility_description`
    +3, `accessibility_label` +10, and `unittests src/main.rs (bump)` 0 · 0 · 0 gone (`evidence/counts.md`). **The
    per-OS tallies** read at CI#39056637407 on `ae8c8daf` (each equal to CI#39043080454's):
    - linux 187 · 971 · 0 · 12; macos and coverage 168 · 968 · 0 · 12; windows 168 · 944 · 0 · 9;
    - the a11y job 6 · 43 · 0 · 0; the stand repeat 141 · 576 · 0 · 21; ios and android build only.
    Pattern `tally`, 3 standing:
    - 2 changed: `test-plan.md:325` (§9 Per-OS test tallies, last read at 2026-10-14-quality-gates: linux 185 · 951 ·
      0 · 12) and `test-plan.md:331` (the dated tally history, which gains this chunk's entry);
    - 1 no-change: `test-plan.md:330` (another leg).
    - At `test-plan.md:331` all 35 offsets are dated entries of earlier chunks: no-change at every one.
    - Leaf `tests-summary.md:30` ×14 (dated, gains an entry); curation `session-learnings.md:104` (no-change).
  - **The coverage exclusion** of `keyboard_compat.rs`. dioxus-native-dom reads 2472/2934 = 84.25 % (byte-equal to the
    baseline), escher-driver 98.19 %, blitz-test-harness 91.69 %, and the leg ends `coverage floor: 3 of 3 crates at or
    above their floors` (entries 40 · 41). Pattern `cov`, 10 standing:
    - 3 changed: `architecture.md:141` @c2609 (the coverage leg's command), `test-plan.md:335` ×5 (the leg's two
      `cargo llvm-cov` lines), `test-plan.md:343` ×2 (§10 Coverage floors: the exclusion, and the reading unchanged);
    - 7 no-change: `architecture.md:66`, `:67` ×2, `:141` @c7251,7320 (the floor script's output lines), `:226`,
      `security-plan.md:250`, `test-plan.md:8`, `:333`, and the registry key file
      `contracts/test-plan/bootstrap-phases-derive-for-route-setup-project.md:3` (cargo-llvm-cov named as a tool).
    - Leaves: `rules/testing.md:47`, `commands.md:53`, `stack.md:39`, `tests-summary.md:42`.
  - **The cargo-deny ignores** 5 → 8. Pattern `deny`, 1 standing, changed: `security-plan.md:237` ×5 (§Dependency
    Security, "Five ignores stand", the zip three and their owner `.andromeda/residuals.md:29`). It gains
    RUSTSEC-2026-0370/0371/0372 and their owner `.andromeda/residuals.md:30`. Leaves: `rules/security.md:26` ×6,
    `security-summary.md:42` ×5.
  - **The workspace integration-test executables** 135 → 137, with 0 named `all` (entry 27). Pattern `exe`, 1
    standing, changed: `test-plan.md:66` @c5008 ("no `all` executable among 107 … and among 128 at
    2026-10-13-upstream-sync-polish-and-ship").
  - **The agent-run results:** `run stand` unmoved at 168 · 0 · 7; `run all` 533 · 0 · 12 → 546 · 0 · 12 (+13, the two
    upstream files; entries 34 · 36). Pattern `runall`, 9 standing:
    - 1 changed: `test-plan.md:116` (the agent-run history, ×36), which gains this chunk's entry; all 36 prior
      offsets are dated, no-change;
    - 8 no-change: `test-plan.md:18`, `:26` ×3, `:107`, `:110`, `:115`, `:122`, `:147`, `obs-plan.md:87` (mechanism
      prose).
    - Leaf `tests-summary.md:30` (dated, gains an entry); the other 13 leaf rows are mechanism, no-change.
  - **The macOS `ACTION_MODIFIER`** `Modifiers::SUPER` → `Modifiers::META`. Pattern `macmod`: its control is unfired over
    the seven masters (`control: macmod, title never fired over the seven masters' pre-pass text (ae8c8daf)`), so no
    master states it.
  - **The residuals ledger** gains one `open` line, `.andromeda/residuals.md:30`. No master site: it is a ledger entry
    (route-resolve's file).
  - Owner patterns: `title` (`Upstream sync ahead of engine debts`) — `owner pattern: control unfired` (the refusal line
    above). `owner-gen` — 1 standing, `security-plan.md:124` (an id's owner, not this chunk's fact), no-change; 2 base
    rows in `playbook.md` (`:58`, `:66`).
  - Sweep reach: 17 patterns · 18 moved facts, 17 with a pattern (the report's own count) · owner pattern control
    unfired · by line 16 lines · 93 offsets.
- **Dev-tool versions:** none changed. cargo-deny's local advisory database moved: it now holds RUSTSEC-2026-0370/0371/0372.
  The proof is the control on a copied config without the three ignores, which reads `advisories FAILED`
  (`evidence/operator-pass.md`, *The fix push*). That is a database, not a tool version.
- **Harness / gate surface:** the coverage leg's exclusion and its pin (Schema / config). `agent-run.sh`, the
  workflows, `coverage_floor.py` and the pinning tests are unchanged (entry 11). The CI-scripts suite reads
  `Ran 100 tests`, `OK`, unmoved: step 7 adds an assertion to an existing case (entry 28).
- **Cross-project / external claims:**
  - inputs I1 · message, the orchestrator, git measurement at the /andromeda-phase take-up, 2026-10-17T12:21Z · copy
    message · n/a — upstream/main measured at `1e5479ae`, 14 commits past the merge base `434e0661`; cited in scope,
    research and plan.
  - inputs I2 · message, the operator at phase P4, 2026-10-17 14:15Z, relaying the founder on fork 2 · copy message ·
    n/a — the pin `23b9b26a9`, the founder's «Утвердить оба (Рекомендую)», and the coverage exclusion; cited in research
    and plan.
  - inputs#I3 · message, the operator, the /andromeda-wrap-session arguments, 2026-10-17 18:10Z · copy message · n/a —
    this wrap's directive: the one residuals line before the new-text listing, then P1 alone. Snapped at this wrap and
    cited here.
  - `inputs.py verify`: `inputs: 3 entries — unchanged 0 · drifted 0 · vanished 0 · broken 0 · altered 0 · unreachable
    0 · n/a 3 · uncited 1 · unparsed 0`. It was fired before this report existed; I3 is cited here. No drift.
  - P5 review card: no inputs entry names one. The phase's cards, `.andromeda/runs/2026-10-17T12-18-00-phase/p5-card-1.md`
    and `p5-card-2.md`, each read `## Beyond the plan` `none`.
  - DioxusLabs/blitz at `23b9b26a9` (the merge's second parent): every reading of upstream's files above is at that
    commit.
  - CI runs on `Turbolet85/escher` (`evidence/ci.md`, `evidence/operator-pass.md`):
    - CI#39043080454 on `984fabd4` (the merge before the reword, tree `45fcd733`): `verdict: green · checks 17/17`;
    - CI#39055555592 on `3eda07bc` (same tree): `verdict: red` at the Dependency audit, `advisories FAILED, licenses
      ok` (the three `error[unmaintained]`);
    - CI#39056637407 on `ae8c8daf`: `verdict: green · checks 17/17 · wall 875 s`. This is the final HEAD's run, the
      record;
    - the chunk start CI#39017887462 on `143b6013`: the tallies' known-positive control.
  - The RustSec advisory database: the three advisories were new between CI#39043080454 and CI#39055555592, over the
    tree `45fcd733` in both.
- **Reverted / negative API facts:** upstream's own `aria-selected` (`parse::<bool>`) and unfiltered `aria-label` reads
  were merged in and dropped in the resolution: two reads of one attribute, where the later silently wins
  (`evidence/merge.md`, region 5). Upstream's four Taffy-main commits past `23b9b26a9` were not taken (the pin, P4 fork
  1). Note that `6032d0b9d` "Bump Taffy to main" is inside the pin and is reverted by upstream's own `85340347f`.
- **Insufficient fixes (written, kept, not the remedy):** none.
- **Spec claims disproved by measurement:**
  - `test-plan.md:66` (§2) states "124 `.rs` files, 123 test targets and the unbuilt `all.rs`". The untouched tree
    `143b6013` measured 125 `.rs` files, `all.rs` included, and 124 targets (`evidence/before.md`, `ls
    tests/blitz-tests/tests/*.rs | wc -l` at 14:35Z). So the master was one short before this chunk. Pattern `files`,
    the row @c2359. The merged tree reads 127/126 (entry 24, `evidence/counts.md`).
  - Two plan figures were measured otherwise and are disposed in Deviations: the predicted 188 result lines read 187,
    and research's 9 tests in `accessibility_label.rs` read 10.
- **Expected amendments (from plan):**
  - architecture §Project Intent, the `Upstream sync:` line → `23b9b26a9` and the merge commit — carried: Counts
    (`pin`, arch `:206`, 1 row).
  - architecture §Stack and Technologies: keyboard-types "0.8", test-that "0.6.0", workspace 0.3.0-beta.4; the Release
    tooling (bump) row leaves — carried: Dependencies, Counts (`kbt` arch `:52`; `testthat` arch `:59`; `wsver` arch
    `:45`, `:220`; `bump` arch `:63`; `tomlc` arch `:63`).
  - architecture §Occupied Resources: `bump` leaves the Names row; nothing is added — carried: Crates / modules, Counts
    (`bump` arch `:154`).
  - architecture §Infrastructure Patterns: the tree's `apps/bump/` and the crate table's bump row leave — carried:
    Counts (`bump` arch `:171`, `:262`).
  - architecture §Standard Contracts: Mutation, query and CSSOM gains the six new reads, still one read per attribute;
    the Test harness re-export gains `NamedKey` — carried: Symbols / APIs, Counts (`aria6` arch `:122`; `reexp` arch
    `:101`, `:130`; `kbt` arch `:130`). 0 master rows state four of the six reads, so that text is new.
  - security-plan §Input Validation, the `aria-label` · `<label for>` row gains `aria-labelledby` inside the ratified
    name crossing, plus a row or clause for the AccessKit description field's text to the platform adapter. That is a
    boundary widening by upstream merge, ratified as upstream's behaviour and pinned by `accessibility_description`. Its
    two sources are named for the founder's confirm on the wrap's card: `aria-description` (its own text) and
    `aria-describedby` (the referenced nodes, as `described_by`). Pattern «Boundary widening by upstream merge»
    (`.andromeda/playbook.md:60-62`). The founder's own answer is «Утвердить оба (Рекомендую)» (inputs#I2), so P2 halts
    on it once, as the answer card. `aria-expanded` · `aria-disabled` · `aria-level` carry closed values and fall under
    «A field inside a ratified carrier» — carried: Symbols / APIs, Coverage of new surfaces (`aria6` security-plan
    `:125`, 1 row; 0 rows for the description field, so new text).
  - security-plan §Input Validation: the "CLI arguments (bump)" row leaves, and §Error Handling's bump line with it —
    carried: Counts (`bump` security-plan `:73`, `:330`).
  - security-plan §Dependency Security, the Pinning sync record: taken at `23b9b26a9`, Taffy unchanged on the registry,
    test-that 0.6.0 reviewed, the four bump-only crates leave, "`deny.toml` unchanged" — **superseded in part**. The
    record is carried (`pin` and `wsver` security-plan `:242`), but `deny.toml` did change: three ignores were added on
    the operator's word after the plan, so its ignores sentence moves 5 → 8 (`deny` security-plan `:237`, 1 row).
  - test-plan §2: files and targets 127/126; the inventory gains `accessibility_description` and `accessibility_label` —
    carried: Counts (`files` test-plan `:66`; `newt` test-plan `:26`, `:66`; `exe` test-plan `:66`).
  - test-plan §9: the workspace tally and the per-OS tallies as the merge's CI run reads them — carried: Counts (`tally`
    test-plan `:325`, `:331`; the run is CI#39056637407 on the final HEAD, equal to CI#39043080454's).
  - test-plan §10 Coverage floors: the coverage leg excludes `keyboard_compat.rs` by filename until upstream retires it;
    the floors are unchanged; the dioxus-native-dom reading — carried: Schema / config, Counts (`cov` test-plan `:335`,
    `:343`).
  - a11y-plan §2 Role derivation: upstream's six reads recorded as upstream's, and the three closed-set reads
    unchanged; §5 and §7 citations into `events/keyboard.rs` and `accessibility.rs` re-pointed — carried: Symbols /
    APIs, Counts (`aria6` a11y-plan `:281`; `aria3` a11y-plan `:46`, `:102` no-change). The citations are P2's citation
    sweep.
  - obs-plan §6 Logged events: citations into `blitz-shell/src/convert_events.rs` and other merged files re-pointed;
    §6's sample line keeps its dated `service.version` — carried: Files. The citations are P2's citation sweep. The
    sample line is a dated reading; `seven_guis` still logs `service.version=0.1.0` (entry 43, `evidence/readings.md`).
  - design-system §Surface: cli: bump's Platform line and its "bump prints …" pattern leave — carried: Counts (`bump`
    design-system `:385`, `:396`).
  - layout-templates §Surface: cli: the `bump` line leaves — carried: Counts (`bump` layout-templates `:71`).
  - Beyond the list, arising at the operator pass: security-plan §Dependency Security, the ignores 5 → 8 and their owner
    line — carried: Schema / config (`deny` security-plan `:237`).
  - Beyond the list, arising at the sweep: arch §Existing Scopes, dioxus-native-dom's row gains the `keyboard_compat`
    module (0 `kbt` rows name it).
- **Coverage of new surfaces:**
  - `aria-labelledby` → computed name (page text to `label`, so to the snapshot `name`, the driver's carriers and the
    adapter) → validation: upstream's reads (referenced ids resolved to built nodes, unknown ids ignored, cycle-guarded
    through `write_text_alternative`) · instrumentation n/a (no log site) · PII: page text inside the ratified name
    crossing (the founder, inputs#I2) · tests: `accessibility_label` 10 (integ) · a11y: accname 2B, names ✓ · tokens n/a.
  - `aria-describedby` · `aria-description` → AccessKit description / `described_by`, to the platform adapter only
    (the snapshot reads no description) → validation: upstream's reads (`aria-description` free text, unbounded) ·
    instrumentation n/a · PII: page text to the adapter, ratified as upstream's behaviour (inputs#I2), the founder's
    confirm due on P2's card · tests: `accessibility_description` 3 (integ) · a11y ✓ · tokens n/a.
  - `aria-expanded` · `aria-disabled` · `aria-level` → closed values (a boolean, `"true"`, a number with tag and role
    defaults) → validation: closed parse · instrumentation n/a · PII n/a · tests: upstream's (#1014; in the merged
    `accessibility_label`/`description` files and blitz-dom) · a11y ✓ · tokens n/a. The snapshot's `enabled` reads the
    `disabled` attribute, not AccessKit, so `aria-disabled` moves no snapshot reading (`snapshot.rs:51-52,154`). No stand
    file states any of the six (`command grep` over `examples/seven_guis/src`: 0).
  - vibey-script `attributes` · `getAttributeNames` · `offsetParent` · `clientTop` · `clientLeft` · `window.event` →
    validation n/a (reads of the document into the script) · instrumentation n/a · PII n/a · tests: upstream's
    (`preact.rs`) · a11y n/a · tokens n/a.
  - `keyboard_compat` (crate-private conversion) → validation n/a · instrumentation n/a · PII n/a · tests: ✗ for its 530
    arms (excluded from coverage by filename, P4 fork 3) · a11y n/a · tokens n/a.

## Deviations from intent

- **Six key sites spell the crate `::keyboard_types`** (implement deviation 1): `snapshot.rs`'s test module and
  `stand_keyboard`, `stand_snapshot_state`, `stand_diff`, `stand_settle`, `stand_snapshot_text`. Each module
  glob-imports `dioxus::prelude::*`, which re-exports Dioxus's keyboard-types 0.7, so the bare path is ambiguous
  (E0659) or resolves to 0.7 (E0599/E0308). The leading `::` names the crate's own 0.8 dependency. It is still a
  spelling change only, in listed files.
- **187 result lines, not the predicted 188:** the bump crate's empty `unittests src/main.rs` line left with it (+2 −1,
  +13 passed).
- **`accessibility_label.rs` holds 10 tests**, where research counted 9.
- **The operator pass reworded the pre-CI commit and force-pushed it**, against the plan's "an amend or a force-push —
  never". Why: the first wrap's scope read (2026-10-17T15:18:36Z, this run dir's gate trail) exited 3 on control C1,
  because the merge commit's subject put `merge upstream/main at 23b9b26a9, ` between the scope and `operator pre-CI
  commit`, so the base fell to HEAD. Authority: the founder's word «Переписать текст, force push (Рекомендую)», by
  dialog in the overseer session, relayed once (`evidence/operator-pass.md`, *The reword*). The tree `45fcd733` and both
  parents are unchanged; `--force-with-lease` named `984fabd4`.
- **The operator pass added a fix push outside the plan:** `deny.toml` gained three advisory ignores, after
  `3eda07bc`'s CI read red on advisories new against an unchanged tree (External decay).
- **The scope record** (`gate.py scope`: `scope: clean — changed 36 · listed 35 · recorded 1 (companion 0 · mechanical
  0 · in-intent 0 · widening 1) · absorbed 0 · excluded 106`):
  - widening · `deny.toml` · serves step 46 (the CI entry, `ci.py conclusion`, red at the Dependency audit on
    3eda07bc) · word: "playbook External decay (routine; unmaintained, not vulnerabilities): fix push. Add
    RUSTSEC-2026-0370, -0371, -0372 to deny.toml ignore, per-ID, reason in the existing form" — the operator, 2026-10-17
    18:03Z. Written at this wrap's P1 on the operator's dialog answer «Widening, your word (Recommended)»; the first
    scope read printed `unrecorded deny.toml`.

## Decisions & corrections

- **The founder's word on the merged ARIA exits:** «Утвердить оба (Рекомендую)» (2026-10-17, by question dialog in the
  overseer session, relayed by the overseer; inputs#I2). `aria-labelledby` names are inside the ratified name crossing,
  and `aria-description` to the adapter is ratified as upstream's behaviour. It is recorded as his word, not
  PROVISIONAL; the P2 card confirms the two description sources.
- **The operator's P4 answers** (inputs#I2): pin `23b9b26a9`, so Taffy stays registry 0.15 and the four Taffy-main
  commits wait for the next sync; the coverage leg excludes `keyboard_compat.rs` by filename.
- **The founder's word at the reword:** «Переписать текст, force push (Рекомендую)» (relayed once,
  `evidence/operator-pass.md`).
- **The operator's word under the playbook's External decay pattern** (2026-10-17 18:03Z): "playbook External decay
  (routine; unmaintained, not vulnerabilities): fix push. …" — three per-ID ignores, the audit leg local green, entries
  45–47 re-fired, and the residuals line riding the wrap's P1 line (`evidence/operator-pass.md`, *The fix push*; this
  wrap's directive, inputs#I3). The residuals line is `.andromeda/residuals.md:30`, `2026-10-17-upstream-sync-engine-debts
  · open · dropping the RUSTSEC-2026-0370 · … ignores …` (target: next). It drops when accesskit_unix moves to zbus 6.
- **The operator's answer at this wrap's scope read:** «Widening, your word (Recommended)» for `deny.toml` (question
  dialog, 2026-10-17, this session).
- **Sweep hazard: the pre-CI commit's subject is matched as a fixed string.** `gate.py scope` and `cites.py added` find
  the base by `git log -F --grep 'chore({marker}): operator pre-CI commit'`. A merge commit given a subject with text
  between the scope and those words (`chore({marker}): merge upstream/main at …, the operator pre-CI commit`) is not
  found, and the base falls to HEAD. That broke control C1 at the first wrap (`gate.py scope` exit 3, this run dir's
  `gate-…json` record of 15:18:36Z) and cost a reword and a force-push. Evidence: `evidence/operator-pass.md`, *The
  reword*.
- **External decay twice in one day:** the Dependency audit read red over an unchanged lock and config at two
  consecutive chunks' pushes. At 07:29Z, zip RUSTSEC-2026-0367/0368/0369 (2026-10-16-fast-failing-mutants). Between
  CI#39043080454 and CI#39055555592, zvariant · zbus_names · zvariant_derive RUSTSEC-2026-0370/0371/0372, here. Each
  push re-reads the advisory database, so a re-push of a green tree can turn red.
- **A merged dependency that two versions now share:** the workspace moved to keyboard-types 0.8 while Dioxus 0.7.3's
  prelude re-exports 0.7. A module that glob-imports `dioxus::prelude::*` must spell the workspace crate
  `::keyboard_types` (implement deviation 1).
- **The plan's preservation guard names `deny.toml`** (entry 11's pathspec), so the operator pass's fix push makes that
  entry read red at the final tree. The fault is the plan's letter, not the code (Outcome, the P7 note).
- **The clean-tree guard and the compaction hook's handoff lines:** before re-firing entry 45, the operator pass held
  `.claude/session-handoff.md`'s 4 AUTO-COMPACT lines aside with `git stash push -- .claude/session-handoff.md`, and
  restored them after (`evidence/operator-pass.md`). That routes around entry 45's `git diff --quiet` guard for a file
  a hook writes.
- **Hygiene's first firing was red on implement's own outcome report** (a host temp prefix at `outcome-….md:82`). It was
  respelled `$TMPDIR/andromeda-gate/…` on the operator's word ("Yes: make that one-line edit, fire entry 44 again").
- **Guard refusals:** at implement, bash-guard refused a `cd` into the cargo registry and one into
  `$TMPDIR/andromeda-gate/…`. At this wrap it refused three `cd`s that would move the session cwd (into the skill's
  references dir, the evidence dir and `.andromeda/`). Each was re-issued once, with absolute paths or a subshell.
- **`rerere` is on on this host:** git printed `Recorded preimage` at the merge and `Recorded resolution` at the pre-CI
  commit for both conflicts. No recorded resolution was replayed.

## Outcome

**Acceptance criteria, re-asserted against the diff:**
- **(arch) The merge recorded with two parents, the coupled pins resolved as one set — MET.** `3eda07bc` has the
  parents `143b6013` · `23b9b26a9` (`evidence/operator-pass.md`); entries 1 · 4 · 5 · 8 · 9 are green.
- **(arch) No port, listener, env read, log site or workspace member added — MET.** Upstream's delta adds 0 such lines
  (entry 19), and `members` loses `apps/bump` (entries 6 · 7). The operator pass's `deny.toml` lines are config, not one
  of these.
- **(security) The lock exactly upstream's set with our two `escher-*` entries at beta.4; test-that reviewed; the audit
  reads `advisories ok` and `licenses ok` "with `deny.toml` unchanged" — UNMET as written.**
  - The lock half is met: entry 10 (no output), entry 9 (2), `evidence/lock.md`.
  - The audit with `deny.toml` unchanged read green at implement (entry 30, before the database moved). It then read
    `advisories FAILED` at CI#39055555592.
  - The final tree's audit is green only with `deny.toml:26-28`, the three ignores the operator's word added (a
    recorded widening).
  - The criterion is unlinked to the matrix (the chunk claims 0 capabilities), so it routes to a P2 escalation.
- **(security) `accessibility.rs` holds one `aria-selected` and one builder `aria-label` read, both ours, our label loop
  before upstream's `aria-labelledby` loop, and upstream's six new reads as written — MET** (entries 12–16, 21:
  `accessibility_states` · `accessibility_names` · `accessibility_label` · `accessibility_description` pass).
- **(security) The founder's word on the merged ARIA text exits recorded as his — MET** (plan, research; carried as an
  expected amendment to P2's card).
- **(tests) 127 files and 126 targets, 0 absent, the two new files each run once, 0 `Running tests/all.rs`, no `all`
  executable — MET** (entries 20 · 24–27).
- **(tests) `fast`, `doc`, `audit`, `a11y` and `coverage` pass as asserted; `run stand`, `run all`, `cargo test -p
  seven_guis` pass — MET** at implement (entries 22 · 29–31 · 33–37 · 40 · 41). `fast` re-ran green on `ae8c8daf`
  (entry 45), and the local audit leg read `advisories ok, licenses ok` after the `deny.toml` edit.
- **(tests) Fork CI green with every job settled, its run id recorded, the per-OS tallies re-read and every moved count
  explained by name — MET on the final HEAD `ae8c8daf`** (CI#39056637407, 17/17, `evidence/ci.md`). `3eda07bc` read
  red at the Dependency audit; the fix push answered it.
- **(obs) Upstream's delta adds 0 tracing/log/print/env sites; `host_log` and `mcp_log` read 0 needles under both
  builds; no scrub set moves; `command` 8 fields, `tool_call` 2 — MET** (entries 19 · 37 · 22, `evidence/sink.md`).
- **(a11y) The a11y leg 6 · 43 · 0; `stand_keyboard`, `range_input_interaction`, `stand_act_keys` green with the tables
  unchanged — MET** (entries 31 · 32 · 34).
- **(a11y) No platform accessibility adapter joins the stand graph — MET** (entry 38, exit 1).
- **(design) `stand_contrast` green in both modes; `stand_screenshot_repeat` 800 × 600, byte-identical across
  processes — MET** (entry 34, `evidence/readings.md`).
- **(layouts) `crud-list` (24, 112.796875, 369 × 419.203125) and `task-body` (0, 46, 800 × 554), and the attribute-only
  names, read unchanged in the standing checks — MET** for `crud-list` (`stand_act_screenshot.rs:130` green).
  `task-body` is pinned by no check (`evidence/before.md`, 0 literal hits) and was not re-measured.
- **(cli) The driver's key-name words, schema, wire, usage line and `help` unchanged — MET** (entry 11 over
  `command.rs`/`schema.rs` at implement; `execute.rs` moves spellings only).

**Gates** (implement's block run, trail `.andromeda/runs/2026-10-17T14-34-19-implement/gate-2026-10-17-upstream-sync-engine-debts.json`,
`entries 47 · green 42 · red 0 · recorded 1 · timeout 0 · not-run 4`; the operator pass's firings in the same trail and
`evidence/operator-pass.md`):
- `git merge-base --is-ancestor 23b9b26a9… HEAD || test "$(git rev-parse -q --verify MERGE_HEAD)" = 23b9b26a9…` — green
- `git grep -n -I -E '^(<<<<<<< |>>>>>>> )' …` — green (exit 1, no output)
- `grep -c -E '^    "packages/escher-(telemetry|driver)",$|…' Cargo.toml` — green (last line 7)
- `grep -c -E '^taffy = \{ version = "0\.15",|^version = "0\.3\.0-beta\.4"$|…' Cargo.toml` — green (last line 11)
- `grep -c -E 'DioxusLabs/taffy|git = ' Cargo.toml` — green (exit 1, 0)
- `git ls-files -- apps/bump | wc -l` — green (0)
- `grep -c -F 'apps/bump' Cargo.toml` — green (exit 1, 0)
- `cargo metadata --locked --format-version 1 > /dev/null` — green
- `grep -A1 -E '^name = "escher-(driver|telemetry)"$' Cargo.lock | grep -c …beta\.4…` — green (2)
- `diff <(git show 23b9b26a9…:Cargo.lock | awk …) <(awk … Cargo.lock …)` — green (no output)
- `git diff --quiet 143b6013… -- .github/workflows … deny.toml tests/blitz-tests/Cargo.toml …` (the preservation
  guard) — green at implement. **At the final tree it reads red, on `deny.toml` alone.** A hand read beside the tool at
  this wrap (2026-10-17T18:29Z): `git diff --quiet 143b6013… -- deny.toml` exit 1, and the guard's pathspec listed
  names `deny.toml` only. See the P7 note below.
- The accessibility entries (`labels.push` … 2 · the six new reads · one `aria-selected` · one `aria-label` · the loop
  order) — green ×5
- `grep -c -F 'pub use keyboard_types::{Key, Modifiers, NamedKey}' …` — green (1)
- `grep -c -E 'Modifiers::SUPER\b' packages/escher-driver/src/execute.rs` — green (exit 1, 0)
- `git diff 434e06615… 23b9b26a… -- '*.rs' | … log/print/env/bind sites` — green (exit 1, 0)
- `python3 -m unittest discover -s .github/scripts -v -k BlitzTestsTargets…` — green
- `cargo test -p blitz-tests --locked --test accessibility_states …` — green
- `bash .github/scripts/ci-leg.sh fast` — green (173.43 s)
- `grep -E '^test result:' target/ci-logs/test.log | awk …` — green (187 · 971 · 0 · 12)
- The files/targets loop — green (exit 1, 0 absent); the two upstream files `Running` once each — green; `grep -c -E
  'Running tests/all\.rs'` — green (exit 1, 0)
- `cargo test --workspace --locked --no-run 2>&1 | awk '/Exec…'` — green (137 · all-target 0)
- `grep -E '^Ran [0-9]+ tests|^OK$|^FAILED' target/ci-logs/ci-scripts.log` — `recorded` (`Ran 100 tests`, `OK`)
- `bash .github/scripts/ci-leg.sh doc` — green · `ci-leg.sh audit` — green (`advisories ok, licenses ok` at implement,
  `deny.toml` unchanged then) · `ci-leg.sh a11y` — green, its tally probe green (6 · 43 · 0)
- `bash scripts/agent-run.sh boot` — green · `run stand` — green (168 · 0 · 7) · `ls … stand_*.rs | wc -l` — green
  (40) · `run all` — green (546 · 0 · 12)
- `cargo test -p seven_guis --locked 2>&1` — green
- `cargo tree --locked -p seven_guis -e normal … accesskit` — green (exit 1)
- `grep -c -F -e "--ignore-filename-regex 'keyboard_compat\.r…" .github/scripts/ci-leg.sh` — green (2)
- `bash .github/scripts/ci-leg.sh coverage` — green (`coverage floor: 3 of 3 crates at or above their floors`) ·
  `grep -c -F 'keyboard_compat' target/coverage/lcov.info` — green (exit 1, 0)
- `cargo build -p seven_guis --bin seven_guis_native --locked` — green
- `RUST_LOG=info timeout 10 target/debug/seven_guis_native; t…` — smoke, green (124, the bound). This is the windowed
  boot; the boot path changed with the merged engine and shell.
- `python -X utf8 ~/.claude/skills/andromeda-tools/scripts/gate.py hygiene` — `leg = 'operator'`, fired as written: red
  once (`P1 …outcome-….md:82 ×1 · tmp`), then green after the respell on the operator's word.
- `bash .github/scripts/ci-leg.sh fast && git diff --quiet &&… git push` — `leg = 'operator'`, through `--operator 45`:
  green on `984fabd4` (141.6 s); re-fired green on `ae8c8daf` (94.79 s, `history moved: … 3eda07bc→ae8c8daf`).
- `python -X utf8 ~/.claude/skills/andromeda-tools/scripts/ci.py conclusion --sha HEAD --wait 3600` — `leg =
  'operator'`, fired as written:
  - green on `984fabd4` (CI#39043080454);
  - red on `3eda07bc` (CI#39055555592, Dependency audit — External decay, the fix push its answer);
  - green on `ae8c8daf` (CI#39056637407, 17/17). The final HEAD's run is the record (`evidence/ci.md`).
- `gh run view <id> -R Turbolet85/escher --log | sed -E …` — `leg = 'operator'`, through `--operator 47`: `recorded` on
  CI#39043080454 and again on CI#39056637407, its figures copied into `evidence/ci.md`, every moved count explained by
  name (the two upstream files +13 passed, bump's empty line gone).

**P7 note — entry 11 at the light gate.** Its pathspec names `deny.toml`, which the operator pass's `ae8c8daf` moved
(a recorded widening, on the operator's word). The entry will therefore read red at P7.1 on this chunk's own edit. It
does not fit the `red — not this chunk's` arm (the edit is this chunk's) or a `defer`, so P7.1 owes a treatment on the
operator's word before the flip. The candidates are:
- `--skip 11:{reason}`, naming the widening;
- a re-run over the guard's pathspec without `deny.toml`, recorded beside the tool.

The (security) criterion's P2 escalation is where to ask it.

**Watches:** none folded.

**Outcome basis:**
- The operator pass ran. Its commits from the base: `3eda07bc` (the pre-CI merge commit, reworded from `984fabd4`) and
  `ae8c8daf` (the `deny.toml` fix). The final HEAD's CI run, CI#39056637407, is green and recorded in `evidence/ci.md`.
- Implement run: `2026-10-17T14-34-19-implement` (outcome: green + smoke), read from its `outcome-….md` (this window
  holds none of it). Its deviations and census are carried above.
- After implement: the `deny.toml` commit, the reword, entries 45–47 re-fired, and `evidence/ci.md` and
  `operator-pass.md` (both untracked at this wrap).
- A first wrap was started at `2026-10-17T15:18:19Z` in this run dir. Its only act was the `gate.py scope` call that
  exited 3 (C1). This wrap reuses the run dir; that record stands in the trail.

**Process hygiene:**
- Implement's census: every `cargo` build and test, the windowed `seven_guis_native` (the bound's 124), and the
  `escher-session` hosts and `mcp` children were terminated, with none in `ps -eo pid,comm,args` at 2026-10-17T14:54:51Z.
- Re-measured at this wrap (`ps -eo pid,comm,args`, by command name `cargo` · `escher-session` · `seven_guis_nati` ·
  `rustc` · `cargo-deny`, 2026-10-17T18:29:32Z): 0 rows.
## Addendum — resumed 2026-10-17
- inputs#I4 · message: the operator, the /andromeda-wrap-session arguments, 2026-10-17 19:06Z · copy message · n/a — this wrap's resume directive, five items (a)–(e).
  - bears on **(security) The lock exactly upstream's set …** (Outcome): supersedes «so it routes to a P2 escalation» — the operator's word resolves it: the criterion "audit passes with `deny.toml` unchanged" is UNMET as written, its letter unchanged and nothing reverted; it rests on `scope-record.md` (the `deny.toml` widening, `ae8c8daf`) and on the External decay cause (the RustSec database moved between CI#39043080454 and CI#39055555592 over the tree `45fcd733`).
  - bears on **P7 note — entry 11 at the light gate** (Outcome): supersedes «P7.1 owes a treatment on the operator's word before the flip» — after entry 11 is read red on `deny.toml` alone, its treatment is `--skip 11:"deny.toml is this chunk recorded widening (scope-record.md, ae8c8daf)"`.
  - bears on **Spec claims disproved by measurement**: adds to it — `test-plan.md:66` moves to the merged 127 files / 126 targets.
  - bears on **Expected amendments (from plan)**: adds to it — the security-plan ARIA entry: P2 stops on the founder's card once, and the operator brings it to him.
  - bears on **Decisions & corrections**: adds to it — curation homes the two auto-memory entries (inputs#I5, inputs#I6) into the project tiers.
- inputs#I5 · ~/.claude/projects/-home-turbolet-dev-projects-escher/memory/fork-ownership-founder-vs-operator.md · copy no-repo · unchanged — a P4 fork whose options each change engine behaviour for every document is the founder's (playbook "An engine behaviour change for every document"); the reach of a fix he already ruled, inside his principle, and the consequences of his answers are the operator's.
  - bears on **Decisions & corrections**: adds to it — a curation candidate (directive e).
- inputs#I6 · ~/.claude/projects/-home-turbolet-dev-projects-escher/memory/owed-checks-go-on-the-earliest-owner.md · copy no-repo · unchanged — at a wrap's route-resolve, a check owed on code the chunk changed is pinned on the earliest markerless entry that can take it, not on the catch-all "Quality gates" entry (the operator, 2026-10-10).
  - bears on **Decisions & corrections**: adds to it — a curation candidate (directive e).
- inputs.py verify, re-fired: `inputs: 6 entries — unchanged 2 · drifted 0 · vanished 0 · broken 0 · altered 0 · unreachable 0 · n/a 4 · uncited 3 · unparsed 0`; moved since the report: uncited 1 → 3 — I3 now cited (`report.md:26 · :225 · :363`), and I4 · I5 · I6 are the three, each cited by this addendum.

## Addendum — resumed 2026-10-18
- inputs#I7 · message: the founder, by question dialog in the overseer session, relayed by the overseer, 2026-10-18T01:43Z · copy message · n/a — the founder's answer to the P2 answer card: option 1, his label «Подтвердить все 10 (Рекомендую)»; the ten ARIA proposals lie inside his «Утвердить оба (Рекомендую)» (inputs#I2), both description sources (`aria-description`'s own text, `aria-describedby`'s targets as `described_by`) included.
  - bears on **Expected amendments (from plan)**: supersedes «the founder's confirm of the two description sources is due on P2's card» — he confirmed them.
  - bears on **Coverage of new surfaces**: supersedes «the founder's confirm due on P2's card» — given.
- inputs.py verify, re-fired: `inputs: 7 entries — unchanged 2 · drifted 0 · vanished 0 · broken 0 · altered 0 · unreachable 0 · n/a 5 · uncited 1 · unparsed 0`; moved since the first addendum: uncited 3 → 1 — I4 · I5 · I6 now cited by it, and I7 is the one, cited by this addendum.

## New text, by line
Generated by `cites.py added` (cites v1.4); pasted by `splice.py`. No line of this section is typed or edited.
The diff: 143b6013 (the parent of the oldest pre-CI commit 3eda07bc) → the work tree.
A row is a block this chunk added: `{first}-{last}`, `@{head}` its head line where not the first, «the head line».

### .github/scripts/ci-leg.sh — added 4 line(s) in 2 range(s)
added: 41-42 · 44-45
### .github/scripts/test_ci_workflows.py — added 1 line(s) in 1 range(s)
added: 327
### Cargo.lock — added 29 line(s) in 29 range(s)
added: 884 · 900 · 919 · 987 · 998 · 1014 · 1036 · 1049 · 1060 · 1071 · 1077 · 1093 · 1107 · 1114 · 1122 · 1132 · 2657
       2680 · 3094 · 3105 · 6444 · 7885 · 8142 · 8144 · 8153 · 8155 · 8158 · 8435 · 8457
### Cargo.toml — added 12 line(s) in 4 range(s)
added: 35 · 45-53 · 145 · 198
### apps/bump/Cargo.toml — deleted
### apps/bump/src/main.rs — deleted
### deny.toml — added 3 line(s) in 1 range(s)
added: 26-28
### justfile — added 1 line(s) in 1 range(s)
added: 91
### packages/blitz-dom/src/accessibility.rs — added 166 line(s) in 8 range(s)
added: 1-3 · 15-16 · 41-46 · 61-102 · 118-126 · 190-217 · 243-295 · 362-384
- 1-3 «use crate::{»
  - 118-125 «fn referenced_dom_ids_to_node_ids<'a>(»
  - 244-252 «fn write_text_alternative(&self, out: &mut FlatText) {»
  - 254-264 «fn write_flat_text_content(&self, out: &mut FlatText) {»
- 267-271 @268 «struct FlatText {»
  - 274-286 «fn push(&mut self, text: &str) {»
  - 288-290 «fn break_word(&mut self) {»
  - 292-295 «fn clear(&mut self) {»
- 362-374 «fn default_level_for_tag(name: &str) -> Option<usize> {»
  - 363-373 @365 «match name {»
- 376-383 «fn default_level_for_role(role: Role) -> Option<usize> {»
  - 377-382 «match role {»
### packages/blitz-dom/src/events/keyboard.rs — added 7 line(s) in 4 range(s)
added: 12 · 27 · 111 · 151-154
### packages/blitz-dom/src/layout/inline.rs — added 28 line(s) in 5 range(s)
added: 5 · 411-415 · 423-428 · 436-438 · 450-462
### packages/blitz-dom/src/node/text.rs — added 11 line(s) in 11 range(s)
added: 6 · 252 · 266 · 280 · 288 · 296 · 310 · 324 · 335 · 352 · 361
### packages/blitz-dom/src/util.rs — added 1 line(s) in 1 range(s)
added: 8
### packages/blitz-shell/src/convert_events.rs — added 7 line(s) in 6 range(s)
added: 5 · 57 · 160 · 167 · 174 · 176-177
### packages/blitz-test-harness/src/lib.rs — added 1 line(s) in 1 range(s)
added: 32
### packages/blitz-vibey-script/src/dom/element.rs — added 84 line(s) in 6 range(s)
added: 4 · 12-13 · 151-153 · 180-181 · 421-468 · 1110-1137
- 421-434 @422 «fn read_attrs(ctx: &DomCtx, node_id: NodeId) -> Vec<(String, String)> {»
  - 424-433 «doc.get_node(node_id)»
- 436-444 «fn get_attribute_names(this: &JsValue, _: &[JsValue], context: &mut Context) -> JsResult<JsValue> {»
  - 439-442 «let names: Vec<JsValue> = read_attrs(&ctx, node_id)»
- 446-467 @449 «fn attributes(this: &JsValue, _: &[JsValue], context: &mut Context) -> JsResult<JsValue> {»
  - 452-464 «let items: Vec<JsValue> = read_attrs(&ctx, node_id)»
- 1110-1123 @1112 «fn offset_parent(this: &JsValue, _: &[JsValue], context: &mut Context) -> JsResult<JsValue> {»
  - 1115-1121 «let parent_id = {»
- 1128-1130 «fn client_top(this: &JsValue, _: &[JsValue], context: &mut Context) -> JsResult<JsValue> {»
- 1132-1136 «fn client_left(this: &JsValue, _: &[JsValue], context: &mut Context) -> JsResult<JsValue> {»
  - 1133-1135 «layout_value(this, context, |node| {»
### packages/blitz-vibey-script/src/runtime.rs — added 15 line(s) in 3 range(s)
added: 1750-1758 · 1822-1826 · 1860
### packages/blitz-vibey-script/tests/preact.rs — added 2 line(s) in 2 range(s)
added: 9 · 57
### packages/dioxus-native-dom/src/events.rs — added 10 line(s) in 10 range(s)
added: 27 · 38 · 344 · 350 · 354 · 358 · 399 · 485 · 637 · 661
### packages/dioxus-native-dom/src/keyboard_compat.rs — new file · 564 line(s)
- 5-8 «use keyboard_types::{»
- 10-12 «pub(crate) fn convert_modifiers(modifiers: BlitzModifiers) -> Modifiers {»
- 14-21 «pub(crate) fn convert_location(location: BlitzLocation) -> Location {»
  - 15-20 «match location {»
- 23-28 «pub(crate) fn convert_key(key: &BlitzKey) -> Key {»
  - 24-27 «match key {»
- 30-342 @31 «fn convert_named_key(key: NamedKey) -> Key {»
  - 32-341 «match key {»
- 344-564 @345 «pub(crate) fn convert_code(code: BlitzCode) -> Code {»
  - 346-563 «match code {»
### packages/dioxus-native-dom/src/lib.rs — added 1 line(s) in 1 range(s)
added: 20
### packages/dioxus-native-dom/src/snapshot.rs — added 4 line(s) in 1 range(s)
added: 635-638
### packages/escher-driver/src/execute.rs — added 24 line(s) in 6 range(s)
added: 4 · 462 · 490-494 · 496-501 · 545-549 · 551-556
### tests/blitz-tests/tests/accessibility_description.rs — new file · 101 line(s)
- 8-28 @9 «fn includes_aria_description_as_description() -> TestResult<()> {»
  - 10-13 «let mut document = HtmlDocument::from_html(»
  - 18-27 «verify_that!(»
- 30-67 @31 «fn includes_aria_describedby_targets_as_described_by_in_order() -> TestResult<()> {»
  - 32-39 «let mut document = HtmlDocument::from_html(»
  - 44-51 «let node_id_with = |predicate: fn(&AccessKitNode) -> bool| {»
  - 52-56 «let expected_node_ids = vec![»
  - 57-66 «verify_that!(»
- 69-93 @70 «fn excludes_aria_describedby_targets_which_are_not_in_the_accessibility_tree() -> TestResult<()> {»
  - 71-78 «let mut document = HtmlDocument::from_html(»
  - 83-92 «verify_that!(»
- 95-101 «fn default_document_config() -> DocumentConfig {»
  - 96-100 «DocumentConfig {»
### tests/blitz-tests/tests/accessibility_label.rs — new file · 305 line(s)
- 8-28 @9 «fn includes_aria_label_as_label() -> TestResult<()> {»
  - 10-13 «let mut document = HtmlDocument::from_html(»
  - 18-27 «verify_that!(»
- 30-58 @31 «fn includes_aria_labelledby_as_labelled_by_for_one_node() -> TestResult<()> {»
  - 32-38 «let mut document = HtmlDocument::from_html(»
  - 43-47 «let (label_node_id, _) = tree_update»
  - 48-57 «verify_that!(»
- 60-104 @61 «fn includes_aria_labelledby_as_labelled_by_for_two_nodes() -> TestResult<()> {»
  - 62-69 «let mut document = HtmlDocument::from_html(»
  - 78-85 «expected_node_ids.push(»
  - 86-93 «expected_node_ids.push(»
  - 94-103 «verify_that!(»
- 106-126 @107 «fn ignores_labelledby_element_which_does_not_correspond_to_element_in_the_dom() -> TestResult<()> {»
  - 108-111 «let mut document = HtmlDocument::from_html(»
  - 116-125 «verify_that!(»
- 128-153 @129 «fn uses_text_of_aria_labelledby_targets_as_label() -> TestResult<()> {»
  - 130-138 «let mut document = HtmlDocument::from_html(»
  - 143-152 «verify_that!(»
- 155-184 @156 «fn aria_labelledby_takes_precedence_over_aria_label_and_content() -> TestResult<()> {»
  - 157-163 «let mut document = HtmlDocument::from_html(»
  - 168-172 «let (span_node_id, _) = tree_update»
  - 173-183 «verify_that!(»
- 186-220 @187 «fn includes_self_reference_and_nested_text_in_aria_labelledby() -> TestResult<()> {»
  - 188-194 «let mut document = HtmlDocument::from_html(»
  - 199-203 «let (button_node_id, _) = tree_update»
  - 204-208 «let (span_node_id, _) = tree_update»
  - 209-219 «verify_that!(»
- 222-245 @223 «fn uses_aria_label_of_aria_labelledby_target() -> TestResult<()> {»
  - 224-230 «let mut document = HtmlDocument::from_html(»
  - 235-244 «verify_that!(»
- 247-272 @248 «fn uses_hidden_aria_labelledby_targets_for_label_without_referencing_them() -> TestResult<()> {»
  - 249-256 «let mut document = HtmlDocument::from_html(»
  - 261-271 «verify_that!(»
- 274-297 @275 «fn falls_back_to_aria_label_when_aria_labelledby_targets_have_no_text() -> TestResult<()> {»
  - 276-282 «let mut document = HtmlDocument::from_html(»
  - 287-296 «verify_that!(»
- 299-305 «fn default_document_config() -> DocumentConfig {»
  - 300-304 «DocumentConfig {»
### tests/blitz-tests/tests/all.rs — added 2 line(s) in 2 range(s)
added: 4 · 6
### tests/blitz-tests/tests/range_input_interaction.rs — added 19 line(s) in 10 range(s)
added: 15-17 · 108-114 · 201 · 230 · 245 · 273 · 309-310 · 344 · 356 · 370
- 15-17 «use blitz_test_harness::{»
### tests/blitz-tests/tests/stand_accessibility_ids.rs — added 3 line(s) in 3 range(s)
added: 20 · 376 · 399
### tests/blitz-tests/tests/stand_contrast.rs — added 3 line(s) in 3 range(s)
added: 18 · 449 · 451
### tests/blitz-tests/tests/stand_diff.rs — added 4 line(s) in 4 range(s)
added: 16 · 413 · 425 · 441
### tests/blitz-tests/tests/stand_keyboard.rs — added 37 line(s) in 24 range(s)
added: 27 · 66 · 97 · 109 · 134 · 141 · 215-224 · 259 · 267 · 290 · 375 · 451 · 474 · 492 · 504 · 568 · 596 · 633 · 635
       640 · 673 · 684-688 · 753 · 758
### tests/blitz-tests/tests/stand_session_fresh.rs — added 2 line(s) in 2 range(s)
added: 8 · 97
### tests/blitz-tests/tests/stand_session_state.rs — added 2 line(s) in 2 range(s)
added: 8 · 117
### tests/blitz-tests/tests/stand_settle.rs — added 2 line(s) in 2 range(s)
added: 13 · 116
### tests/blitz-tests/tests/stand_snapshot_state.rs — added 4 line(s) in 4 range(s)
added: 12 · 334 · 336 · 346
### tests/blitz-tests/tests/stand_snapshot_text.rs — added 2 line(s) in 2 range(s)
added: 9 · 216
