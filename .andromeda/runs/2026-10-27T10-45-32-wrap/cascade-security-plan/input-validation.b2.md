### input-validation#1.26 · values · input · ## Input Validation · lines 96–98
- **Boundary:** Script fetch
  - **What to validate:** Bytes
  - **How (stack-specific):** Invalid `data:` URLs or non-UTF-8 script bytes produce `FetchError::InvalidData` (packages/blitz-vibey-script/src/fetch.rs:60-67)
### input-validation#1.27 · values · input · ## Input Validation · lines 99–101
- **Boundary:** Script type
  - **What to validate:** `type` attribute
  - **How (stack-specific):** Only scripts with type empty, `text/javascript`, `application/javascript` or `module` are executed (packages/blitz-vibey-script/src/document.rs:307-316)
### input-validation#1.28 · values · input · ## Input Validation · lines 102–104
- **Boundary:** HTML parsing
  - **What to validate:** Scripting flag
  - **How (stack-specific):** HTML is parsed with scripting disabled (packages/blitz-html/src/html_sink.rs:109; packages/blitz-html/src/html_sink.rs:149)
### input-validation#1.29 · values · input · ## Input Validation · lines 105–107
- **Boundary:** Fetched stylesheets
  - **What to validate:** Encoding
  - **How (stack-specific):** Must be valid UTF-8, else the load responds `Err("Invalid UTF8")` (packages/blitz-dom/src/net.rs:145-147; packages/blitz-dom/src/net.rs:262-264)
### input-validation#1.30 · values · input · ## Input Validation · lines 108–110
- **Boundary:** Fetched iframe HTML
  - **What to validate:** Encoding
  - **How (stack-specific):** Decoded with `String::from_utf8_lossy` (packages/blitz-dom/src/net.rs:548-551)
### input-validation#1.31 · values · input · ## Input Validation · lines 111–113
- **Boundary:** Fetched images
  - **What to validate:** Format
  - **How (stack-specific):** Decoded by the `image` crate with guessed format, then an SVG parse fallback, else an error string (packages/blitz-dom/src/net.rs:571-606)
### input-validation#1.32 · values · input · ## Input Validation · lines 114–116
- **Boundary:** Favicon
  - **What to validate:** Image bytes
  - **How (stack-specific):** Must decode as a raster image or SVG to be accepted; tests reject HTML payloads, truncated PNGs and garbage (apps/browser/src/favicon.rs:53-68; apps/browser/src/favicon.rs:95-112)
### input-validation#1.33 · values · input · ## Input Validation · lines 117–119
- **Boundary:** Fetched fonts
  - **What to validate:** Format
  - **How (stack-specific):** Sniffed from the first four bytes (`wOFF`, `wOF2`, `OTTO`, `0x00010000`, `true`) when no format hint is given (packages/blitz-dom/src/net.rs:310-331); witnessed since 2026-10-14-quality-gates on a hint-less `data:` source — a `wOF2` body registered, a body no sniff matches registered as nothing (tests/blitz-tests/tests/font_face_sniffed_source.rs:50-99)
### input-validation#1.34 · values · input · ## Input Validation · lines 120–122
- **Boundary:** Fetched fonts
  - **What to validate:** Length and compression
  - **How (stack-specific):** Font bytes shorter than 4 bytes pass through, and failed WOFF decompression falls back to the original bytes (packages/blitz-dom/src/util.rs:17-39)
### input-validation#1.35 · values · input · ## Input Validation · lines 123–125
- **Boundary:** `@font-face` source
  - **What to validate:** URL
  - **How (stack-specific):** A source whose URL cannot be resolved is skipped instead of panicking (packages/blitz-dom/src/net.rs:494-500). A boundary widening that arrived with the 2026-10-10 upstream merge (upstream #1109), ratified by the founder (2026-10-10, by question dialog in the overseer session, relayed verbatim by the overseer as his chosen option label «Утвердить + проверка»; the English wording is the overseer's) as upstream's behaviour: a source with no format hint whose URL has no extension — a `data:` URL — was skipped and is now fetched, its format sniffed from its bytes as the Fetched fonts row states (packages/blitz-dom/src/net.rs:458-462); no test of ours and none in the merged delta covered it at the merge (searched: `font-face` or `font_face` over the delta's added lines, 0 hits); since 2026-10-14-quality-gates blitz-tests' `font_face_sniffed_source` does — offline, every request held by a hand provider, a font context holding no family, both layout modes: the hint-less `data:` source is requested exactly once and, answered with a `wOF2` body, registered (the text's width 0 before the answer, above 0 after, no further request), and a body no sniff matches registers nothing (tests/blitz-tests/tests/font_face_sniffed_source.rs:50-73; tests/blitz-tests/tests/font_face_sniffed_source.rs:75-99) — the check «Утвердить + проверка» asked for; escher routes no agent- or user-supplied URL through this path today — no driver verb takes a URL, and the stand boots on its bundled font, offline but for the load page (`stand::boot_load`), whose one fixed stylesheet request `http://stand.test/load.css` the in-process `LoadNet` answers — no `blitz_net`, no `@font-face` source and no agent- or user-supplied URL — as measured at escher-0.1.0/chunks/2026-10-10-upstream-sync-agent-surfaces/report.md
### input-validation#1.36 · values · input · ## Input Validation · lines 126–128
- **Boundary:** SVG data
  - **What to validate:** Encoding and DTD
  - **How (stack-specific):** Rejects non-UTF-8 input and parses XML with `allow_dtd: true` (packages/blitz-dom/src/node/svg.rs:102-108)
### input-validation#1.37 · values · input · ## Input Validation · lines 129–131
- **Boundary:** SVG data
  - **What to validate:** Compression
  - **How (stack-specific):** Gzip-compressed SVG (SVGZ) is detected by magic bytes and decompressed before parsing (packages/blitz-dom/src/node/svg.rs:93-100)
### input-validation#1.38 · values · input · ## Input Validation · lines 132–134
- **Boundary:** SVG `viewBox`
  - **What to validate:** Numbers
  - **How (stack-specific):** Must be exactly four finite non-negative numbers; a zero width or height is flagged as degenerate (packages/blitz-dom/src/node/svg.rs:39-53)
### input-validation#1.39 · values · input · ## Input Validation · lines 135–137
- **Boundary:** Inline SVG source
  - **What to validate:** Parse
  - **How (stack-specific):** Parsed with `parse_svg_image`; a parse error is logged and the element left without image data (packages/blitz-dom/src/layout/construct.rs:451-471)
### input-validation#1.40 · values · input · ## Input Validation · lines 138–140
- **Boundary:** CSS declarations
  - **What to validate:** Property/value pairs
  - **How (stack-specific):** Validated by parsing; invalid declarations are ignored per CSSOM (packages/blitz-dom/src/resolved_style.rs:162-182; packages/blitz-dom/src/resolved_style.rs:218-251; packages/blitz-dom/src/cssom.rs:716-757)
### input-validation#1.41 · values · input · ## Input Validation · lines 141–143
- **Boundary:** CSS declarations
  - **What to validate:** `set_style_property` / `remove_style_property`
  - **How (stack-specific):** Reject unsupported property names and invalid values by returning `false` (packages/blitz-dom/src/node/element.rs:807-823; packages/blitz-dom/src/node/element.rs:873-877)
### input-validation#1.42 · values · input · ## Input Validation · lines 144–146
- **Boundary:** Selectors
  - **What to validate:** `selectorText` setter
  - **How (stack-specific):** An invalid selector list leaves the rule unchanged (packages/blitz-dom/src/cssom.rs:858-915)
### input-validation#1.43 · values · input · ## Input Validation · lines 147–149
- **Boundary:** Selectors
  - **What to validate:** Query strings
  - **How (stack-specific):** Parsed with `parse_author_origin_no_namespace` and parse errors are returned (packages/blitz-dom/src/query_selector.rs:204-210)
### input-validation#1.44 · values · input · ## Input Validation · lines 150–152
- **Boundary:** `<style>` element
  - **What to validate:** Text
  - **How (stack-specific):** HTML entities decoded before parsing (packages/blitz-dom/src/document.rs:1297-1302)
### input-validation#1.45 · values · input · ## Input Validation · lines 153–155
- **Boundary:** Legacy attributes
  - **What to validate:** `bgcolor`
  - **How (stack-specific):** Accepted only as `#` followed by 3 or 6 hex digits; anything else is ignored (packages/blitz-dom/src/stylo.rs:898-919; packages/blitz-dom/src/stylo.rs:1186-1193)
### input-validation#1.46 · values · input · ## Input Validation · lines 156–158
- **Boundary:** Legacy attributes
  - **What to validate:** Dimensions
  - **How (stack-specific):** Go through Stylo's HTML dimension-value parser, with a nonzero variant for table width and cells (packages/blitz-dom/src/stylo.rs:921-950; packages/blitz-dom/src/stylo.rs:1060-1066)
### input-validation#1.47 · values · input · ## Input Validation · lines 159–161
- **Boundary:** Legacy attributes
  - **What to validate:** SVG `width` / `height`
  - **How (stack-specific):** Reject negative and unparseable values (packages/blitz-dom/src/stylo.rs:952-982)
### input-validation#1.48 · values · input · ## Input Validation · lines 162–164
- **Boundary:** Legacy attributes
  - **What to validate:** `border`, body margin
  - **How (stack-specific):** Parsed as unsigned integers (packages/blitz-dom/src/stylo.rs:1133; packages/blitz-dom/src/stylo.rs:1167)
### input-validation#1.49 · values · input · ## Input Validation · lines 165–167
- **Boundary:** Markup numeric attributes
  - **What to validate:** `rows`, `cols`, `width`/`height`, `<col span>`, `colspan`
  - **How (stack-specific):** `.parse().ok()` with default fallbacks: textarea `rows` (default 2) and `cols` (packages/blitz-dom/src/layout/mod.rs:280-287), replaced-element `width`/`height` (packages/blitz-dom/src/layout/mod.rs:384-391), `<col span>` floored at 1 (packages/blitz-dom/src/layout/table.rs:469-473), `colspan` default 1 (packages/blitz-dom/src/layout/table.rs:626-629)
### input-validation#1.50 · values · input · ## Input Validation · lines 168–170
- **Boundary:** Markup numeric attributes
  - **What to validate:** `rowspan`
  - **How (stack-specific):** Clamped to 1..=65534 (packages/blitz-dom/src/layout/table.rs:630-634)
