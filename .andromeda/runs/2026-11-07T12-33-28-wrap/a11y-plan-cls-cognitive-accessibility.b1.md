@answer n=1 entity=cognitive-accessibility#1.1 v=no
why: the error and 404 pages live in apps/browser/assets, which the session's dioxus-native-dom bridge and probe changes do not reach
@answer n=2 entity=cognitive-accessibility#1.2 v=no
why: the flight booker's invalid cues, tab_order focus sequence and only-back-btn-focused read sit in stand and seven_guis test and example files the session does not reach, and the focus and key rules it relies on are unchanged
@answer n=3 entity=cognitive-accessibility#2.1 v=no
why: the history relative time labels live in apps/browser/src/browser_history.rs, a file the session does not reach
@answer n=4 entity=cognitive-accessibility#2.2 v=no
why: the tab tooltip lives in browser.css and tab_strip.rs, files the session does not reach
@answer n=5 entity=cognitive-accessibility#2.3 v=no
why: the status bar's hovered link and loading message live in apps/browser/src/status_bar.rs, a file the session does not reach
@answer n=6 entity=cognitive-accessibility#2.4 v=no
why: the seven_guis card descriptions and tags live in examples/seven_guis/src/app.rs, a file the session does not reach
@answer n=7 entity=cognitive-accessibility#2.5 v=no
why: the stand load page's status text turns on stylesheet arrival and LOAD_ANSWER_DELAY in stand.rs and stand_load_page.rs, neither reached, and it names no MountedData, focus or render-time read
