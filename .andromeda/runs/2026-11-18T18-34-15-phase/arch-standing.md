# arch · standing conventions

- S1 · CI Python tests use unittest · conventions
  decision/ci-python-unittest · CI Python tests use unittest — The CI Python tests use `unittest`, one `TestCase` per function.
- S2 · CI helper scripts on the Python stdlib · stack-and-technologies
  decision/ci-scripts-stdlib-python · CI helper scripts on the Python stdlib — The CI helper scripts, the coverage floor among them, use the Python stdlib and PyYAML, which the ci-scripts leg installs when absent; that leg needs Python 3.11 or later for tomllib. · why: no package added
- S3 · Escher capitalised in prose and its name spelled one way · project-intent
  decision/escher-capitalised-in-prose · Escher capitalised in prose — The front page spells the name Escher with a capital in its prose and title, while crate, binary and path names, URLs and the e-mail stay unchanged.
  decision/escher-name-spelling · Spelling of the Escher name — The name is spelled «Escher» in prose and titles; crate, binary and path names, URLs and the e-mail stay unchanged.
