# Classified — security · input-validation.b2

## Answers
input-validation#1.26 · no · the script fetch's InvalidData path belongs to blitz-vibey-script, which the Not needed list keeps outside the chunk
input-validation#1.27 · no · the file: URL check on script fetch is blitz-vibey-script's document_may_load, outside the chunk under Not needed
input-validation#1.28 · no · the script type gate in blitz-vibey-script is script runtime, which the chunk does not reach
input-validation#1.29 · no · the scripting flag is blitz-html's parse setting, a unit the chunk does not edit, and the scope names no script-driven markup
input-validation#1.30 · no · the UTF-8 check on fetched stylesheets is net-provider intake, which the Not needed list says no debt changes
input-validation#1.31 · no · the lossy decode of fetched iframe HTML is net-provider intake, which no debt changes
input-validation#1.32 · no · image format sniffing is net-provider intake, which no debt changes
input-validation#1.33 · no · favicon byte checks sit in apps/browser, outside the chunk under Not needed
input-validation#1.34 · no · font format sniffing is fetched-font intake, which no debt changes, and the bundled font is not a fetched source
input-validation#1.35 · no · font length and WOFF fallback are fetched-font intake, which no debt changes
input-validation#1.36 · yes · the stand's offline boot on its bundled font, its one in-process load.css answer and the absence of any URL-taking driver verb are what the contrast and load-fetch rows run on, so the chunk must keep them; the @font-face intake rule itself is unchanged
input-validation#1.37 · no · SVG DTD and encoding rules are svg.rs intake, which the Not needed list keeps unchanged
input-validation#1.38 · no · SVGZ detection is svg.rs intake, which no debt changes
input-validation#1.39 · no · the viewBox number rule is svg.rs's SVG value check, not a unit the chunk edits, and the degenerate rule for visible_region is the chunk's own
input-validation#1.40 · no · inline SVG parsing and its logged parse error sit in blitz-dom's layout/construct.rs, which the chunk does not edit
input-validation#1.41 · yes · authored transform declarations that visible_region reads, and the colour declarations the contrast rows read, pass this validation and invalid ones are dropped before the changed geometry sees them, so the degenerate and non-finite tests must author values it accepts
input-validation#1.42 · no · set_style_property's rejection is a CSSOM write the scope does not name, and the transform the chunk reads arrives as authored CSS
input-validation#1.43 · no · the selectorText setter is a CSSOM write the scope does not name, and the new :open check is a check of what already runs, not of a rule written through a setter
input-validation#1.44 · no · query-string parsing in query_selector.rs is a path the scope does not name, and the scope names no selector argument
input-validation#1.45 · no · HTML entity decoding in style text runs before the declaration parse, and the scope names no style text path the chunk changes or reads
input-validation#1.46 · no · the bgcolor mapping is a stylo.rs rule the scope does not name, and the scope names no legacy colour attribute that a contrast row reads
input-validation#1.47 · no · the HTML dimension parser for table and cell sizes is a stylo.rs rule the scope does not name, and no re-read check reads a table or cell dimension
input-validation#1.48 · no · SVG width and height legacy parsing is a stylo.rs rule the scope does not name
input-validation#1.49 · no · border and body margin parsing is a stylo.rs rule the scope does not name, and the scope names neither attribute
input-validation#1.50 · no · the numeric markup attribute fallbacks sit in layout code the chunk does not edit, and the scope names no numeric attribute as the carrier of a value its geometry reads
