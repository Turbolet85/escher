@row_add n=1 row=decision/locked-builds area=dependency-security rec=1,15
name: Locked builds
rule: Every build, the flake, the bundle and every CI cargo leg, passes --locked so the lockfile is verified.
except: examples/wasm_hello, a standalone workspace with no Cargo.lock
@row_add n=2 row=decision/workspace-inherited-dependency-versions area=dependency-security rec=2
name: Workspace-inherited dependency versions
rule: Dependency versions are inherited from the workspace with workspace = true rather than declared inline in each manifest.
@row_none n=1 rec=3 type=5
@row_none n=2 rec=4 type=6
@row_add n=3 row=decision/serde-json-untrusted-input-reader area=dependency-security rec=5
name: serde_json as the untrusted input reader
rule: serde_json, a workspace dependency with no preserve_order feature, is the reader of the MCP server's untrusted stdin, of a kept check's bytes and of the driver's answer line the replay compares.
@row_add n=4 row=decision/png-shot-encoder area=dependency-security rec=6
name: png as the shot encoder
rule: The png crate is the encoder of a shot in escher-driver.
@row_add n=5 row=decision/no-base64-crate area=dependency-security rec=6
name: No base64 crate
rule: No base64 crate is taken; a shot's base64 is written by hand.
@row_add n=6 row=decision/color-crate-luminance-source area=dependency-security rec=7
name: color crate as luminance source
rule: The test harness's contrast reader takes its luminance from the color crate already in the graph through blitz-dom, adding no package to Cargo.lock.
@row_add n=7 row=decision/test-harness-no-tracing-no-style area=dependency-security rec=7
name: Test harness takes no tracing and no style
rule: blitz-test-harness takes neither the tracing crate nor the style crate as a dependency.
@row_add n=8 row=decision/ci-actions-pinned-to-sha area=dependency-security rec=8
name: CI actions pinned to commit SHAs
rule: Every action a CI workflow uses is pinned to a 40-hex commit SHA, with its ref kept as a trailing comment.
guard: test_ci_workflows.py
@row_add n=9 row=decision/cargo-deny-audit-tool area=dependency-security rec=8,20
name: cargo-deny as the audit tool
rule: cargo-deny 0.20.2, with the root deny.toml, is the dependency audit tool.
@row_add n=10 row=decision/cargo-llvm-cov-coverage-tool area=dependency-security rec=8
name: cargo-llvm-cov as the coverage tool
rule: cargo-llvm-cov 0.9.1 is the coverage tool installed in CI.
@row_none n=3 rec=9 type=5
@row_add n=11 row=decision/blitz-tests-not-published area=dependency-security rec=10
name: blitz-tests not published
rule: The blitz-tests crate is never published (publish = false).
@row_add n=12 row=absence/automated-dependency-updates area=dependency-security rec=11
capability: Automated dependency update tooling, such as dependabot
@row_add n=13 row=decision/dependency-audit-ci-gate area=dependency-security rec=12,22
name: Dependency audit CI gate
rule: The CI audit job runs the dependency audit on every pull request and push, in the slow tier behind the fast jobs, uncached, its log uploaded on failure.
@row_none n=4 rec=13 type=5
@row_add n=14 row=absence/signed-artifacts area=dependency-security rec=14
capability: Signed release artifacts produced by the fork
@row_add n=15 row=decision/signing-material-removed-after-build area=dependency-security rec=14,25
name: Signing material removed after the build
rule: A signing key or keystore written from secrets for a build is removed in an always-run step.
@row_none n=5 rec=16 type=5
@row_add n=16 row=decision/licence-gate area=dependency-security rec=17
name: Licence gate
rule: The audit leg runs upstream's licences table, held byte for byte in the root deny.toml, on every push over the whole workspace's graph, so the fork gates on licences.
except: upstream's own licenses job stays reverted
@row_none n=6 rec=18 type=S
@row_add n=17 row=absence/authentication-authorization area=bootstrap-phases-derive-for-route-setup-project rec=19
capability: Authentication or authorization code
@row_add n=18 row=decision/telemetry-sink-target-allowlist area=bootstrap-phases-derive-for-route-setup-project rec=21
name: Telemetry sink target allowlist
rule: escher's own log sink redacts engine-target messages and fields, redacts the content-named fields of escher targets, and drops every record from a target outside its two prefix sets at every level.
except: the upstream apps' stdout subscribers and the WPT runner's env_logger stay unscrubbed
@row_none n=7 rec=23 type=S
@row_none n=8 rec=24 type=S
@row_add n=19 row=decision/ci-secrets-in-github-actions area=secret-management rec=25
name: CI secrets in GitHub Actions
rule: Production secrets live in GitHub Actions secrets and vars and are passed only to the steps that use them.
@row_add n=20 row=decision/live-runs-use-operator-login area=secret-management rec=26
name: Live runs use the operator's login
rule: The cold-agent live run and the MCP live trial reach the model provider through the operator's own Claude Code login held outside the repository, with no API key, env read, CI secret or auth library.
@row_add n=21 row=decision/live-evidence-holds-no-credentials area=secret-management rec=26,38
name: Live evidence holds no credentials
rule: No committed live evidence, artifact, event or verdict carries a key, auth header, credential-file name, token field, e-mail address or the operator's login.
@row_none n=9 rec=27 type=7
@row_none n=10 rec=28 type=S
@row_none n=11 rec=29 type=5
@row_none n=12 rec=30 type=5
@row_none n=13 rec=31 type=5
@row_none n=14 rec=32 type=5
@row_none n=15 rec=33 type=5
@row_add n=22 row=contract/environment-variables/rust-log area=secret-management rec=34
boundary: Environment variables
item: RUST_LOG
promise: Holds the tracing filter directives escher-telemetry reads, defaulting to warn when unset or unparsable.
@row_add n=23 row=decision/driver-reads-only-rust-log area=secret-management rec=35
name: Driver reads only RUST_LOG
rule: The escher-driver library and the escher-session binary read no environment variable beyond RUST_LOG.
@row_none n=16 rec=36 type=5
@row_none n=17 rec=37 type=5
@row_none n=18 rec=39 type=S
@row_none n=19 rec=40 type=S
@row_none n=20 rec=41 type=4
relied: A failed fetch shows the user error.html with the error's Debug text in its error paragraph.
@row_none n=21 rec=42 type=4
relied: Error pages are flagged is_error and skip history recording and favicon probing.
@row_add n=24 row=contract/rdme-exit-codes/1 area=error-handling rec=43
boundary: rdme exit codes
item: 1
promise: The argument is neither URL nor file, or no README.md is found; a message is written to stderr.
@row_none n=22 rec=44 type=5
@row_none n=23 rec=45 type=S
@row_none n=24 rec=46 type=4
relied: Persistence callers never receive a Result; errors are logged at warn and swallowed.
@row_none n=25 rec=47 type=4
relied: A failed stylesheet request leaves the pending critical resources, so it blocks no render.
@row_none n=26 rec=48 type=4
@row_none n=27 rec=49 type=4
relied: NetHandler::error is a default that does nothing, so a handler sees no failure unless it overrides it.
@row_none n=28 rec=50 type=4
@row_none n=29 rec=51 type=4
relied: Uncaught script errors are logged and not propagated, and a script that fails to resolve or fetch is skipped.
@row_none n=30 rec=52 type=4
@row_none n=31 rec=53 type=4
@row_none n=32 rec=54 type=4
@row_none n=33 rec=55 type=4
