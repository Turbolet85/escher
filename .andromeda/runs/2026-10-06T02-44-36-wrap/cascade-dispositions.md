# Cascade dispositions — 2026-10-06-headless-stand

**The search.** `cascade.py sweep --patterns-file cascade-patterns.toml` (16 patterns, every control fired on the
pre-pass masters at baseline d57a6c69) over the seven masters, every `.andromeda/registries/**` file, the three curation
homes, the two judgment bases and the leaf bodies. Patterns: the disabled-ness mechanism (`disabled-parsed`,
`not-disabled`, `falsy-checked`), the telemetry actor (`escher-stand`, `stand-installs`), the HarnessOptions shape
(`harnessopts`, `harness-cites`), stale seven_guis citations (`app-cites`, `lib-cite`), fonts (`font-skip`,
`system-fonts-dep`, `wasm-dejavu`), counts (`count-416`), the manifest convention (`default-feat`), the scope rows
(`bt-devdeps`, `sg-tasks-row`). Line profile read first (`splice.py summary`): architecture.md carries 16 lines over
2 000 chars; every architecture hit below was read on its full line via the Read tool, not a grep view.

**Sections also read whole** (not reachable by a token): arch §Standard Contracts (Test harness · Dioxus DOM bridge ·
the new Headless stand bullet), §Occupied Resources → Names, §Existing Scopes rows seven_guis / blitz-tests;
test-plan §3 Construction/Core/Pump, §7 Seed strategies + Builders, §8 fakes table + Real dependencies, §9 Fonts +
Local baseline; a11y-plan §5 Focus order + Script and framework exposure, §8 Error recovery; design-system
§Typography Loading and §Surface: web-spa Tokens; security-plan §Input Validation rows 100-101; obs-plan §3 Logging
stack.

**Not looked for:** other Dioxus boolean attributes' consumers in the masters (`readonly`, `required`, `hidden`,
`multiple`, `selected`, `open`, `autofocus`) — the pass records their literal-`"false"` write as a fact, not a retired
claim.

## Rows
| row | disposition |
|---|---|
| security-plan.md:101 disabled-parsed (standing, edited) | amended this pass — "Parsed as a boolean value for focusability …; its presence alone … sets the DISABLED state …" (cross-master hit the security detector pointed at, folded in as a routine amendment) |
| .claude/rules/a11y.md:28 not-disabled (leaf) | re-derived — "`disabled` does not parse as `true`" + the two-keyed bullet |
| architecture.md:134 falsy-checked (standing, edited) | amended — `checked` or `disabled` clears; other booleans literal |
| .claude/docs/services/dioxus-native-dom.md:18 falsy-checked (leaf) | re-derived |
| obs-plan.md:67 / :68 stand-installs (new) | amended — `seven_guis_native` installs; the headless stand installs none (true claims) |
| escher-stand · 0 rows | the retired actor wording is gone from every master and leaf (control fired at obs-plan.md:67 pre-pass) |
| architecture.md:129 harnessopts (standing, edited) | amended — the `{ width, height }` quote stays as the test-site citation, followed by the eight-field statement |
| test-plan.md:216 harnessopts (standing, edited) | amended — eight fields |
| harness-cites · 0 rows | every stale harness.rs range re-pointed (test-plan §3 :84/:85/:86, §7 :216, §8 RecordingHandler :153-178) |
| app-cites · 0 rows | layout-templates.md:10 re-pointed (`app.rs:122-151 / 153-172 / 174-328`) |
| lib-cite · 0 rows | architecture.md Deployment model re-pointed (`lib.rs:7-24`) |
| test-plan.md:52 font-skip (standing, edited) | amended — the skip covers the pre-existing tests; stand checks assert unconditionally |
| test-plan.md:253 font-skip (standing) | no change — true for the pre-existing blitz-dom tests; the stand bullet added beside it |
| .claude/rules/testing.md:30 font-skip (leaf) | re-derived — stand checks' boot + no-skip rule |
| test-plan.md:282 system-fonts-dep (standing, edited) | amended — stand checks excepted |
| design-system.md:127 wasm-dejavu (standing) | no change — WASM claim true; the native stand bullet added below it |
| design-system.md:410 wasm-dejavu (standing) | no change — §Surface: web-spa Tokens, WASM-only and true |
| .claude/docs/design-summary.md:24 wasm-dejavu (leaf) | re-derived — the native stand added |
| .claude/docs/gotchas.md:47 wasm-dejavu (leaf) | re-derived — the stand's bundled-font boot + the `woff` requirement |
| test-plan.md:288 count-416 (standing, edited) | amended — the headless-stand re-count appended; 416 kept as the telemetry-bootstrap reading |
| architecture.md:105 default-feat (standing, edited) | amended — the seven_guis default-features exception |
| .claude/docs/conventions.md:18 default-feat (leaf) | re-derived |
| architecture.md:64 bt-devdeps (standing, edited) | amended — seven_guis added |
| architecture.md:257 sg-tasks-row (standing, edited) | amended — the `stand` module and native deps |

## Leaves re-derived (step 3)
Table floor + provenance headers: CLAUDE.md `GENERATED:setup:modules` (seven_guis stand module), `:pointer-table`
(stand.rs + stand checks), `:architecture` (the stand boots headlessly); warnings / overview / workflow recomputed —
no change. `.claude/docs/`: stack.md (Testing deps), conventions.md (Manifests), commands.md (stand-checks command,
fonts troubleshooting), gotchas.md (Font-dependent output), services/seven_guis.md, services/blitz-test-harness.md,
services/dioxus-native-dom.md, tests-summary.md, a11y-summary.md, design-summary.md, obs-summary.md;
security-summary.md recomputed — no change (it carries no `disabled` or stand fact). `.claude/rules/`: testing.md,
verification-harness.md, a11y.md, observability.md; security.md recomputed — no change. services/blitz-dom.md and
workflow.md — their source sections did not change. No curation-home or judgment-base hit.

## Outside the cascade — source, not a leaf
`tests/blitz-tests/tests/dioxus_falsy_disabled.rs:4-6` — the regression test's `//!` doc states the retired claim
("blitz-dom keys disabled-ness on the attribute's presence, so every enabled Dioxus control was styled disabled and
dropped from the focus order"). Wrap touches no source: routed to the next chunk as a CARRY (P5) and to the handoff.
