# design · standing conventions

- S1 · each new check is built with system fonts and never paints or reads with text skipped · typography
  contract/driver-screenshot/fonts-unavailable · driver screenshot · fonts-unavailable — A `screenshot` of an app whose font context resolves no `sans-serif` family is refused `fonts-unavailable` with nothing painted; a shot never paints with text skipped.
  decision/screenshot-never-skips-text · Screenshot never paints with text skipped — A driver screenshot of an app whose font context resolves no sans-serif family is refused with nothing painted, so a shot never paints text skipped.
- S2 · each new driver verb or argument has its help entry, its MCP tool result form, its exit statuses from the schema's table and its session error with a remedy, with no colour on its output · surface
- S3 · each new command-line or MCP output line follows the escher-session stdout forms and is read by agents · surface-cli
