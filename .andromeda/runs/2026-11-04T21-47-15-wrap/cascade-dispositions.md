# Cascade dispositions — 2026-11-04-dioxus-runtime-probe wrap (step 2)

The search: one `cascade.py sweep` (cascade v1.24) over the five apply writers' `@pattern` records (`apply-{architecture,security-plan,test-plan,obs-plan,a11y-plan}.md`), baseline `fd2e9938` (the parent of the oldest pre-CI commit `53ae48a6`), every control fired. Patterns and the phrasings they key: `arch-01` «resolved 0.13.5» (pulldown-cmark's old resolution) · `sec-01` «owned by a `.andromeda/residuals.md` line» · `sec-02` «Eight ignores stand» (ignore case) · `sec-03` «RUSTSEC-2026-0368 and RUSTSEC-2026-0369» (the three-ID zip passage) · `tp-01` «passed 733 · failed 0 · ignored 12» (the old `run all`) · `tp-02` «157 `.rs` files, 156 test targets» (the old blitz-tests count) · `a11y-01` «that reading owned by a `.andromeda/residuals.md` line» · `a11y-02` «owned by a `.andromeda/residuals.md` line that reads `re-carried:0.2.0`». Main's own text (`edits-main.md`, tx 128) restates no claim beyond `sec-01`'s. Not looked for: the MountedData record's old wording across masters — each of its four sites (architecture fct-apdputpm, security-plan fct-7ni72vcj, test-plan fct-kgita7a3, a11y-plan fct-gmhtpdel) was amended in this pass by its own proposal from the P1 pre-pass sweep's `mounted` and `coords` rows.

Counts, copied from the listing: `total (8 patterns) · 43 rows over 14 files` · `per class · new 0/0 · standing 21/8 · leaf 21/5 · curation 1/1 · base 0/0`.

Zero-row patterns (each control fired, a statement about the pattern): `arch-01`, `sec-02`, `tp-01`, `tp-02` — the old resolution, count, `run all` reading and blitz-tests count stand nowhere in the masters.

## Rows

- `sec-01` standing ×12 — fct-j4ls6r2g (architecture/established-decisions:78), fct-ivrm3he5 (architecture/standard-contracts:33), fct-d2yejoup (security-plan/input-validation:215), fct-agnqdsgq (security-plan/input-validation:241), fct-kbjv4usk (a11y-plan:9), fct-zyrpeqlm (a11y-plan:27), fct-cixbakze (a11y-plan:35), fct-5i3imozy (a11y-plan:135), fct-p7eqwwyx (a11y-plan/a11y-strategy:9), fct-ngdl2hcv (a11y-plan/a11y-strategy:21), fct-br3yxafx (a11y-plan/keyboard-navigation:62) → no change: the `.andromeda/residuals.md` → store-row move is the bases door's (`031e983d9`), not this chunk's work; the family is routed whole to a `CARRY:` at P5 (rule rul-zokdndgi, fanout-results.md's standing direction) · fct-4ncs7geh (architecture/project-intent:11, `edited`) → amended: this pass rewrote the fact (fanout-architecture--cascade-g1.md 11), so main's edit gave it the store-row owner (`edits-main.md`, tx 128)
- `sec-01` curation ×1 — `.claude/docs/session-learnings.md:52-53` → curation home — P3: a learning quoting the clause as an example; no stale claim of its own
- `sec-01` leaf ×14 — `.claude/rules/a11y.md:25` · `:27` · `:32` · `:33` · `:35`, `.claude/rules/security.md:17` · `:26`, `.claude/docs/a11y-summary.md:21` · `:22` · `:23` · `:25` · `:26`, `.claude/docs/security-summary.md:42`, `.claude/docs/services/seven_guis.md:30` → leaf — re-derived at step 3 (each leaf joins the set); the path wording itself follows its masters, the family's CARRY
- `sec-03` standing ×1 — fct-a4qh6ioi (security-plan/dependency-security:9, `edited`) → no change: the amended sentence names the three IDs and then RUSTSEC-2026-0429, a true claim of the new text
- `sec-03` leaf ×1 — `.claude/docs/security-summary.md:42` → leaf — re-derived (three zip IDs → four, pulldown-cmark 0.13.6)
- `a11y-01` leaf ×1 — `.claude/docs/a11y-summary.md:25` → leaf — re-derived (the reading's owner now the field owed to «The UI graph»)
- `a11y-02` standing ×8 — fct-ivrm3he5, fct-d2yejoup, fct-agnqdsgq, fct-kbjv4usk, fct-zyrpeqlm, fct-cixbakze, fct-p7eqwwyx, fct-ngdl2hcv (the same rows as `sec-01`'s) → no change: the family's CARRY
- `a11y-02` leaf ×5 — `.claude/rules/a11y.md:25` · `:27`, `.claude/rules/security.md:17`, `.claude/docs/a11y-summary.md:21` · `:26` → leaf — re-derived

## Leaves

The set: `cascade.py leaves` (pass mode — this chunk closes no epoch), step 2's five leaf files added by `--add`; `re-pointed .andromeda/architecture.md · 1 line(s), citation digits alone — not a changed source`; `total · pass · 14 group(s) · 24 leaves · 645139 B · unmapped 1` — `unmapped .claude/rules/host-linux.md (host)`: the host leaf, a statement no source states, no comparer's and no step-3 edit. Comparers 14/14 (`fanout.py check` PASS). The apply: `total · blocks 25 · applied 15 · held 10`.

Decision rule: a stale row is applied; an omission is applied only where a reader of the leaf would otherwise act against the source — else the detail stays in its master and the omission is declined.

- a11y-plan-1 (pass) · 1 finding · applied 1 (`a11y-summary.md:25#1`, stale: the reading's owner → measured by link 2, its field owed to «The UI graph») · by hand 0 · declined 0
- a11y-plan-2 (pass) · 1 · applied 0 · declined 1 (`rules/a11y.md:28#1`, omission: the mark's over-inclusion is conservative — a reader keys the element either way; the detail stays in a11y-plan §7)
- architecture-1 (pass) · 0
- architecture-2 (pass) · 3 · applied 1 (`services/dioxus-native-dom.md:31#1`, stale: the hazard's mechanism, its pin and its route owner) · declined 2 (`:16#1`, `:22#1`, omissions: the listener attributes' over-report and the check's over-inclusion — detail of standard-contracts, no reader acts against it)
- architecture-3 (pass) · 1 · applied 0 · declined 1 (`workflow.md:15#1`, omission: the port's routing is project-intent's and the route's)
- architecture+obs-plan (pass) · 0
- architecture+security-plan+test-plan-1 (pass) · 0
- architecture+security-plan+test-plan+obs-plan+a11y-plan-1 (pass) · 1 · applied 0 · declined 1 (`CLAUDE.md:98#1`, omission: the audit before a push is carried by `.claude/rules/security.md`, loaded on every turn — applied there (security-plan-2 #1); CLAUDE.md takes no second copy)
- architecture+test-plan (pass) · 1 · applied 0 · declined 1 (`commands.md:63#1`, omission: `cargo test -p blitz-tests --test {name}` already names the form; the target's detail is test-plan's)
- obs-plan (pass) · 2 · applied 0 · declined 2 (`obs-summary.md:49#1`, `:43#1`, omissions: the probe's silence and PII rows stay in log-coverage and pii-scrubbing-compliance; no reader acts against them)
- security-plan-1 (pass) · 7 · applied 6 (`security-summary.md:42#1` omission — the fourth zip ID, a reader of three IDs would read RUSTSEC-2026-0429's ignore as unaccounted; `#3`–`#6` stale — the owners as store residual rows, `res-h7acz56c` for zip; `#7` omission — the audit's local run before a push) · declined 1 (`#2`, omission: «never calls `extract`» against «never extracts to disk», the same posture)
- security-plan-2 (pass) · 5 · applied 5 (`rules/security.md:26#1` omission — run the audit before a push; `#2`, `#4`, `#5` stale — store residual rows; `#3` omission — the fourth zip ID, the zip owner `res-h7acz56c`)
- test-plan-1 (pass) · 3 · applied 2 (`tests-summary.md:18#1` stale — `run all` 743 · 0 · 12; `:37#1` omission — the hazard's pin and its route owner, so a reader does not fix it ad hoc) · declined 1 (`:21#1`, omission: the target's detail stays in test-scope-summary)
- test-plan-2 (pass) · 0

Totals: findings 25 · applied 15 · by hand 0 · declined 10.
