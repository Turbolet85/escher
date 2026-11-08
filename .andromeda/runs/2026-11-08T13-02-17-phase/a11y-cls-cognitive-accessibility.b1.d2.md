@answer n=1 entity=cognitive-accessibility#1.1 v=no
why: the error and 404 pages' text in apps/browser assets is not reached by a read of the screen's actionable nodes
@answer n=2 entity=cognitive-accessibility#1.2 v=yes
why: the flight booker's Book button gets a disabled attribute on a typed non-date, the keying the read's disabled cause must match (items 5 and 6), and the stand's tab-order sequence rests on the focusability rule (item 7) that decides the read's focusable part
@answer n=3 entity=cognitive-accessibility#2.1 v=no
why: relative time labels in browser_history.rs are not reached by the read
@answer n=4 entity=cognitive-accessibility#2.2 v=no
why: tab tooltips in the browser are not reached by the read
@answer n=5 entity=cognitive-accessibility#2.3 v=no
why: the status bar's hovered link target is not reached by the read
@answer n=6 entity=cognitive-accessibility#2.4 v=no
why: the seven_guis card descriptions and tags in app.rs are not on the read's path
@answer n=7 entity=cognitive-accessibility#2.5 v=no
why: the load page's paragraph load-done is a text node the read never lists, its display none carried by the snapshot's own visibility rule, and the chunk does not run stand_load_page
