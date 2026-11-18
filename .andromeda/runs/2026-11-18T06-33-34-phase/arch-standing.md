# arch · standing conventions

- S1 · CI Python tests use unittest, on the stdlib alone, within the Python 3.11 floor of the ci-scripts leg · conventions · stack-and-technologies
  decision/ci-python-unittest · CI Python tests use unittest — The CI Python tests use `unittest`, one `TestCase` per function.
  decision/ci-scripts-python-floor · Python 3.11 floor for the ci-scripts leg — The `ci-scripts` leg needs Python 3.11 or later.
  decision/ci-scripts-stdlib-python · CI helper scripts on the Python stdlib — The CI helper scripts, the coverage floor among them, use the Python stdlib and PyYAML, which the ci-scripts leg installs when absent; that leg needs Python 3.11 or later for tomllib. · why: no package added
- S2 · new tests carry descriptive names stating the behaviour they pin · conventions
  decision/descriptive-test-names · Descriptive test names — Tests carry descriptive snake_case names.
- S3 · a named constant states its rationale where it is declared · conventions
- S4 · the ci-scripts leg of the one CI leg runner runs the cold-agent contract tests; a new test joins it with no new job · standard-contracts
  contract/ci-leg-legs/ci-scripts · ci-leg.sh legs · ci-scripts — `ci-scripts` is a leg of the CI leg script; it runs the Python tests that pin the CI and agent scripts.
  decision/a11y-leg-widens-without-new-job · Accessibility leg widens in place — The a11y leg gains tests with no new job, no required status check and no workflow change. · guard: LegScriptTest.test_gate_legs_dispatch_to_cargo_and_write_their_log
  decision/ci-contract-pinned · CI workflow contract pinned by tests — The fork CI legs, jobs and artifacts are held by a test suite run in the ci-scripts leg. · guard: test_ci_workflows.py
  decision/one-ci-leg-runner · One CI leg runner — Every linux CI job runs its leg through one bash leg runner, the same command the dev host runs. · except: The cross-platform test matrix, its linux android entry included, runs cargo or cross directly.
