# security-plan · picked accounting · group 1 of 3

## Picked
threat-model-summary#1.1 · not touched · its entry points and guards live in blitz-net, blitz-html, svg.rs, iframe.rs, net.rs and blitz-dom document.rs, none reached, and on the reached element.rs the report's change is `flush_open_state` (:488-489, :543-549) alone, so the SubDocument claim holds; the `open` presence read is F3, an input-validation fact
threat-model-summary#2.1 · proposed · its trust-boundary line (:18, the report's `fetcher` changed row) says the fetcher takes `file:` and `data:` with no document check; `cee6b8731` adds `document_may_load`, a `file:` load only for a `file:` document, checked by `fetch()`, `import` and `<script src>`
