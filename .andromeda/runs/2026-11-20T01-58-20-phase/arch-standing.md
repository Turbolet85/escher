# arch · standing conventions

- S1 · Each new shared test module and helper carries module and item docs that state what it is for · conventions
  decision/module-and-item-docs · Module and item documentation — Modules, files and examples open with `//!` docs, examples describing what they demonstrate, and public items carry `///`. · except: The driver crate opens every file with `//!`; upstream engine modules and examples largely open without it, and some upstream public items carry no `///`.
  decision/module-docs · Module and file docs — Modules and files open with `//!` docs. · except: The driver crate opens every file with `//!`; upstream engine modules largely open without it.
  decision/public-item-docs · Public item docs — Public items carry `///` docs. · except: Some upstream public items carry no `///`, a node's tree accessor among them.
- S2 · Each expect or unwrap in new helper code carries a justification · conventions
  decision/justified-expect-unwrap · expect and unwrap need a justification — An allow of `clippy::expect_used` or `clippy::unwrap_used` stands next to a justification.
- S3 · An assertion or row-reading helper that panics on a check's behalf is marked track-caller, so a failure points at the calling check · conventions
  decision/track-caller-row-helpers · Track-caller assertion helpers — Assertion helpers are `#[track_caller]`; the MCP listing test and the requirement-sweep test keep their outer assertions and row walk in the test function and assert each row through private ones whose messages name the row and index only.
- S4 · New test code passes the formatting and clippy gate, with any clippy allow kept local to its item · conventions
  contract/ci-leg-legs/fast · ci-leg.sh legs · fast — The `fast` leg runs fmt, clippy, test, ci-scripts and embolden in order, stopping at the first red: the local pre-push gate. After a green verdict, with `CI` unset or empty, it runs the target sweep, its line printed and its exit discarded; a red leg calls no sweep.
  contract/ci-leg-legs/fmt · ci-leg.sh legs · fmt — The `fmt` leg runs `cargo fmt --all --check`.
  decision/local-pre-push-gate · Local pre-push gate — The fast leg run — fmt, clippy, test, ci-scripts, embolden, stopping at the first red — is the local pre-push gate; on a dev host, after a green verdict and skipped under CI, it ends by running the target sweep, whose refusal or failure never reddens it.
- S5 · Test names stay descriptive, and integration test files keep one behavior per file · conventions
  decision/descriptive-test-names · Descriptive test names — Tests carry descriptive snake_case names.
