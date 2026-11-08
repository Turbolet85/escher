@answer n=1 entity=screen-reader-support#1.1 v=no
why: its text-run rule lives in packages/blitz-dom, which the session did not reach
@answer n=2 entity=screen-reader-support#1.2 v=no
why: the aria-label naming rule lives in packages/blitz-dom, which the session did not reach
@answer n=3 entity=screen-reader-support#1.3 v=no
why: the label-binding rule lives in packages/blitz-dom, which the session did not reach
@answer n=4 entity=screen-reader-support#1.4 v=no
why: the select tree and option reading live in unreached blitz-dom and snapshot code, and the session changes neither
@answer n=5 entity=screen-reader-support#1.5 v=no
why: the expanded reading lives in packages/blitz-dom, which the session did not reach
@answer n=6 entity=screen-reader-support#1.6 v=yes mode=stale
why: docs/capabilities.md is reached whole, so the cited lines 245-247 may have moved
@answer n=7 entity=screen-reader-support#1.7 v=no
why: the unmeasured assistive-technology reading is untouched by the session
@answer n=8 entity=screen-reader-support#1.8 v=no
why: the multiple-select bounds rule lives in unreached snapshot and blitz-dom code
@answer n=9 entity=screen-reader-support#1.9 v=no
why: evt.values reads multiple_select_values in unreached dioxus_document.rs, which the session did not change
@answer n=10 entity=screen-reader-support#2.1 v=yes mode=stale
why: tests/blitz-tests/tests/common/mod.rs is reached whole, so the cited table lines 17-140, 142-145 and 149-185 may have moved
@answer n=11 entity=screen-reader-support#2.2 v=yes mode=add-to
why: F4 joins this graph read assertion, the actions answer being a further read that changes nothing and holds no value or typed text
@answer n=12 entity=screen-reader-support#3.1 v=yes mode=add-to
why: F6 adds the flight-return-date disabled reading and the crud-person-14 off-screen reading to its refusal examples, and the actions read is one more reader of this model
@answer n=13 entity=screen-reader-support#3.2 v=no
why: clickable's own reading is unchanged, and the walk's exact reading gaining readers does not move what clickable reads
@answer n=14 entity=screen-reader-support#3.3 v=yes mode=add-to
why: F1 joins the graph readers, actionable_nodes being one more read-only reader that lists the graph's nodes in pre-order as ActionableNode with id, owner and keyed
@answer n=15 entity=screen-reader-support#3.4 v=no
why: NodeState keeps its fields and NODE_FIELDS stays fourteen, the session adding no node key
@answer n=16 entity=screen-reader-support#3.5 v=no
why: the open-list label reading is unchanged, and its citation sits in an unreached test
@answer n=17 entity=screen-reader-support#3.6 v=no
why: the native checked reading lives in packages/blitz-dom, which the session did not reach
@answer n=18 entity=screen-reader-support#4.1 v=yes mode=contradict
why: the listener leg now reads exact current listeners through the one Readings rule (F2) and no longer the data-dioxus-id mark, so the over-inclusion claim no longer holds
@answer n=19 entity=screen-reader-support#4.2 v=yes mode=add-to
why: F5 joins this per-task table with the ACTIONABLE_AT_BOOT counts, counter 2, flight-booker 7, timer 3, crud 11, cells 678 and load 2
@answer n=20 entity=screen-reader-support#4.3 v=no
why: the probe's link 2 reading in unreached dioxus_runtime_probe.rs is unchanged by the session
@answer n=21 entity=screen-reader-support#4.4 v=yes mode=add-to
why: F9 joins the per-element stand readings, Cells 461 of 678 cells reading off-screen for click, type and screenshot while all 678 take scroll
@answer n=22 entity=screen-reader-support#4.5 v=yes mode=add-to
why: F8 joins this listener account, a listener removed on a re-render being read by neither reader now that the walk's exact reading also feeds unkeyed_actionable and actionable_nodes
@answer n=23 entity=screen-reader-support#4.6 v=no
why: cells-grid's aria_label and absent role are unchanged by the session
@answer n=24 entity=screen-reader-support#4.7 v=no
why: the check still reads empty on the load page, and the report shows its claim holds on the exact listener leg
@answer n=25 entity=screen-reader-support#5.1 v=no
why: the attribute-set roles and states on the stand are unchanged, and the actionable check still returns nothing there
