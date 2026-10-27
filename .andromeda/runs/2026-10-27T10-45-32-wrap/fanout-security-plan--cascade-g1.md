proposals:
  - detector: D-cascade
    severity: warning
    file: .andromeda/security-plan-detail/threat-model-summary.md
    near: 18
    section: "## Threat Model Summary"
    change: "In the script-execution vector's Trust boundary line, after the UnsupportedScheme sentence, insert: A `file:` URL loads only for a `file:` document — `document_may_load(document_url, url)` refuses it for any other document with a `FetchError`, checked by JS `fetch()`, module `import` and `<script src>` before anything is read, a narrowing of the boundary, not a widening (packages/blitz-vibey-script/src/fetch.rs:40-46); the rest of the line (the `file:` read, `isTrusted` true, same-origin and CORS observed absent) unchanged."
    sidecar: "threat-model-summary — the script-execution trust boundary gains the document check: a `file:` URL loads only for a `file:` document (`document_may_load`, upstream `cee6b8731`, taken on the founder's word), checked by fetch(), import and <script src>; a narrowing."
    rationale: "report Changes › Symbols / APIs (blitz-vibey-script `document_may_load`, fetch.rs:40-46, checked by runtime/fetch.rs 40-44, runtime/modules.rs and document.rs:163-169) and Counts `fetcher` (threat-model-summary.md:18 changed, the trust boundary; a narrowing, same-origin and CORS still absent); Decisions & corrections records `cee6b8731` as a narrowing."
    basis: .andromeda/security-plan-detail/threat-model-summary.md:18
note: "Body security-plan.md:13 (the script-execution broad line) still holds — the report's `fetcher` row lists it no-change, the fetcher still taking only `file:` and `data:`; F1 (the document check as an Input Validation rule) and F3 (the `open` presence read) are input-validation facts for the new-detail pass, not this group's."
