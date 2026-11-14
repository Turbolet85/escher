# design · standing conventions

- S1 · each new test or fixture page that builds a document is built on the bundled font context, never on the host's system fonts · typography
- S2 · each new check of a run's state reads it through the driver's existing command line output, read by an agent, never by comparing image bytes · surface-cli
