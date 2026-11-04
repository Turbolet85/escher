@answer n=1 entity=input-validation#1.126 v=no
why: inline SVG serialization well-formedness is a serializer rule that the probe neither serializes nor reads
@answer n=2 entity=input-validation#1.127 v=yes
why: the probe calls the harness query helpers, which panic on an invalid selector (task item 8), so it must pass only valid fixed selectors
@answer n=3 entity=input-validation#1.128 v=no
why: WPT runner timeout-quarantine entries; the WPT runner is not needed by this chunk (not_needed item 5)
@answer n=4 entity=input-validation#1.129 v=no
why: WPT runner fuzzy range parsing; the WPT runner is not needed by this chunk (not_needed item 5)
@answer n=5 entity=input-validation#1.130 v=no
why: WPT runner checkLayout attribute parsing and its selector panic; the WPT runner is not needed by this chunk (not_needed item 5)
@answer n=6 entity=input-validation#1.131 v=no
why: WPT runner wrapper HTML escaping; the WPT runner is not needed by this chunk (not_needed item 5)
@answer n=7 entity=input-validation#1.132 v=no
why: WPT runner script type classification; script execution is not needed by this chunk (not_needed item 3)
@answer n=8 entity=input-validation#1.133 v=no
why: WPT net provider data URL decoding; the net provider per-input checks are not needed by this chunk (not_needed item 2)
@answer n=9 entity=input-validation#1.134 v=no
why: WPT harness page message JSON parsing; the WPT runner is not needed by this chunk (not_needed item 5)
@answer n=10 entity=input-validation#1.135 v=no
why: CI PR lookup in the wpt-post-results workflow; the upstream-only workflows are not needed by this chunk (not_needed item 4)
@answer n=11 entity=input-validation#1.136 v=no
why: CI comment splice in wpt_diff_to_pr.py; the wpt-post-results workflow is not needed by this chunk (not_needed item 4)
@answer n=12 entity=input-validation#1.137 v=yes
why: examples/seven_guis is a reached path reached whole, and the flight booker select handler writing flight_type is the handler-to-state link the probe reads
@answer n=13 entity=input-validation#1.138 v=yes
why: the cells-grid keydown and click early-return handlers sit in examples/seven_guis, reached whole, and are the node-to-handler and click-listener reading the probe measures
@answer n=14 entity=input-validation#1.139 v=no
why: the widget color parse in examples/custom_widget.rs and the TodoMVC trim in examples/preact are neither reached nor read by the probe
@answer n=15 entity=input-validation#1.140 v=no
why: graphite and google fixture form attributes; the fixture pages are not needed by this chunk (not_needed item 11) and are not reached
