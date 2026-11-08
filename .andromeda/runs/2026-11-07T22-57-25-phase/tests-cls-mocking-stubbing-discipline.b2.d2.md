@answer n=1 entity=mocking-stubbing-discipline#1.26 v=no
why: the WPT diff script's --dry-run and run_url=None tests; the chunk touches no WPT (task item 2 not needed)
@answer n=2 entity=mocking-stubbing-discipline#2.1 v=no
why: skip-free font tests in blitz-dom document.rs hover_state_tests; the chunk edits no document.rs, and the stand font rule (task item 17) is carried by the stand checks, not this unit
@answer n=3 entity=mocking-stubbing-discipline#3.1 v=no
why: absence of mocking libraries in CI and build config; a re-check every change owes, the chunk adds no mock dependency and edits no CI or Cargo file in scope
@answer n=4 entity=mocking-stubbing-discipline#3.2 v=no
why: absence of mocking libraries in the apps; a re-check every change owes, apps/browser is not needed and the chunk adds no mock
@answer n=5 entity=mocking-stubbing-discipline#3.3 v=no
why: absence of mocks or stubs in slice s03; a re-check every change owes, the chunk's new tests drive a real DioxusDocument or the stand's Session and add no double
@answer n=6 entity=mocking-stubbing-discipline#3.4 v=no
why: absence of mocks or fixtures in slice s06; a re-check every change owes, the chunk adds no mock or fixture double to that slice
@answer n=7 entity=mocking-stubbing-discipline#3.5 v=no
why: absence of mocks or fixtures in slice s07; a re-check every change owes, the differential oracle and restyle paths are not needed and the chunk adds no double
@answer n=8 entity=mocking-stubbing-discipline#3.6 v=no
why: absence of mocks, fakes or stubs in slice s08; a re-check every change owes, the chunk adds no double to that slice
@answer n=9 entity=mocking-stubbing-discipline#3.7 v=no
why: absence of mock or stub types in slice s09; a re-check every change owes, the chunk adds no mock or stub type, its shims being forbidden by task item 4
@answer n=10 entity=mocking-stubbing-discipline#3.8 v=no
why: absence of mocks, stubs or fakes in slice s11; a re-check every change owes, the log-scrub checks read the real sinks and the chunk adds no double
