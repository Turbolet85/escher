@row_set n=1 row=decision/file-urls-only-from-file-documents cell=rule seen=2 fact=F1 detector=D-security-input
value: A file: URL is read only for a file: document or a top-level load: a script fetch, module import or script src, every engine stylesheet, @import, font, image or iframe load, and in the browser app a favicon and a link navigation; blitz-net refuses before a disk read, a navigation drops silently.
basis: fct-ytg535zb, fct-pesxy5mo
because: The browser app now refuses a non-file document's file: favicon candidate and its file: link navigation by the same rule, so the row's list of gated loads must name them.
@row_set n=2 row=decision/file-url-only-for-file-documents cell=rule seen=2 fact=F2 detector=D-security-input
value: A file: URL loads only for a file: document or a top-level load; for any other document, remote pages included, script fetch, module import and script src fail, blitz-net refuses engine loads before a disk read, and the browser app names no file: favicon and drops a file: navigation silently.
basis: fct-ytg535zb, fct-bmifaxwg, fct-bhe3bvls
because: A remote page can no longer have a local file probed as its favicon or reach one by a link navigation in the browser app, so the threat row's reach of the control is stale without them.
@row_add n=1 row=decision/file-gate-covers-document-started-loads area=input-validation fact=F3 detector=D-security-input
name: File gate covers document-started loads
rule: The file: gate covers the loads a document starts: its sub-resources, its favicon and its link navigations.
except: In the browser app the user's own loads, from the URL bar, history, home, the about pages and the first tab, are top-level loads and pass through no gate.
basis: fct-ytg535zb
because: The chunk gated the browser app's document-started loads and left the user's own loads ungated, a convention with its exception no standing row states.
