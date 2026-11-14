@row_set n=1 row=decision/file-urls-only-for-file-documents cell=rule seen=3 fact=F1 detector=none
value: A `file:` URL is loaded only for a `file:` document: otherwise a script's `fetch()`, `import` or `<script src>` is refused with a `FetchError`, a blitz-net sub-resource load reads no local file, and the browser app drops a `file:` favicon or link navigation silently; user loads are not gated.
basis: fct-ytg535zb
because: The browser app's favicon candidate and its navigation provider now refuse a `file:` URL for a non-`file:` document, silently, while the user's own loads stay ungated, so the row's rule, which names only script and engine sub-resource loads, must say the app's two gates too.
@row_none n=1 fact=F2 type=4
relied: A provider that needs a Dioxus runtime is exercised inside a virtual DOM run and read out through a thread-local after the rebuild; the code and the report's test notes hold it, no other part relies on it.
@row_none n=2 fact=F3 type=7
what: One call site in the browser app's document loader reached by no test, an absence inside a single file; the report's stated limit and the code hold it.
