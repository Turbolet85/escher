# arch · standing conventions

- S1 · every new CI helper test is a Python unittest on the stdlib alone, at the ci-scripts leg's Python 3.11 floor · stack-and-technologies · conventions
  decision/ci-python-unittest · CI Python tests use unittest — The CI Python tests use `unittest`, one `TestCase` per function.
- S2 · every new test takes a descriptive name stating the behaviour it pins · conventions
  decision/descriptive-test-names · Descriptive test names — Tests carry descriptive snake_case names.
- S3 · every new committed command-line flow script runs under plain sh · stack-and-technologies
