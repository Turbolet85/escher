@answer n=1 entity=input-validation#1.126 v=no
why: the chunk changes no engine parsing or serialization, and inline SVG serialization is not the graph's snapshot text
@answer n=2 entity=input-validation#1.127 v=no
why: the scope names no selector query or harness helper that the chunk runs
@answer n=3 entity=input-validation#1.128 v=no
why: the WPT runner is outside the chunk and the timeout-quarantine list is not read by the graph
@answer n=4 entity=input-validation#1.129 v=no
why: the WPT fuzzy-range parsing sits in the WPT runner, which the chunk does not reach
@answer n=5 entity=input-validation#1.130 v=no
why: the WPT checkLayout attribute handling sits in the WPT runner, which the chunk does not reach
@answer n=6 entity=input-validation#1.131 v=no
why: the WPT wrapper HTML escaping sits in the WPT runner, which the chunk does not reach
@answer n=7 entity=input-validation#1.132 v=no
why: the WPT script type rule sits in the WPT runner, and the graph runs no document scripts
@answer n=8 entity=input-validation#1.133 v=no
why: the data URL decoding sits in the WPT net provider, which the chunk does not reach and the graph fetches nothing
@answer n=9 entity=input-validation#1.134 v=no
why: the WPT page message parsing sits in the WPT runner, which the chunk does not reach
@answer n=10 entity=input-validation#1.135 v=no
why: the PR lookup is a CI script, and CI scripts are outside the chunk
@answer n=11 entity=input-validation#1.136 v=no
why: the comment splice is a CI script, and CI scripts are outside the chunk
@answer n=12 entity=input-validation#1.137 v=yes
why: the chunk moves the seven_guis signals onto the registry, and the flight booker mode select's closed match writes the flight_type signal
@answer n=13 entity=input-validation#1.138 v=yes
why: the chunk moves the seven_guis signals onto the registry, and the cells-grid key handler writes the current and editor state the graph reads
@answer n=14 entity=input-validation#1.139 v=no
why: the widget color parse and the TodoMVC input trim sit in examples the chunk does not touch
@answer n=15 entity=input-validation#1.140 v=no
why: the graphite and google fixture form rules are fixture-page checks the chunk does not reach
