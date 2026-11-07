@answer n=1 entity=cognitive-accessibility#1.1 v=no
why: the error and 404 page strings live in apps/browser assets, which the chunk does not reach, and no check it re-asserts reads them.
@answer n=2 entity=cognitive-accessibility#1.2 v=yes
why: the invalid-date cue is witnessed by the stand_* checks the chunk re-asserts (focus sequence and snapshot focused node, aria-invalid read from the accessibility tree the chunk refreshes, and the disabled attribute on the falsy-attribute replay path).
@answer n=3 entity=cognitive-accessibility#2.1 v=no
why: the browser history relative time labels are in apps/browser, which the chunk does not reach, and no named check witnesses them.
@answer n=4 entity=cognitive-accessibility#2.2 v=no
why: the browser tab tooltip is in apps/browser, which the chunk does not reach, and no named check witnesses it.
@answer n=5 entity=cognitive-accessibility#2.3 v=no
why: the browser status bar link target is in apps/browser, which the chunk does not reach, and no named check witnesses it.
@answer n=6 entity=cognitive-accessibility#2.4 v=no
why: the seven_guis card descriptions and tags are app content the chunk neither edits nor re-asserts, and the entity names no check that witnesses them.
@answer n=7 entity=cognitive-accessibility#2.5 v=yes
why: the stand_load_page check is one the chunk re-asserts through stand_*, and it witnesses the load-done text and its display none until the stylesheet lands after the Load click, whose mutation order the buffered replay must keep.
