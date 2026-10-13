# Report — 2026-10-13-stand-contrast-harness

**Chunk:** Stand contrast harness — the text/background pair of every stand control measured, per-pair result recorded
**Date:** 2026-10-13T18:00Z (wrap run `.andromeda/runs/2026-10-13T18-00-36-wrap/`, P1 in its own window)
**Commits:** since `last_wrap` 2026-10-13T16:37:46Z (`git log --since`):
- `198bf85cb` — chore(2026-10-13-upstream-sync-polish-and-ship): the upstream sync. This is the previous wrap's own
  commit.
- `dbc3049fb` — chore(2026-10-13-stand-contrast-harness): operator pre-CI commit, for the run this chunk verdict reads.

## Changes (structured — detectors read this)
- **Files:** the basis is `git diff --name-only 198bf85c` (the parent of the oldest pre-CI commit `dbc3049f`) plus the
  work tree. Run dirs are left out of this list.
  - `packages/blitz-test-harness/src/contrast.rs` — new, 308 lines.
  - `packages/blitz-test-harness/src/lib.rs` — lines 16-17, 21 and 28 added.
  - `packages/blitz-test-harness/Cargo.toml` — line 27 added.
  - `Cargo.lock` — line 1068 added.
  - `tests/blitz-tests/tests/stand_contrast.rs` — new, 277 lines.
  - The chunk folder: `plan.md`, `research.md`, `scope.md`, `inputs/` (I1, I2), and `evidence/pairs.md`,
    `evidence/counts.md`, `evidence/operator-15.txt` and `evidence/ci.md`.
  - The phase's bookkeeping: `.andromeda/master-route.md` (the pending record), `escher-0.1.0/working-route.md` (the
    `[2026-10-13-stand-contrast-harness]` marker on line 88), `.andromeda/friction-log.ndjson` and
    `.claude/session-handoff.md`.
  - The coordinates are from `## New text, by line`.
- **Symbols / APIs:** all new public items of `blitz-test-harness`, re-exported at the root as `pub use
  contrast::{TextContrast, contrast_ratio};` (`lib.rs:28`).
  - `pub fn contrast_ratio(a, b) -> f32` (`contrast.rs:13-23`). It is the WCAG 2.x ratio `(L1 + 0.05) / (L2 + 0.05)`,
    lighter over darker, with `L` from the `color` crate's `OpaqueColor::relative_luminance` after `split()`. The alpha
    is ignored. Its doc names Chrome's `GetContrastRatio`.
    - The body is blitz-paint's `pub(crate) contrast_ratio` (`packages/blitz-paint/src/color.rs:29-35`), restated in
      the harness. blitz-paint is untouched, and its helper keeps its four in-crate callers.
  - `pub struct TextContrast { text, background, ratio, font_size_px, font_weight }` (`contrast.rs:25-41`). It derives
    `Clone, Copy, Debug, PartialEq`. `text` and `background` are `color::AlphaColor<Srgb>`, opaque.
    - `is_large_text()` (`:44-48`): true at `font_size_px >= 24.0`, or at `>= 56/3` (14 pt) together with
      `font_weight >= 700`.
    - `required_ratio()` (`:50-53`): 3.0 for large text, else 4.5.
  - `impl<D: Document> Harness<D> { pub fn text_contrast(&self, node_id) -> Option<TextContrast> }`
    (`contrast.rs:56-91`, head `:70`). It reads and changes nothing: no `doc.mutate()`, no input, pump or settle, and
    every node is reached by `get_node`.
    - **The text-bearing element** is `node_id` itself when it is a text input (its element's `text_input_data()`).
      Otherwise it is the parent element of the first text node, in tree order under `node_id`, that holds a character
      other than whitespace (`:93-119`).
    - **The text** is that element's computed `color` (`clone_color().as_color_color()`), `font-size`
      (`clone_font_size().computed_size().px()`) and `font-weight` (`clone_font_weight().value()`).
    - **The background** (`background_behind`, `:121-147`) walks from that element through its element ancestors. Each
      element's `background-color` is resolved against its own `color`. Translucent layers are gathered until the
      first opaque one; with none before the root, the base is the canvas over opaque white. The canvas
      (`:168-187`) is the root element's background, else `body`'s when the root's reads `Color::TRANSPARENT`. The
      layers are composited source-over (`over`, `:189-198`) and the text is composited over the result.
    - **`None` cases:** no such text; an element on the walk without primary styles; or an element on the walk, up to
      and including the first opaque one, whose `opacity` is below 1, whose `filter` list is not empty, or whose
      `background-image` holds a layer other than `none` (`paints_more_than_colour`, `:149-156`). The image layers are
      read through `BaseDocument::resolved_style_value(node, "background-image")`, split on `,`, because the harness
      takes no `style` crate.
  - The crate-doc gains one bullet naming `text_contrast` (`lib.rs:16-17`).
  - No other symbol changed. `contrast_ratio` and `TextContrast` have their two first callers in
    `stand_contrast.rs`, beside the crate's own unit tests.
- **Crates / modules:** blitz-test-harness gains a private module `contrast`, its sixth (`mod contrast;`, `lib.rs:21`).
  The others are harness · input · inspect · screenshot · settle (`lib.rs@dbc3049f`). No crate is added.
- **Dependencies:** blitz-test-harness takes `color = { workspace = true }` (`Cargo.toml:27`; the workspace declares
  `color = "0.3"`, resolved 0.3.3, already in the graph through blitz-dom).
  - `Cargo.lock` gains one line, `"color",`, inside blitz-test-harness's `dependencies`. No `[[package]]` enters: gate
    entry `git diff 198bf85c… -- Cargo.lock | grep -c '^+\[\[package\]\]'` read 0 at exit 1.
  - The audit reads `advisories ok` with `deny.toml` unchanged.
  - blitz-test-harness takes no `tracing`: gate entry `grep -c -E 'tracing' packages/blitz-test-harness/Cargo.toml`
    read 0.
  - blitz-tests gains no dependency: it reads the colours through `to_rgba8()` and its fields without naming the type.
- **Schema / config:** none.
- **Spec-master edits:** none. No master or registry file changed in this chunk: the site sweep's pre-pass reads them
  equal to HEAD.
- **Counts / qualifiers moved:**
  - **`mod common;` readers: 17 → 18.** Basis: `grep -l 'mod common;' tests/blitz-tests/tests/*.rs | wc -l` reads 18
    (`evidence/counts.md`). Masters stating 17, from the site sweep's `test-totals` pattern:
    - `architecture.md:263` "read through `mod common;` by 17 stand checks";
    - `test-plan.md:66` "read through `mod common;` by 17 `stand_*.r…`".
    - `test-plan.md:100`: the readers listed by name, "… do, 17 in all". `stand_contrast` is not in the list.
    - The other `test-totals` master rows, true claims sharing the token, no change: `architecture.md:115` (how a
      check declares the module) and `test-plan.md:10` (seven_guis' own `tests/common`, "read through `mod common;` by
      all seven" of its checks — another module).
    - Each of these was read at its line in this P1.
  - **`run stand`: 36 files → 37; passed 139 → 141; failed 0; ignored 7** (gate entry `bash scripts/agent-run.sh run
    stand`, `run.end` ts 2026-10-13T17:34:01Z; `evidence/counts.md`). The master stating 139 · 0 · 7 is
    `test-plan.md:330` (`test-totals` pattern).
  - **Workspace test leg: 179 → 180 result lines; 893 → 900 passed; failed 0; ignored 12.** Read from
    `target/ci-logs/test.log` of this run's `fast` leg, written 2026-10-13T17:35:53Z. The +7 are the five `contrast`
    unit tests and the two `stand_contrast` tests. The master stating 179 · 893 is `test-plan.md:330` (`test-totals`
    pattern).
  - **blitz-test-harness unit tests: 17 → 22** (gate entry `cargo test -p blitz-test-harness --locked`, `test result:
    ok. 22 passed`). The phase baseline states 17.
  - **blitz-test-harness's private modules: five → six.** The site sweep found no master row stating the module count
    in the `arch-harness` pattern's wording. arch §Existing Scopes' count is the plan's expected entry; its site is
    the detector's to locate.
- **Dev-tool versions:** none.
- **Harness / gate surface:**
  - A new harness reader, `Harness::text_contrast` (see Symbols).
  - A new stand check target, `stand_contrast`, picked up by `scripts/agent-run.sh run stand` with no script change
    (`run.start` 37 files).
  - No CI step, leg, `agent-run` verb or status shape changed. The check runs in the workspace `test` leg and per
    package, not in the `a11y` leg (the `a11y` leg's three files are untouched).
- **Cross-project / external claims:**
  - **The CI run this chunk's operator pass read:** CI#38440450055, on `dbc3049fb7d7`, conclusion success, verdict
    green, checks 16/16, wall 1178 s (`evidence/ci.md`). It is the witness of the MSRV build and the windows, macos,
    ios and android legs. This wrap's own commit adds to that tree.
  - **inputs.py verify** (P1): `inputs: 2 entries — unchanged 0 · drifted 0 · vanished 0 · broken 0 · altered 0 ·
    unreachable 0 · n/a 2 · uncited 1 · unparsed 0`.
    - I1 · message: the operator, in the /andromeda-phase invocation, 2026-10-13 · copy · n/a (a message has no live
      source). It is cited at `scope.md:70`, `research.md:7`, `plan.md:226`, `plan.md:403` and `plan.md:417`, and by
      this report (inputs#I1). Its fact: the operator relays the founder — no mutation runs (the mutation tier stays
      unstarted); pipeline tool stdout is redirected to a file and read whole, never piped into
      head/tail/grep/cut/sed/awk.
    - I2 · message: the operator, at the P5 review, 2026-10-13 · copy · n/a. `verify` printed it UNCITED before this
      report existed; it is cited here (inputs#I2). Its fact: the operator's word on the P5 card
      `.andromeda/runs/2026-10-13T17-00-57-phase/p5-card-1.md` (authority: the operator), «yes — apply. … The four
      Beyond-the-plan CARRYs to working-route.md:94 are approved with it.» The card is on disk and read whole. Its
      `## Beyond the plan` rows, verbatim:

      | # | Proposal | Owner |
      |---|---|---|
      | 1 | The colour-changing states this chunk does not measure: hover and active on the primary buttons and `back-btn`, the flight booker's toggle in its other position, an invalid date (`#c53030` on `#fff5f5`), and the date and buttons in their non-boot enabled state. Each is SC 1.4.3 text. | "Stand a11y assertions" (working-route.md:94) — a `CARRY:` at this chunk's wrap |
      | 2 | The CRUD list rows (`crud-person-{id}`, role `option`) are outside `controls()` and not measured. A selected row is predicted `#ffffff` on `#4a6cf7` = 4.39, which fails. | "Stand a11y assertions" (working-route.md:94) — a `CARRY:` at this chunk's wrap |
      | 3 | Re-colouring every pair this chunk records as `fail`. SC 1.4.3 cannot be asserted green on the stand until they pass. | "Stand a11y assertions" (working-route.md:94) — a `CARRY:` at this chunk's wrap |
      | 4 | Text outside the controls (task titles, labels, value displays) is SC 1.4.3 text too, and the entry names controls only. | "Stand a11y assertions" (working-route.md:94) — a `CARRY:` at this chunk's wrap |

      Authority: `word: "yes — apply. … The four Beyond-the-plan CARRYs to working-route.md:94 are approved with it."
      — the operator, 2026-10-13 (inputs#I2)`. These four rows are P5's route-resolve input: four `CARRY:` lines on
      "Stand a11y assertions" (working-route.md:94), each on that recorded word.
- **Reverted / negative API facts:** none shipped and removed. A temporary test, `dump_the_reading`, was written into
  `stand_contrast.rs` to take the step-9 reading and removed before the gates ran (`evidence/pairs.md`, its opening).
- **Insufficient fixes (written, kept, not the remedy):** none.
- **Spec claims disproved by measurement:** none. The scope's and the plan's hypothesis was measured and held (see the
  record below). a11y-plan §3's "observed absent" and §6's "NOT YET MEASURED" are discharged by building the harness and
  taking the reading — expected amendments, not disproved claims.
- **The measured record** (the a11y and design detectors' fact; `tests/blitz-tests/tests/stand_contrast.rs:41-129`
  `RECORD`, and `evidence/pairs.md`):
  - **Basis:** 18 rows, one per `controls(task)` row over `LeanTask::ALL`. Booted through `seven_guis::stand::boot`
    (800 × 600 · scale 1 · Light · the bundled DejaVu Sans · offline), at each task's boot state, read from computed
    styles. The record is identical in both layout modes.
  - **Verdicts:** **pass 9** · **fail 5** · **exempt-inactive 3** · **no-text 1**.
  - **pass:** `back-btn` ×4 (`#333333` on `#f0f0f0`, 11.09, 13 px · 400); `flight-return` (`#444444` on `#fafafa`,
    9.33, 14 · 400); `flight-start` (`#1a1a1a` on `#ffffff`, 17.40, 15 · 400); `crud-filter`, `crud-name` and
    `crud-surname` (`#000000` on `#ffffff`, 21.00, 14 · 400).
  - **fail:** all five read `#ffffff` on `#4a6cf7` at 4.39 (unrounded 4.3939724) against 4.5:
    - `counter-increment` (16 px · 500);
    - `flight-one-way`, the active toggle at boot (14 · 600);
    - `flight-book` (15 · 500);
    - `timer-reset` (16 · 500);
    - `crud-create` (14 · 500).
  - **exempt-inactive** (the snapshot's `enabled` is `Some(false)`): `flight-return-date` (`#999999` on `#ececec`,
    2.41, 15 · 400); `crud-update` and `crud-delete` (`#ffffff` on `#b0b8d8`, 1.96, 14 · 500).
  - **no-text:** `timer-duration` (the range input).
  - No text is large (13–16 px, weight 400–600), so every measured threshold is 4.5.
  - **Measured against declared:** no measured colour differs from its stylesheet value. `#333` and `#444` read
    `#333333` and `#444444`. The three CRUD inputs declare no colour, and read the inherited initial black on the UA
    `input { background-color: white }`.
  - **The painted witness:** the pixel at the left edge plus border plus half the left padding, at the box's vertical
    middle, matches the measured background within 1 per channel on all 17 measured rows in both modes. Every such box
    has a left padding of at least 2 px, and no measured control holds focus.
  - **Every ratio sits beside the stand accent `#4a6cf7`:** design-system.md:24 (sweep pattern `design-primary`, 1
    master row) records `#4a6cf7` as the seven_guis accent "on page `#f5f5f5` with text `#1a1a1a`". White text on it is
    not stated there. That fact is for §Color Palette's amendment.
- **Expected amendments (from plan):** each line names its site-sweep pattern (`report-patterns.toml`, fired as
  `cascade.py sweep --pre-pass`, `report-sweep.txt`) and its `standing` master rows. The registries are read with the
  masters.
  - **a11y-plan §3 — the contrast verification harness built; the keyed contract `contrast-verification-harness-setup`
    discharged** — carried: Symbols / APIs, the measured record.
    - Pattern `a11y-harness` (`contrast verification harness|contrast-verification-harness`, case-insensitive),
      standing 3, all a11y-plan's: `a11y-plan.md:77` ("**Contrast verification harness:** contrast checks are observed
      absent — see §6."), `registries/a11y-plan-contracts.toml:9` (the label) and
      `registries/contracts/a11y-plan/bootstrap-phases-derive-for-route-setup-project.md:3` (the key's text). All
      three are stale.
    - Leaf row: `.claude/docs/a11y-summary.md:32` (the label routed to "Stand contrast harness") — the cascade's.
  - **a11y-plan §6 — Color contrast: the stand's 18 rows as measured, counted by verdict, the record's home
    `tests/blitz-tests/tests/stand_contrast.rs` and `evidence/pairs.md`** — carried: the measured record.
    - Pattern `a11y-absent` (`design-token contrast pairs|contrast checks are observed absent`), standing 3, all
      a11y-plan's:
      - `a11y-plan.md:77`, the §3 line above;
      - `a11y-plan.md:264` ("> NOT YET MEASURED — design-token contrast pairs and focus-ring, target-size and
        typography-readability tokens …"). Stale as to contrast pairs only; focus-ring, target size and typography stay
        unmeasured.
      - the key file's line 3, as above.
    - Also read: `a11y-plan.md:220` heads **Color contrast:** — the §6 home.
  - **architecture §Standard Contracts — Test harness: the `contrast` module and its exports; §Existing Scopes,
    blitz-test-harness's module count; the crate's `color` dependency** — carried: Symbols / APIs, Crates / modules,
    Dependencies.
    - Pattern `arch-harness` (`Harness::screenshot`, the harness-surface listing's wording), standing 5:
      - `architecture.md:130` ×2 and `architecture.md:255` — the harness contract and the escher-driver row naming the
        screenshot. These are the surface lists the contrast reader joins.
      - `test-plan.md:47`, `obs-plan.md:70` and `obs-plan.md:106` — those masters' own entries.
    - No master row states the module count in that wording (see Counts).
    - Leaves: `CLAUDE.md:35` and `:110`, `.claude/rules/observability.md:24`, `.claude/docs/obs-summary.md:49`,
      `.claude/docs/services/blitz-paint.md:32`, and `.claude/docs/services/blitz-test-harness.md:15` and `:30` — the
      cascade's.
  - **test-plan §1, §2, §3, §4, §9 — the new target, its two tests, the harness's new unit tests, `mod common;`
    readers, `run stand` and workspace totals as measured** — carried: Counts / qualifiers moved, Harness / gate
    surface.
    - Pattern `test-totals` (`139 · 0 · 7|893 passed|179 result lines|mod common;`), standing 6:
      - stale: `architecture.md:263` (17 readers), `test-plan.md:66` ×4 (17 readers), `test-plan.md:100` (the named
        reader list, "17 in all"), `test-plan.md:330` ×2 (179 · 893);
      - no change: `architecture.md:115` and `test-plan.md:10` (seven_guis' own module).
    - Leaves: `.claude/rules/testing.md:15` and `.claude/docs/tests-summary.md:30` — the cascade's.
  - **obs-plan §3 — `stand_contrast` joins the silent census by name** — carried: the gate entry `cat
    tests/blitz-tests/tests/stand_contrast.rs | grep -c -E 'println!|…|bind\('` read 0 at exit 1. The check installs
    no subscriber, reads no env var, prints nothing and binds no listener.
    - Pattern `obs-census` (`password_painted_masked`, the census's last named member), standing 12. The census
      site is `obs-plan.md:70` ×2. The other 11 rows — `architecture.md:263` ×2, `security-plan.md:126`,
      `test-plan.md:12`, `:18`, `:26` ×3, `:52` ×2, `:66` ×2, `:116`, `:330`, `a11y-plan.md:236` and `:275` — are true
      claims about that check: no change.
  - **obs-plan §4 — the contrast reader opens no span** — carried: the gate entry `sed '/^#\[cfg(test)\]/,$d'
    packages/blitz-test-harness/src/contrast.rs | grep -c -E 'println!|…|tracing::|\.mutate\(|std::env'` read 0 at
    exit 1, and the manifest takes no `tracing`.
    - Pattern `obs-silent` (`print, panic and log nothing`), standing 2: `obs-plan.md:70` and `obs-plan.md:106`, the
      harness screenshot's entries, which the reader joins.
    - Leaf `.claude/docs/obs-summary.md:49` — the cascade's.
  - **design-system §Color Palette — the measured pairs beside the declared stand values** — carried: the measured
    record.
    - Pattern `design-primary` (`4a6cf7`, case-insensitive), standing 1: `design-system.md:24`, the seven_guis accent
      row.
    - Leaves: `.claude/docs/design-summary.md:13` and `.claude/docs/services/seven_guis.md:24` — the cascade's.
  - **security-plan §Dependency Security — `color` taken by blitz-test-harness with no package entering the lock and
    the audit green** — carried: Dependencies.
    - Pattern `sec-lock` (`no package enter(s|ed|ing)`), standing 4: `security-plan.md:244` and `:245` (§Dependency
      Security's reviewed-at-chunk entries, the second blitz-test-harness's), and `architecture.md:57` and `:101`
      (arch's dependency rows).
    - Leaves: `.claude/docs/services/escher-driver.md:11` and `.claude/docs/stack.md:26` — the cascade's.
- **Coverage of new surfaces:**
  - `Harness::text_contrast` (a test-harness reader, not an external surface) → validation n/a (it takes a `NodeId`
    from its caller; a stale one reads `None` through `get_node`) · instrumentation n/a (none by obs-plan §4; census
    0) · PII n/a (it holds colours and font metrics; the check's messages name the mode, task and row only) · tests
    unit (`contrast.rs` tests, 5) + integration (`stand_contrast`, 2, both modes, both builds) · a11y ✓ (this is the
    SC 1.4.3 instrument; it gates nothing — "Stand a11y assertions" owns the gate) · tokens n/a.
  - `contrast_ratio` / `TextContrast` → validation n/a · instrumentation n/a · PII n/a · tests unit (the ratio table,
    5 rows; the threshold table, 5 rows) · a11y ✓ · tokens n/a.

## Deviations from intent
- **Step 4, the background-image check:** read through `BaseDocument::resolved_style_value(node,
  "background-image")`, any layer other than `none` counting, rather than a match on stylo's computed `Image::None`.
  Naming that type needs a `style` dependency in blitz-test-harness, which the plan does not list. Opacity and filter
  are read from computed styles as planned (implement outcome, Deviations).
- **Step 7, the record's shape:** `Pass(pair(text, background, ratio, size, weight))` — tuple variants over one `Pair`
  struct with a `const fn pair` — rather than struct variants repeating the five fields. The fields and their
  statement (8-bit hex, ratio at two decimals, whole px, the weight as a number) are as planned.
- **Step 8, the snapshot check:** "the snapshot lists every control id" is asserted per row inside `measure`, beside
  the `query` resolution, not as a separate pass before the rows.
- **Step 9:** the reading was taken by a temporary `dump_the_reading` test whose panic message carried the rows. It was
  removed before the gates ran (the census entry over `stand_contrast.rs` read 0).
- **Step 5, the unit tests:** two stated tables (ratios, thresholds), each with its row count asserted, plus three
  fixture tests, rather than one table. The rows are as planned.
- scope record: none — `gate.py scope` clean at implement P4 and at this P1: `scope: clean — changed 5 · listed 5 ·
  recorded 0 (companion 0 · mechanical 0 · in-intent 0 · widening 0) · absorbed 0 · excluded 105`, base 198bf85c.

## Decisions & corrections
- The harness reads the background-image layers through blitz-dom's `resolved_style_value` string rather than taking
  `style` as a dependency (`contrast.rs:149-156`). This keeps the manifest change to `color` alone, the one the plan
  listed.
- blitz-tests has no `color` dependency, and a test written to name `color::AlphaColor` would need one. The check reads
  the colours only through method calls and fields (`contrast.text.to_rgba8()` then `.r .g .b`), so
  `tests/blitz-tests/Cargo.toml` — inside the preservation entry's path list — stays untouched. A first draft
  introduced an alias naming the type and was rewritten before any build.
- The `Cargo.lock` line was already written when the first `cargo test -p blitz-test-harness --locked` ran, right
  after the manifest Edit. That run read exit 0 under `--locked` with the lock already moved. What wrote the line was
  not measured (the PostToolUse hook or the editor's language server).
- Gate tool header: `logs $TMPDIR/andromeda-gate/…` is printed while `TMPDIR` is unset in the Bash tool's shell. A read
  through the literal variable resolved to `/andromeda-gate/…` and failed; the logs were under the host temp root. One
  re-read. Logged to evolve (implement fix-loop, `2026-10-13T17:36:44Z-b`).
- Hygiene (operator entry 15) refused the implement outcome file on its first firing: its Decisions line named the
  host temp path literally (`P1 …outcome-2026-10-13-stand-contrast-harness.md:78 ×1 · tmp`). The line was respelled and
  the entry re-fired `hygiene: clean` (`evidence/operator-15.txt`). This is a recurrence of the standing rule that a
  run-dir file respells a host prefix (implement Setup 3; inputs-contract §snap), met while writing about that very
  path.
- The Bash guard refused a `cd` into `.andromeda/` at this P1 ("cd … moves the session cwd for every later call").
  It was re-issued once as a subshell `( cd … && … )` and ran.
- `cascade.py sweep --pre-pass` refused the first patterns file at exit 2: two pattern ids were over 16 characters
  (`a11y-contrast-nym`, `obs-harness-silent`; the §Tool limit is 1–16). Renamed and re-fired at exit 0.
- **Operator directions this session:**
  - After implement: «Run the operator pass and stop before the wrap. Fire 15 (hygiene). Commit as
    `chore(2026-10-13-stand-contrast-harness): operator pre-CI commit, for the run this chunk verdict reads`. Then fire
    16 (fast, clean-tree guard, push) and 17 (CI conclusion), and write evidence/ci.md. Report the sha, the run id and
    the verdict. Don't start the wrap.» — the operator, 2026-10-13, in this session. The pass ran as directed:
    - entry 15 green on its second firing;
    - commit `dbc3049f`;
    - entry 16 green (`history moved: refs/remotes/origin/build/escher-0.1.0 198bf85c→dbc3049f`);
    - entry 17 green (CI#38440450055).
  - At this wrap's invocation: «run P1 only and stop» — the operator, 2026-10-13. This is the two-window form; P2
    resumes in this run dir (Setup 2a).
- Standing word, inputs#I1 (the operator relaying the founder): no mutation run took place. Every pipeline tool was
  run bare or redirected to a file and read whole — no recurrence of the 2026-10-13 Tier-2 learning.
- The P5 card's four CARRYs (inputs#I2) are P5's input at this wrap (see Cross-project / external claims).

## Outcome
- **Acceptance criteria**, each re-asserted against the diff:
  - (a11y) 18 rows, one result each, in `controls(task)` order. `every_stand_control_reads_its_recorded_pair` asserts
    the count (18), the order and the reading row for row → **met** (`stand_contrast.rs:194-225`).
  - (a11y) exempt-inactive exactly when `enabled == Some(false)`. `measure` decides it from the snapshot's
    `state.enabled` alone (`stand_contrast.rs:160-184`). The three exempt rows are the three predicted → **met**.
  - (a11y) The ratio is WCAG 2.x over the `color` crate's luminance: the unit table reads 21.0 both ways, 1.0, 4.48 and
    4.39 within 0.005 → **met** (`contrast.rs:223-241`).
  - (a11y) The record is identical with incremental layout off and on: `records[0] == records[1]` asserted → **met**.
  - (design) Colours come from computed styles; the background resolves by the root-else-body canvas rule
    (`contrast.rs:168-187`); the painted witness holds within 1 per channel → **met**.
  - (design) `evidence/pairs.md` sets each measured colour beside its declared value and names the differences (none)
    → **met**.
  - (layouts) Every pair is measured on the lean task as mounted, at 800 × 600 · scale 1 · Light, keyed by stable id,
    `back-btn` once per task. The preservation entry read exit 0 over `examples` and the rest → **met**.
  - (tests) `stand_contrast` is green per package and under the workspace build → **met**.
  - (tests) `run stand` picks up `stand_contrast` with no script change: 37 files, failed 0 → **met**.
  - (tests) `fast` and `doc` pass; the workspace totals are recorded (180 · 900 · 0 · 12) → **met**.
  - (arch) No engine crate changes; the preservation entry read exit 0 → **met**.
  - (arch) Every new public item carries `///`; the doc leg passed → **met**.
  - (security) No package enters `Cargo.lock` (0); audit `advisories ok` with `deny.toml` unchanged → **met**.
  - (security) The reader and the check censuses both read 0 → **met**.
  - (obs) No span, event or subscriber is added; the `tracing` manifest census reads 0 → **met**.
  - (ci) The operator's CI read on the pushed sha reads `verdict: green` → **met** (CI#38440450055 on `dbc3049fb7d7`).
- **Gates** — the plan's `[[gate]]` entries by `run`. Every implement entry was green on its first firing (run
  `.andromeda/runs/2026-10-13T17-25-26-implement/`: 14 green · 0 red · 3 not run, the operator legs). The operator
  entries were fired after it.
  - `cargo test -p blitz-test-harness --locked` — green · exit 0 · lacks FAILED (22 passed).
  - `cargo test -p blitz-tests --locked --test stand_contrast` — green · exit 0 · lacks `running 0 tests` · lacks
    FAILED.
  - `cargo test --workspace --locked --test stand_contrast` — green, the same atoms.
  - `cargo test -p blitz-tests --locked --test stand_snapshot --test stand_accessibility_ids --test stand_act_ids
    --test stand_session_fresh --test stand_screenshot_repeat --test stand_act_screenshot --test accessibility_states`
    — green, the same atoms.
  - `git diff 198bf85c… -- Cargo.lock | grep -c '^+\[\[package\]\]'` — green · exit 1 · last line 0.
  - `sed '/^#\[cfg(test)\]/,$d' packages/blitz-test-harness/src/contrast.rs | grep -c -E …` — green · exit 1 · last
    line 0.
  - `cat tests/blitz-tests/tests/stand_contrast.rs | grep -c -E …` — green · exit 1 · last line 0.
  - `grep -c -E 'tracing' packages/blitz-test-harness/Cargo.toml` — green · exit 1 · last line 0.
  - `git diff --quiet 198bf85c… -- packages/blitz-paint … deny.toml` — green · exit 0.
  - `bash scripts/agent-run.sh boot` — green · exit 0.
  - `bash scripts/agent-run.sh run stand` — green · exit 0 (141 · 0 · 7 over 37 files).
  - `bash .github/scripts/ci-leg.sh audit` — green · exit 0 (`advisories ok`).
  - `bash .github/scripts/ci-leg.sh fast` — green · exit 0.
  - `bash .github/scripts/ci-leg.sh doc` — green · exit 0.
  - `python -X utf8 ~/.claude/skills/andromeda-tools/scripts/gate.py hygiene` (`leg = 'operator'`):
    - first firing red (`contains 'hygiene: clean'` ✗; one host temp path in the implement outcome file);
    - the line respelled;
    - second firing green, `hygiene: clean` · `history: unmoved` (`evidence/operator-15.txt`).
  - `bash .github/scripts/ci-leg.sh fast && git diff --quiet && git diff --cached --quiet && git push origin
    build/escher-0.1.0` (`leg = 'operator'`) — green · exit 0 · `history moved:
    refs/remotes/origin/build/escher-0.1.0 198bf85c→dbc3049f`, the push it exists to make (`evidence/ci.md`).
  - `python -X utf8 ~/.claude/skills/andromeda-tools/scripts/ci.py conclusion --sha HEAD --wait 1800` (`leg =
    'operator'`) — green · exit 0 · `verdict: green` (CI#38440450055 on `dbc3049fb7d7`, 16/16, 1178 s;
    `evidence/ci.md`).
  - No `defer`, no `--void`, no `--skip`, no live leg, no round. Smoke: skipped — no boot-path / UI-surface change (the
    plan lists no smoke or self-verify entry).
- **Watches:** none folded on this entry.
- **Outcome basis:**
  - The operator pass ran: Setup 4's commit list from `198bf85c` is `dbc3049fb` alone, and the final HEAD's CI run is
    CI#38440450055, recorded in `evidence/ci.md`. HEAD is that commit. The tree since then holds `evidence/ci.md`, this
    wrap's run dir and the implement run's updated `gate-2026-10-13-stand-contrast-harness.json` trail.
  - Implement's P4 report is held whole in this window, and also persisted as
    `.andromeda/runs/2026-10-13T17-25-26-implement/outcome-2026-10-13-stand-contrast-harness.md`. It is the one
    implement run dir for this marker, and its outcome is `green`. Its line 78 was respelled at the operator pass,
    with no fact changed.
  - No plan revision took place between implement and this report.
- **Process hygiene:** implement P4's census found every process its runs started terminated (cargo, rustc, the
  `agent-run` builds and stand binaries, the CI legs). The operator pass's three entries were measured at exit through
  the gate tool's lines. Re-measured here at 2026-10-13T18:04Z with `ps -eo pid,comm,args`: no `cargo`, `rustc`,
  `seven_guis`, `escher-session`, `stand_*` or `code-graph` process. This wrap's background code-graph refresh, started
  at Setup, has finished (`.andromeda/cache/.refresh-done`).
## Addendum — resumed 2026-10-13
- inputs#I3 · inputs/ copy · message · n/a — the operator relaying the overseer at this resume: the working-route.md:94
  CARRY on the windowed witness of the three states spells the founder's ratification «relayed by the overseer», copied
  from sp-amend :385 / arch-amend :534; rule A applied there too if it reads as covering a route line, and listed. Stop
  at an answer card if one is raised; otherwise wrap to the end, push, read CI and stop. No mutation tier, no next skill.
  - bears on **Cross-project / external claims**: adds to it (P5's four CARRYs land on the same line, :94).
  - bears on **Operator directions this session**: adds to it (the resume's word, beside «run P1 only and stop»).
- inputs#I4 · inputs/ copy · message · n/a — the operator's correction of I3: the founder's label is «Ратифицировать»,
  in Cyrillic, as arch-amend :534 holds it; his transliteration in I3 is withdrawn and written nowhere. On that word
  the label was respelled in `relay-3.md` and in I3's copy, so I3 reads `copy altered` — the respell, the one change.
  - bears on **inputs#I3** above: supersedes its label's spelling.
- I3 · n/a → copy altered — DRIFT by the operator's directed respell (inputs#I4), no other byte moved.
- inputs.py verify, re-fired: `inputs: 4 entries — unchanged 0 · drifted 0 · vanished 0 · broken 0 · altered 1 ·
  unreachable 0 · n/a 3 · uncited 1 · unparsed 0`; moved since the report: I2 uncited → cited; I3 and I4 new, cited
  here.

## New text, by line
Generated by `cites.py added` (cites v1.4); pasted by `splice.py`. No line of this section is typed or edited.
The diff: 198bf85c (the parent of the oldest pre-CI commit dbc3049f) → the work tree.
A row is a block this chunk added: `{first}-{last}`, `@{head}` its head line where not the first, «the head line».

### Cargo.lock — added 1 line(s) in 1 range(s)
added: 1068
### packages/blitz-test-harness/Cargo.toml — added 1 line(s) in 1 range(s)
added: 27
### packages/blitz-test-harness/src/contrast.rs — new file · 308 line(s)
- 13-23 @19 «pub fn contrast_ratio(a: Color, b: Color) -> f32 {»
- 25-41 @28 «pub struct TextContrast {»
- 43-54 «impl TextContrast {»
  - 44-48 @46 «pub fn is_large_text(&self) -> bool {»
  - 50-53 @51 «pub fn required_ratio(&self) -> f32 {»
- 56-91 «impl<D: Document> Harness<D> {»
  - 57-90 @70 «pub fn text_contrast(&self, node_id: NodeId) -> Option<TextContrast> {»
- 93-101 «fn text_bearing_element<'a>(doc: &'a BaseDocument, node: &'a Node) -> Option<&'a Node> {»
  - 94-99 «if node»
- 103-119 «fn first_text_parent<'a>(doc: &'a BaseDocument, node: &'a Node) -> Option<&'a Node> {»
  - 104-117 «for &child_id in &node.children {»
- 121-147 «fn background_behind(doc: &BaseDocument, from: &Node) -> Option<Color> {»
  - 125-136 «while let Some(node) = current.filter(|node| node.is_element()) {»
  - 137-140 «let base = match opaque {»
  - 141-146 «Some(»
- 149-156 «fn paints_more_than_colour(doc: &BaseDocument, node: &Node) -> Option<bool> {»
  - 151-153 «if styles.clone_opacity() < 1.0 || !styles.get_effects().filter.0.is_empty() {»
- 158-166 «fn background_colour(node: &Node) -> Option<Color> {»
  - 160-165 «Some(»
- 168-187 @170 «fn canvas(doc: &BaseDocument) -> Color {»
  - 171-173 «let Some(root) = doc.try_root_element() else {»
  - 175-177 «if root_background != Color::TRANSPARENT {»
  - 178-186 «root.children»
- 189-198 @190 «fn over(src: Color, dst: Color) -> Color {»
  - 192-194 «if alpha >= 1.0 {»
- 200-308 @201 «mod tests {»
  - 204-206 «fn hex(rgb: u32) -> Color {»
  - 208-211 «fn rgb8(colour: Color) -> [u8; 3] {»
  - 213-221 «fn reading(font_size_px: f32, font_weight: f32) -> TextContrast {»
  - 223-229 «const RATIOS: [(u32, u32, f32); 5] = [»
  - 231-241 @232 «fn the_ratio_is_wcag_2() {»
  - 243-249 «const THRESHOLDS: [(f32, f32, f32); 5] = [»
  - 251-259 @252 «fn large_text_asks_three_and_other_text_four_and_a_half() {»
  - 261-278 @262 «fn a_button_reads_its_own_text_and_background() {»
  - 280-299 @281 «fn a_translucent_ancestor_is_composited_over_the_page() {»
  - 301-307 @302 «fn an_element_with_no_text_reads_none() {»
### packages/blitz-test-harness/src/lib.rs — added 4 line(s) in 3 range(s)
added: 16-17 · 21 · 28
### tests/blitz-tests/tests/stand_contrast.rs — new file · 277 line(s)
- 17-26 @20 «struct Pair {»
- 28-39 @30 «enum Reading {»
- 41-129 @42 «const RECORD: &[(LeanTask, &str, Reading)] = &[»
  - 43-47 «(»
  - 48-52 «(»
  - 53-57 «(»
  - 58-62 «(»
  - 63-67 «(»
  - 68-72 «(»
  - 73-77 «(»
  - 78-82 «(»
  - 83-87 «(»
  - 89-93 «(»
  - 94-98 «(»
  - 99-103 «(»
  - 104-108 «(»
  - 109-113 «(»
  - 114-118 «(»
  - 119-123 «(»
  - 124-128 «(»
- 131-139 «const fn pair(text: u32, background: u32, ratio: f32, size: u32, weight: f32) -> Pair {»
  - 132-138 «Pair {»
- 141-143 «fn mode(incremental: bool) -> &'static str {»
- 145-147 «fn pack(r: u8, g: u8, b: u8) -> u32 {»
- 149-158 «fn measured(contrast: TextContrast) -> Pair {»
  - 151-157 «Pair {»
- 160-184 @161 «fn measure(harness: &Harness<DioxusDocument>, task: LeanTask, incremental: bool) -> Vec<Reading> {»
  - 163-183 «controls(task)»
- 186-192 @187 «fn control_rows() -> Vec<(LeanTask, &'static str)> {»
  - 188-191 «LeanTask::ALL»
- 194-225 @195 «fn every_stand_control_reads_its_recorded_pair() {»
  - 199-204 «for (index, ((task, id), (stated_task, stated_id, _))) in rows.iter().zip(RECORD).enumerate() {»
  - 207-220 «for incremental in [false, true] {»
  - 221-224 «assert!(»
- 227-277 @228 «fn the_painted_background_is_the_measured_background() {»
  - 229-276 «for incremental in [false, true] {»
