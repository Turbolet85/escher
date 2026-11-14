# P5 controls — 2026-11-13-local-files-for-local-documents

Every control fired as the entry itself: `gate.py run --chunk 2026-11-13-local-files-for-local-documents --docs-root
. --entry {n} --project-root $TMPDIR/claude-1000/-home-turbolet-dev-projects-escher/8986e442-39f4-4925-b176-6fc721038ca5/scratchpad/ctl`,
the clone a `git clone --shared` of the project at HEAD `8a166d6aa3a368afa57e94f05b30a3237730afc0`, no `--run-dir`.

| entry | clone role | input | reading |
|---|---|---|---|
| 6 (census) | unmodified | none | green · last line 0 (as on the tree) |
| 6 (census) | control, must fail | `fn __ctl() { println!("x"); }` appended to `packages/blitz-dom/src/iframe.rs` | red · last line 1 |
| 8 (shared modules guard) | unmodified | none | green · exit 0 (as on the tree) |
| 8 (shared modules guard) | control, must fail | `// ctl` appended to `tests/blitz-tests/tests/fixtures/mod.rs` | red · exit 1 |
| 7 (witness silence) | must-pass | `tests/blitz-tests/tests/local_files_for_local_documents.rs` holding `//! clean witness stand-in` and `fn main() {}` | green · last line 0 |
| 7 (witness silence) | control, must fail | the same file with `std::thread::sleep(…)` appended | red · last line 1 |

The construction guard's known-positive (the operator's review, `relay-1.md`) was taken in the same clone after its
reset to HEAD. The input was `packages/blitz-dom/clippy.toml` holding `disallowed-methods = [{ path =
"blitz_traits::net::Request::get", reason = "build sub-resource requests through stamped_request" }]`, with no other
edit. The command was `cargo clippy -p blitz-dom --locked --all-targets -- -D clippy::disallowed-methods`, typed with
the clone as root and its own target dir under the project's ignored `target/tmp/ctl-clippy`. It differs from entry
9's `run` by the root alone: entry 9 fires on the tree, and the tree holds no `clippy.toml` before this chunk.

The reading: red, exit 101, with one finding. `error: use of a disallowed method blitz_traits::net::Request::get -->
packages/blitz-dom/src/net.rs:32:19`, the note carrying the configured reason, on clippy 1.99.0. So the crate-level
config is read, and it names today's one construction site. Entry 9's must-pass (the same config plus the `expect` at
that site, green) and its control with the `expect` removed are implement's `gate.py control` run (plan step 7).

Entry 4's control is implement's red-before-green run (plan step 7, `gate.py control`), the target being the file this
chunk writes.
