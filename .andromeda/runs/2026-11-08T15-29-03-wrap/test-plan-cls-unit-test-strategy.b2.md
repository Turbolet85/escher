@answer n=1 entity=unit-test-strategy#25.1 v=yes mode=stale
why: the session's new unit tests land in reached files actionable.rs and actions.rs, and the claim of no unit tests in slice s03 cannot be shown to hold on them
@answer n=2 entity=unit-test-strategy#25.2 v=yes mode=stale
why: the reached escher-driver actions.rs adds two unit tests and a new file, so the eight-file no-tests reading of slice s07 is stale
@answer n=3 entity=unit-test-strategy#25.3 v=no
why: blitz-vibey-script is untouched by the session and its tests stay unchanged
@answer n=4 entity=unit-test-strategy#25.4 v=yes mode=stale
why: stand_actions.rs joins tests/blitz-tests, moving slice s12's 61-file count to 62
