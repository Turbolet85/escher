@answer n=1 entity=input-validation#1.126 v=no
why: the rule is the inline SVG markup serializer, which the chunk does not change; the snapshot text it edits is not that serializer and the scope names no SVG output
@answer n=2 entity=input-validation#1.127 v=no
why: the harness selector helper is test-harness code the graph read does not query by selector, and the scope does not name it
@answer n=3 entity=input-validation#1.128 v=no
why: a WPT runner quarantine-list rule, and the WPT runner is outside the graph's reach
@answer n=4 entity=input-validation#1.129 v=no
why: a WPT runner fuzzy-range rule, outside the graph's reach
@answer n=5 entity=input-validation#1.130 v=no
why: a WPT runner checkLayout rule, outside the graph's reach
@answer n=6 entity=input-validation#1.131 v=no
why: a WPT runner wrapper-HTML escaping rule, outside the graph's reach
@answer n=7 entity=input-validation#1.132 v=no
why: a WPT runner script-type rule, outside the graph's reach
@answer n=8 entity=input-validation#1.133 v=no
why: a WPT runner data: URL rule, and the graph fetches nothing so the net provider is not reached
@answer n=9 entity=input-validation#1.134 v=no
why: a WPT runner page-message rule, outside the graph's reach
@answer n=10 entity=input-validation#1.135 v=no
why: a CI workflow PR-lookup rule, and CI secrets and supply chain are not needed for this chunk
@answer n=11 entity=input-validation#1.136 v=no
why: a CI comment-splice rule in a helper script the chunk does not touch
@answer n=12 entity=input-validation#1.137 v=yes
why: the scope moves the seven_guis stand's signals onto the registry, and the flight booker's mode select writes its flight_type signal by the closed match this rule states
@answer n=13 entity=input-validation#1.138 v=yes
why: the cells-grid onkeydown handler and its editor write the stand's moved signals, and the graph reads that handler and the state it writes
@answer n=14 entity=input-validation#1.139 v=no
why: the custom widget's color parse and the TodoMVC input trim are example code the chunk neither edits nor registers
@answer n=15 entity=input-validation#1.140 v=no
why: the graphite and google fixture form inputs are fixture pages the chunk does not touch, and the masking covers password and file inputs only
