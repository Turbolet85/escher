# obs · CI Integration — category read

## Carried
- (.andromeda/obs-plan-detail/ci-integration.md:12) The widened `a11y` leg must keep its output in its own per-leg log written by `ci-leg.sh`, uploaded only on failure from `target/ci-logs/` alone and holding no user data, so the checks the chunk adds to the leg print nothing of a user's into it.
  > | Per-leg CI log | each ci.yml leg's merged stdout+stderr, written by `ci-leg.sh` to `target/ci-logs/{leg}.log` (matrix: `target/ci-logs/matrix-{platform}.log`), truncated at the leg's start; uploaded only on failure as artifact `ci-log-{job id}` (`if-no-files-found: ignore`), kept 7 days, from `target/ci-logs/` alone; unscrubbed build output (no user data — §8) | (.github/scripts/ci-leg.sh:88-89; .github/workflows/ci.yml:48-54; .github/workflows/ci.yml:416-425) |
- (.andromeda/obs-plan.md:278) Widening the `a11y` leg and its job must add no CI artifact: the fork's CI keeps exactly two uploads, neither holding user data.
  > the fork's CI uploads two artifacts — a leg's merged log on failure and the coverage report on success, each kept 7 days, neither holding user data.
- (.andromeda/obs-plan.md:285) The re-read adds no snapshot artifact upload and no CI resource attribute; both stay unmeasured and outside this chunk.
  > NOT YET MEASURED — snapshot artifact upload and CI resource attributes: the reading recorded none

## Bindings
- **Per-leg CI log ↔ test-plan §CI legs (the `a11y` leg in `.github/scripts/ci-leg.sh` and its `LegScriptTest` a11y tuple in `.github/scripts/test_ci_workflows.py`)**: the leg's target list grows, and its log stays `target/ci-logs/a11y.log` with the upload-on-failure rule unchanged (.andromeda/obs-plan-detail/ci-integration.md:12)
- **Per-leg CI log ↔ obs §3 silence of the candidate checks**: the leg log holds no user data only while the targets moved into the leg stay silent. The engine-debts checks (`accessibility_merged_states`, `inert_subtree`, `accessibility_active_descendant_unread`, `accessibility_states`, `stand_keyboard`) and the select checks (`select_interaction`, `stand_act_select`) are recorded silent at (.andromeda/obs-plan.md:86) and (.andromeda/obs-plan.md:94), and the leg must keep them that way (.andromeda/obs-plan-detail/ci-integration.md:12)
- **Fork CI artifacts ↔ obs §8 PII Scrubbing**: neither upload holds user data, which is the §8 "unscrubbed build output (no user data)" rule (.andromeda/obs-plan.md:278)
