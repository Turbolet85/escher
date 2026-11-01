# tests · standing conventions

- S1 · a new blitz-tests check is one file and one target per behaviour under `tests/blitz-tests/tests/`, with no new `[[test]]` table · .andromeda/test-plan-detail/test-strategy.md
- S2 · a regression test is named for the bug it guards and states what exposed it · .andromeda/test-plan-detail/test-strategy.md
- S3 · a stand check runs in both layout modes under the bundled font with no skip path, and holds no sleep or wall-clock read unless wall time is its subject · .andromeda/test-plan-detail/integration-test-strategy.md
  (.andromeda/test-plan-detail/test-strategy.md:46) strategy
- S4 · a check compared byte for byte across OSes holds no shot · .andromeda/test-plan-detail/test-strategy.md
- S5 · a fixture more than one check reads is stated once in `tests/blitz-tests/tests/fixtures/mod.rs`, a one-reader page stays in its check's file, and test data is stated in code · .andromeda/test-plan-detail/test-data-fixtures.md
  (.andromeda/test-plan-detail/test-data-fixtures.md:20) data-seeds
  (.andromeda/test-plan-detail/test-data-fixtures.md:28) data-seeds
  (.andromeda/test-plan-detail/test-data-fixtures.md:30) data-seeds
  (.andromeda/test-plan-detail/test-data-fixtures.md:31) data-seeds
  (.andromeda/test-plan-detail/test-data-fixtures.md:35) data-seeds
- S6 · a new unit test sits in a `#[cfg(test)]` module beside its code, or a `#[path]` file of its own, adding no constructor, `pub` or test shim to the code under test · .andromeda/test-plan-detail/unit-test-strategy.md
- S7 · a new test uses hand-written stand-ins, with no mocking library or new test framework · .andromeda/test-plan-detail/mocking-stubbing-discipline.md
  (.andromeda/test-plan-detail/mocking-stubbing-discipline.md:46) mocking
  (.andromeda/test-plan-detail/mocking-stubbing-discipline.md:47) mocking
  (.andromeda/test-plan-detail/mocking-stubbing-discipline.md:48) mocking
  (.andromeda/test-plan-detail/mocking-stubbing-discipline.md:49) mocking
  (.andromeda/test-plan-detail/mocking-stubbing-discipline.md:50) mocking
  (.andromeda/test-plan-detail/mocking-stubbing-discipline.md:51) mocking
  (.andromeda/test-plan-detail/mocking-stubbing-discipline.md:52) mocking
  (.andromeda/test-plan-detail/mocking-stubbing-discipline.md:53) mocking
- S8 · a new CI step or leg runs through `ci-leg.sh`, every cargo leg `--locked` but `fmt`, actions pinned to SHAs, no `secrets.`, read-only permissions and every package install bounded · .andromeda/test-plan-detail/ci-integration.md
  (.andromeda/test-plan-detail/ci-integration.md:64) ci
- S9 · a check reading a sink's capture counts lines by target, never in total · .andromeda/test-plan-detail/ci-integration.md
- S10 · a new test holds the zero-failure budget, with no retry · .andromeda/test-plan-detail/quality-gates-coverage-targets.md
- S11 · a new test moves the recorded `run stand` and `run all` readings, re-counted with where they were measured · §3 Test Harness Contract, Agent-run contract, Proof
