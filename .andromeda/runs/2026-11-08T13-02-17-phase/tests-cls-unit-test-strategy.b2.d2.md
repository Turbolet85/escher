@answer n=1 entity=unit-test-strategy#25.1 v=yes
why: the record's absence of unit tests in slice s03 binds where the read's graph, snapshot and actionable units named in the scope (items 3 to 5) sit in that slice and must place their tests.
@answer n=2 entity=unit-test-strategy#25.2 v=no
why: slice s07's tests sit in one file and the scope names no unit of that slice; the read's named unit tests are the escher-driver schema's.
@answer n=3 entity=unit-test-strategy#25.3 v=no
why: blitz-vibey-script's source is untouched and the chunk reaches no script engine.
@answer n=4 entity=unit-test-strategy#25.4 v=no
why: the chunk adds integration stand checks and fixture or builder functions in tests/blitz-tests, not an in-file cfg(test) module, so the absence binds nothing its code or tests must honour.
