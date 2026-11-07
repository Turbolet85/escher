@answer n=1 entity=input-validation#1.126 v=no
why: inline SVG serialization output; the chunk changes the render path, not the serializer, and the scope names no serialization check
@answer n=2 entity=input-validation#1.127 v=no
why: a rule over the test harness's selector parsing and scoped queries; the scope names no query helper and the buffered render parses no selector
@answer n=3 entity=input-validation#1.128 v=no
why: WPT runner timeout-quarantine list; the WPT runner is not on the Dioxus bridge (task item 12)
@answer n=4 entity=input-validation#1.129 v=no
why: WPT fuzzy-range parsing; the WPT runner is not on the Dioxus bridge render path
@answer n=5 entity=input-validation#1.130 v=no
why: WPT checkLayout attribute and selector parsing; the WPT runner is not on the Dioxus bridge render path
@answer n=6 entity=input-validation#1.131 v=no
why: WPT wrapper HTML escaping; the WPT runner is not on the Dioxus bridge render path
@answer n=7 entity=input-validation#1.132 v=no
why: WPT script-type classification; the Dioxus bridge runs no document scripts (task item 6)
@answer n=8 entity=input-validation#1.133 v=no
why: data: URL decoding in the WPT net provider; the buffered render fetches nothing (task item 5)
@answer n=9 entity=input-validation#1.134 v=no
why: WPT page-message JSON parsing; the WPT runner is not on the Dioxus bridge render path
@answer n=10 entity=input-validation#1.135 v=no
why: CI PR lookup in the wpt-post-results workflow; no workflow is edited (task item 10)
@answer n=11 entity=input-validation#1.136 v=no
why: comment splice in the wpt_diff_to_pr.py CI script; the chunk does not reach that script
@answer n=12 entity=input-validation#1.137 v=no
why: 7GUIs example validation (flight booker, cells tokenizer, circle drawer, mode select); examples are not reached, the scope names no example, and the closed match runs on DOM input events the port does not change
@answer n=13 entity=input-validation#1.138 v=no
why: cells-grid keyboard handling and focus; the Enter-focus runs in an event handler outside the render path, which the port leaves in the same mounted-event order, and no example is edited
@answer n=14 entity=input-validation#1.139 v=no
why: widget color parsing and TodoMVC input trimming; examples are not reached by the chunk
@answer n=15 entity=input-validation#1.140 v=no
why: example fixture form inputs and maxlength; form paths are unchanged (task item 13) and the fixtures are not edited
