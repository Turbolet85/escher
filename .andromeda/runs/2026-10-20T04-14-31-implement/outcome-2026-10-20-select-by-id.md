# /andromeda-implement — 2026-10-20-select-by-id · outcome

**Outcome: green + operator pass done** (moved from `surfaced` on 2026-10-20). By the operator's word («On my word, an
operator edit of plan.md: add events/mod to gate entry 12's whitelist …» — the operator, 2026-10-20), entry 12's
alternation gained `events/mod` (`evidence/gate-12-amend.md`), and its re-fire (firing 3) reads green. The operator
pass then ran in plan order: 20 `hygiene: clean`; the pre-CI commit `14818317c`; 21 green, `history moved:
refs/remotes/origin/build/escher-0.2.0 2f099735→14818317`; 22 `verdict: green · checks 17/17`, CI#39371468367; 23
recorded, with the per-OS tallies in `evidence/ci.md`, each moved count explained by name. Next: `/andromeda-wrap-session`.

The text below is the report as first written, when the outcome was `surfaced`. The chunk is functionally complete: 18 of the block's 19 non-operator entries are green
(second full firing, trail `.andromeda/runs/2026-10-20T04-14-31-implement/gate-2026-10-20-select-by-id.json`), smoke
skipped with a stated reason, v020-08 `implemented`. One entry, 12, reads red on a defect of the plan's own probe — not
of the tree — surfaced below; nothing was authored and `plan.md` is unchanged.

## Gates (firing 2)
- green: 1 blitz-dom lib · 2 dioxus-native-dom lib · 3 escher-driver (describe count row unmoved) · 4
  `select_interaction` (7 passed) · 5 `stand_act_select` (5 passed) · 6 `accessibility_label` 13 + `accessibility_roles`
  6 · 7 no wall read in blitz-dom · 8 `select.rs` silent / panic-free · 9 the two new checks silent · 10 two
  `info_span!` · 11 no manifest, lock, shared module or stand file · 13 no extra untracked file · 14 `agent-run.sh boot`
  · 15 `run stand` 47 files, passed 190 failed 0 ignored 7 · 16 a11y leg 6 · 6 · 4 · 8 · 5 · 14 = 43 · 17 `ci-leg.sh
  fast` · 18 tally `lines 204 passed 1083 failed 0 ignored 12` · 19 `ci-leg.sh doc`.
- red: **12** (`exit 1` expected, read `exit 0`, count 1) — see Surfaced.
- not run: 20-23, `leg = 'operator'`, the operator pass's.
- Firing 1 read 17 red: clippy `type_complexity` on two test-table types (`node/select.rs` tests,
  `snapshot.rs` tests), fixed by type aliases; firing 2 green. 2 iterations.
- Counts recorded in `escher-0.2.0/chunks/2026-10-20-select-by-id/evidence/counts.md`; step 3's premise measurement in
  `evidence/checked-mechanism.md` (both modes: `fx-pick` 62×24 holding Banana 55.17 wide, 55×24 holding Cherry 48.02
  after `CHECKED` moved).
- No source-delta deferral (Rust changed throughout); walk-class 0 marked.

## Surfaced — the plan's write-set probe omits a companion its own step requires
- Entry 12's alternation lists `packages/blitz-dom/src/events/(keyboard|pointer)\.rs` but not
  `packages/blitz-dom/src/events/mod.rs`. Step 8 adds `DomEventData::Change` and says "every exhaustive match the new
  variant breaks gains its arm (the compiler names them)"; the compiler names two in `events/mod.rs`
  (`map_dom_event_to_ui_event`, `handle_dom_event`), neither with a wildcard arm, so the edit cannot be avoided. The
  probe reads 1 on a correct tree: `git diff --name-only … | grep -c -v -x -E '…'` lists that one file.
- Its P5 must-fail control used `lib.rs`, never the companion the plan anticipates. The file is recorded
  (`scope-record.md`, companion, serves step 8) and `gate.py scope` reads `clean`.
- Resolution is the wrap's or a re-plan's: the entry's alternation gains `events/mod` (or the light gate records the
  red with this basis). Nothing else in the tree is outside the write set.

## Files changed
- New (3): `packages/blitz-dom/src/node/select.rs` · `tests/blitz-tests/tests/select_interaction.rs` ·
  `tests/blitz-tests/tests/stand_act_select.rs`.
- Modified (17): `packages/blitz-dom/assets/default.css` · `packages/blitz-dom/src/{accessibility,document,form,mutator}.rs`
  · `packages/blitz-dom/src/events/{keyboard,pointer,mod}.rs` · `packages/blitz-dom/src/node/{element,mod}.rs` ·
  `packages/blitz-traits/src/events.rs` · `packages/blitz-vibey-script/src/runtime.rs` ·
  `packages/dioxus-native-dom/src/{dioxus_document,snapshot}.rs` · `packages/escher-driver/src/{schema,schema_tests}.rs`
  · `tests/blitz-tests/tests/accessibility_label.rs`.
- Ledger: `escher-0.2.0/verification-matrix.json` — v020-08 `planned → implemented`, ref
  `tests/blitz-tests/tests/stand_act_select.rs::at_boot_the_snapshot_reads_the_select_its_name_value_and_options ·
  ::type_with_a_label_chooses_that_option · ::the_tab_walk_skips_the_disabled_select_before_and_after_the_choices`.
- Chunk evidence: `evidence/checked-mechanism.md`, `evidence/counts.md`; `scope-record.md` (1 line).

## Deviations from plan, with justifications
1. **`events/mod.rs` edited** — scope record: ``- `packages/blitz-dom/src/events/mod.rs` · companion · serves step 8 ·
   self``. Two exhaustive `DomEventData` matches gain a `Change` arm (no UI event; no default action).
2. **Re-seed hook (step 4) — one site for the six tree entry points, plus document entry.** `maybe_record_node`, which
   every listed entry point already calls on the parent, gains a `select` arm; and `process_added_subtree` records a
   `select` entering the document. The second is beyond the plan's list: a Dioxus static template is built once and
   deep-cloned, so its options never pass through an append, and `fx-none` (no option written `selected`) read no
   value until it was added (measured: `select_interaction` firing 0, 6 of 7 red). Both are in `mutator.rs`, a listed
   file.
3. **A `selected` write is exclusive** — it sets that option chosen and clears its select's other options at once
   (HTML: setting an option's selectedness true on a select without `multiple` sets every other option's false), then
   the flush's algorithm runs. With step 1's "two or more → the last" alone, a write on an option before the chosen one
   would lose to it.
4. **Typeahead state as `select::Typeahead`** (select, text, the clock reading) in `select.rs`, held on `BaseDocument`
   as `select_typeahead` (the plan's site); 1 s is `TYPEAHEAD_TIMEOUT`, compared `<` on `Clock::now`.
5. **An option's `label` attribute names its tree node** (accessibility.rs, after `aria-label`), so "named by their
   labels" holds for an option whose label differs from its text; asserted in the snapshot unit test (`Cherries`).
6. **Form submission (step 11) writes every chosen, enabled option** — HTML's step as written — so a `multiple` select
   needs no second path; a single-choice select holds at most one.
7. **`runtime.rs` takes a comment change only.** A real `Change` is already dispatched by name through
   `dispatch_dom_event` (a plain `Event`, `event.rs`'s `_` arm), and the synthesized `change` stays keyed to a
   checkbox or radio target, so one choice never fires two.
8. **`select_interaction`'s fixture app is controlled** — its `oninput` writes `selected` from the event's value.
   Measured: Dioxus re-writes a dynamic `selected` attribute on each re-render, so an app that re-renders without
   updating its state reverts the user's choice — the app keeps control, as the plan says of a range `value` write.
   `stand_act_select`'s page writes `selected` statically and has no handler, so it is unaffected.
9. **Clippy fix** — `type Row` / `type SelectOption` aliases in the two unit-test tables.
10. A click on a select clears the document's text selection (`text_selection.clear()`), the plan's "any text
    selection the click made inside it".

## Step 17 — sweeps and stated limits
- Sweeps: the step-16 Tab walk (`stand_act_select`), `stand_keyboard` (a11y leg, 14 passed) and `accessibility_roles`
  (6 passed) green with their files untouched.
- `click --id fx-apple` (an unchosen option, bounds its select's) is refused `covered` — what the executor reads; and
  `type --id fx-off --text X` `disabled`, both with the snapshot unchanged.
- Limits, for the wrap: a `multiple` select and an `optgroup`'s options are not modelled (a `multiple` select lays out
  as an empty bordered box and takes no key; its options still read in the tree); no popup is painted and an option is
  not clicked; a checkbox, radio or text input fires no engine `change` (vibey-script's synthesis stays); an option
  with no author key reading in `unkeyed_actionable` is the plan's statement, not measured here; a controlled app
  must write `selected` from its `input` event (deviation 8).
- Merge surface (upstreamable, for the wrap's `CARRY:`): `blitz-dom` `node/select.rs`, `document.rs`,
  `events/{keyboard,pointer,mod}.rs`, `mutator.rs`, `accessibility.rs`, `form.rs`, `node/element.rs`,
  `assets/default.css`; `blitz-traits` `events.rs`; `blitz-vibey-script` `runtime.rs`.

## Smoke (P3)
skipped — no boot-path change: no binary or entry point touched, the plan lists no `smoke` or `self-verify` entry, and
the UA `select` / `option` rules have no windowed self-verify target (the stand holds no select); the stand's headless
boot ran as P2 entry 15.

## Process hygiene
| process | started by | final state |
|---|---|---|
| `escher-session`, `cargo`, `seven_guis_native`, `agent-run.sh`, `target/debug/deps/stand_*` | the gate block (entries 14-17) | `census: none running` (`gate.py census`, trail `gate-2026-10-20-select-by-id.json`) |
| throwaway probe targets `zz_checked_probe`, `zz_debug` | this run (step 3's measurement; one debug read) | ran to exit under `cargo test`; source files removed |

## Decisions & corrections
- Guard refusal: the Bash guard refused one `cat` heredoc writing a throwaway debug target ("a new document goes
  through the Write tool"); re-issued once through the Write tool.
- Measured fact: Dioxus re-writes a dynamic `selected` on re-render (deviation 8) — a controlled select's app must
  track its choice.

## Next
Resolve entry 12 (wrap: the alternation gains `events/mod`, or the red is recorded with the basis above), then the
OPERATOR PASS — entries 20 (hygiene), 21 (pre-CI commit, `fast`, push), 22 (CI conclusion) and 23 (per-OS tallies) —
and only then `/andromeda-wrap-session`.
