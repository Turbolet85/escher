# Classified — security · input-validation.b2

## Answers
input-validation#1.26 · no · the stand runs no page scripts, so the script fetcher's byte rule is not reached
input-validation#1.27 · no · the stand runs no page scripts, so no script `type` attribute is executed
input-validation#1.28 · no · the stand parses no remote markup and runs no script, so the scripting flag reaches no chunk path
input-validation#1.29 · no · Not-needed remote-content row: the offline stand fetches no stylesheet
input-validation#1.30 · no · Not-needed remote-content row: the stand has no iframe or fetched sub-document
input-validation#1.31 · no · remote image decode the offline stand never fetches, and the load in flight names no image
input-validation#1.32 · no · Not-needed favicon row, and the favicon code sits in the browser app, which the chunk does not reach
input-validation#1.33 · no · Not-needed font row: the stand boots on its bundled font, with no fetched @font-face
input-validation#1.34 · no · Not-needed font row: no fetched font reaches the stand's offline boot
input-validation#1.35 · no · Not-needed @font-face row: no agent or user URL reaches this path, and the stand boots offline
input-validation#1.36 · no · Not-needed SVG row: the offline stand carries no SVG parse path
input-validation#1.37 · no · Not-needed SVG row: no gzip-compressed SVG reaches the stand
input-validation#1.38 · no · Not-needed SVG row: no SVG viewBox is parsed on the stand
input-validation#1.39 · no · Not-needed SVG row: the stand's offline markup carries no inline SVG
input-validation#1.40 · yes · the stand's styled screens and the re-pinned contrast rows read the colours this declaration parse keeps or drops
input-validation#1.41 · no · a CSSOM setter reached only by page script, which the stand never runs
input-validation#1.42 · no · a CSSOM rule setter reached only by page script, which the stand never runs
input-validation#1.43 · no · no call or screen supplies a selector string, so the query parser's error return is not reached
input-validation#1.44 · no · no reached file edits a stylesheet, and the `<style>` entity decode changes only entity-bearing text
input-validation#1.45 · no · Not-needed legacy-attribute row: the stand's markup sets no legacy colour attribute
input-validation#1.46 · no · Not-needed legacy-attribute row: the stand's markup sets no legacy dimension attribute
input-validation#1.47 · no · Not-needed SVG and legacy row: the stand carries no SVG element with width or height
input-validation#1.48 · no · Not-needed legacy-attribute row: no legacy border or body margin on the stand
input-validation#1.49 · no · the task names no rows, cols, span or size attribute in the chunk's markup, so only defaults are read
input-validation#1.50 · no · the task names no rowspan in the chunk's markup, so the clamp reads no authored value
