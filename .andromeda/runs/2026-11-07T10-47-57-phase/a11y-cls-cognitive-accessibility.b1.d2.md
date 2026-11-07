@answer n=1 entity=cognitive-accessibility#1.1 v=no
why: the browser's error and 404 page text lives in apps/browser assets the chunk does not touch
@answer n=2 entity=cognitive-accessibility#1.2 v=yes
why: the stand's forward focus sequence, the snapshot's focused node and the stand_* focus checks are re-asserted by the chunk, and Tab focus is the inverted pins' precondition (task items 6 and 11)
@answer n=3 entity=cognitive-accessibility#2.1 v=no
why: the history list's relative time labels in browser_history.rs are not reached by the render bridge
@answer n=4 entity=cognitive-accessibility#2.2 v=no
why: the browser tab tooltips in tab_strip.rs and browser.css are not reached by the chunk
@answer n=5 entity=cognitive-accessibility#2.3 v=no
why: the browser status bar text in status_bar.rs is not reached by the chunk
@answer n=6 entity=cognitive-accessibility#2.4 v=no
why: the seven_guis card descriptions and tags in app.rs are in an app shell the chunk does not change
@answer n=7 entity=cognitive-accessibility#2.5 v=yes
why: the stand's load page paragraph and its display timing are witnessed by stand_load_page, a stand_* check the chunk re-asserts through the buffered render
