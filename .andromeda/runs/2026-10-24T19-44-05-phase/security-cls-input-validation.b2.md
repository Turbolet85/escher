# Classified — security · input-validation.b2

## Answers
input-validation#1.26 · no · script fetch is script execution, which the chunk does not reach since the stand runs no document script
input-validation#1.27 · no · the script type gate is script execution, which the chunk does not reach since the stand runs no document script
input-validation#1.28 · no · the scripting flag shapes only script and noscript markup, the stand runs no document script, so no chunk line depends on it
input-validation#1.29 · no · fetched stylesheets are remote content, outside the chunk's needs since the stand boots offline
input-validation#1.30 · no · iframe HTML decoding is remote content, outside the chunk's needs since the stand boots offline
input-validation#1.31 · no · fetched image decoding is remote content, outside the chunk's needs since the stand boots offline
input-validation#1.32 · no · the favicon is apps/browser code, which the chunk does not reach
input-validation#1.33 · no · @font-face format sniffing is remote-font handling, outside the chunk's needs since the stand boots on its bundled font
input-validation#1.34 · no · font length and WOFF fallback are remote-font handling, outside the chunk's needs since the stand boots on its bundled font
input-validation#1.35 · no · an unresolved @font-face source is remote-font handling, and the chunk's checks route no URL through it
input-validation#1.36 · no · SVG data handling is outside the chunk's needs, which the task's Not needed list names
input-validation#1.37 · no · SVGZ decompression is outside the chunk's needs, which the task's Not needed list names
input-validation#1.38 · no · the SVG viewBox check is outside the chunk's needs, which the task's Not needed list names
input-validation#1.39 · no · inline SVG parsing binds no chunk line, since no Need names SVG and the chunk edits no markup
input-validation#1.40 · no · invalid CSS declarations are dropped by the resolver, and the chunk edits no stylesheet and reads computed values only
input-validation#1.41 · no · the setter's rejection binds no chunk line, since the chunk's tests set no style property
input-validation#1.42 · no · the selectorText setter edits CSSOM rules, which the chunk's tests do not call
input-validation#1.43 · yes · parse errors come back as a Result, which new test queries must take in place of the harness helper's panic on an invalid selector
input-validation#1.44 · no · style-element entity decoding binds no chunk line, since the chunk edits no stylesheet text
input-validation#1.45 · no · legacy bgcolor is outside the chunk's Needs, which name no presentational colour, and the chunk edits no markup
input-validation#1.46 · no · legacy dimension attributes are outside the chunk's Needs, which name none, and the chunk edits no markup
input-validation#1.47 · no · SVG width and height are outside the chunk's needs, which the task's Not needed list names
input-validation#1.48 · no · border and body-margin attributes are outside the chunk's Needs, which name neither, and the chunk edits no markup
input-validation#1.49 · no · the numeric attributes named here are outside the chunk's Needs, which name none of them, and the chunk edits no markup
input-validation#1.50 · no · the rowspan clamp is outside the chunk's Needs, which name no table attribute, and the chunk edits no markup
