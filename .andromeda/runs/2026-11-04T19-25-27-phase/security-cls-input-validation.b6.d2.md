@answer n=1 entity=input-validation#1.126 v=no
why: inline SVG serialization output is not read or changed by the probe; the existing inline_svg_serialize test in the reached directory only keeps holding
@answer n=2 entity=input-validation#1.127 v=yes
why: the probe's fixed valid selectors go through the harness query helper, which panics on an invalid selector (needs item 8)
@answer n=3 entity=input-validation#1.128 v=no
why: WPT runner timeout-quarantine entries are outside the chunk's reads (not-needed item 5)
@answer n=4 entity=input-validation#1.129 v=no
why: WPT fuzzy range parsing is outside the chunk's reads (not-needed item 5)
@answer n=5 entity=input-validation#1.130 v=no
why: WPT checkLayout attribute and selector parsing is outside the chunk's reads (not-needed item 5)
@answer n=6 entity=input-validation#1.131 v=no
why: WPT wrapper HTML escaping is outside the chunk's reads (not-needed item 5)
@answer n=7 entity=input-validation#1.132 v=no
why: WPT script type classification belongs to script execution, which the probe does not run (not-needed item 3)
@answer n=8 entity=input-validation#1.133 v=no
why: WPT net provider data: URL handling is outside the chunk; the probe fetches nothing remote (not-needed items 2 and 5)
@answer n=9 entity=input-validation#1.134 v=no
why: WPT harness page-message JSON parsing is outside the chunk's reads (not-needed item 5)
@answer n=10 entity=input-validation#1.135 v=no
why: the CI PR lookup in the upstream wpt-post-results workflow is not reached and is not-needed (item 4)
@answer n=11 entity=input-validation#1.136 v=no
why: the CI comment splice script is not reached and is not-needed (item 4)
@answer n=12 entity=input-validation#1.137 v=yes
why: the flight booker's select#flight-type handler writes its flight_type signal by a closed match, a handler-to-state link (link 3) the probe may read in examples/seven_guis; the date, cell-token and circle-clamp rules do not bind
@answer n=13 entity=input-validation#1.138 v=yes
why: the cells-grid onkeydown listener and the click early return inside a cell's open editor are node-to-handler listeners (link 2) readable in examples/seven_guis, which the served task may be
@answer n=14 entity=input-validation#1.139 v=no
why: the widget color parse sits in an upstream example and the TodoMVC trim in the vendored Preact page, both outside the reached paths
@answer n=15 entity=input-validation#1.140 v=no
why: fixture form-input attributes in examples/assets are not reached and are not a masking or listener rule the probe needs
