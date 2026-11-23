# arch · standing conventions

- S1 · Each integration test file in blitz-tests holds one behaviour and is its own test target, pinned by the targets check. · conventions · standard-contracts
- S2 · Each CI workflow change keeps actions pinned to commit SHAs, read-only permissions and upstream-only guards on upstream workflows. · stack-and-technologies · infrastructure-patterns
  decision/browser-publish-upstream-only · Browser publish job is upstream only — The browser bundle publish job is repository-guarded to DioxusLabs/blitz and does not run on the escher fork.
- S3 · Each CI helper script test uses unittest on the Python stdlib. · conventions · stack-and-technologies
  decision/ci-python-unittest · CI Python tests use unittest — The CI Python tests use `unittest`, one `TestCase` per function.
  decision/ci-scripts-stdlib-python · CI helper scripts on the Python stdlib — The CI helper scripts, the coverage floor among them, use the Python stdlib and PyYAML, which the ci-scripts leg installs when absent; that leg needs Python 3.11 or later for tomllib. · why: no package added
