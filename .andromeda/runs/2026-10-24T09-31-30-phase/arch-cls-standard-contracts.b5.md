# Classified — arch · standard-contracts.b5

## Answers
standard-contracts#23.1 · no · escher-session's CLI is Not needed, the browser and WPT runner mains take only the mimalloc allocator line, and the listed example CLIs are outside the take's delta
standard-contracts#24.1 · no · the browser app's decisions, contract and runtime are Not needed, and mimalloc is its only reach, an allocator rather than app behaviour
standard-contracts#25.1 · no · the WPT runner's decisions, outputs and execution model are Not needed (upstream-only runner, conformance numbers out of scope), and mimalloc is only an allocator line in its main
standard-contracts#26.1 · yes · the ci-leg.sh legs (fast, doc, audit, coverage with its keyboard_compat.rs exclusion, stand-repeat, msrv), the blitz-tests target pin and the per-platform ci-log artifacts are named Needs
