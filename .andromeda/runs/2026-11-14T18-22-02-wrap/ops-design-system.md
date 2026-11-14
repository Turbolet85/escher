@row_set n=1 row=decision/favicon-from-first-icon-link cell=except seen=1 fact=F1 detector=none
value: A page that is not itself a file document drops a candidate naming a local file; its default /favicon.ico and a file page's local favicon are kept.
basis: fct-ytg535zb
because: The favicon candidate is now kept only where a document may load it, so a remote page's first icon link naming a file: URL no longer becomes its favicon, an exception the row must state.
