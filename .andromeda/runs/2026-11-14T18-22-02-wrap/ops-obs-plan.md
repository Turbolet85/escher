@row_set n=1 row=decision/no-file-loads-from-non-file-documents cell=rule seen=2 detector=none
value: A file: load asked by a non-file: document is refused, adding no log site: a script fetch rejects with a TypeError, a module import fails, a script source records a script error, an engine fetch fails with fixed text naming no URL; the browser app's favicon and link navigation drop it silently.
basis: fct-ytg535zb, fct-pesxy5mo, fct-x4w72td2
because: The browser app now refuses a non-file: document's file: favicon and link-click navigation silently, raising no typed forbidden error and adding no log site, so the rule's «refused as a typed forbidden error» no longer covers every refusal.
