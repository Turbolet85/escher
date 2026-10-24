# Classified — security · input-validation.b5

## Answers
input-validation#1.90 · no · registerProperty's argument checks sit in blitz-vibey-script, and the stand runs no document script (Not needed: script execution)
input-validation#1.91 · no · setProperty and cssText sit in blitz-vibey-script, and the stand runs no document script (Not needed: script execution)
input-validation#1.92 · no · the DOM node-argument checks are blitz-vibey-script's script API, which the stand never runs
input-validation#1.93 · no · the geometry.js conversions are blitz-vibey-script's, which the stand never runs
input-validation#1.94 · no · document.getSelection is blitz-vibey-script's script API, which the stand never runs
input-validation#1.95 · no · sqlite statement binding is browser-app persistence, which the chunk does not reach (Not needed: browser app persistence)
input-validation#1.96 · no · stored-URL re-parsing is browser-app persistence, which the chunk does not reach
input-validation#1.97 · no · the serializer's unescaped text in style and script is engine output that no chunk row serializes, and no engine code is edited
input-validation#1.98 · no · the inline SVG round trip lives in inline_svg_serialize.rs, a test file the chunk does not reach, and no chunk row serializes markup
input-validation#1.99 · yes · the test-harness rule binds new test code, which parses selectors as a Result and reads stale NodeIds through get/contains_key, since the harness query helper panics on an invalid selector
input-validation#1.100 · no · WPT timeout-quarantine entries belong to the WPT runner, which the chunk does not reach (Not needed: the WPT runner)
input-validation#1.101 · no · WPT fuzzy ranges belong to the WPT runner, which the chunk does not reach
input-validation#1.102 · no · WPT checkLayout attribute parsing belongs to the WPT runner, which the chunk does not reach
input-validation#1.103 · no · WPT wrapper HTML belongs to the WPT runner, which the chunk does not reach
input-validation#1.104 · no · WPT script-type filtering belongs to the WPT runner, which the chunk does not reach
input-validation#1.105 · no · WPT data: URL decoding is the WPT runner's net provider, which the chunk does not reach (Not needed: remote content fetching)
input-validation#1.106 · no · WPT page-message parsing belongs to the WPT runner, which the chunk does not reach
input-validation#1.107 · no · the wpt-post-results PR lookup is a workflow the chunk does not reach, and its CI edit moves only the a11y leg's test list and its pin
input-validation#1.108 · no · the comment splice is in wpt_diff_to_pr.py, which the chunk does not reach, and its CI edit moves only ci-leg.sh's a11y tuple and test_ci_workflows.py's pin
input-validation#1.109 · yes · the flight booker's date checks set the invalid readings that the stand checks read on that task, and the new select row and the re-asserted Tab and contrast rows touch it
input-validation#1.110 · no · the widget's color parse and the TodoMVC input trim are example code outside the stand, which the chunk does not reach
input-validation#1.111 · no · the graphite and google fixture inputs are example assets that no stand row or chunk test reads
