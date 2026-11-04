@answer n=1 entity=mocking-stubbing-discipline#1.26 v=no
why: the dry-run stub of the GitHub API lives in the WPT diff script, which the chunk does not reach
@answer n=2 entity=mocking-stubbing-discipline#2.1 v=no standing=S5
why: no skip path for a font-dependent check is the headless-stand rule S5 owns, and the blitz-dom document.rs fixtures are not reached
@answer n=3 entity=mocking-stubbing-discipline#3.1 v=no standing=S7
why: absence of mocking libraries in CI and build config is the no-mocking-library convention S7, and the probe adds none
@answer n=4 entity=mocking-stubbing-discipline#3.2 v=no standing=S7
why: absence of mocking libraries in the apps is the no-mocking-library convention S7, and the chunk reaches no browser crate or adds one
@answer n=5 entity=mocking-stubbing-discipline#3.3 v=no standing=S7
why: absence of mocks or stubs in slice s03 is the mocking convention S7, and the probe adds no mock, stub or fake type
@answer n=6 entity=mocking-stubbing-discipline#3.4 v=no standing=S7
why: absence of mocks or fixtures in slice s06 is the mocking convention S7, and the probe adds no mock or stub instance
@answer n=7 entity=mocking-stubbing-discipline#3.5 v=no standing=S7
why: absence of mocks or fixtures in slice s07 is the mocking convention S7, and the probe adds no mock or stub instance
@answer n=8 entity=mocking-stubbing-discipline#3.6 v=no standing=S7
why: absence of mocks, fakes or stubs in slice s08 is the mocking convention S7, and the probe adds no mock or stub instance
@answer n=9 entity=mocking-stubbing-discipline#3.7 v=no standing=S7
why: absence of mock or stub types in slice s09 is the mocking convention S7, and the probe adds no mock or stub type
@answer n=10 entity=mocking-stubbing-discipline#3.8 v=no standing=S7
why: absence of mocks, stubs or fakes in slice s11 is the mocking convention S7, and the probe adds no mock or stub instance
