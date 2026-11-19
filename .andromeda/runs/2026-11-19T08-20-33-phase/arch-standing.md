# arch · standing conventions

- S1 · a new or changed CI helper or sweep script is written on the Python standard library, within the Python 3.11 floor of the ci-scripts leg · stack-and-technologies
  decision/ci-scripts-python-floor · Python 3.11 floor for the ci-scripts leg — The `ci-scripts` leg needs Python 3.11 or later.
  decision/ci-scripts-stdlib-python · CI helper scripts on the Python stdlib — The CI helper scripts, the coverage floor among them, use the Python stdlib and PyYAML, which the ci-scripts leg installs when absent; that leg needs Python 3.11 or later for tomllib. · why: no package added
  decision/cold-agent-judge-stdlib-python · Cold-agent judge on stdlib Python — The cold-agent pipe's judge is python3 on its standard library, with no package added. · why: ratified
- S2 · a new or changed check of a CI script uses unittest and descriptive test names · conventions
  decision/ci-python-unittest · CI Python tests use unittest — The CI Python tests use `unittest`, one `TestCase` per function.
  decision/descriptive-test-names · Descriptive test names — Tests carry descriptive snake_case names.
- S3 · workflow actions stay pinned to commit SHAs and CI permissions stay read-only in any edited workflow job · infrastructure-patterns
  decision/actions-pinned-to-commit-sha · Actions pinned to commit SHAs — Every action a CI workflow uses is pinned to a 40-hex commit SHA, its ref kept as a trailing comment. · guard: test_ci_workflows.py
  decision/ci-read-only-permissions · Read-only CI permissions — CI runs under a workflow-level contents: read permission with no job-level grant.
  decision/pinned-actions · SHA-pinned actions — Every uses: of every workflow is pinned to a 40-hex commit SHA, its ref kept as a trailing comment, and every rust-toolchain step names its toolchain. · guard: test_ci_workflows.py
- S4 · dependencies are declared once in the workspace and crates inherit the workspace package metadata in any edited manifest · conventions
  decision/app-crate-takes-own-tracing · App crates that log take tracing themselves — An app crate that logs takes tracing itself as a workspace dependency. · why: dioxus-native's prelude re-exports no tracing.
  decision/driver-uses-tracing · tracing in the driver — The driver takes tracing from the workspace with no cargo feature.
  decision/inherited-package-metadata · Package metadata inherited from the workspace — Package metadata is inherited with `*.workspace = true`.
  decision/lockstep-package-versions · Blitz packages versioned together — Blitz packages are versioned together through the one workspace version and the in-repo `=` pins.
  decision/lockstep-versioning · Blitz packages versioned together — The published blitz library crates are versioned together through the workspace version and the in-repo `=` pins.
  decision/published-crates-versioned-together · Published crates versioned together — The published packages/ library crates are versioned together through the workspace version and exact = pins between them.
  decision/shared-workspace-package · Shared workspace package metadata — Member crates inherit one workspace version, the MIT OR Apache-2.0 license and metadata, and the in-repo blitz crates are pinned to that exact version.
  decision/workspace-dependencies · Dependencies declared once in the workspace — Dependencies are declared once in `[workspace.dependencies]` and consumed as `{ workspace = true }`. · except: The WPT runner's direct `mimalloc` dependency.
  decision/workspace-licence · Crates take the workspace licence — Crates take their licence from the workspace's dual licence. · except: wgpu_texture is MIT with a SixtyFPS header; accesskit_xplat is Apache-2.0; blitz-dom's default.css carries MPL 2.0; stylo_taffy adds MPL 2.0
  decision/workspace-lockstep-versions · Blitz packages versioned together — Blitz packages are versioned together through the one workspace version and exact in-repo pins.
  decision/workspace-shared-dependencies · Shared workspace dependencies — The Cargo workspace declares its dependencies once in a shared workspace dependency table.
