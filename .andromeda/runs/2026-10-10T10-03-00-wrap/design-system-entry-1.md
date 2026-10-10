
## 2026-10-10-driver-cli — the driver command line's output form
**Section:** §Surface: cli → Component Patterns
**Change:** a new bullet: `escher-session` prints one line of JSON on stdout per command — the answer, the refusal or a session error's object, keyed by the driver schema's own words — and nothing else there; a usage error prints one fixed line on stderr with stdout empty, a session error its fixed message on stderr; no colour and no colour dependency, no ESC byte on any answer line, and no code asks whether a stream is a terminal, so the output is the same piped or not; the exit status carries the class, 0 accepted · 1 refused · 2 usage · 3 session error.
**Why:** the surface's reader is an agent, so its form is the schema's words, uncoloured (the founder's answers on the shape of stdout and on exit codes, 2026-10-10, relayed by the overseer). No detector covers an output form; the amendment was raised from the plan's reviewed list.
**Kept:** the WPT runner's colours and the surface's two NOT YET MEASURED sections stand as they were.
**Ref:** .andromeda/runs/2026-10-10T10-03-00-wrap/
