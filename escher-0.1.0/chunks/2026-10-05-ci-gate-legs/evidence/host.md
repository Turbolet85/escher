# Host prerequisite — coverage leg (plan step 8)

Operator-visible host change on the dev host, approved at the P5 review (2026-10-05).

- Before: `rustup component list --installed` listed no `llvm-tools` component on the default toolchain.
- Command: `rustup component add llvm-tools-preview` → exit 0 (`info: downloading component llvm-tools`), 2026-10-05T22:55Z.
- After: `llvm-tools-x86_64-unknown-linux-gnu` installed; toolchain `rustc 1.99.0 (b940084d7 2026-09-28)`.
- Tool already present: cargo-llvm-cov 0.9.1 (research.md §Measurements), the version ci.yml installs.

CI carries the same prerequisite as the coverage job's `dtolnay/rust-toolchain` input
`components: llvm-tools-preview`.
