# arch · standing conventions

- S1 · a new or changed CI workflow test is a stdlib unittest · conventions
  decision/ci-python-unittest · CI Python tests use unittest — The CI Python tests use `unittest`, one `TestCase` per function.
- S2 · a new CI helper or fetch script in Python uses the standard library only and runs on the Python 3.11 floor · stack-and-technologies
  decision/ci-scripts-python-floor · Python 3.11 floor for the ci-scripts leg — The `ci-scripts` leg needs Python 3.11 or later.
  decision/ci-scripts-stdlib-python · CI helper scripts on the Python stdlib — The CI helper scripts, the coverage floor among them, use the Python stdlib and PyYAML, which the ci-scripts leg installs when absent; that leg needs Python 3.11 or later for tomllib. · why: no package added
- S3 · every action a new CI step uses is pinned to a commit SHA · stack-and-technologies · infrastructure-patterns
  decision/actions-pinned-to-commit-sha · Actions pinned to commit SHAs — Every action a CI workflow uses is pinned to a 40-hex commit SHA, its ref kept as a trailing comment. · guard: test_ci_workflows.py
  decision/pinned-actions · SHA-pinned actions — Every uses: of every workflow is pinned to a 40-hex commit SHA, its ref kept as a trailing comment, and every rust-toolchain step names its toolchain. · guard: test_ci_workflows.py
- S4 · every new CI job, step or pin is held by the CI workflow contract tests · standard-contracts
- S5 · every cargo leg a new step runs is locked · standard-contracts · infrastructure-patterns
- S6 · a new Windows script entry is a pass-through to the shared script · standard-contracts
  decision/windows-entries-pass-through · Windows script entries are pass-throughs — The Windows entries of the agent-run and cold-agent pipes and of the target sweep are pass-throughs with no logic.
- S7 · a new CI job uploads its logs on failure · infrastructure-patterns
  decision/ci-failure-logs · CI logs uploaded on failure — Every leg job uploads its own log on failure, kept 7 days, and the coverage job also uploads its report on success, kept 7 days.
