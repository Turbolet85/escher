@leaf_group group=masters
@leaf_read leaf=.claude/docs/gotchas.md lines=68
last: - Decisions and rationale: `doc:architecture` (the records s
@leaf_read leaf=CLAUDE.md lines=166
last: <!-- USER:session-learnings end -->
@leaf_edit n=1 leaf=.claude/docs/gotchas.md line=27 class=omission
leaf-text: drops focus to the root element, the tree's focus naming the window.
source: dec-3p3pgzo2 «A focused node that becomes inert gives focus up at the flush, in every document, with no DOM blur or focusout; disabled and hidden take no part»
replace-with: drops focus to the root element, the tree's focus naming the window; a focused node made inert gives focus up at the flush with no blur or focusout, while one made disabled or hidden keeps focus and takes keys as the tree's focus names the window (a named SC 2.4.3 gap).
@leaf_edit n=2 leaf=CLAUDE.md line=57 class=omission
leaf-text: `error` · `password` print `[redacted]`
source: dec-4zd5yain «Under an escher_ target the sink prints every field except url, href, src, html, text, value, attrs, path, request, error, password and panic.payload, which are redacted»
replace-with: `error` · `password` · `panic.payload` print `[redacted]`
@leaf_edit n=3 leaf=CLAUDE.md line=58 class=stale
leaf-text: (`examples/wasm_hello` aside — a standalone workspace)
source: dec-opt3h4ax «Every cargo leg of the CI leg script runs with `--locked`. · except: `fmt`, and `examples/wasm_hello`, which has no `Cargo.lock`»
replace-with: (`fmt` and `examples/wasm_hello` aside — the latter a standalone workspace with no `Cargo.lock`)
