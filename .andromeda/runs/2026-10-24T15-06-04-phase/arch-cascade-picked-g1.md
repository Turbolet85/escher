# arch · picked accounting · group 1 of 7

## Picked
design-philosophy#1.1 · not in scope · the chunk adds, splits or rewires no crate and moves no crate boundary; any fork-gated change to `text_extent.rs` or `lint.rs` stays inside its existing crate — a re-check every change owes
design-philosophy#2.1 · carried · the reached fixtures module holds `HeldNet`, a `NetProvider` the fixture boot injects, and new fixtures boot offline through injected providers
design-philosophy#3.1 · carried · `text_extent`'s RTL, vertical and transformed-box limits are readings measured against browser behaviour, and any defect fix to the reached `text_extent.rs` is written against a named spec
design-philosophy#4.1 · carried · every new fixture check runs both layout modes, and `text_extent` resolves by versioned `NodeId`
design-philosophy#5.1 · carried · every witness the sweep names or adds is a headless harness check living in blitz-tests' or seven_guis' `tests/`
stack-and-technologies#1.1 · carried · blitz-tests' manifest (a merge surface) inherits edition and rust-version from the workspace, and the reached `lint.rs` and `text_extent.rs` build at rust-version 1.91.0
stack-and-technologies#1.3 · not in scope · Stylo's versions live in the root manifest the chunk does not reach, blitz-tests consumes `style` as `{ workspace = true }` unchanged — a dependency re-check every change owes
stack-and-technologies#1.5 · not in scope · taffy's version lives in the root manifest the chunk does not reach, blitz-tests consumes it as `{ workspace = true }` unchanged — a dependency re-check every change owes
stack-and-technologies#1.6 · carried · the reached `text_extent.rs` reads Parley's inline layout, and the RTL and vertical-text limit checks read that text layout through the font context
stack-and-technologies#1.7 · not in scope · the chunk adds no shot reading, and blitz-tests consumes anyrender and anyrender_vello_cpu as `{ workspace = true }` unchanged — a dependency re-check every change owes
stack-and-technologies#1.8 · not in scope · the graphics crates' versions live in the root manifest, and no reached file takes a graphics crate the chunk moves — a dependency re-check every change owes
stack-and-technologies#1.9 · carried · keyboard-types is a blitz-tests dev-dependency in the merge-surface manifest, and a new stand check that presses a key under a Dioxus prelude glob import must name `::keyboard_types`
stack-and-technologies#1.10 · not in scope · accesskit's version lives in the root manifest, and the witnesses read the accessibility tree as built, no accessibility dependency moved — a dependency re-check every change owes
stack-and-technologies#1.13 · not in scope · Dioxus's versions live in the root manifest, and the stand is driven as built with no Dioxus dependency moved — a dependency re-check every change owes
stack-and-technologies#1.14 · carried · png is a blitz-tests dev-dependency in the merge-surface manifest, and serde_json is escher-driver's alone, so no new check takes it
stack-and-technologies#1.16 · carried · tracing, tracing-log and test-that are blitz-tests dev-dependencies in the merge-surface manifest, and the stderr measurement reads the tracing sink
stack-and-technologies#1.20 · carried · blitz-tests' dev-dependency set is the merge-surface manifest every new check target lands in, and seven_guis' `cli_flow` is a stand-repeat target
stack-and-technologies#1.22 · carried · `ci-leg.sh`, `coverage_floor.py`, `test_blitz_tests_targets.py` and `test_ci_workflows.py` are reached CI files whose runner, Python and SHA-pin rules must hold
stack-and-technologies#1.23 · carried · the coverage floors, the stand-repeat budget, the fmt/clippy/doc gates and the host-only mutation tier are the gates the entry names and v020-02's witness
