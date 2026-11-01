# security · Secret Management — category read

## Carried
- (.andromeda/security-plan.md:288) Whatever shape is taken, the CI secret set stays exactly these, passed only to the steps that use them; a take that carries upstream workflow edits must add no secret outside this set without recording it.
  > - **Production (CI):** secrets live in GitHub Actions secrets and vars — a macOS signing key and an Android keystore written from secrets for the build and removed in `always()` steps, the Apple certificate and keystore passwords passed from secrets, and the `WPT_GITHUB_TOKEN` and `GITHUB_TOKEN` tokens passed to the steps that use them.
- (.andromeda/security-plan.md:292) The taken delta must bring no secret read into source, so the delta read the scope owes for env reads also answers this line.
  > **Never in code:** secret reads in source are observed absent in every slice that searched:
- (.andromeda/security-plan.md:294) Every env read the taken delta brings must be non-secret and must enter this registry, by name and site, at the wrap; the scope's sink re-check names env reads as part of its read.
  > **Environment values read (none secret-bearing):**
- (.andromeda/security-plan.md:295) `tests/all.rs` must stay at `test = false` and never built under any take, as the scope's blitz-tests rule also requires, and a take that edits these files must re-point these citations.
  > - `env!("CARGO_MANIFEST_DIR")` at build time (examples/screenshot.rs:172), in a test (packages/blitz-vibey-script/tests/preact.rs:13), since 2026-10-19-session-recording in escher-driver's unit tests, which reach the workspace `target/tmp` through it (packages/escher-driver/src/mcp_tests.rs:650-651) and in upstream's `tests/all.rs` of blitz-tests, a target held at `test = false` and never built (tests/blitz-tests/tests/all.rs:78-95)
- (.andromeda/security-plan.md:296) These env reads stand in upstream-owned files a take can edit, so each must be re-read at the take, staying non-secret with its citation correct against the taken tree.
  > - `HOME`, used for the iOS cache path (packages/blitz-net/src/lib.rs:46)
  > - `PAINT_TREE_BENCH_HTML`, an HTML file path for an ignored benchmark (tests/blitz-tests/tests/paint_tree_bench.rs:262-269)
  > - `MULTI_DOC_N`, `MULTI_DOC_SEQ` and `MULTI_DOC_FLAT`, read by the root example `multi_doc` alone through two `std::env::var` call sites: a document count parsed with a default fallback and two flags read through its `env_flag` (examples/multi_doc.rs:17-19; examples/multi_doc.rs:22; examples/multi_doc.rs:36-40; examples/multi_doc.rs:55-58)
  > - `WPT_DIR` (plus compile-time `CARGO_MANIFEST_DIR`) (wpt/runner/src/main.rs:472; wpt/runner/src/main.rs:494)
- (.andromeda/security-plan.md:300) `RUST_LOG` must remain the sink's one filter env read with this default, unchanged by the take, because the sink allowlist re-check and the capture checks at 0 needles stand on it.
  > - `RUST_LOG`, the tracing filter directives escher-telemetry's `EnvFilter` reads, defaulting to `warn` when unset or unparsable — not secret (packages/escher-telemetry/src/lib.rs:144)
- (.andromeda/security-plan.md:301) The escher-driver library and the `escher-session` binary must read no env var beyond `RUST_LOG` after the take, matching the scope's "No new crate, port, env var or listener of ours".
  > the escher-driver library's code and the `escher-session` binary read no env var beyond `RUST_LOG` through `escher_telemetry::init`
- (.andromeda/security-plan.md:302) Upstream's new and edited test files run under the take, so any new `std::env::temp_dir()` site they bring must be read and registered as test-only here, and a take that moves the `tests/dom.rs` helper must re-point its citation.
  > - `TMPDIR`, through `std::env::temp_dir()` — on unix that variable, else `/tmp` — at two test-only sites:
- (.andromeda/security-plan.md:306) This definition is the measure the delta read applies: nothing in the taken delta may carry these values, and any new credential-shaped value must be classified against it.
  > **What counts as secret (as observed):** CI signing material — the macOS signing key, the Android keystore and its passwords, the Apple certificate and its password — and the `WPT_GITHUB_TOKEN` and `GITHUB_TOKEN` tokens (see Storage above); on the operator host, the operator's Claude Code login the cold-agent live run and the 2026-10-10-mcp-surface live trial use, held by the `claude` CLI — no artifact, event or verdict carries it.
- (.andromeda/security-plan-detail/secret-management.md:8) A take that carries edits to the upstream-only publish or WPT workflows must keep each secret's handling as listed, the `always()` removals included, and re-point these line citations; a take that leaves those workflows alone leaves them standing.
  > - **Production (CI):** secrets live in GitHub Actions secrets and vars:
  >   - A macOS signing key is written from a secret to the path in `vars.APPLE_API_KEY_PATH` and removed in an `always()` step (.github/workflows/publish-browser.yml:106-108; .github/workflows/publish-browser.yml:179-181)
  >   - An Android keystore is base64-decoded from a secret to `apps/browser/keystore.jks`, `password` fields (`jks_password`, `key_password`) are appended to Dioxus.toml from a secret, and the keystore is removed in an `always()` step (.github/workflows/publish-browser.yml:110-120; .github/workflows/publish-browser.yml:183-185)
  >   - Apple certificate and certificate `password` are passed as env from secrets; API issuer, key id and key path come from `vars` (.github/workflows/publish-browser.yml:167-172)
  >   - A `token` field is read from secret `WPT_GITHUB_TOKEN` for the cross-repo dispatch (.github/workflows/wpt.yml:108-111)
  >   - The workflow `token` secret `GITHUB_TOKEN` is passed as `github-token` and `GH_TOKEN` (.github/workflows/wpt-post-results.yml:28; .github/workflows/wpt-post-results.yml:32; .github/workflows/wpt-post-results.yml:46)
- (.andromeda/security-plan-detail/secret-management.md:23) `examples/seven_guis/src/tasks/cells.rs` is in the predicted conflict set, so its resolution must leave the formula `Token` enum the only match for this search.
  > only the formula `Token` enum in cells.rs matched
- (.andromeda/security-plan-detail/secret-management.md:24) A take reaching blitz-dom must keep the `password` occurrence a role mapping and no credential, with this citation correct against the taken tree.
  > the only `password` occurrence maps the input type `password` to `Role::PasswordInput` (packages/blitz-dom/src/accessibility.rs:547)
- (.andromeda/security-plan-detail/secret-management.md:29) A take must keep this count true or record the new match, as the delta read of env sites decides.
  > - searched: `std::env|env::var|env!` over the 32 s10 files (one match, the test below)
- (.andromeda/security-plan-detail/secret-management.md:31) Upstream's new test files enter the test crate under a take that carries them, so this absence must be re-read over the files they add.
  > - searched: `token|password|secret|api[_-]?key|authorization|cookie` (case-insensitive) over the 61 s12 files

## Bindings
- **Environment values registry ↔ scope's sink allowlist re-check and body §Logging & Monitoring**: the scope's delta read names every new env read, the registry records each as non-secret, and the sink that `RUST_LOG` filters is the allowlist the same re-check reads, (.andromeda/security-plan.md:294) and (.andromeda/security-plan.md:300).
- **`escher-session`'s env reads ↔ body §Input Validation**: the driver schema's and command line's "reads no env var" rules must hold together with this registry's statement of the same fact after the take, (.andromeda/security-plan.md:301).
- **CI signing secrets ↔ body §Dependency Security, Supply chain integrity**: the signed-artifact line cites the same publish-browser secret steps, so a take moving them moves both records, (.andromeda/security-plan-detail/secret-management.md:8).
- **`tests/all.rs` env read ↔ test plan and scope's blitz-tests rule**: the target held never built is what keeps this read out of every run, and the target list pin at `.github/scripts/test_blitz_tests_targets.py` guards it, (.andromeda/security-plan.md:295).
