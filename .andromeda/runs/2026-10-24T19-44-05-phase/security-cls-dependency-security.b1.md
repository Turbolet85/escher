# Classified — security · dependency-security.b1

## Answers
dependency-security#1.0 · yes · the audit line at ci-leg.sh:38 (`cargo deny --locked ... check advisories licenses`) sits in the merge surface the a11y edit reaches, so its `--locked` flag and the deny.toml ignores it reads must survive
dependency-security#2.1 · no · the 2026-10-10 sync's icu and lock edges are a past record in Cargo.toml:202-206 and Cargo.lock, neither reached, and the chunk moves no dependency
dependency-security#2.2 · no · the 2026-10-13 sync's Taffy and lock-set record sits in Cargo.toml and Cargo.lock, neither reached, and the chunk moves no dependency
dependency-security#2.3 · no · the 2026-10-17 keyboard-types and test-that bumps sit in Cargo.toml and Cargo.lock, neither reached, and the chunk moves no dependency
dependency-security#2.4 · no · the 2026-10-18 sync moved no dependency, a record over Cargo.toml, Cargo.lock and deny.toml, none of them reached
dependency-security#2.5 · no · the 2026-10-19 sync moved no dependency, a record over Cargo.toml, Cargo.lock and deny.toml, none of them reached
dependency-security#2.6 · no · the 2026-10-24 sync's lock edges (mimalloc, tokio 1.54.1, twelve re-pointed edges) sit in Cargo.lock and the browser and WPT manifests, none reached
dependency-security#3.1 · no · the read-only serde_json rule lives in escher-driver's manifest and its MCP, kept-check and replay readers, none reached, and the task names no serde_json use
dependency-security#4.1 · no · png serves the shot encoder and blitz-tests' screenshot checks, both kept out of the leg, and the chunk adds no dependency
dependency-security#5.1 · yes · the contrast rows read through Harness::text_contrast, whose luminance source is the color dependency blitz-test-harness takes, used as is with no dependency entering Cargo.lock
dependency-security#6.1 · yes · the licence gate is the `licenses` check on the audit line at ci-leg.sh:38, a merge surface the a11y edit reaches, and it must still run on every push
