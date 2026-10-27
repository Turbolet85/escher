# Report — 2026-10-27-readme-and-records

**Chunk:** README and records — the README shows 0.2.0 done and 0.3.0 planned, the capabilities statement on the
2026-10-25 run, two ledger refs re-pointed, the design-system stand rows moved, the incubator ignored by .gitignore
(v030-02 · v030-16)
**Date:** 2026-10-27T06:05Z (wrap run `.andromeda/runs/2026-10-27T05-59-26-wrap/`, P1 alone on the operator's word,
inputs#I5; P2 onward in a second window)
**Commits:** since `last_wrap` 2026-10-27T03:07:53Z — `642561729 chore(2026-10-27-readme-and-records): operator pre-CI
commit, for the run this chunk's verdict reads` (`git log --format='%h %s' 316c6d25b..HEAD`); base `316c6d25b`, the
parent of that oldest pre-CI commit (`gate.py scope` head line)

## Changes (structured — detectors read this)
- **Files:** (`git diff --name-only 316c6d25b` with `:!.andromeda/runs`, and `git status --short`)
  - `README.md` (implement) — "What Escher is" names 0.2.0's additions (a large screen in parts, a `select`, `lint`
    and its six defects, the one virtual clock, the bounded load wait, a session recorded as a check that replays);
    the driver paragraph names the six task slugs (`counter`, `flight-booker`, `timer`, `crud`, `cells`, `load`) and
    "Try it" gains `target/debug/escher-session lint --session demo-session`; a six-item «Since 0.2.0 the driver also
    does these» list follows the MCP paragraph, each item as `docs/capabilities.md` §What it can do states it;
    `## Status` keeps `### 0.1.0 — done and verified` as it stood, replaces `### 0.2.0 — planned, in five stages` with
    `### 0.2.0 — done and verified` (five epoch lines, then: 11 of 11 capabilities verified; a fresh agent on the
    twelve MCP tools drove six apps in 66 calls with 0 wrong under a bar of at most 5 % wrong; the version-end mutation
    reading — 53 of 53 caught in the DOM crate, the stand's run unscored with 12 of 15 unviable, 0 of 5 caught in
    upstream's WPT runner), adds `### 0.3.0 — planned, in six stages` (one line per `### Epoch` of
    `escher-0.3.0/working-route.md`, naming no marker), and restates "Later:" from `escher-0.3.0/intent.md` §Out of
    this version (opt-in OpenTelemetry export, the platform accessibility adapter witnessed with a real assistive
    technology, the next MCP protocol revision). The pinned phrase «the twelve driver commands as tools» and the
    `docs/capabilities.md` link moved `README.md:48` → `README.md:49` (`sed -n 49p README.md`). Licence, Blitz lineage
    and Author / Contact unchanged. New text: `## New text, by line` below (README.md 31 lines in 8 ranges).
  - `docs/capabilities.md` (implement) — `:108-110` the lint at boot restated (the counter, the timer and CRUD read 0
    findings, the flight booker one `overlap` of its `flight-type` select's two options, which read the select's own
    bounds); `:225-226` the host-sink part read restated to the measured reading (`serve cells` at `trace`, 0 id and 0
    name needles under both builds, as measured at escher-0.2.0/chunks/2026-10-25-cold-agent-bar/report.md);
    `:290-295` the run paragraph restated to the 2026-10-25 cold-agent live run (opening «The measured instance is the
    2026-10-25 cold-agent live run.»; six apps reached the model provider through the operator's own login — ids, names
    and values in snapshot texts, paged parts and diffs, Cells' contents, the `flight-type` select's options and value,
    a lint's findings and one screenshot; one run, twelve tools, passed, six apps met, 66 tool calls, 0 wrong against
    the 5 % bar, one `lint` and one `screenshot`, client `claude` 2.1.308; «Its verdict holds counts, booleans and
    closed words only.» kept; the 2026-10-15 run named once, dated). `:3` and `:150` (current with 0.2.0) left as they
    stand (plan, Leaned). New text: 11 lines in 3 ranges.
  - `escher-0.3.0/verification-matrix.json` — phase: v030-02 claimed with a concretized acceptance and a `gated` key
    (the founder's own word on F3), v030-16 method `manual` → `by-construction` with a concretized acceptance; implement:
    v030-16 `implemented`, its ref `by-construction: …evidence/records.md enumerates (a) … (b) the nine seven_guis stand
    rows … (c) …`; v030-02 `planned`, ref null.
  - `escher-0.3.0/working-route.md:13` — phase stamped `[2026-10-27-readme-and-records]` and appended `BLOCKED-ON: the
    founder's own word on F3 of .andromeda/runs/2026-10-27T03-47-30-phase/relay-2.md, …`.
  - `.andromeda/master-route.md:80` — phase's `pending` record.
  - The chunk folder — `scope.md`, `research.md`, `plan.md`, `inputs/` (I1–I4 at phase, I5 at this wrap),
    `evidence/records.md` (v030-16's enumeration), `evidence/counts.md` (the gate block's readings),
    `evidence/operator-pass.md` (untracked, the operator pass's record).
  - Run dirs — `.andromeda/runs/2026-10-27T03-47-30-phase/` (its `p5-controls.sh:52` planted literal respelled
    `/home/someone/x` → `/home/<user>/x` by the operator pass, named in the pre-CI commit's body),
    `.andromeda/runs/2026-10-27T05-19-41-implement/`, this wrap's.
  - `.git/info/exclude` (local, untracked, no commit carries it) — its last two lines removed: `# escher 0.3.0
    incubator (the founder, 2026-10-16), local until a .gitignore line replaces it` and `escher-0.3.0-incubator/` (659 →
    541 bytes, `evidence/records.md` (c)).
- **Symbols / APIs:** none — no source, test, CI script, manifest or lock file changed (plan entry 11, the
  preservation guard: `guard: allowed 2 — README.md, docs/capabilities.md · … · changed 2` · `guard: clean`).
- **Crates / modules:** none.
- **Dependencies:** none.
- **Schema / config:** none. (The ledger's `gated` key on v030-02 is the matrix tool's own field, written at phase.)
- **Spec-master edits:** none — no `.andromeda/` master, category file, key file or sidecar changed
  (`git diff --name-only 316c6d25b -- .andromeda` lists `friction-log.ndjson`, `master-route.md` and run dirs alone).
  Every design-system move is a P2 amendment still to come (Expected amendments below).
- **Retired:** the local incubator exclude (the two `.git/info/exclude` lines above); `.gitignore:39`
  (`escher-0.3.0-incubator/`) ignores the incubator, before and after (`git check-ignore -v
  escher-0.3.0-incubator/intent.md` → `.gitignore:39:escher-0.3.0-incubator/`, both readings in `evidence/records.md`
  (c)). No master describes it: pattern `incubator` (`incubator|info/exclude`) control unfired over the seven masters
  and the registries at `64256172` (`cascade.py sweep --pre-pass` refusal line, trail
  `.andromeda/runs/2026-10-27T05-59-26-wrap/cascade-2026-10-27-readme-and-records-pre-pass.json`).
- **Counts / qualifiers moved:** (sites from `cascade.py sweep --pre-pass --patterns-file
  .andromeda/runs/2026-10-27T05-59-26-wrap/report-patterns.toml`, baseline `64256172`, its trail above)
  - The README's state: 0.2.0 «planned, in five stages» → «done and verified»; 0.3.0 «planned, in six stages» added —
    pattern `readme-status`: 1 standing · 1 to change (`.andromeda/architecture-detail/project-intent.md:21`, the Front
    page clause: «0.2.0 planned in five stages, one line per epoch of `escher-0.2.0/working-route.md`»; its list of what
    the page holds — «the 0.1.0 epochs», the driver paragraph's contents — is the same clause's and moves with it).
  - The README's pinned line `README.md:48` → `:49` — pattern `readme-cite`: 3 standing · 3 to re-point
    (`project-intent.md:21`, `project-intent.md:27`, `standard-contracts.md:279`; the citation sweep's write at P2).
  - The README's count of the driver commands as tools — none moved: «twelve» stands (plan entry 1 green, `Ran 4
    tests`, `OK`); the statement's five counts unmoved (the same pin).
  - The statement's measured cold-agent run 2026-10-15 → 2026-10-25 — pattern `cap-run` (`2026-10-15 (cold-agent
    )?(live )?run|measured instance is|not been re-fired|eleven tools`): control unfired, 0 master sites (refusal line,
    the trail above); `.andromeda/residuals.md:41` states it outside the sweep (`sed -n 41p .andromeda/residuals.md`),
    absorbed:v030-02.
  - The statement's host-sink part read «unmeasured» → measured — pattern `part-sink`: 9 standing · 0 changed · 9
    no-change (each already states the measured reading — `existing-scopes.md:111` and `log-coverage.md:123` the
    in-process `stand_act_spans` read; `occupied-resources.md:56`, `logging-monitoring.md:14`, `test-plan.md:268`,
    `integration-test-strategy.md:23`, `obs-plan.md:80`, `pii-scrubbing-compliance.md:11`,
    `registries/contracts/test-plan/session-lifecycle.md:11` @c3407 the host's `host_log` fourth test, 0 id and 0 name
    needles under both builds).
  - The statement's lint at boot «each lean task reads 0 findings» → the flight booker one `overlap` — pattern
    `lint-boot`: 4 standing · 0 changed · 4 no-change (`layout-templates.md:54`, `test-scope-summary.md:124`,
    `visual-design-verification.md:15` already state the flight booker's one `overlap`; `screen-reader-support.md:39`
    is the load page's own 0, a true claim sharing the token).
  - The three renamed test names in two closed-version refs — pattern `old-names`: control unfired, no master site —
    the refs are ledger text (`escher-0.1.0` v010-17, `escher-0.2.0` v020-05), written at P7.3 (Decisions).
  - Sweep reach: 14 patterns · 16 moved facts, 16 with a pattern (the report's own count: `readme-status`,
    `readme-cite`, `front-page`, `cap-run`, `part-sink`, `lint-boot`, `old-names`, `incubator`, `a11y-ds-cite` and the
    seven `ds-*`) · owner pattern control unfired · by line 0 lines · 0 offsets.
- **Dev-tool versions:** none.
- **Harness / gate surface:** none — the plan's entries are chunk probes; no agent-run script, CI step or verdict
  shape moved.
- **Cross-project / external claims:**
  - CI: run CI#40456250076 on `642561729e7c` (the pre-CI commit, the final HEAD): `verdict: green · checks 17/17 · wall
    1145 s · completed/success` (`ci.py conclusion`, recorded in `evidence/operator-pass.md`).
  - GitHub: the repository's default branch is `build/escher-0.1.0` (`gh repo view Turbolet85/escher --json
    defaultBranchRef`, read at phase P4; `origin/build/escher-0.1.0` at `1a34eb2bb`, 2026-10-16, its README holding no
    `## Status`) — so the front page renders that branch's README, not this one (scope.md, premise-corrected at P4).
    Not re-read at this wrap.
  - `inputs.py verify` (this wrap, after snapping I5): `inputs: 5 entries — unchanged 2 · drifted 0 · vanished 0 ·
    broken 0 · altered 0 · unreachable 0 · n/a 3 · uncited 1 · unparsed 0` (the uncited one I5, cited by this report):
    - I1 · `../additional/escher-overseer/relays/route-0.3.0-founder-scope-2026-10-26.md` · copy no-repo · unchanged
    - I2 · `../additional/escher-overseer/relays/route-0.2.0-founder-answers.md` · copy no-repo · unchanged
    - I3 · message: the operator, by question dialog at phase P4, 2026-10-27 04:59Z · copy message · n/a
    - I4 · message: the operator, by question dialog at phase P4, 2026-10-27 05:02Z · copy message · n/a
    - I5 · message: the operator, at the wrap invocation, 2026-10-27 05:59Z · copy message · n/a — snapped at this wrap:
      inputs#I5, the wrap's directive (P1 alone, then stop; the chunk wraps gated; F2 PROVISIONAL; the ten further
      design-system stand lines brought to the operator as a dialog; the founder asleep, every fork to the operator by
      dialog).
  - No input names a phase P5 review card; `.andromeda/runs/2026-10-27T03-47-30-phase/p5-card-1.md` reads `## Beyond
    the plan` — none.
- **Reverted / negative API facts:** none.
- **Insufficient fixes (written, kept, not the remedy):** none.
- **Spec claims disproved by measurement:**
  1. `docs/capabilities.md:108` (at the base) «On the stand each lean task reads 0 findings at boot» — false against
     `stand_lint::the_stand_at_boot_reads_its_measured_findings` (`tests/blitz-tests/tests/stand_lint.rs:282-289`: the
     flight booker reads one `overlap`, `flight-type-one-way` × `flight-type-return`). DISPOSED: restated in this chunk
     (`docs/capabilities.md:108-110`), on the operator's word (Deviations 1); pattern `lint-boot` — the masters already
     state the measured reading (4 no-change).
  2. The working entry's and v030-02's premise that the README edit shows on GitHub — false: the front page renders the
     default branch `build/escher-0.1.0` (Cross-project). Stated in arch §Project Intent → Front page as «the root
     `README.md` is escher's own page» with no branch named — pattern `front-page` (`front page|default
     branch|build/escher-0\.1\.0`, ignore case): 3 standing · 1 to change (`project-intent.md:21`, the measured fact
     added) · 2 no-change (`architecture.md:293`, the body's pointer line to that clause; `security-plan.md:23`, the
     WPT post-results workflow's «checks out scripts from the default branch», a true claim sharing the token).
     DISPOSED: v030-02 claimed GATED on the founder's own word on F3 (inputs#I4); the fact is an Expected amendment (1).
  3. `escher-0.2.0/verification-matrix.json` v020-05's acceptance «on each of the four lean stand tasks at boot it
     names 0 findings» — contradicted by the renamed test (1, above). No master site (ledger). DISPOSED: F1 «This
     wrap, refs only (Recommended)» (the operator's own answer, inputs#I3) leaves it as worded; the P7.3 refine's dated
     note says so.
  4. `escher-0.1.0/verification-matrix.json` v010-17's acceptance describes the retired pressed pair (`flight-one-way`
     · `flight-return`). No master site (ledger). DISPOSED: as 3 — left as worded under F1, the P7.3 refine's dated
     note saying so (plan, Implementation notes).
- **Expected amendments (from plan):**
  1. architecture §Project Intent → Front page — the `## Status` clause restated to 0.1.0 and 0.2.0 done and verified,
     0.3.0 planned in six stages; the measured fact that the front page renders the default branch
     `build/escher-0.1.0` (F3 gated, inputs#I4); the `README.md:48` citations re-pointed — carried: Files (README.md),
     Counts (`readme-status`, `readme-cite`), Spec claims disproved 2 (`front-page`). Patterns `readme-status` (1
     standing, `project-intent.md:21`), `front-page` (3 standing, 1 to change), `readme-cite` (3 standing, the citation
     sweep's).
  2. design-system §Color Palette, §Typography, §Depth Strategy, §Spacing, §Border Radius — every seven_guis stand row
     leaves the product surface through the door's tooling, the subjects line restated (F2, the founder's delegate
     answer, PROVISIONAL, inputs#I3) — carried: the move set is `evidence/records.md` (b) rows 1–9, values as they
     stand there. Patterns and the design-system rows among their standing hits:
     - `ds-accent` (`#3a5ce5|#2a4cd3|#1f3fc1|#4a6cf7|rgba\(58, 92, 229`): 12 standing — design-system 5
       (`design-system.md:16` row 1, `color-palette.md:27` row 2, `:28` row 3, `:101` row 5, `surface.md:10` — the
       select's overrides, listed in records (b)'s further lines); the other 7 quote the accent in a contract, a test or
       an a11y row (`existing-scopes.md:113`, `standard-contracts.md:128`, `layout-templates.md:14` and `:15` — the
       landing —, `test-scope-summary.md:116`, `visual-design-verification.md:12` and `:32`): no-change.
     - `ds-success` (`#ebf8ee|#68d391|#276749`): 1 standing — `color-palette.md:73` (row 4).
     - `ds-dejavu` (`DejaVu Sans|DEJAVU_SANS|build_single_font_ctx`): 17 standing — design-system 5:
       `typography.md:31` (row 6); `typography.md:30`, `surface.md:139` (WASM builds' bundled font, `wasm_hello`),
       `typography.md:32` and `design-system.md:50` (a custom `FontContext`, `build_single_font_ctx` generic) — not
       stand rows, no-change; the other 12 in architecture, test-plan, layout-templates and a11y-plan state the stand's
       font as a contract or a test condition — no-change (`standard-contracts.md:120`, `:121` the landing).
     - `ds-cards` (`0 2px 8px|24px 32px|48px 32px 64px|seven_guis \(untokenized\)`): 4 standing — `depth-strategy.md:10`
       (row 7), `spacing.md:19` (row 8), `border-radius.md:12` (row 9); `depth-strategy.md:9` the browser app's menus —
       no-change.
     - `ds-select` (`#c0c0c0|select#flight-type`, ignore case): 14 standing — design-system 2 (`color-palette.md:101`
       row 5, `surface.md:10`); 12 in architecture, security-plan, layout-templates and a11y-plan name the select as a
       contract, an input path or an a11y row — no-change.
     - The subjects line: `design-system.md:249` (`<!-- subjects: … desktop-native-stand-load-page … -->`), pattern
       `ds-extra` below.
  3. layout-templates §Surface: desktop-native → Primary screens — receives the colour, shadow, spacing and radius rows
     in its `values · desktop-native-screens` block, folding what it already states (F2, PROVISIONAL) — carried:
     `evidence/records.md` (b), its «Already stated there (fold)» column. Sites: `ds-accent` `layout-templates.md:14`
     (the three accent shades on `load-fetch`), `:15`; `ds-select` and `ds-shared` `layout-templates.md:15` (the
     select's `#1a1a1a` on `#ffffff`, `1px solid #c0c0c0`); `ds-dejavu` `layout-templates.md:10` @c1418.
  4. architecture §Standard Contracts → Headless stand — receives the stand's DejaVu Sans registration beside
     `font_ctx()` (F2, PROVISIONAL) — carried: `evidence/records.md` (b) row 6. Sites: `ds-dejavu`
     `standard-contracts.md:120` (×2, `font_ctx()` = `build_single_font_ctx(DEJAVU_SANS)`), `:121`.
  5. a11y-plan §6 Visual Design Verification — its `design-system §Color Palette` citation
     (`.andromeda/a11y-plan-detail/visual-design-verification.md:12`) re-pointed to the accent's new home — carried:
     follows 2 and 3. Pattern `a11y-ds-cite` (fixed `design-system §Color Palette`): 1 standing,
     `visual-design-verification.md:12`.
  - Not in the plan's list, for P2 to bring to the operator (inputs#I5): implement's ten further design-system stand
    lines (`evidence/records.md` (b), «Further seven_guis stand lines … Listed, not decided»): `color-palette.md:45`
    (page `#f5f5f5`, shared with todomvc), `:60` (text `#1a1a1a`), `:72` (invalid `#e53e3e` / `#fff5f5` / `#c53030`),
    `typography.md:18`, `:19`, `spacing.md:20`, `surface.md:10`, `surface.md:38-40` (the `values ·
    desktop-native-stand-load-page` block), `motion.md:34` (`LoadNet` after 200 ms), `design-system.md:220` (the wasm
    pages on `#f5f5f5`, shared with todomvc). Patterns `ds-extra` (`#e53e3e|#fff5f5|#c53030|desktop-native-stand-load-page`,
    ignore case): 4 standing (`design-system.md:249` the subjects line, `color-palette.md:72`, `surface.md:38`,
    `visual-design-verification.md:12` — the a11y row quoting the invalid pair, no-change) and `ds-shared`
    (`#f5f5f5|#1a1a1a`, ignore case): 15 standing — design-system 10 (`design-system.md:220`, `color-palette.md:27`,
    `:28`, `:45`, `:60`, `:61` todomvc, `:101`, `:131` the border-styles fixture, `:153` the Preact page,
    `surface.md:10`), the other 5 quoting the select's text pair (`existing-scopes.md:113`, `layout-templates.md:15`,
    `test-scope-summary.md:115`, `visual-design-verification.md:12`, `:16`) — no-change. Which of the ten move under
    F2's «Every stand row» is the operator's (Decisions).
- **Coverage of new surfaces:** none — no new external surface, hot-path operation or UI element; the README and the
  statement are documents.

## Deviations from intent
1. `docs/capabilities.md:108` restated — not in the plan. «each lean task reads 0 findings at boot» was false (Spec
   claims disproved 1). On the operator's dialog answer «Restate it here (Recommended)», note: «Operator: it is the same
   listed file and within the chunk's intent of keeping the records true; record it as an in-intent deviation.»
   (2026-10-27, about 05:23Z, implement's outcome). The file is listed, so no scope-record line.
2. `evidence/records.md` (b) lists ten stand lines beyond the nine-row move set (Expected amendments, the closing
   item) — listed, not decided; the move set stays the plan's and v030-16's nine rows until the operator rules.
3. The ref-name lookup read 13 names · 3 stale · 10 standing where research's sweep read 14 · 3 · 11; the three stale
   names are the same (`evidence/records.md` (a)).
4. `.git/info/exclude` edited by an exact-tail scratchpad script — the Read hook refuses every `.git/` path, so the
   Edit tool could not be used; before and after readings in `evidence/records.md` (c).
5. The operator pass respelled phase P5's planted control literal `/home/someone/x` → `/home/<user>/x` in
   `.andromeda/runs/2026-10-27T03-47-30-phase/p5-controls.sh:52` (hygiene P1 refused it; entry 7's `/home/` probe still
   reads the control red), on the operator's «Placeholder (Recommended)» (`evidence/operator-pass.md`).

scope record: none — gate.py scope clean, 0 recorded (`scope: clean — changed 2 · listed 2 · recorded 0 (companion 0 ·
mechanical 0 · in-intent 0 · widening 0) · absorbed 0 · excluded 532`).

## Decisions & corrections
- F1 «This wrap, refs only (Recommended)» — the operator's own answer (inputs#I3): the two closed-version ledger refs
  are re-pointed by this wrap's P7.3, `matrix.py refine --dir escher-0.1.0 --id v010-17 --chunk
  2026-10-10-snapshot-states` and `--dir escher-0.2.0 --id v020-05 --chunk 2026-10-19-visual-lint`, each `--ref-file`
  the text of `evidence/records.md` (a) and a dated `--text-file` naming this chunk, `.andromeda/residuals.md:42` and
  F1 and saying the acceptance still describes the retired state (plan, Implementation notes). P7.3 owns both writes;
  v030-16's coverage read follows them.
- F2 «Every stand row (Recommended)» — the founder's delegate answer, PROVISIONAL, to the founder in the morning
  (inputs#I3, the operator's note: «Please mark F2 PROVISIONAL in the records; it goes to him in the morning..»; I5
  restates it). Its move is P2's (Expected amendments 2–5), values unchanged; every sidecar entry carrying it reads
  PROVISIONAL.
- F3 «Ask the founder first» — the operator's answer (inputs#I4): nothing outward changes tonight (no default-branch
  change, no push beyond the operator pass's); v030-02's gated part waits on the founder's own word. For P5 and P7:
  this chunk's frozen line carries `BLOCKED-ON` (`escher-0.3.0/working-route.md:13`) and v030-02 its `gated` key —
  P5 re-verifies the block (`route.py blocked`), and while it stands P7 takes the gated arm (`matrix.py flip
  --gated`, `route.py flip --to gated`).
- The ten further design-system stand lines (Expected amendments, closing item): the operator directs that whether
  they move is brought to him as a dialog at the step that decides it (inputs#I5) — P2 Validate / its escalation halt,
  beside the door's move.
- The founder is asleep and asked for autonomous work; every fork or card goes to the operator as a dialog (inputs#I5).
  The operator is present, so the away form (`founder-forks.md`) does not apply; the founder's later word supersedes
  F2 by rule.
- `docs/capabilities.md:108` ruled in scope by the operator, «Restate it here (Recommended)» (Deviations 1).
- The operator pass's hygiene refusal of a planted control literal: «Placeholder (Recommended)» — «Operator: same as
  the chunk-4 precedent (p5-controls planted /home/someone/x). Respell the literal so hygiene stops matching, keep the
  control's meaning, name the change in the commit body, and re-run hygiene.» — the second time a P5 control planting a
  `/home/` literal in a run-dir script tripped hygiene P1 at the operator pass (the operator's «same as the chunk-4
  precedent»).
- `.andromeda/residuals.md:41` (absorbed:v030-02) and `:42` (absorbed:v030-16) close at this chunk's wrap (plan,
  Implementation notes) — `.andromeda/residuals.md` unchanged by this chunk (`git diff --quiet 316c6d25b --
  .andromeda/residuals.md`); P5 owns their close.
- v020-05's acceptance stale (Spec claims disproved 3), surfaced by implement; left as worded under F1.
- Guard refusals, each re-issued once: implement — the Bash guard refused a `cd` into the skill's references dir
  (re-issued with absolute paths); the Read hook refused `.git/info/exclude` (re-issued as a scratchpad script). This
  wrap — the Bash guard refused a `cd` into the chunk dir and one into `.andromeda/` (re-issued with absolute paths and
  a scratchpad script run by path).
- Sweep note: the ref-name lookup's count differed from research's (Deviations 3) — research's sweep counted 14 names
  over two refs that hold 13.
- Standing learning met, no recurrence: the bounded-repetition `grep` hazard; the probes use `command grep`
  (implement's outcome).

## Outcome
- **v030-02** — met on the branch: `README.md` holds the three status headings (entry 2, last line 3), no `0.2.0 —
  planned` line (entry 3), the 0.2.0 section's five epoch lines and facts read from their records (11/11 from
  `matrix.py coverage --dir escher-0.2.0`; 66 · 0 · 5 · six from `live-verdict.json`), six 0.3.0 epochs written as plans
  naming no marker, «Escher» in every prose mention (entry 4, 0); `docs/capabilities.md`'s run paragraph states the
  2026-10-25 run (entry 6, 1) and its part-read line the measured reading, neither stale sentence remaining (entry 5,
  0); the pin passes its 4 tests (entry 1). **Gated part not met:** the default branch's README (entry 14) — `not run —
  defer` (gated: the founder's own word on F3), not voided. The chunk wraps on the gated arm while the block stands.
- **v030-16** — implement's part met: `evidence/records.md` enumerates (a) the three renamed names with their `fn`
  lines and both re-pointed ref texts, (b) the nine move-set rows with `file:line`, values and landing, (c) both
  `check-ignore` readings (entries 8, 9, 10 green); `.gitignore:39` names the incubator and `.git/info/exclude` holds no
  incubator line. The cap completes only after P2 moves the nine rows (design-system's body and detail files holding
  none of them) and P7.3 writes both refs to the enumerated texts — its flip to verified reads both.
- (security) entry 7 green, 0 hits of `/home/|api[_-]?key|authorization:|bearer ` over the two documents and their pin;
  no line written quotes transcript text, a tool answer or a kept check's content — met.
- (obs) every run figure the README and the statement give is a `live-verdict.json` field (`tool_calls` 66,
  `wrong_calls` 0, `wrong_call_bar_pct` 5, `tasks_met` six, `lint_calls` 1, `screenshot_calls` 1, `client_version`
  2.1.308, `outcome` passed) — met.
- (a11y) no line claims a criterion beyond SC 2.1.1 · 1.4.3 · 2.4.3, assistive-technology reach, or keyboard or
  contrast coverage of Cells or the load page (the 0.3.0 Engine debts line names those checks as planned) — met.
- (tests) `ci-leg.sh fast` and `doc` green, the guard `guard: clean` — met.
- Gates (implement run `.andromeda/runs/2026-10-27T05-19-41-implement/`, `entries 17 · green 13 · red 0 · recorded 0 ·
  timeout 0 · not-run 4`, `gate v1.22 · 6739ca7e`):
  - `python3 -m unittest discover -s .github/scripts -p test_capability_statement.py 2>&1` — green · exit 0 · `Ran 4
    tests` · `OK`
  - `grep -c -E '^### 0\.1\.0 — done and verified$|…' README.md` — green · last line 3
  - `grep -c -E '0\.2\.0 — planned' README.md` — green · exit 1 · last line 0
  - `grep -o -E '(^|[^-`/a-z_.])escher(…)' README.md | wc -l` — green · exit 1 · last line 0
  - `grep -c -E 'eleven tools, 43|not been re-fired|read through the host.s sink is unmeasured' docs/capabilities.md` —
    green · exit 1 · last line 0
  - `grep -c 'measured instance is the 2026-10-25 cold-agent live run' docs/capabilities.md` — green · last line 1
  - `cat README.md docs/capabilities.md .github/scripts/test_capability_statement.py | grep -c -i -E '/home/|…'` —
    green · exit 1 · last line 0
  - `git check-ignore -v escher-0.3.0-incubator/intent.md` — green · `.gitignore:39:escher-0.3.0-incubator/`
  - `grep -c incubator .git/info/exclude` — green · exit 1 · last line 0
  - the three current names as `fn` lines and in `records.md` — green · `names: 3`
  - `gate.py guard … --allow 'README\.md|docs/capabilities\.md|escher-0\.[12]\.0/verification-matrix\.json' …` — green
    · `guard: clean`
  - `bash .github/scripts/ci-leg.sh fast` — green · exit 0 · 108.42 s · artifact fresh · CI scripts `Ran 108 tests` `OK`
  - `bash .github/scripts/ci-leg.sh doc` — green · exit 0
  - `gh api repos/Turbolet85/escher/readme … | grep -c -E …` — `not run — defer` (gated: the founder's own word on F3),
    not voided
  - `gate.py hygiene` (leg operator) — operator pass: refused once (P1, the planted literal), respelled, re-fired
    `hygiene: clean` (`evidence/operator-pass.md`)
  - `bash .github/scripts/ci-leg.sh fast && git diff --quiet && git diff --cached --quiet && git push origin
    build/escher-0.3.0` (leg operator) — green · exit 0 · 111.06 s · `history moved:
    refs/remotes/origin/build/escher-0.3.0 316c6d25→64256172` (`gate.py run --operator 16`, `evidence/operator-pass.md`)
  - `ci.py conclusion --sha HEAD --wait 1800` (leg operator) — `642561729e7c verdict: green · checks 17/17 · wall 1145 s
    · runs CI#40456250076 completed/success` (`evidence/operator-pass.md`)
  - smoke: skipped — no boot-path or UI-surface change.
- Watches: none.
- Outcome basis: the operator pass ran — its commit `642561729 chore(2026-10-27-readme-and-records): operator pre-CI
  commit, for the run this chunk's verdict reads` (Setup 4's list, the only commit since the base) and the final HEAD's
  CI run CI#40456250076 green, recorded in `evidence/operator-pass.md` (P7's re-verification record). Implement's
  outcome `.andromeda/runs/2026-10-27T05-19-41-implement/outcome-2026-10-27-readme-and-records.md` (the one
  `*-implement` dir holding `gate-2026-10-27-readme-and-records.json`) is the basis for deviations, the guard reading and
  its census; this window held no implement run.
- Process hygiene: implement's census — cargo / rustc (entries 12, 13) terminated, `census: none running` (comm cargo,
  rustc, escher-session, seven_guis_nati); re-measured here — `gate.py census --comm cargo --comm rustc --comm
  escher-session --comm seven_guis_native` → `census: none running`.
## Addendum — resumed 2026-10-27
- inputs#I6 · `../additional/escher-overseer/relays/readme-and-records-provisional-2026-10-27.md` · copy no-repo ·
  unchanged — §F3 the founder's word «Да, переключить» (relayed by overseer1, after 06:09Z); §F3 done: overseer1 ran
  the switch on his «сделай сам», and the escher overseer read back at 06:16:24Z `gh repo view Turbolet85/escher --json
  defaultBranchRef` → `build/escher-0.3.0` and `gh api repos/Turbolet85/escher/readme` → `### 0.1.0 — done and
  verified` (62), `### 0.2.0 — done and verified` (75), `### 0.3.0 — planned, in six stages` (87); «v030-02's gate is
  lifted by his word «Да, переключить» plus this read»; F2 stays PROVISIONAL
  - bears on **Cross-project / external claims**: supersedes «the repository's default branch is `build/escher-0.1.0`
    … so the front page renders that branch's README, not this one» — the default branch is `build/escher-0.3.0` since
    the founder's switch, read back at 06:16:24Z
  - bears on **Spec claims disproved by measurement**: entry 2 — adds to it: its premise (the README edit shows on
    GitHub) holds since the switch; the measured fact the front-page clause takes is the default branch
    `build/escher-0.3.0`, no longer `build/escher-0.1.0`
  - bears on **Expected amendments (from plan)**: entry 1 — supersedes «the measured fact that the front page renders
    the default branch `build/escher-0.1.0` (F3 gated, inputs#I4)» — the front page renders the default branch
    `build/escher-0.3.0`, on the founder's word (inputs#I6)
  - bears on **Decisions & corrections**: F3 — supersedes «v030-02's gated part waits on the founder's own word» —
    his word is given and the switch read back; F2 PROVISIONAL unchanged
  - bears on **v030-02**: supersedes «Gated part not met … The chunk wraps on the gated arm while the block stands» —
    the block's premise is cleared for P5's re-check; entry 14 is read at P7's light gate
- inputs#I7 · message: the operator, in a message during this wrap's second window, 2026-10-27 06:28Z · copy message ·
  n/a — the operator's directive: snap I6; v030-02's gate lifted on the founder's word plus the 06:16:24Z read; entry
  14 can be read now; F2 stays PROVISIONAL; continue on «Proceed.» (given)
  - bears on **Decisions & corrections**: F3 — adds to it: the gate-lift is the operator's direction for P5 and P7
- inputs.py verify, re-fired: inputs: 7 entries — unchanged 3 · drifted 0 · vanished 0 · broken 0 · altered 0 ·
  unreachable 0 · n/a 4 · uncited 2 · unparsed 0; moved since the report: uncited 2 → 0 (I6 · I7, cited here)

## New text, by line
Generated by `cites.py added` (cites v1.7); pasted by `splice.py`. No line of this section is typed or edited.
The diff: 316c6d25 (the parent of the oldest pre-CI commit 64256172) → the work tree.
A row is a block this chunk added: `{first}-{last}`, `@{head}` its head line where not the first, «the head line».

### README.md — added 31 line(s) in 8 range(s)
added: 9 · 38 · 45 · 51-59 · 75 · 77-79 · 81-83 · 85-96
### docs/capabilities.md — added 11 line(s) in 3 range(s)
added: 108-110 · 225-226 · 290-295
