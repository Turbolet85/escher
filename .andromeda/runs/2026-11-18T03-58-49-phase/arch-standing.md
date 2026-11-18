# arch · standing conventions

- S1 · a new test of a CI helper script is written with Python's unittest · conventions
  decision/ci-python-unittest · CI Python tests use unittest — The CI Python tests use `unittest`, one `TestCase` per function.
- S2 · a CI helper script and its tests use the Python standard library only and run on the Python 3.11 floor of the ci-scripts leg · stack-and-technologies
  decision/ci-scripts-python-floor · Python 3.11 floor for the ci-scripts leg — The `ci-scripts` leg needs Python 3.11 or later.
  decision/ci-scripts-stdlib-python · CI helper scripts on the Python stdlib — The CI helper scripts, the coverage floor among them, use the Python stdlib and PyYAML, which the ci-scripts leg installs when absent; that leg needs Python 3.11 or later for tomllib. · why: no package added
