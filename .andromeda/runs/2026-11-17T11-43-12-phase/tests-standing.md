# tests · standing conventions

- S1 · every test file joining the workspace is one integration test target of its own, pinned by the blitz-tests targets check · test-strategy · 1-test-scope-summary
  decision/session-checks-share-one-module · Session checks share one module — What the checks that hold a session share, in process and by a host process of their own, is stated in one session test module with no test and no test target of its own, beside the stand checks' module. · why: kept beside the stand checks' module because that module was held unchanged
  decision/shared-fixtures-in-one-module · Shared fixtures in one module — The fixture boot and hold, and every fixture more than one check reads, are stated once in one fixtures module with no test and no test target, holding no print, tracing call, env read, file, socket, listener or blitz_net. · except: a page with one reader, or whose ids differ, stays in its own check file
  decision/shared-stand-tables-in-one-module · Shared stand tables in one module — The tables and helpers the stand checks share are stated once in one shared test module with no test and no test target of its own; the tables are stated expectation, never a second role or name mapping. · except: stand_element_ids keeps its own boot (it books the flight), and task_keys, ceiling, component, the cards array and each file's SECRET stay per-file
- S2 · every CI step added uses actions pinned to a commit SHA, under read-only permissions and no secrets · unit-test-strategy · 4-unit-test-strategy
  decision/ci-rust-cache · SHA-pinned Rust cache — Every compiling CI job uses the SHA-pinned Swatinem/rust-cache. · except: coverage
- S3 · every CI job or step added keeps the leg script, cache, locked cargo and failure-log upload conventions of the existing jobs · ci-integration · 9-ci-integration
  contract/ci-artifacts/ci-log-job-id · CI logs and artifacts · ci-log-{job id} — The leg's log, uploaded only on failure and kept 7 days.
  decision/cargo-legs-locked · Cargo legs run locked — Every cargo leg of CI runs with --locked. · except: fmt, and examples/wasm_hello, which has no Cargo.lock
  decision/ci-cache-save-branches · Cache saved on main and build branches — CI caches are saved only on main and build/* branches.
  decision/ci-shared-test-cache · Shared CI build cache — Every compiling CI job but coverage uses a SHA-pinned Swatinem/rust-cache saved on main and build branches; the a11y and stand-repeat jobs restore the test job's cache without saving.
  decision/shared-test-cache · Shared workspace test cache — The test job saves the cache under shared-key workspace-test, which the a11y and stand-repeat jobs restore without saving.
