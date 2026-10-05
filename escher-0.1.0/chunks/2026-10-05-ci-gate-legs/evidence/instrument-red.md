# Instrument red → green — `.github/scripts/test_ci_workflows.py` (plan step 6)

The extended `test_ci_workflows.py` was run against the UNTOUCHED `.github/workflows/ci.yml` and
`.github/scripts/ci-leg.sh` (base `15e36b8a`) before either was edited. Steps 6 and 4 were taken in
that order (6 first) so the leg-script assertions met the untouched script.

## Red — untouched workflows, 2026-10-05T22:52Z

`python3 -m unittest discover -s .github/scripts` → exit 1 · `Ran 23 tests` · `FAILED (failures=50, errors=5)`

| test | reading on the untouched tree |
|---|---|
| `test_b_compiling_jobs_cache_and_save_on_main_and_build` | FAIL — the `coverage` / `a11y` jobs are absent, and the test job's cache has no `shared-key` |
| `test_d_linux_jobs_run_their_leg_through_the_script` | FAIL — the linux job set lacks `audit` / `a11y` / `coverage` |
| `test_e_leg_jobs_upload_their_own_log_on_failure` | ERROR ×3 — subtests `audit`, `a11y`, `coverage` (KeyError, no such job) |
| `test_j_every_action_is_pinned_to_a_commit_sha` | FAIL ×39 — every `uses:` in ci.yml (tag / branch refs: `@v4`, `@v7`, `@v2`, `@master`, `@stable`, `@v1.3.1`, `@latest`) |
| `test_k_pinned_rust_toolchain_names_its_toolchain` | FAIL ×4 — `build-features-default`, `test-features-default`, `build-counter`, `doc` (`@stable` with no `toolchain` input) |
| `test_l_token_is_read_only_at_the_workflow_level` | FAIL — no workflow-level `permissions` |
| `test_m_a11y_job_is_named_for_accessibility` | ERROR — no `a11y` job |
| `test_n_coverage_report_is_its_own_artifact` | ERROR — no `coverage` job |
| `LegScriptTest.test_doc_leg_documents_the_whole_workspace` | FAIL — the doc arm carried neither `--workspace` nor `--no-deps` |
| `LegScriptTest.test_gate_legs_dispatch_to_cargo_and_write_their_log` | FAIL ×3 — `audit`, `a11y`, `coverage` exit 2 (unknown leg) |

Unchanged and green on the untouched tree (the control half): `test_a`, `test_c`, `test_f`, `test_g`,
`test_h`, `test_i`, and the three original `LegScriptTest` tests.

## Green — the chunk's tree

`python3 -m unittest discover -s .github/scripts` → exit 0 · `Ran 23 tests` · `OK` (16 at base → 23:
`test_ci_workflows.py` 12 → 19, `test_wpt_diff_to_pr.py` 4).
