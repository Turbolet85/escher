# P5 review — 2026-10-16-handler-splits-and-readme · "Handler splits and README"

Plan: `escher-0.2.0/chunks/2026-10-16-handler-splits-and-readme/plan.md` · research: `…/research.md` · scope: `…/scope.md`
(premise-closed at P3).

## Goal
- **A. Handler splits.**
  - escher's own default-action arms leave blitz-dom's three over-ceiling handlers as named private functions:
    - the range pointerdown fraction and arm (`handle_pointerdown`);
    - the button-arm body (`handle_click`);
    - the range-key arm body (`handle_key_or_input_event`).
  - `parse_float` is split into its steps (sign · digits · exponent).
  - Behaviour is unchanged for every document.
  - Upstream's arms are not touched.
- **B. README.** The current state replaces the 0.1.0 description:
  - 0.1.0 done and verified, with its three recorded facts:
    - 17/17 capabilities verified;
    - the cold-agent run: 43 calls, 0 wrong;
    - the version-end audit: 0 surviving mutants of 1,558.
  - 0.2.0 planned, one line per epoch of its five.
  - «Escher» capitalized in prose, title and headings included.
  - Crate, binary and path names stay lowercase.
  - Source: the founder's word, inputs#I2.

## A finding research made, and how the plan reads it
The same instrument the audit used (`rust-code-analysis-cli` 0.0.25) reproduces the audit's figures at HEAD exactly:
74 · 40 · 56 · 19. At the upstream merge base `434e0661`, upstream's own bodies of the three handlers already measure
69 · 35 · 50. So no split of escher's arms alone takes a parent under 15.

The plan reads the criterion as escher's branches leaving the ceiling:
- every extracted function and every `parse_float` step at or under 15;
- each parent below its HEAD figure;
- `parse_float` out of the over-ceiling set (the three files: 6 names → 5).

The parents keep their upstream figure as a floor, per the audit's own proposal ("leave the upstream-merged entrants
to upstream") and arch's upstream-sync line. This was decided by those artifacts, so it was not asked as a fork.

## Top acceptance criteria
- The complexity entry's last line reads exactly `over-ceiling: handle_click handle_key_or_input_event handle_pointerdown
  handle_pointermove touch_action_pan_axes`, and the three parents read below 74 · 40 · 56. The before and after
  readings go in `evidence/`.
- The signatures are kept. Range writes go only through `apply_range_move`, and Enter only through `activates_on_enter` →
  `dispatch_activation_click`. Every new function is private (the `pub` census reads 0).
- The diff touches only the three blitz-dom sources and `README.md` in the code tree: no test, stand, manifest or
  lockfile edit, and no new file.
- The added-line census reads 0 for `unwrap`, `expect`, panic, print, `tracing`, span and env.
- These pass unchanged, in both layout modes and under both builds: the pins `range_input_interaction`,
  `stand_keyboard`, `focusability_updates`, `stand_accessibility_ids`, `stand_act_range`, `stand_act_keys`,
  `stand_contrast`, `host_log`, `mcp_log`, and blitz-dom's 97 unit tests.
- `ci-leg.sh fast`, `doc` and `a11y` exit 0. CI on the pushed sha reads `verdict: green`, stand-repeat included.
- README:
  - no lowercase «escher» left in its prose;
  - no capital in a crate, binary or path name;
  - the 0.2.0 epochs and the three 0.1.0 facts present;
  - the 0.1.0-era claims gone;
  - every Front-page element kept;
  - no credential word.

## Touchpoints
- `packages/blitz-dom/src/events/pointer.rs`
- `packages/blitz-dom/src/events/keyboard.rs`
- `packages/blitz-dom/src/node/range.rs`
- `README.md`
- new: `escher-0.2.0/chunks/2026-10-16-handler-splits-and-readme/evidence/` (with `complexity.py`, the instrument's reader)

## Test commands (18 entries)
- **Unit:** `cargo test -p blitz-dom --locked`.
- **Integration:**
  - `cargo test -p blitz-tests --locked` over seven pinning files;
  - `cargo test -p seven_guis --locked --test host_log --test mcp_log`;
  - `ci-leg.sh a11y`.
- **Probes:**
  - the complexity entry;
  - the changed-file and untracked-file guards;
  - the added-line and `pub` censuses;
  - four README probes.
- **Lint:** `ci-leg.sh fast` and `ci-leg.sh doc`.
- **Operator:** hygiene → push → `ci.py conclusion`.

No boot smoke, no live leg, and no mutation run (inputs#I1).

**Baselines:**
- Entries 1–3 and 13 read green on the untouched tree.
- Entries 4, 9, 11 and 12 read red on the chunk's own subject:
  - 4: the reader is absent;
  - 9: 8 prose lines;
  - 11: the 0.2.0 epochs are absent;
  - 12: 3 old-era claims.
- Every guard has a must-fail and a must-pass control fired in a scratch clone
  (`.andromeda/runs/2026-10-16T14-07-11-phase/p5-controls.md`).
- Entry 4's two controls are step 1's one-shot pair at implement.

## Concretization preview (verification matrix)
- **v020-01 · Codebase structure** — **not claimed.** Its acceptance has legs in this entry and in "File splits"
  (`escher-0.2.0/working-route.md:13` names v020-01), so this chunk advances it without fully verifying it. Step 4 (after
  the word) writes one dated `notes` line saying so. The cap stays pooled for "File splits".
- No other capability is touched.

## Expected amendments (wrap)
- architecture §Project Intent → Front page: the README's new state.
- The `file:line` citations into `events/{pointer,keyboard}.rs` and `node/range.rs`, re-pointed by the measured line map:
  - architecture §Established Decisions [Scrolling and input], §Standard Contracts → Events;
  - security-plan §Input Validation, §Error Handling;
  - obs-plan §6 / §8;
  - a11y-plan §5.

## Beyond the plan
none — no deferral, no route insertion, and no owner needed for a part left out. The upstream-merged entrants the
audit lists, and upstream's bodies of the three handlers, are outside 0.2.0's intent, which names "the escher-authored
growth". This chunk defers nothing, so no residuals line and no founder fork arise (inputs#I1's deferral rule has
nothing to apply to).

Apply? (yes / review / cancel)
