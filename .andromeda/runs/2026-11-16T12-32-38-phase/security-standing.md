# security · standing conventions

- S1 · A new test in the engine test crate takes no tracing and no style setup of its own, and the test crate stays unpublished. · dependency-security
  decision/blitz-tests-not-published · blitz-tests not published — The blitz-tests crate is never published (publish = false).
  decision/test-harness-no-tracing-no-style · Test harness takes no tracing and no style — blitz-test-harness takes neither the tracing crate nor the style crate as a dependency.
- S2 · A new engine unit test that hits or lays out a node before its styles resolve reads none or false rather than panicking. · error-handling
