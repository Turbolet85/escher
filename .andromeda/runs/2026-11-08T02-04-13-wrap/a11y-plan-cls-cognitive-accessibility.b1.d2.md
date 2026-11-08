@answer n=1 entity=cognitive-accessibility#1.1 v=no
why: the error and 404 pages live in apps/browser/assets, which the session's reached files do not include
@answer n=2 entity=cognitive-accessibility#1.2 v=yes mode=stale
why: flight_booker.rs and tests/blitz-tests/tests/common/mod.rs are reached whole by the session, so the cited line ranges in them may have moved, and the report holds the invalid-date cue and Tab orders but not those line citations
@answer n=3 entity=cognitive-accessibility#2.1 v=no
why: browser_history.rs sits in apps/browser, which the session does not reach
@answer n=4 entity=cognitive-accessibility#2.2 v=no
why: the tab tooltip lives in apps/browser (browser.css and tab_strip.rs), which the session does not reach
@answer n=5 entity=cognitive-accessibility#2.3 v=no
why: status_bar.rs sits in apps/browser, which the session does not reach
@answer n=6 entity=cognitive-accessibility#2.4 v=no
why: the seven_guis card text lives in app.rs, which is not among the session's reached files
@answer n=7 entity=cognitive-accessibility#2.5 v=no
why: the load page lives in stand.rs and stand_load_page.rs, neither reached by the session, and the report holds its Tab order unchanged
