@answer n=1 entity=cognitive-accessibility#1.1 v=no
why: the error and 404 pages live under apps/browser/assets, which the session's change does not reach
@answer n=2 entity=cognitive-accessibility#1.2 v=no
why: the shared common module is reached, but the report leaves the error-recovery cues and the Tab orders unchanged, so the cited invalid state and focus sequences still hold
@answer n=3 entity=cognitive-accessibility#2.1 v=no
why: the history relative-time labels in apps/browser/src/browser_history.rs are not reached by the session
@answer n=4 entity=cognitive-accessibility#2.2 v=no
why: the tab tooltip code and CSS in apps/browser are not reached by the session
@answer n=5 entity=cognitive-accessibility#2.3 v=no
why: the status bar code in apps/browser/src/status_bar.rs is not reached by the session
@answer n=6 entity=cognitive-accessibility#2.4 v=no
why: the seven_guis card descriptions and tags in examples/seven_guis/src/app.rs are not reached, only the seven_guis test files are
@answer n=7 entity=cognitive-accessibility#2.5 v=no
why: the load page's status text in examples/seven_guis/src/stand.rs and stand_load_page.rs is not reached by the session
