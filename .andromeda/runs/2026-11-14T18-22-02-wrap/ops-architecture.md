@row_set n=1 row=decision/file-url-load-rule cell=rule seen=2 fact=F1 detector=none
value: A `file:` URL loads only for a document whose URL is `file:`, or as the user's own top-level load: script fetch, imports and sources read a refusal as Forbidden, the browser drops a page's navigation silently, and blitz-net refuses any other engine request with FileForbidden before any disk read.
basis: fct-ytg535zb; fct-pesxy5mo
because: The browser app's navigation provider now drops, loading and logging nothing, a navigation a non-file document starts to a file: URL, so such a navigation is no top-level load and the rule must say so.
@row_set n=2 row=decision/sub-resource-requests-carry-initiator cell=rule seen=1 detector=none
value: Every engine sub-resource request (linked stylesheet, its reload, @import, @font-face, CSS image, img, iframe) carries its loading document's URL as initiator; a request with no initiator is a top-level load an embedder or the user's own load asked for, or a navigation its page may start.
basis: fct-ytg535zb; fct-pesxy5mo
because: A navigation a non-file page starts to a file: URL no longer becomes a top-level load in the browser app, so «a navigation asked for» must read as the user's own loads and the navigations a page is allowed.
@row_set n=3 row=decision/justified-expect-unwrap cell=except seen=1 detector=none
value: The browser app's favicon test module allows `clippy::expect_used` with no justification beside it.
basis: fct-qb4ltnez
because: The browser app now holds a second test module whose allow stands beside a justification, so the exception must name the favicon test module alone.
@row_add n=1 row=decision/browser-favicon-file-gate area=standard-contracts fact=F2 detector=none
name: Browser favicons name no local file for remote pages
rule: The browser app keeps a page's favicon candidate only where the page's document may load it, so a non-`file:` page names no `file:` favicon and neither the favicon probe nor the favicon cache is reached for one; a `file:` page's `file:` favicon and a remote page's own /favicon.ico stand.
basis: fct-ytg535zb; fct-bmifaxwg
because: The chunk gates the browser's favicon candidate by document_may_load, a decision about which local files a remote page may reach that no standing row states.
