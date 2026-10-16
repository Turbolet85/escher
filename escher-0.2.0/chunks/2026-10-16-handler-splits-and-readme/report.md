# Report — 2026-10-16-handler-splits-and-readme

**Chunk:** Handler splits and README — blitz-dom's range and activation handler arms as named functions, parse_float in steps; the README at the current state, «Escher» capitalized (v020-01)
**Date:** 2026-10-16
**Commits:** since `last_wrap` 2026-10-15T10:20:45Z (`git log --since`):
- `af1a00ad5` chore(2026-10-15-cold-agent-test): the previous chunk's wrap;
- `1a34eb2bb` chore(epoch6): the version-end code audit;
- `d236e9186` chore(route): escher-0.2.0 working route, requirements and verification matrix — this chunk's base;
- `c71f7ed14` chore(2026-10-16-handler-splits-and-readme): operator pre-CI commit, for the run this chunk verdict reads.

## Changes (structured — detectors read this)
- **Files:** the diff basis is `d236e918` (the parent of the operator pre-CI commit `c71f7ed1`) → the work tree.
  - Code tree, 3 files: `packages/blitz-dom/src/events/pointer.rs` · `packages/blitz-dom/src/events/keyboard.rs` ·
    `packages/blitz-dom/src/node/range.rs`.
  - Repository root: `README.md`.
  - Chunk folder: `escher-0.2.0/chunks/2026-10-16-handler-splits-and-readme/{scope,research,plan}.md` (phase's);
    `evidence/{complexity.py, complexity-before.md, complexity-after.md, readme.md, ci.md}` (implement's and the
    operator pass's); `inputs/` (I1–I4).
  - Bookkeeping: `.andromeda/friction-log.ndjson`, `.andromeda/master-route.md`, `.claude/session-handoff.md`,
    `escher-0.2.0/{working-route.md, verification-matrix.json}` (the phase's promotion), and the run dirs
    `.andromeda/runs/2026-10-16T14-07-11-phase/`, `…T14-27-34-implement/` and `…T14-54-43-wrap/`.
  - No test, stand, manifest, lockfile, CI or script file moved. The basis is `git diff --name-only d236e918`, and gate
    entries 5 and 6 read 0.
- **Symbols / APIs:** seven new private functions, none `pub` or `pub(crate)`; the `^\+\s*pub` census reads 0. Their
  ranges are copied from `## New text, by line`:
  - `events/pointer.rs`:
    - `fn range_track_fraction(layout: &taffy::Layout, x: f32) -> f32` (566-577): the range classification arm's
      content-box fraction, clamped to [0, 1], 0 for a zero-width content box.
    - `fn press_range_input(doc, target, button, fraction, dispatch_event)` (579-594): the `ClickTarget::RangeInput`
      arm's body — a main-button pointerdown calls `apply_range_move(target, RangeMove::Fraction(..))`.
    - `fn click_button(doc, node_id, submits, dispatch_event)` (890-907): the body of `handle_click`'s button arm —
      focus through `generate_focus_events` / `set_focus_to`, then `submit_form` on the form owner when `submits`.
  - `events/keyboard.rs`:
    - `fn press_range_input_key(doc, node_id, key: &Key, dispatch_event)` (141-158), beside `activates_on_enter`: the
      range-key arm's body, with its WHATWG range-state citation. The arrows step, `Home` · `End` go to the minimum and
      maximum, every other key returns without moving, and the one write is `apply_range_move`.
  - `node/range.rs`:
    - `fn take_sign` (272-275) — `-` or `+`, used for the number's sign and the exponent's;
    - `fn take_digits` (277-284) — a run of ASCII digits, used for the integer, the fraction and the exponent digits;
    - `fn take_exponent` (286-301) — `e`/`E`, sign, digits, read as no exponent without digits;
    - a `use std::{iter::Peekable, str::Chars};` line (6-7).
  - Changed bodies, every signature kept: `handle_pointerdown`, `handle_click`, `handle_key_or_input_event` (each
    `pub(crate)`), and `parse_float` (private, `fn(&str) -> Option<f64>`, its rustdoc and WHATWG link kept).
  - Remaining callers are unchanged (research's code-graph trace):
    - `handle_click` ← `events/mod.rs:215` and its own label arm;
    - `handle_pointerdown` ← `events/mod.rs:192`;
    - `handle_key_or_input_event` ← `events/mod.rs:218, :232`;
    - `parse_float` ← `RangeRules::new` ×4, `sanitize_attribute` and the unit test.
  - `apply_range_move` stays the one range write path (now called from `press_range_input` and
    `press_range_input_key`). `activates_on_enter` → `dispatch_activation_click` stays the one Enter path, still
    inline in `handle_key_or_input_event`.
  - The range-focus block in `handle_click` is unchanged in place.
  - Behaviour: none changed for any document. The element/type sets and the `disabled` presence reads are where they
    were. Event order and counts are unchanged (the pinning tests pass unchanged — Outcome).
  - Line map, by `git diff -U0 d236e918 -- packages/blitz-dom/src` hunk headers (old → new):
    - `events/pointer.rs`:
      - 1–462 unchanged;
      - 463–474 → 463–465;
      - old 475–517 −9;
      - 518–524 → 509;
      - old 525–580 −15;
      - +30 inserted at 566–595;
      - old 581–810 +15;
      - 811–820 → 826;
      - old 821–883 +6;
      - +19 inserted at 890–908;
      - old 884– +25.
    - `events/keyboard.rs`:
      - 1–107 unchanged;
      - 108–117 → 108;
      - old 118–149 −9;
      - +19 inserted at 141–159;
      - old 150– +10.
    - `node/range.rs`:
      - +2 at 6–7;
      - old 6–240 +2;
      - 241–249 → 243–244;
      - 252–262 → 247–252;
      - 273–287 → 263;
      - old 288–295 −24;
      - +31 at 272–302;
      - old 296– +7.
      - `parse_float` was 234–294 (rustdoc 230–233) and is now 236–270 (rustdoc 232–235).
  - Citations whose cited text moved INTO a new function, not just shifted:
    - `pointer.rs:463-474` (the fraction);
    - `pointer.rs:517-525` (the range arm);
    - the button-arm body inside `pointer.rs:800-821` (the arm is now 815–827, its body in `click_button` 890–907);
    - `keyboard.rs:103-117` (the range arm, now 103–108 with its body at 141–158);
    - `range.rs:230-294` (`parse_float`, now 232–270 plus the steps at 272–301).
- **Crates / modules:** none added, removed or re-shaped. Every change is inside blitz-dom's existing `events::pointer`,
  `events::keyboard` and `node::range` modules.
- **Dependencies:** none (no `Cargo.toml` or `Cargo.lock` change — gate entry 5).
- **Schema / config:** none.
- **Spec-master edits:** none. No master or sidecar changed in this chunk's diff (`git diff --name-only d236e918 --
  .andromeda/*.md` lists only `master-route.md`, the phase's promotion record).
- **Counts / qualifiers moved:** the cognitive-complexity figures of the three over-ceiling handlers and of
  `parse_float`. Instrument: `rust-code-analysis-cli` 0.0.25, `metrics.cognitive.sum` per function space; the
  readings are in `evidence/complexity-before.md` and `complexity-after.md`.
  - `handle_click` 74 → 71; `handle_pointerdown` 40 → 35; `handle_key_or_input_event` 56 → 53. The upstream floors at
    `434e0661` are 69 · 35 · 50.
  - `parse_float` 19 → 10. The three files' over-ceiling set goes from six names to five.
  - No master states any of these figures. The site sweep's `complexity` pattern (`cognitive|over-ceiling|complexity
    ceiling`, ignore case) reads 2 standing hits: `a11y-plan.md:322` and `:334`. Both are "Cognitive Accessibility",
    another subject sharing the token, so no change. The figures live in the audit's
    `.andromeda/runs/2026-10-15T10-44-59-code-audit/proposals.md` §M1, a run record and not a master.
  - Test counts did not move:
    - blitz-dom 97 unit tests;
    - `node/range.rs` 7 inline `#[test]`s (`grep -c '#\[test\]'` reads 7 at `d236e918` and in the tree);
    - the 20-row float-parser table.
    - The statements of them at `test-plan.md:15`, `:153` and `:331` hold unchanged.
- **Dev-tool versions:** none — `rust-code-analysis-cli` re-read at 0.0.25 (dev host, `rust-code-analysis-cli
  --version`, 2026-10-16), the audit's instrument, unchanged.
- **Harness / gate surface:** none in the project. `evidence/complexity.py` is a chunk-evidence reader of
  `rust-code-analysis-cli` JSON for the plan's complexity entry. It lives in the chunk folder, and no CI step, script
  or test reads it.
- **Cross-project / external claims:**
  - CI#38917429198 measured `c71f7ed14d5b` and read `verdict: green · checks 17/17 · wall 897 s`, `Stand checks
    repeated` among them (`evidence/ci.md`, `ci.py conclusion --sha HEAD --wait 1800`, fired as written). This wrap's
    commit adds no source to that tree.
  - `inputs.py verify` (`inputs: 4 entries — unchanged 1 · … · n/a 3 · uncited 2 · unparsed 0`, the two uncited
    entries being I3 and I4, cited by this report):
    - `I1 · copy message · n/a` — the operator's directive at /andromeda-phase (the README part the founder's word;
      pipeline tools bare or read whole; no mutation run without the founder's new word; forks as dialogs; every
      deferral a residuals line naming his label). Cited in scope, research and plan.
    - `I2 · copy no-repo · unchanged` — the founder's README word (2026-10-16), «чтоб эшер там с большой буквы
      писался», with the overseer's reading. Cited in scope, research and plan.
    - inputs#I3 · `copy message · n/a` — the operator's directive for the operator pass: hygiene as written, the
      pre-CI commit message, entries 17 and 18, `evidence/ci.md`, stop before the wrap. Snapped at this wrap,
      `.andromeda/runs/2026-10-16T14-54-43-wrap/relay-1.md`.
    - inputs#I4 · `copy message · n/a` — «run P1 only and stop», the operator's word at this wrap's invocation (the
      two-window form). Snapped at this wrap, `…/relay-2.md`.
  - No `drifted`, `vanished` or `broken` entry.
  - The phase's P5 review card, `.andromeda/runs/2026-10-16T14-07-11-phase/p5-card-1.md`, is named by no input's
    `why`. Read anyway, its `## Beyond the plan` is "none — no deferral, no route insertion, and no owner needed for a
    part left out".
  - The README's three 0.1.0 figures, each re-read at implement:
    - `matrix.py coverage --dir escher-0.1.0`: `verified 17/17`;
    - `escher-0.1.0/chunks/2026-10-15-cold-agent-test/report.md:263-264`: 43 tool calls, 0 wrong;
    - `.andromeda/runs/2026-10-15T10-44-59-code-audit/proposals.md:73-88`: mutants 33+228+239+403+525+130 = 1,558,
      0 missed.
- **Reverted / negative API facts:** none.
- **Insufficient fixes (written, kept, not the remedy):** the audit's §M1 asked for the three handlers "no longer over
  the complexity ceiling". The splits shipped and are correct, yet the three parents stay over 15 at 71 · 35 · 53. The
  remainder is upstream's own body (69 · 35 · 50 at `434e0661`).
  - research measured this and the plan carried it as its criterion: each parent below its HEAD figure, upstream's
    floor kept. It was not a surprise at implement.
  - Owner of the remainder: upstream, per the audit's proposal ("leave the upstream-merged entrants to upstream") and
    architecture §Project Intent's upstream-sync line. No residual line was minted (P5 card: "no deferral").
- **Spec claims disproved by measurement:** none. Scope's premise was corrected at phase, not here: the audit's "no
  longer over the ceiling" for the parents was measured impossible without reshaping upstream's arms (`scope.md:13`,
  `[premise-corrected: …]`), and plan and acceptance carry the corrected reading.
- **Expected amendments (from plan):** each search is the site sweep `cascade.py sweep --pre-pass --patterns-file
  .andromeda/runs/2026-10-16T14-54-43-wrap/report-patterns.toml`, baseline `c71f7ed1`. Figures are its `standing`
  counts per pattern.
  - **architecture §Project Intent → Front page** — the README now states the current state. **Carried:** Files ·
    `README.md`; the facts are in Changes and in `evidence/readme.md`:
    - the title is `# Escher`, «Escher» in all prose, code spans, URLs and the e-mail unchanged;
    - `## Plans` is replaced by `## Status`: `### 0.1.0 — done and verified` (its six stages kept as listed, past
      tense, then the three facts above) and `### 0.2.0 — planned, in five stages` (one line per epoch of
      `escher-0.2.0/working-route.md`, names verbatim);
    - "being built (see Plans)" is dropped; "What Escher is" states what 0.1.0 built in the present tense;
    - "Beyond 0.1.0: …" is replaced by `Later:` from `escher-0.2.0/intent.md` §Out of this version;
    - kept: Why it exists · Built on Blitz · Try it · the CI badge (the one image) · License · Author / Contact.
    - Search: pattern `front-page` (`[Ff]ront [Pp]age|README\.md`), 3 standing:
      - `architecture.md:210` — the Front page clause (×5), the site this fact amends ("the 0.1.0 epochs" now sit
        under `### 0.1.0 — done and verified` beside the 0.2.0 epochs);
      - `architecture.md:138` (CLIs, `rdme` resolving a README) — no change, another claim sharing the token;
      - `security-plan.md:330` (Error Handling, `rdme`'s exit) — no change, the same.
  - **architecture §Established Decisions [Scrolling and input] and §Standard Contracts → Events** — the named
    functions and the re-pointed citations. **Carried:** Symbols / APIs (the seven functions and the line map).
    - Search: pattern `pointer-rs`, 30 standing over arch · security · obs · a11y. The arch rows: `:19`, `:87` ×12
      ([Scrolling and input]), `:91`, `:109`, `:111`, `:123` ×2 (Events), `:151`, `:195` ×2, `:197`.
    - Pattern `keyboard-rs`, 18 standing. The arch rows: `:19`, `:87` ×9, `:114`, `:123` ×4, `:151`, `:195`, `:197`
      ×2.
    - Pattern `range-rs`, 6 standing. The arch rows: `:87` ×4 (no line digits), `:126` (Nodes and widgets,
      `range.rs:6-48` → 8–50 by the map).
    - Pattern `handlers`, 5 standing. The arch rows: `:87` ×2, `:123` ×6 — the signatures they name stand.
    - The digits are cites.py's at P2; the moved-into-a-function citations listed in Symbols / APIs need a read.
  - **security-plan §Input Validation (range, keyboard-activation, `disabled` rows) and §Error Handling** — citations
    re-pointed by the line map. **Carried:** Symbols / APIs (the line map).
    - `pointer-rs` security rows: `:87` (link href), `:120` ×5 (`disabled`), `:127` ×4 (range), `:128` (keyboard
      activation — `pointer.rs:800-821`, the arm whose body moved to `click_button`), `:140`, `:192`/`:194` (Data
      Protection), `:370` (Error Handling), `:391`, `:396`.
    - `keyboard-rs` security rows: `:120` ×2, `:127` ×2, `:128` ×5, `:192`, `:343`, `:370`.
    - `range-rs` security rows: `:127` ×4 (`range.rs:230-294` → `parse_float` 232–270 and its steps 272–301).
    - `handlers` security rows: `:127` (`parse_float`), `:128` ×2 (`handle_click`) — the names stand.
    - No added line holds `unwrap`, `expect`, a panic macro, a print, a `tracing` site, a span or an env read (gate
      entry 7 reads 0), so §Error Handling's panic-path list gains nothing.
  - **obs-plan §6 / §8** — the `events/pointer.rs` link-href log-site citations re-pointed where lines moved.
    **Carried:** Symbols / APIs (the line map: the `a` arm's `tracing` sites, old 791–797, shift +15 to 806–812).
    - `pointer-rs` obs rows: `:13` (§1), `:193`, `:194` (§6), `:290` ×2 (§8). No log site was added or removed.
  - **a11y-plan §5** — citations re-pointed. **Carried:** Symbols / APIs (the line map).
    - `pointer-rs` a11y rows: `:148` ×2, `:157`, `:158` (button click focus, `pointer.rs:800-821`), `:159`, `:160`,
      `:161` ×4 (range click), `:170`, `:261` ×2.
    - `keyboard-rs` a11y rows: `:145`, `:165` ×2 (range keys, `keyboard.rs:103-117`), `:166` ×5, `:168`, `:169` ×2.
    - `handlers` a11y row: `:166` (`handle_click` — stands).
  - Leaf hits, for P2's cascade: `pointer-rs` 1, `keyboard-rs` 2, `range-rs` 4, `handlers` 7 — CLAUDE.md, the
    `a11y.md` and `security.md` rules, and the docs.
- **Coverage of new surfaces:** no new external surface, hot-path op or UI element. The seven private functions move
  existing arms of existing input paths:
  - `range_track_fraction` / `press_range_input` (pointerdown on a range input) → validation n/a (geometry, clamped) ·
    instrumentation n/a (none before, none added) · PII n/a · tests integ (`range_input_interaction`,
    `stand_act_range`, both layout modes) · a11y kbd/focus ✓ (`stand_keyboard`, a11y leg) · tokens n/a.
  - `click_button` (click on a button) → validation n/a · instrumentation n/a · PII n/a · tests integ
    (`stand_keyboard`, `focusability_updates`, `stand_accessibility_ids`) · a11y focus ✓ · tokens n/a.
  - `press_range_input_key` (range keys) → validation n/a (closed key match) · instrumentation n/a · PII n/a · tests
    integ (`range_input_interaction`, `stand_act_keys`, `stand_act_range`) · a11y kbd ✓ · tokens n/a.
  - `take_sign` / `take_digits` / `take_exponent` (the float parser over attribute text from any document) →
    validation ✓ (the WHATWG grammar, finite filter, `None` on error, unchanged readings) · instrumentation n/a · PII
    n/a · tests unit (the 20-row `the_float_parser_reads_the_html_grammar` and the 7 `node/range.rs` tests) · a11y
    n/a · tokens n/a.
  - `README.md` → validation n/a · instrumentation n/a · PII n/a (no secret, credential word, path outside the
    repository, or cold-agent figure beyond the three; gate entry 12) · tests the four README probes (entries 9–12) ·
    a11y n/a · tokens n/a.

## Deviations from intent
- **The parents stop above their upstream floors.** `handle_click` is at 71 (floor 69) and
  `handle_key_or_input_event` at 53 (floor 50); `handle_pointerdown` reaches 35.
  - The remainder is what the plan keeps in place: the range-focus `if` in `handle_click`'s walk (step 3), and the
    range-key and Enter conditions in the `else if` chain (step 4).
  - The acceptance asks "below 74 · 40 · 56", which holds. The plan predicted "a few points toward" the floor.
- **The range-focus block was not routed through `click_button`'s focus half.** Step 3 made it optional ("only if the
  event order stays the same"). Left in place, the order is trivially identical.
- **`take_sign` reads the exponent's sign as well as the number's.** The same closed set, `-` or `+`, so the readings
  are unchanged (the 20-row table passes unchanged).
- **`node/range.rs` gains one `use std::{iter::Peekable, str::Chars};` line (6–7)** for the step functions'
  signatures. It is in a listed file and changes no visibility.
- **The README keeps the 0.1.0 epochs as a list under `### 0.1.0 — done and verified`**, because architecture §Project
  Intent → Front page names "the 0.1.0 epochs" as a kept element.
  - The 0.2.0 epoch lines word the route's entries plainly: "the nine engine debts 0.1.0 recorded fixed and proven,
    pinned as a limit, or kept" (route: "nine recorded engine deferrals"); "the six mutants that only timed out" (route:
    "the six timeout-only mutants").
  - "Built on Blitz" keeps "Upstream changes are merged in at the end of each stage of work" unchanged, as the plan
    directs (step 6). 0.2.0's route places each sync at the head of the next stage, the same moment.
- **The operator pass was run by the agent in the implementing session**, on the operator's directive (inputs#I3), not
  by the operator by hand:
  - entry 16 fired as written;
  - the pre-CI commit `c71f7ed1` staged the whole tree (77 files), as the previous chunk's pre-CI commit `539defbae`
    did;
  - entry 17 fired through `gate.py run --plan … --operator 17` in the operator pass's form, with no `--run-dir`;
  - entry 18 fired as written.
- **This wrap is split in two windows** on the operator's word «run P1 only and stop» (inputs#I4): P1 ends with this
  report; P2–P7 resume in `.andromeda/runs/2026-10-16T14-54-43-wrap/`.

Scope record: none. `gate.py scope` at this wrap reads `scope: clean — changed 4 · listed 4 · recorded 0 (companion 0 ·
mechanical 0 · in-intent 0 · widening 0) · absorbed 0 · excluded 74`, base `d236e918`.

## Decisions & corrections
- **The operator ran the operator pass through the agent, in-session** (inputs#I3, the operator's words, 2026-10-16):
  «Run the operator pass and stop before the wrap. Fire entry 16 (hygiene) AS WRITTEN, by its own command line. Commit
  as `chore(2026-10-16-handler-splits-and-readme): operator pre-CI commit, …`. Then fire 17 (fast, clean-tree, push)
  and 18 (CI via ci.py), and write evidence/ci.md.»
  - The outcome: `hygiene: clean`; `c71f7ed1`; entry 17 green, `history moved: refs/remotes/origin/build/escher-0.2.0
    d236e918→c71f7ed1`; CI#38917429198 `verdict: green · checks 17/17`.
  - Evidence: `evidence/ci.md`.
- **Entry 17 was fired without `--run-dir`.** The decision was the agent's: the operator-pass row of gate-contract
  §Firing forms names that form ("no run dir (the pass runs under no skill)"). Its effect was that no trail landed in
  the just-committed implement run dir before the clean-tree guard (`git diff --quiet && git diff --cached --quiet`).
  - Measured: the tree read 0 status rows after the commit and after the push.
  - Not measured: whether a trail written with a run dir would have reddened the guard.
- **The two-window wrap** — «run P1 only and stop» (inputs#I4, the operator's word at invocation). This report closes
  P1, and the resume is Setup 2a in `.andromeda/runs/2026-10-16T14-54-43-wrap/`.
- **Sweep hazard — `cognitive` reads two subjects.** In the masters the token stands only in a11y-plan §8 "Cognitive
  Accessibility" (`a11y-plan.md:322`, `:334`), never for the code-complexity metric.
  - A sweep for the audit's complexity figures with `cognitive` reads 2 hits that are a different subject.
  - Pattern and measurement: `complexity` = `cognitive|over-ceiling|complexity ceiling` (ignore case), 2 standing,
    both §8.
- **Instrument with known-verdict controls before any source edit.** The complexity entry fired on the untouched tree
  read red (74 · 40 · 56, `parse_float` in the set). The same reader over `git show 434e0661:` copies read green (69 ·
  35 · 50, the entry's exact last line). Both controls are recorded before the first edit (`evidence/complexity-before.md`),
  and the post-edit reading is in `complexity-after.md`.
  - The decision is the plan's step 1. It is recorded because it took a new reader (`evidence/complexity.py`) from 0
    to a validated instrument in one firing each way.
- **The README's 0.1.0 figures were re-read at their sources before writing**, not copied from the plan:
  - `matrix.py coverage --dir escher-0.1.0` (`verified 17/17`);
  - `report.md:263-264` (43 / 0);
  - `proposals.md:73-88` (the six units summed to 1,558, 0 missed).
- **A probe defect, the agent's own.** A one-line `python3 -c` reading `inputs/andromeda-inputs.json` as a dict raised
  `AttributeError: 'list' object has no attribute 'get'`; the manifest is a JSON list. It was re-read as a list. No
  pipeline tool was involved.
- **Recurrence of standing learnings:** none observed. Pipeline tools were run bare or redirected to a file and read
  whole (`cascade.py sweep` → scratch file, read whole). No bounded-repetition grep was used, and no guard or hook
  refused a call.

## Outcome
Acceptance criteria, re-asserted against the diff:
- (arch) **Met.**
  - Signatures are kept for `handle_click`, `handle_pointerdown`, `handle_key_or_input_event` and `parse_float` (the
    diff changes none of their `fn` lines).
  - Range arms write only through `apply_range_move` (`press_range_input`, `press_range_input_key`).
  - Enter activates only through `activates_on_enter` → `dispatch_activation_click`.
  - Every extracted function is private (entry 8: 0 added `pub` lines).
- (arch) **Met.** Complexity: `parse_float` 10, every step ≤ 2 and every extracted function ≤ 2, against a limit of
  15. The parents are 71 · 35 · 53, below 74 · 40 · 56. The over-ceiling set is exactly the five names (entry 4's
  `last line` atom). Before and after are recorded in `evidence/`.
- (arch) **Met.** No code-tree file is touched beyond the three blitz-dom sources (entry 5 last line 0), and there is
  no untracked code file (entry 6 last line 0).
- (security) **Met.** The added-line census reads 0 (entry 7).
- (tests) **Met.**
  - `cargo test -p blitz-dom --locked`: 97 passed, the float table unchanged.
  - The seven blitz-tests files: 43 passed, 4 · 8 · 8 · 3 · 1 · 5 · 14. No test file was edited.
- (tests) **Met.** `ci-leg.sh fast` and `ci-leg.sh doc` exit 0, at implement and again as entry 17's own fast gate on
  `c71f7ed1`.
- (a11y) **Met.** `ci-leg.sh a11y` exits 0 over six files, 6 · 6 · 4 · 8 · 5 · 14.
- (obs) **Met.** `host_log` 1 and `mcp_log` 1 pass built alone; the workspace-built half passes in the fast leg.
- (design/layouts) **Met.** `stand_contrast` 5 passed, and no `examples/` file moved.
- (README) **Met** (entries 9–12):
  - 0.1.0 is stated done and verified with its three facts, and 0.2.0 planned by its five epochs;
  - 0 lowercase prose «escher»;
  - 0 capital in a crate, binary or path name;
  - the 0.1.0-era claims are gone and every Front-page element is kept.
- (security) **Met.** No credential word (entry 12).
- (tests) **Met.** CI on the pushed sha reads `verdict: green`: CI#38917429198 on `c71f7ed14d5b`, `checks 17/17`, the
  stand-repeat leg (`Stand checks repeated`) among them.
- **Met.** No `verification-matrix.json` capability is claimed (`matrix.py show --chunk`: `claimed by
  2026-10-16-handler-splits-and-readme: 0`). v020-01 stays pooled for "File splits".

Gates (implement's full firing, `.andromeda/runs/2026-10-16T14-27-34-implement/gate-run-1.txt`, then the operator
pass):
- `cargo test -p blitz-dom --locked` — green · exit 0.
- `cargo test -p blitz-tests --locked --test range_input_interaction --test stand_keyboard --test focusability_updates
  --test stand_accessibility_ids --test stand_act_range --test stand_act_keys --test stand_contrast` — green · exit 0.
- `cargo test -p seven_guis --locked --test host_log --test mcp_log` — green · exit 0.
- `d=$(mktemp -d) && rust-code-analysis-cli … && python3 …/evidence/complexity.py "$d"` — green · exit 0 · `last line
  over-ceiling: handle_click handle_key_or_input_event handle_pointerdown handle_pointermove touch_action_pan_axes`.
- `git diff --name-only d236e918… | grep -c -v -x -E '…'` — green · exit 1 · last line 0.
- `git ls-files --others --exclude-standard -- packages examples tests apps wpt | wc -l` — green · exit 0 · last line 0.
- the added-line census `git diff … | grep -c -E '^\+.*(\.unwrap\(|…)'` — green · exit 1 · last line 0.
- the `pub` census `git diff … | grep -c -E '^\+\s*pub(\(| )'` — green · exit 1 · last line 0.
- `grep -c -E '(^|[^-_/a-z`])escher($|[^-_/a-z`])' README.md` — green · exit 1 · last line 0.
- `grep -c -E 'Escher[-_/]|[-_/]Escher' README.md` — green · exit 1 · last line 0.
- the README presence loop (`for s in 'Engine debts' … 'Author / Contact'; do grep -q -F …`) — green · exit 0.
- `grep -c -i -E 'being built|is built in six stages|Beyond 0\.1\.0|api[_-]?key|secret|password|token' README.md` —
  green · exit 1 · last line 0.
- `bash .github/scripts/ci-leg.sh a11y` — green · exit 0.
- `bash .github/scripts/ci-leg.sh fast` — green · exit 0 (158 s).
- `bash .github/scripts/ci-leg.sh doc` — green · exit 0.
- `python -X utf8 ~/.claude/skills/andromeda-tools/scripts/gate.py hygiene` (leg operator) — fired as written:
  `hygiene: clean`, exit 0. Recorded in `evidence/ci.md`.
- `bash .github/scripts/ci-leg.sh fast && git diff --quiet && git diff --cached --quiet && git push origin
  build/escher-0.2.0` (leg operator) — green · exit 0 · `history moved: refs/remotes/origin/build/escher-0.2.0
  d236e918→c71f7ed1`, the intended push. Recorded in `evidence/ci.md`.
- `python -X utf8 ~/.claude/skills/andromeda-tools/scripts/ci.py conclusion --sha HEAD --wait 1800` (leg operator) —
  exit 0 · `verdict: green · checks 17/17 · wall 897 s · runs CI#38917429198 completed/success`. Recorded in
  `evidence/ci.md`.

There is no `defer` entry and no deferral; the Rust delta is 3 files and every Rust gate ran. Smoke: skipped — no
binary, entry point or boot path changed, and the plan lists no `smoke` or `self-verify` entry.

Watches: none folded.

Outcome basis:
- The operator pass ran. Its commit list from `d236e918` is one commit, `c71f7ed14 chore(2026-10-16-handler-splits-and-readme):
  operator pre-CI commit, for the run this chunk verdict reads`. The final HEAD's CI run, CI#38917429198 green on
  `c71f7ed14d5b`, is recorded in `evidence/ci.md`.
- This window holds implement's run whole. Its persisted report,
  `.andromeda/runs/2026-10-16T14-27-34-implement/outcome-2026-10-16-handler-splits-and-readme.md`, agrees: green,
  then the operator pass appended.
- One implement run dir exists for this marker, and no plan revision lies between it and this report.
- The post-implement artifacts are `evidence/ci.md` and the outcome file's operator-pass section. Both were written
  after `c71f7ed1` and ride this wrap's commit.

Process hygiene:
- Implement's census: the cargo and ci-leg children (entries 1–3, 13–15), `rust-code-analysis-cli` with
  `complexity.py`, and the P1 `cargo build` / `cargo fmt --check` all terminated.
- The operator pass started entry 17's fast leg and push and entry 18's `ci.py` poll, both of which exited. This
  wrap's background code-graph refresh (`scripts/code-graph.py refresh`) completed.
- Re-measured here with `ps -eo pid,comm,args` after the refresh, matching `escher-session`, `seven_guis`, `cargo `,
  `rustc`, `rust-code-analysis`, `code-graph` and `blitz_tests`: 0 matches. Nothing is left running.
## Addendum — resumed 2026-10-16
- inputs#I5 · message: the operator, 2026-10-16T14:59:20Z, at this wrap resume invocation · copy · n/a — the operator's directive for the resumed wrap: stop at an answer card if one is raised, otherwise wrap to the end (push, read CI, report) and stop, no next skill.
  - bears on **Deviations from intent**: adds to «This wrap is split in two windows» — the second window runs P2–P7 on this word.
- inputs.py verify, re-fired: inputs: 5 entries — unchanged 1 · drifted 0 · vanished 0 · broken 0 · altered 0 · unreachable 0 · n/a 4 · uncited 1 · unparsed 0; moved since the report: uncited 2 → 1 (I5, cited by this addendum); no verdict moved.
## New text, by line
Generated by `cites.py added` (cites v1.4); pasted by `splice.py`. No line of this section is typed or edited.
The diff: d236e918 (the parent of the oldest pre-CI commit c71f7ed1) → the work tree.
A row is a block this chunk added: `{first}-{last}`, `@{head}` its head line where not the first, «the head line».

### README.md — added 23 line(s) in 11 range(s)
added: 1 · 7 · 9 · 17 · 50 · 52-54 · 61 · 63-73 · 77 · 81 · 85
### packages/blitz-dom/src/events/keyboard.rs — added 20 line(s) in 2 range(s)
added: 108 · 141-159
- 141-158 @144 «fn press_range_input_key(»
  - 150-156 «let to = match key {»
### packages/blitz-dom/src/events/pointer.rs — added 54 line(s) in 5 range(s)
added: 463-465 · 509 · 566-595 · 826 · 890-908
- 566-577 @569 «fn range_track_fraction(layout: &taffy::Layout, x: f32) -> f32 {»
  - 572-576 «if width > 0.0 {»
- 579-594 @580 «fn press_range_input(»
  - 587-593 «if button == MouseEventButton::Main {»
- 890-907 @891 «fn click_button(»
  - 897-903 «generate_focus_events(»
  - 904-906 «if submits && let Some(form_owner) = doc.controls_to_form.get(&node_id) {»
### packages/blitz-dom/src/node/range.rs — added 42 line(s) in 5 range(s)
added: 6-7 · 243-244 · 247-252 · 263 · 272-302
  - 248-252 «let fraction = if chars.next_if_eq(&'.').is_some() {»
- 272-275 @273 «fn take_sign(chars: &mut Peekable<Chars<'_>>) -> Option<char> {»
- 277-284 @278 «fn take_digits(chars: &mut Peekable<Chars<'_>>) -> String {»
  - 280-282 «while let Some(digit) = chars.next_if(char::is_ascii_digit) {»
- 286-301 @288 «fn take_exponent(chars: &mut Peekable<Chars<'_>>) -> String {»
  - 289-291 «if chars.next_if(|c| matches!(c, 'e' | 'E')).is_none() {»
  - 294-296 «if digits.is_empty() {»
