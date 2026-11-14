@leaf_group group=architecture+security-plan+test-plan+obs-plan
@leaf_read leaf=CLAUDE.md lines=163
last: <!-- USER:session-learnings end -->
@leaf_edit n=1 leaf=CLAUDE.md line=25 class=omission
leaf-text: HTTP through reqwest, filesystem, `data:` URIs
source: dec-ykas6pyc «decision/url-scheme-routing · Fetching by URL scheme — `data:` URLs are decoded locally, `file:` URLs are read from disk only for a top-level load or a `file:` loading document and are otherwise refused»
replace-with: HTTP through reqwest, filesystem (a `file:` URL only for a top-level load or a `file:` document, any other refused `FileForbidden` before a disk read), `data:` URIs
