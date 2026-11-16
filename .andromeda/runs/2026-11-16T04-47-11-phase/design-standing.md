# design · standing conventions

- S1 · each new test of the driver runs against a minimal fixture where the stand's tasks lack a case, built with system fonts, and runs in both layout modes · typography · surface
  decision/test-engine-with-system-fonts · Test engine built with system fonts — The blitz-dom engine is built with the system-fonts feature for the blitz-tests crate. · why: Without system fonts text measures 0x0 and font-dependent assertions pass vacuously.
  decision/tests-built-with-system-fonts · Tests built with system fonts — The blitz-tests crate builds `blitz-dom` with the `system-fonts` feature, which is also enabled by default when testing the whole workspace. · why: Without system fonts text measures 0x0 and font-dependent assertions pass vacuously.
- S2 · each new answer field of the driver is listed in the schema's table, described by help and the MCP tool listing, and written the same on every ratified carrier · surface · surface-cli
- S3 · each new log line or span field stays content-free under the telemetry scrub and carries no screen content · surface
