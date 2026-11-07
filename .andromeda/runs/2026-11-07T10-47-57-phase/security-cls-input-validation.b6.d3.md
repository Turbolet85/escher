@answer n=1 entity=input-validation#1.126 v=no
why: inline SVG serialization is blitz-dom output the chunk neither edits nor re-runs, and no re-run check covers it.
@answer n=2 entity=input-validation#1.127 v=no
why: test-harness selector parsing and scoped queries are harness behaviour the chunk does not edit or re-run.
@answer n=3 entity=input-validation#1.128 v=no
why: WPT timeout-quarantine list validation is WPT runner input, which the Dioxus bridge does not reach.
@answer n=4 entity=input-validation#1.129 v=no
why: WPT fuzzy range parsing is WPT runner input, which the Dioxus bridge does not reach.
@answer n=5 entity=input-validation#1.130 v=no
why: WPT checkLayout attribute parsing is WPT runner input, which the Dioxus bridge does not reach.
@answer n=6 entity=input-validation#1.131 v=no
why: WPT wrapper HTML escaping is WPT runner code the chunk does not edit or reach.
@answer n=7 entity=input-validation#1.132 v=no
why: WPT script type classification is WPT runner code, and the Dioxus bridge runs no document scripts.
@answer n=8 entity=input-validation#1.133 v=no
why: WPT data URL decoding is net provider code, and the buffered render fetches nothing.
@answer n=9 entity=input-validation#1.134 v=no
why: WPT page message JSON parsing is WPT runner code the chunk does not edit or reach.
@answer n=10 entity=input-validation#1.135 v=no
why: the wpt-post-results PR lookup is CI workflow logic, and no workflow is edited.
@answer n=11 entity=input-validation#1.136 v=no
why: the CI comment splice marker check is in a CI script the chunk does not edit or reach.
@answer n=12 entity=input-validation#1.137 v=no
why: the 7GUIs example validations (flight booker, cells tokenizer, circle clamp) are example-app code the chunk neither edits nor uses.
@answer n=13 entity=input-validation#1.138 v=no
why: the cells-grid key handling is an example app's event-handler behaviour the scope does not name, and the render-time buffer does not alter it.
@answer n=14 entity=input-validation#1.139 v=no
why: the widget color parsing and TodoMVC trim are example-app code the chunk does not edit or reach.
@answer n=15 entity=input-validation#1.140 v=no
why: the graphite and google fixture form attributes are example fixtures, and no form path changes.
