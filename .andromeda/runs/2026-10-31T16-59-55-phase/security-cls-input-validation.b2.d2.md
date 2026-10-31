# Classified — security · input-validation.b2

## Answers
input-validation#1.26 · no · the script fetch's bytes rule; the list fetches nothing and adds no script path
input-validation#1.27 · no · the file: URL gate over fetch, module import and script src; the list fetches nothing and adds no script path
input-validation#1.28 · no · the script type attribute; the list reads and runs no script
input-validation#1.29 · no · the scripting flag of HTML parsing; the chunk changes no parse
input-validation#1.30 · no · fetched stylesheet encoding; the chunk fetches no stylesheet
input-validation#1.31 · no · fetched iframe HTML decoding; no iframe content is read
input-validation#1.32 · no · fetched image format; the list paints option text, no image
input-validation#1.33 · no · favicon bytes; outside the select
input-validation#1.34 · no · fetched font format sniffing; the list paints from the document already parsed, no fetched font
input-validation#1.35 · no · font length and compression; no fetched font enters the list
input-validation#1.36 · no · @font-face source URL; no URL enters the list and no driver verb takes one
input-validation#1.37 · no · SVG data encoding; no SVG is read
input-validation#1.38 · no · SVG compression; no SVG is read
input-validation#1.39 · no · SVG viewBox; no SVG is read
input-validation#1.40 · no · inline SVG parse; no SVG is read
input-validation#1.41 · yes · the new default colour declarations in assets/default.css must pass the CSS parsing rule, invalid ones ignored
input-validation#1.42 · no · set_style_property and remove_style_property; the scope names no call of theirs
input-validation#1.43 · no · selectorText setter; no rule is written
input-validation#1.44 · no · query-string parsing; the driver resolves an option by its stable element id, not by a selector
input-validation#1.45 · no · style element text entities; no style element is read
input-validation#1.46 · no · legacy bgcolor attribute; the list's colours are engine defaults
input-validation#1.47 · no · legacy dimension attributes; the chunk reads no table or cell width
input-validation#1.48 · no · SVG width and height; no SVG is read
input-validation#1.49 · no · border and body margin; the chunk reads neither
input-validation#1.50 · yes · the chunk's new read of select `size` falls under the markup numeric rule: a parse with a default fallback
</content>
