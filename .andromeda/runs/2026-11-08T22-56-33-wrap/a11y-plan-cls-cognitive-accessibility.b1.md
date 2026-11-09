@answer n=1 entity=cognitive-accessibility#1.1 v=no
why: the session does not touch the error or 404 page assets or the browser app
@answer n=2 entity=cognitive-accessibility#1.2 v=yes mode=stale
why: the cited shared module tests/blitz-tests/tests/common/mod.rs is reached whole by the session (mod common declarers 23 to 24) and the tab_order lines 47-63 it cites may have moved, with no report showing the claim holds on the changed file
@answer n=3 entity=cognitive-accessibility#2.1 v=no
why: the history relative time labels in the browser app are not in any file the session reached
@answer n=4 entity=cognitive-accessibility#2.2 v=no
why: the tab tooltip in the browser app is not in any file the session reached
@answer n=5 entity=cognitive-accessibility#2.3 v=no
why: the status bar hover and loading text in the browser app is not in any file the session reached
@answer n=6 entity=cognitive-accessibility#2.4 v=no
why: the seven_guis card descriptions and tags in app.rs are not in any file the session reached
@answer n=7 entity=cognitive-accessibility#2.5 v=no
why: the stand load page lives in stand.rs and stand_load_page.rs, neither reached, and the new stand_handler_coverage target does not touch the load page
