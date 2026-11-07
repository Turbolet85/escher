@answer n=1 entity=test-data-fixtures#3.24 v=no
why: the browser entries sit in apps/browser, which the session does not reach
@answer n=2 entity=test-data-fixtures#3.25 v=no
why: the .test TLD test URLs sit in apps/browser, which the session does not reach
@answer n=3 entity=test-data-fixtures#3.26 v=no
why: the favicon PNG bytes sit in apps/browser, which the session does not reach
@answer n=4 entity=test-data-fixtures#3.27 v=no
why: the fetch body sits in blitz-vibey-script's tests, which the session does not reach
@answer n=5 entity=test-data-fixtures#3.28 v=no
why: the stand load-page network answer sits in the seven_guis stand and its stand tests, which the session leaves untouched
@answer n=6 entity=test-data-fixtures#3.29 v=no
why: the render-blocking stylesheet answers sit in render_blocking_stylesheet.rs, which the session does not reach
@answer n=7 entity=test-data-fixtures#3.30 v=no
why: the blitz-net loopback listeners sit in blitz-net's unit tests, which the session does not reach
@answer n=8 entity=test-data-fixtures#4.1 v=no
why: the session adds no fixture file or loader, and the task records fixtures as not reached
