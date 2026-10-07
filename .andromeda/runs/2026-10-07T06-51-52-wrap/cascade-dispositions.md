# Cascade dispositions — 2026-10-07-driver-session (wrap run 2026-10-07T06-51-52-wrap)

Written from the sweep's own listing after every body of the pass was applied and the leaves re-derived
(`cascade.py sweep`, run twice: once after the bodies, 2026-10-07T07:12Z, and again after the leaves,
2026-10-07T07:16Z, both times read from the listings' file times; the dispositions below are of the second listing, 51 lines). Baseline `7d9f351d`, the parent of the
chunk's one pre-CI commit. Every pattern's known-positive control fired on the pre-pass masters.

## The search
Eleven patterns (`cascade-patterns.toml`), one per retired claim or wording of the pass, over the seven masters, every
`.andromeda/registries/**` file, the three curation homes, the two judgment bases and the leaf bodies:
`provisional` · `pending-word` (the three Epoch 3 PROVISIONAL items) · `no-listener` (the retired "no listener / no
socket" claim and its mechanism: "nothing is started, polled", "binds no", "no daemon", "No served API") ·
`not-logged` ("logged nowhere", "written to no log", "id's only exit") · `one-installer` (every wording that names
`seven_guis_native` as the one binary installing the sink) · `one-module` ("one shared module", "by six stand checks",
"the one statement of what the stand checks share") · `old-counts` (548 · 0 · 5, 131 result lines, 63 stand checks) ·
`stand-manifest` (`seven_guis/Cargo.toml:29-34`, "blitz-test-harness and blitz-traits") · `tests-manifest` (the stale
blitz-tests manifest ranges) · `scrub-reach` ("Past escher's scrub", "reach is that sink", "discharged for escher's
own", "scrubs by allowlist") · `no-app-tests` (the slice-s04 "no tests" readings).

Not swept by pattern, and why: the root-`Cargo.toml` line re-point is a number shift with no wording to key on — it
was applied by a measured rewrite instead (`cargo-repoint.txt`: 82 sites read, 76 moved, each verified by comparing the
base file's cited lines with the working tree's lines at the new numbers, 6 unmoved — `:1` ×2, `:3`, `:4` ×2, `:17`);
leaves cite no root-`Cargo.toml` line (`(?<![\\w/.\\-])Cargo\\.toml:\\d+` over CLAUDE.md, `.claude/rules/*`,
`.claude/docs/**`: 0 hits).

## Rows, by pattern
- **provisional** — masters 0 · leaves 0 (the first listing's 7 leaf rows — `rules/a11y.md:26`, `docs/a11y-summary.md:24`,
  `docs/security-summary.md:27`, `docs/services/blitz-dom.md:19`, `blitz-shell.md:18`, `dioxus-native-dom.md:18`,
  `:19` — re-derived to "ratified by the founder, 2026-10-07") · base 2: `playbook.md:40`, `:42` — the rule this pass
  appended on the operator's direction; its own subject, no change.
- **pending-word** — 0 rows everywhere after the leaves (the same 7 leaf rows before them).
- **no-listener** — masters 3 standing: `architecture.md:150` (edited; "no socket or port" is true of the two older
  re-executed children and is now followed by the two that host a socket — no further change) · `test-plan.md:106`
  (edited; the sentence now says the script itself listens on nothing) · `test-plan.md:119` (the cold-agent pipe
  "binds no port or socket" — true, not this chunk's, no change). Leaves 4, all re-derived and true as they now read:
  `CLAUDE.md:44` (no network port; the one listener named) · `rules/verification-harness.md:20` (the script starts no
  daemon; the two session checks named) · `docs/tests-summary.md:15` (the same) · `docs/security-summary.md:41` (the
  pending "MCP surface" line quoting that route entry's own wording, "no listener or auth surface" — a statement about
  a future entry, no change).
- **not-logged** — 0 rows (control fired at `security-plan.md:107`): the three wordings no longer stand anywhere.
- **one-installer** — masters: `obs-plan.md:172`, `:173` (edited, both name the two binaries) · `obs-plan.md:243` (new,
  names both). Leaves 0 after re-derivation (`rules/observability.md:18`, `docs/tests-summary.md:17` before it).
- **one-module** — `test-plan.md:99` matched "the one statement of what the stand checks share" in the first listing;
  amended to "the one statement of the tables and helpers the stand checks share", with the sibling bullet for
  `session_common/mod.rs` beneath it. 0 rows after.
- **old-counts** — masters 2 standing: `test-plan.md:115` and `:322` — earlier links of the two re-count chains
  (historical readings, each dated by its chunk), the new link appended after them; no change. Leaf 1:
  `docs/tests-summary.md:29` — re-derived: 572 · 0 · 7 over 140 leads, the 548 figure stands as the previous link.
- **stand-manifest** — masters 0. Leaf 1: `docs/services/seven_guis.md:15` — re-derived (the native dependencies now
  listed as escher-telemetry, blitz-test-harness, blitz-traits and escher-driver); the pattern's second alternative
  still matches the new wording — a true claim sharing the token.
- **tests-manifest** — 0 rows (control fired at `architecture.md:64`): the stale ranges are re-measured to
  `:14-25; :27-29; :31-40` (architecture), `:15-40` (security-plan) and `:33` (a11y-plan, `accesskit`).
- **scrub-reach** — masters 4 standing: `security-plan.md:376` (edited: the past-the-scrub list gains the
  non-allowlisted-target class) · `obs-plan.md:290` (edited: the same class, with the by-level reading) ·
  `obs-plan.md:294` ("escher's own sink scrubs by allowlist in its formatter" — accurate as written: it describes engine
  targets and content-named fields; no change) · `obs-plan.md:295` (edited: both installers, the limit, the owner).
  Leaves 4, all re-derived: `CLAUDE.md:47` · `rules/observability.md:32` · `docs/obs-summary.md:42` (the allowlist
  statement, accurate) · `docs/obs-summary.md:43` (the past-the-scrub list, extended).
- **no-app-tests** — masters 3 standing, all edited to keep the slice reading as the reading at that search and add
  what holds since: `test-plan.md:10`, `:185`, `:208`.

## Leaves re-derived (step 3), by changed source
- architecture → `CLAUDE.md` `GENERATED:setup:overview` (the `packages/` line), `:modules` (new `escher-driver` bullet;
  the `seven_guis` bullet), `:warnings` (the listener line; the scrub line), `:pointer-table` (new Driver session row),
  `:architecture` (the driver-session clause), `:deeper-topics` (the services list) · `docs/commands.md` (the host
  binary and the two new test commands) · `docs/conventions.md` (the `publish = false` list) · `docs/stack.md` (testing
  deps) · `docs/services/seven_guis.md`, `blitz-dom.md`, `blitz-shell.md`, `dioxus-native-dom.md` · new
  `docs/services/escher-driver.md`.
- security-plan → `docs/security-summary.md` (lead, the threat-model list, the snapshot-content and logged-values
  rows, the pending list) · `rules/security.md` (§Surfaces: the lead bullet and a new bullet for the session socket).
- test-plan (and its new key file) → `docs/tests-summary.md` (the agent-run line, the stand log format, the coverage
  list, the local baseline) · `rules/testing.md` (Framework → Integration) · `rules/verification-harness.md` (the
  State bullet).
- obs-plan (and its key file) → `docs/obs-summary.md` (the sink's installers, the past-the-scrub list, the pending
  list) · `rules/observability.md` (the escher-binaries bullet, the scrub bullet) · `docs/services/escher-telemetry.md`.
- a11y-plan → `docs/a11y-summary.md` · `rules/a11y.md` (the refresh's status).
- layout-templates → `docs/design-summary.md` (the cli line).
- design-system — not amended; no leaf.

Not looked for: whether the windowed stand's stderr carries ids at `debug` — unmeasured, and the bodies say so.
`README.md:36` ("not the agent driver, which is planned") was read and left: still true.
