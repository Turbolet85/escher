# Cascade dispositions — the 2026-10-09-audit-corrections-agent-surfaces wrap

The pass: thirteen body amendments in five masters — architecture 3 (§Stack and Technologies → CI/CD · §Standard Contracts → CI contracts · §Infrastructure Patterns → CI/CD), security-plan 1 (§Input Validation, a new row), test-plan 7 (§1 two rows · §3 Agent-run contract → Proof · §4 CI workflows and leg script, two amendments on one bullet · §9 two), a11y-plan 1 (§7), obs-plan 1 (§9). No key file was edited. Beside the amendments, the first citation sweep re-pointed 40 citations and a hand 4 (`citation-dispositions.md`); a re-point is no amendment and is not swept for.

## The search
- Pattern set: `cascade-patterns.toml`, 17 patterns, derived after the last body was applied; every control fired on the pre-pass masters (baseline `6d626775`, the parent of the pre-CI commit). Listings: `sweep.txt` (after the body edits, before the leaf edits) and `sweep-after.txt` (after the leaf edits).
- The counts this chunk moved: `64 tests` / `Ran 64` · `the 23 of the files above` · `passed 109` / `109 passed` · `656 passed` / `passed 656`.
- How the masters said CI installs a package: `Ubuntu jobs install` / `jobs install` · `libfontconfig1-dev` · `python3-yaml` / `ensuring PyYAML` / `ensures as Ubuntu` / `PyYAML` · `package install` / `package-install` · `apt` as a word, `apt-pkgs`, `apt deps`.
- What the masters say the CI-script test file pins: `test_ci_workflows` / `LegScriptTest` / `CiWorkflowTest` · `CI workflows and leg script` · `the contract is pinned by`.
- The artifact claim obs-plan's new sentence rests on: `one fork-CI artifact` / `` `target/ci-logs/` alone ``.
- The fixture and the value reading: `stand_snapshot_state` · `checkbox and a radio` / `password mask on an in-file fixture` · `the editor's text` / `typed text read(s|ing) back` · `textarea`.
- Counts, copied from `sweep.txt`: total (17 patterns) · 92 rows over 15 files; per class · new 25/5 · standing 45/7 · leaf 22/8 · curation 0/0 · base 0/0. From `sweep-after.txt`: total (17 patterns) · 106 rows over 16 files; per class · new 25/5 · standing 45/7 · leaf 36/9 · curation 0/0 · base 0/0 — the 14 added leaf rows are this pass's re-derived leaf text.
- Long lines were read by offset or by a bounded window around the match, never from the listing's view: architecture.md 66 · 136 · 141 · 180 · 263, security-plan.md 119, test-plan.md 26 · 115 · 144 · 326, a11y-plan.md 270.
- Not swept, by the tool's reach: the sidecars, the chunk folders, the run dirs, the working route. The working route was read by hand for `apt-get`, `apt-install`, `textarea`, `64 tests`, `libfontconfig`, `package install` and `timeout-minutes`: 12 hits on line 69, this chunk's own frozen line, which states the gap the chunk closed and is compacted at the flip, and 1 on line 86 (`textarea`, the "Range input interaction" entry — read at route-resolve). The two judgment bases hold none of `apt`, `textarea`, `64 tests`; `docs/session-learnings.md` holds none of `apt-get`, `textarea`, `libfontconfig`, `64 tests`, `PyYAML`.
- What was NOT looked for: a statement of the leg count worded without `64` and `tests` together. The sweep's `ran-64` read 0 rows, and the leaf `tests-summary.md:29` still said "(64 CI-script tests)" — found by reading the line the `ci-tests` row named, not by the pattern. It is re-derived below; the miss is the pattern's, stated here so the 0 is not read as an absence.

## Every row of `sweep.txt`

**Zero-row patterns (controls fired)** — `ran-64`, `the-23`, `ubuntu-install`: the retired wording stands in no master, key file, leaf, curation home or judgment base. See the note above for the one leaf wording `ran-64` could not reach.

**new — 25 rows over 5 files: this pass's own text, no further change**
- `architecture.md` 66 (`pkg-install`, `apt` ×3) · 141 (`pkg-install` ×2, `apt` ×7) · 180 (`pyyaml`, `pkg-install`, `apt` ×3) · `security-plan.md:72` (`apt` ×5, `ci-tests`) · `test-plan.md` 8 (`pkg-install`, `apt` ×2, `ci-tests` ×2) · 144 (`pyyaml`, `pkg-install`, `apt` ×6) · 315 (`fontconfig`, `pyyaml` ×3, `pkg-install` ×3, `apt` ×8) · 26 (`textarea` ×3) · 326 (`editor-text`, `textarea`) · `obs-plan.md:333` (`pkg-install`, `apt` ×2) · `a11y-plan.md:270` (`textarea` ×2).

**standing — 45 rows over 7 files**
- Amended lines where swept text still stands, each re-read for a duplicate of what the amendment retired:
  - `architecture.md:66` (`pyyaml` ×4, `ci-tests` ×3): "ensures as Ubuntu `python3-yaml`" is kept and now says how — a guarded step, not reached in CI#37985678276. No second statement of the old unconditional reading on the line.
  - `architecture.md:141` (`ci-tests` ×4, `contract-pinned`, `one-artifact`): "the contract is pinned by `test_ci_workflows.py`" stands for the leg contract; the install contract follows with its own pins. "the one fork-CI artifact outside `target/ci-logs/`" is still true — this chunk adds no artifact.
  - `architecture.md:180` (`fontconfig` ×3, `ci-tests`): the amended clause; the pin-test citation on the line was re-pointed by the sweep and its claim is unchanged.
  - `test-plan.md:8` (`pyyaml` ×2): the amended row.
  - `test-plan.md:144` (`ci-tests` ×7, `ci-bullet`): the amended bullet. Its title is kept — "CI workflows and leg script" is the label the plan and earlier chunk artifacts cite, the retitle one proposal carried was not applied, and the bullet's text names the install script.
  - `test-plan.md:26` (`state-file` ×2, `fixture-reads`, `editor-text`): the amended clause; "typed text reading back as the value" earlier on the line is the stand's text inputs, true as it stands.
  - `test-plan.md:115` (`passed-109`, `state-file` ×2) and `test-plan.md:326` (`passed-656`, `state-file` ×2): the 2026-10-07-driver-command-spans reading and the "+7 `stand_snapshot_state`" clause are their chunks' own readings and stay; this pass's re-count follows each.
  - `a11y-plan.md:270` (`state-file` ×4, `editor-text`): the amended clause and the hand re-pointed citation.
- Citations the sweep or a hand re-pointed, their claims unchanged: `architecture.md:136` and `:263`, `security-plan.md:114` (`state-file`, the file's `//!` doc, now `1-7`) · `security-plan.md:119` (`state-file`, the password test, `458-507`) · `security-plan.md:52` and `:234` (`ci-tests`; `:234` also `apt`, the pinned apt-cache action, true) · `test-plan.md:98` (`state-file`, `79-86`) · `a11y-plan.md:73` and `:204` (`state-file`, `267-303`).
- True claims that share a token, no change:
  - `architecture.md:65` (`fontconfig`): the dev host's packages stand in for CI's `libfontconfig1-dev` — still what CI installs.
  - `architecture.md:263` (`fixture-reads`): "the password mask on an in-file fixture" names the fixture by one case it proves, not by a list of its controls; a fifth control does not falsify it. Left, as the architecture detector also read it.
  - `test-plan.md:99` (`state-file`): the list of checks that read `mod common;`.
  - `obs-plan.md:326` and `:327` (`one-artifact`): failure logs from `target/ci-logs/` alone, and the coverage report as the one artifact outside it — both still true; the new sentence at `:333` rests on them.
  - `registries/contracts/test-plan/session-lifecycle.md:11` (`pkg-install`): the hang of run 37673662374 as history, "a package-install step having hung twice" — the record test-plan §9's new bullet points at. No edit.
  - `textarea` — `architecture.md:89` (the focusable elements), `:136` (@c12780: the snapshot's `value` reader, which already names "a `textarea` or a text-entry `input`" — the mechanism this chunk's test now proves, read by window), `:142` (the google fixture's query textarea), `security-plan.md:110` (`rows` / `cols` parsing), `:162` (the same fixture's maxlength), `design-system.md:116` (the monospace default), `a11y-plan.md:104` (the role mapping), `:116`, `:141`, `:144`.

**leaf — 22 rows over 8 files: 8 leaf lines re-derived in step 3, the rest true as they stand**
- Re-derived from the amended bodies:
  - `.claude/docs/tests-summary.md:21` (`fixture-reads`, `editor-text`) — the fixture's readings gain the textarea's typed text.
  - `.claude/docs/tests-summary.md:29` (`ci-tests`) — the invariant and the install script's contract tests named, the count 64 → 70, the bounded install stated with its witness.
  - `.claude/docs/tests-summary.md:30` (`passed-109`, `state-file`) — this chunk's reading leads (657 · 0 · 10 over 154; `run stand` 110 passed); the earlier ones stay as their chunks'.
  - `.claude/docs/commands.md:6` (`fontconfig`, `apt`) — "(as CI)" now says CI installs it through the script. A line for the script's call form and exits is added under "CI legs".
  - `.claude/docs/stack.md:42` (`pyyaml`, `ci-tests`) — the install script beside the leg runner, the PyYAML install as a guard.
  - `CLAUDE.md:85` (`ci-tests`) — the CI pipeline pointer row names the install script.
  - `.claude/docs/a11y-summary.md:22` (`state-file`) — the value reading gains the textarea.
  - `.claude/docs/obs-summary.md:32` (no row: the leaf of obs-plan §9, enumerated by provenance) — the install's output stays in the step log.
- Left, true as they stand: `CLAUDE.md:92` and `.claude/docs/stack.md:9` (`fontconfig` — Linux needs the package; the dev host's stand-ins) · `.claude/rules/testing.md:17` and `.claude/docs/commands.md:30` (`pyyaml`, `ci-tests` — the leg needs PyYAML) · `.claude/rules/a11y.md:34` and `.claude/docs/a11y-summary.md:14` (`state-file` — the Tab key-press check) · `.claude/docs/commands.md:27` (`state-file` — a stand-check command line) · `.claude/docs/services/dioxus-native-dom.md:17` (`state-file`, `editor-text` — the value reader as architecture `:136` states it, which this pass did not amend) · `.claude/rules/a11y.md:30` (`textarea` — the focusable elements).
- Enumerated by provenance and left: `.claude/docs/security-summary.md` and `.claude/rules/security.md` (security-plan) list no script-argument boundary — neither `agent-run.sh`'s nor `cold-agent.sh`'s row is there — so the new §Input Validation row has no leaf line to re-derive; `.claude/rules/observability.md` (obs-plan) states nothing of CI artifacts; `.claude/rules/verification-harness.md` (test-plan §3) states no stand count.

**curation — 0 rows. base — 0 rows.**

## After the leaf edits (`sweep-after.txt`)
`ran-64`, `the-23` and `ubuntu-install` read 0 rows; `fixture-reads` reads no leaf row (the leaf wording was re-derived); `passed-109` reads one leaf row, the earlier chunk's reading kept in `tests-summary.md:30` beside this chunk's. Every other leaf row is this pass's own re-derived text or a line left above. `registry.py check` over the five amended masters: 0 defects each.
