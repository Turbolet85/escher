@answer n=1 entity=input-validation#1.126 v=no
why: inline SVG serialization output rule in blitz-tests; the probe serializes no SVG and the chunk changes no serializer.
@answer n=2 entity=input-validation#1.127 v=yes
why: the harness query helper panics with "invalid selector" on an unparseable selector, which the chunk's needs name, so the probe must pass only valid fixed selectors.
@answer n=3 entity=input-validation#1.128 v=no
why: WPT runner timeout-quarantine checks; the WPT runner is not_needed and not reached by the task files.
@answer n=4 entity=input-validation#1.129 v=no
why: WPT runner fuzzy-range parsing; the WPT runner is not_needed and not reached by the task files.
@answer n=5 entity=input-validation#1.130 v=no
why: WPT runner checkLayout attribute parsing and selector panic; the WPT runner is not_needed and not reached.
@answer n=6 entity=input-validation#1.131 v=no
why: WPT runner wrapper HTML escaping; the WPT runner is not_needed and not reached by the task files.
@answer n=7 entity=input-validation#1.132 v=no
why: WPT runner script-type classification; script execution is not_needed and the WPT runner is not reached.
@answer n=8 entity=input-validation#1.133 v=no
why: WPT runner data: URL decoding in the net provider; the net provider's per-input checks are not_needed and the probe fetches nothing remote.
@answer n=9 entity=input-validation#1.134 v=no
why: WPT runner page-message JSON parsing; the WPT runner is not_needed and not reached by the task files.
@answer n=10 entity=input-validation#1.135 v=no
why: CI PR lookup in the wpt-post-results workflow; that workflow is not_needed and not reached.
@answer n=11 entity=input-validation#1.136 v=no
why: CI comment splice in wpt_diff_to_pr.py; the wpt-post-results workflow is not_needed and not reached.
@answer n=12 entity=input-validation#1.137 v=yes
why: the scope leaves the served task open, so the flight booker's flight_type select closed match over its two option values binds as a fork the handler-to-state read must pass; its date and cells rules sit in the reached examples/seven_guis tree.
@answer n=13 entity=input-validation#1.138 v=yes
why: the cells-grid onkeydown key handler in the reached examples/seven_guis tree, with its Enter-to-editor and focus return, is the key-handler shape the probe's handler reads and the MountedData key-handler hazard concern.
@answer n=14 entity=input-validation#1.139 v=no
why: the custom widget's color parse and the TodoMVC trim sit in upstream examples and a preact page outside the reached seven_guis tree, and the probe runs Dioxus apps only.
@answer n=15 entity=input-validation#1.140 v=no
why: graphite and google fixture form attributes; fixture pages are not reached and their own checks are not_needed.
