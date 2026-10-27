# Classified — security-plan · input-validation.b1

## Answers
input-validation#1.1 · no · the URL parse sits in examples/screenshot.rs and examples/paint_bench.rs, which the session did not reach
input-validation#1.2 · no · the numeric fallbacks sit in examples/paint_bench.rs, examples/screenshot.rs and examples/multi_doc.rs, none of them reached
input-validation#1.3 · no · the filename filter sits in examples/screenshot.rs, which the session did not reach
input-validation#1.4 · no · the preact_script path check sits in examples/preact_script.rs, which the session did not reach
input-validation#1.5 · no · scripts/agent-run.sh is not among the reached files and blitz-tests keeps its 147 files and 146 targets, so the selection rule holds
input-validation#1.6 · no · scripts/cold-agent.sh is not among the reached files and its fixed STAND_PROMPT is untouched, since no script changed
input-validation#1.7 · no · apt-install.sh is not among the reached files and no CI workflow, ci-leg.sh line or leg changed
input-validation#1.8 · no · coverage_floor.py is not among the reached files, its floors (97, 83, 90) hold, and the entity quotes no coverage reading
input-validation#1.9 · no · the cold-agent check's bounds and reader sit in scripts/cold-agent.sh and the MCP server, neither reached, and the twelve-verb count holds
input-validation#1.10 · no · host, client and wire are unchanged, and the lint findings the socket carries were re-read against the diff and are unmoved
input-validation#1.11 · no · the state-directory rules sit in host.rs, which the session did not change
input-validation#1.12 · no · the recording-directory rules sit in session.rs and recording.rs, which the session did not reach
input-validation#1.13 · no · the strict check reader, replay and recording bounds are unchanged and the two committed checks are byte-unchanged and green
input-validation#1.14 · no · the label grammar sits in session.rs, which the session did not reach
input-validation#1.15 · no · the lint clause (no argument, findings always present, fonts-unavailable before any read, no step run) was re-read against the lint.rs diff and is unmoved, and the schema, command and executor files are unchanged
input-validation#1.16 · no · cli.rs is not among the reached files, and the lint findings it prints were re-read against the diff and are unmoved
input-validation#1.17 · no · the MCP framing, listing and result code are unchanged, the listing test still asserts the twelve tools in VERBS order, and the lint result was re-read unmoved
input-validation#1.18 · no · the urlbar parse sits in apps/browser, which the session did not reach
input-validation#1.19 · no · the navigation scheme limits sit in apps/browser and dioxus-native's link handler, neither of which the session reached
input-validation#1.20 · no · the href resolution sits under packages/blitz-dom, where nothing changed
input-validation#1.21 · no · the DocumentUrl parse sits in blitz-dom's url.rs, where nothing changed
input-validation#1.22 · no · the fragment decode sits in blitz-dom's scrolling.rs, where nothing changed
input-validation#1.23 · no · the file: reader sits in blitz-net, which the reached files do not name
input-validation#1.24 · no · the data: decoder sits in blitz-net, which the reached files do not name
input-validation#1.25 · no · the shell's data: provider sits in blitz-shell, which the reached files do not name
