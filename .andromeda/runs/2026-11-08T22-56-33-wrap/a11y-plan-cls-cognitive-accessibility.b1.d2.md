@answer n=1 entity=cognitive-accessibility#1.1 v=no
why: the session reaches no browser error or 404 asset, and no file those pages rest on
@answer n=2 entity=cognitive-accessibility#1.2 v=yes mode=stale
why: tests/blitz-tests/tests/common/mod.rs is reached whole (mod common declarers 23 to 24), so the cited shared tab_order lines 47-63 may have moved
@answer n=3 entity=cognitive-accessibility#2.1 v=no
why: the session touches no browser history code, and browser_history.rs is not among the reached files
@answer n=4 entity=cognitive-accessibility#2.2 v=no
why: tab tooltips live in the browser app, which the session does not reach
@answer n=5 entity=cognitive-accessibility#2.3 v=no
why: the status bar code in apps/browser is not among the reached files
@answer n=6 entity=cognitive-accessibility#2.4 v=no
why: the seven_guis card text in examples/seven_guis/src/app.rs is not among the reached files
@answer n=7 entity=cognitive-accessibility#2.5 v=no
why: the stand load page lives in stand.rs and stand_load_page.rs, neither reached by the session
