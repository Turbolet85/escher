@row_set n=1 row=decision/no-file-loads-from-non-file-documents cell=rule seen=1 fact=F1 detector=D-obs-pii
value: A file: load asked by a non-file: document is refused as a typed forbidden error: a script fetch rejects with a TypeError, a module import fails, a script element's source is recorded as a script error, and an engine sub-resource fetch fails with fixed text naming no URL or path, adding no log site.
basis: fct-z4iggkni, fct-vvhzkhrr, fct-nfuvmyp2, fct-oaky27yp
because: blitz-net now refuses a file: sub-resource fetch for a non-file: document with the fixed-text FileForbidden provider error through the handler's error path, so the rule that named script loads alone must name the engine's sub-resource refusal too.
@row_none n=1 fact=F2 type=9
what: the favicon probe and file: navigation from a remote page are open items owed to a provisional route entry; the route holds them.
