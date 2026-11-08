@answer n=1 entity=input-validation#1.126 v=no
why: inline SVG serialization is the blitz-dom serializer, which the UI graph does not touch
@answer n=2 entity=input-validation#1.127 v=no
why: the test harness selector parsing is not reached; the graph reads the vdom and adds no selector queries
@answer n=3 entity=input-validation#1.128 v=no
why: WPT timeout-quarantine entries are the WPT runner, which the chunk does not reach
@answer n=4 entity=input-validation#1.129 v=no
why: WPT fuzzy-range parsing is the WPT runner, which the chunk does not reach
@answer n=5 entity=input-validation#1.130 v=no
why: WPT checkLayout attribute parsing is the WPT runner, which the chunk does not reach
@answer n=6 entity=input-validation#1.131 v=no
why: WPT wrapper HTML escaping is the WPT runner, which the chunk does not reach
@answer n=7 entity=input-validation#1.132 v=no
why: WPT script-type classification is the WPT runner, which the chunk does not reach
@answer n=8 entity=input-validation#1.133 v=no
why: WPT data: URL processing is the WPT net provider, which the chunk does not reach
@answer n=9 entity=input-validation#1.134 v=no
why: WPT page-message parsing is the WPT runner, which the chunk does not reach
@answer n=10 entity=input-validation#1.135 v=no
why: CI PR lookup in the wpt-post-results workflow is CI, which the chunk does not touch
@answer n=11 entity=input-validation#1.136 v=no
why: the CI comment splice script is CI, which the chunk does not touch
@answer n=12 entity=input-validation#1.137 v=yes
why: the seven_guis stand's signals move onto the registry, and the flight-type select's closed match writes the flight_type signal this rule rules over
@answer n=13 entity=input-validation#1.138 v=yes
why: the cells-grid keydown handler writes the current signal the chunk moves onto the registry, so its closed key match is handler and state the graph reads
@answer n=14 entity=input-validation#1.139 v=no
why: the widget color parse and the TodoMVC input trim sit in examples the chunk does not touch
@answer n=15 entity=input-validation#1.140 v=no
why: the graphite and google fixture form attributes are fixture HTML the chunk does not touch, and the masking rule covers only password and file inputs
