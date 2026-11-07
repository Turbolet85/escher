@answer n=1 entity=test-data-fixtures#3.24 v=no
why: the helpers build browser entries in apps/browser and its persistence crate, which the session's dioxus-native-dom and probe changes do not reach
@answer n=2 entity=test-data-fixtures#3.25 v=no
why: the .test TLD test URLs are cited in apps/browser files, which the session does not reach
@answer n=3 entity=test-data-fixtures#3.26 v=no
why: the favicon bytes are encoded in apps/browser/src/favicon.rs, which the session does not reach
@answer n=4 entity=test-data-fixtures#3.27 v=no
why: the fetch body is cited in packages/blitz-vibey-script tests, which the session does not reach
@answer n=5 entity=test-data-fixtures#3.28 v=no
why: the stand load page answer lives in examples/seven_guis and its stand tests, which the task marks not reached
@answer n=6 entity=test-data-fixtures#3.29 v=no
why: the stylesheet answers are cited in render_blocking_stylesheet.rs, which the session does not reach
@answer n=7 entity=test-data-fixtures#3.30 v=no
why: the loopback listeners sit in packages/blitz-net unit tests, which the session does not reach
@answer n=8 entity=test-data-fixtures#4.1 v=no
why: the session adds no fixture file or loader, and the task marks the fixtures inventory not reached
