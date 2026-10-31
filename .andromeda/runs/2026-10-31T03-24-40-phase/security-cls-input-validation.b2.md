# Classified — security · input-validation.b2

## Answers
input-validation#1.26 · no · script fetch bytes sit in blitz-vibey-script, which the task's Not-needed list excludes
input-validation#1.27 · no · `file:` gating for script fetch, `fetch()` and imports sits in blitz-vibey-script, which the debts do not reach
input-validation#1.28 · no · script `type` gating sits in blitz-vibey-script, which the debts do not reach
input-validation#1.29 · no · the scripting flag shapes `<noscript>` parsing; the scope names no scripting or noscript path, and the chunk's code consumes whatever DOM the parse yields
input-validation#1.30 · no · fetched-stylesheet UTF-8 intake is net intake, which the Not-needed list excludes and no debt changes
input-validation#1.31 · no · iframe HTML decoding is net intake, which no debt changes
input-validation#1.32 · no · image format decoding is net intake, which no debt changes
input-validation#1.33 · no · favicon acceptance sits in apps/browser, outside the six debts and the three checks
input-validation#1.34 · no · font format sniffing is net intake; the stand measures its bundled font, not a sniffed one
input-validation#1.35 · no · font length and WOFF fallback are net intake, which no debt changes
input-validation#1.36 · no · `@font-face` source URLs are net intake, and escher routes no supplied URL through them
input-validation#1.37 · no · SVG data encoding and DTD handling are net intake, and `node/svg.rs` is not reached
input-validation#1.38 · no · SVGZ decompression is SVG-data net intake, which no debt changes
input-validation#1.39 · no · `viewBox` numbers rule over an SVG unit that `visible_region` does not read
input-validation#1.40 · no · the inline SVG parse and its log sit in `layout/construct.rs`, which the chunk does not reach
input-validation#1.41 · yes · the authored `transform` reaches `visible_region` only as a declaration this parse accepts, invalid ones ignored; the scope names that transform and the re-pinned translated-box value reads it
input-validation#1.42 · no · `set_style_property` is a programmatic setter the scope does not name; the authored transform arrives through 1.41
input-validation#1.43 · no · the `selectorText` setter is a CSSOM editing path the scope does not name
input-validation#1.44 · no · query-string parse errors reach only callers that pass a query, and the scope names no query call
input-validation#1.45 · no · `<style>` entity decoding happens before CSS parsing, so the parser's value domain is unchanged; the scope names no entity-coded CSS
input-validation#1.46 · no · `bgcolor` is a legacy background path; the contrast reader takes `color` and no `style`
input-validation#1.47 · no · the dimension-attribute parse, with its nonzero table and cell variant, is a layout parse the scope does not name
input-validation#1.48 · no · SVG `width`/`height` parsing is an SVG path the chunk does not reach
input-validation#1.49 · no · the `border` and body-margin parses are layout attributes the scope does not name
input-validation#1.50 · no · the numeric markup attributes are layout parses; the chunk's readers (`inert`, `checked`, `open`, `multiple`) are not numeric parses
