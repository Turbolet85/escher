@leaf_group group=architecture-1
@leaf_read leaf=.claude/docs/conventions.md lines=61
last: - Architecture: `doc:architecture` (the records store) · dir
@leaf_read leaf=.claude/docs/services/blitz-dom.md lines=44
last: - `doc:architecture` (the records store) · `.claude/docs/sta
@leaf_read leaf=.claude/docs/services/blitz-paint.md lines=35
last: - `doc:architecture` (the records store) · `.claude/docs/ser
@leaf_read leaf=.claude/docs/services/blitz-shell.md lines=34
last: - `doc:architecture` (the records store) · `.claude/docs/ser
@leaf_read leaf=.claude/docs/services/blitz-traits.md lines=33
last: - `doc:architecture` (the records store) · `.claude/docs/ser
@leaf_edit n=1 leaf=.claude/docs/conventions.md line=57 class=omission
leaf-text: Android entry `#[unsafe(no_mangle)] pub fn android_main`.
source: dec-nqb3jdw6 «Android entry points are `android_main` calling `set_android_app`.»
replace-with: Android entry `#[unsafe(no_mangle)] pub fn android_main`, calling `set_android_app`.
@leaf_edit n=2 leaf=.claude/docs/conventions.md line=7 class=omission
leaf-text: Modules and files open with a `//!` doc; examples open with a `//!` comment saying what they demonstrate.
source: dec-n4rgyoyp «Modules, files and examples open with `//!` docs, examples describing what they demonstrate, and public items carry `///`.»
replace-with: Modules and files open with a `//!` doc; examples open with a `//!` comment saying what they demonstrate; public items carry `///` — upstream engine modules and examples largely open without it, and some upstream public items carry no `///`.
@leaf_edit n=3 leaf=.claude/docs/conventions.md line=17 class=omission
leaf-text: Dependencies declared once in `[workspace.dependencies]` and consumed as `{ workspace = true }`;
source: dec-nxphwkzn «Dependencies are declared once in `[workspace.dependencies]` and consumed as `{ workspace = true }`. · except: The WPT runner's direct `mimalloc` dependency.»
replace-with: Dependencies declared once in `[workspace.dependencies]` and consumed as `{ workspace = true }` (the WPT runner's direct `mimalloc` dependency excepted);
@leaf_edit n=4 leaf=.claude/docs/services/blitz-traits.md line=15 class=stale
leaf-text: clipboard, file dialog — no-op defaults)
source: ctr-vcjlevve «Every method defaults to a no-op, clipboard reads to an error and the file dialog to an empty list.»
replace-with: clipboard, file dialog — no-op defaults, clipboard reads defaulting to an error and the file dialog to an empty list)
@leaf_edit n=5 leaf=.claude/docs/services/blitz-dom.md line=12 class=omission
leaf-text: Taffy 0.15 (upstream's git source pinned to one revision,
source: dec-qxbkesvi «and is part of blitz-dom's public API through its signatures, with no re-export.»
replace-with: Taffy 0.15 (part of blitz-dom's public API through its signatures, with no re-export; upstream's git source pinned to one revision,
@leaf_edit n=6 leaf=.claude/docs/services/blitz-dom.md line=15 class=omission
leaf-text: every value sanitized to `[min, max]` and the nearest grid value)
source: dec-t3244gzx «every value sanitized to [min, max] and the nearest step, the greater on a tie.»
replace-with: every value sanitized to `[min, max]` and the nearest grid value, the greater on a tie)
