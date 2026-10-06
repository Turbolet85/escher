# design extract

## No domain coverage
The chunk builds a headless telemetry bootstrap (subscriber, stderr/file sink, service identity, panic hook, scrub layer, opt-in OTel export) and renders no UI, so it uses no colour, type, spacing, depth, radius, motion or icon tokens. The only adjacent section, design-system §Surface: cli, records the as-built terminal output of other binaries (WPT runner, paint_bench, screenshot, bump) and sets no rule for log formatting or stream choice.
