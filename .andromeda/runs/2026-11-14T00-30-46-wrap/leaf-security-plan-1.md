@leaf_group group=security-plan-1
@leaf_read leaf=.claude/docs/security-summary.md lines=57
last: **Full plan:** `.andromeda/security-plan.md`.
@leaf_edit n=1 leaf=.claude/docs/security-summary.md line=14 class=omission
leaf-text: and a `file:` URL loads only for a `file:` document — `fetch()`, a module `import` and `<script src>` each refused otherwise with a `FetchError`
source: dec-gsml2o7x «A file: URL loads only for a file: document or a top-level load; for any other document, remote pages included, JS fetch, module import and script src refuse it with a FetchError»
replace-with: and a `file:` URL loads only for a `file:` document or a top-level load — `fetch()`, a module `import` and `<script src>` each refused otherwise with a `FetchError`
@leaf_edit n=2 leaf=.claude/docs/security-summary.md line=15 class=stale
leaf-text: blitz-net reads any `file:` path with no restriction — through `Url::to_file_path` off wasm, a non-path URL a typed `InvalidInput` error;
source: dec-ik2uwgla «every engine stylesheet, @import, font, image or iframe load, may read a file: URL only for a file: document or a top-level load, in every document, remote pages included; blitz-net refuses any other before a disk read.»
replace-with: blitz-net reads a `file:` URL only for a `file:` document or a top-level load, refusing any other stylesheet, `@import`, font, image or iframe load before a disk read with `FileForbidden` (fixed text «file: load refused for a non-file document», no URL or path, no log site of its own) — through `Url::to_file_path` off wasm, a non-path URL a typed `InvalidInput` error;
