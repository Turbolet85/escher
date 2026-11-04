@answer n=1 entity=input-validation#1.76 v=no
why: a checkbox and radio click's change event; the scope names no checkbox or radio, and the probe's reads are listener-map and element-id reads
@answer n=2 entity=input-validation#1.77 v=no
why: a single select's choice, keys, typeahead and click; the scope names no select, and the probe drives no select's choice
@answer n=3 entity=input-validation#1.78 v=no
why: the select list's document field and its readers; the probe reads no select list
@answer n=4 entity=input-validation#1.79 v=no
why: an open select list's keys; the scope names no open list and no key on one
@answer n=5 entity=input-validation#1.80 v=no
why: a pointer in and around an open select list; the scope names no open list and no pointer on one
@answer n=6 entity=input-validation#1.81 v=no
why: hostile markup under an open select list; the scope names no select list and no hostile select markup
@answer n=7 entity=input-validation#1.82 v=no
why: a multiple select's keys and clicks; the scope names no multiple select
@answer n=8 entity=input-validation#1.83 v=no
why: optgroup and its label within a select; the scope names no optgroup
@answer n=9 entity=input-validation#1.84 v=no
why: a multiple select's chosen set as event values; the re-read of dioxus_document.rs is its poll mechanism, which this rule does not rule over, and the scope names no multiple select
@answer n=10 entity=input-validation#1.85 v=no
why: the driver's type verb on a select; the probe types into no select and the scope names no select typeahead
@answer n=11 entity=input-validation#1.86 v=no
why: the public select readers and closed_select_of; the probe calls none of them
@answer n=12 entity=input-validation#1.87 v=no
why: the select list's public readers and close writer; the probe calls none of them and reads no select list label
@answer n=13 entity=input-validation#1.88 v=yes
why: the password and file input value mask as the snapshot, diff, driver and MCP carriers show it; the scope requires a password's or file input's value to stay MASKED_VALUE on every carrier the probe touches
@answer n=14 entity=input-validation#1.89 v=no
why: aspect-ratio coercion in replaced-element layout; the four link readings read no box geometry
@answer n=15 entity=input-validation#1.90 v=no
why: inline-box height clamping in line layout; the four link readings read no box geometry
@answer n=16 entity=input-validation#1.91 v=no
why: text_extent and its lint caller; the probe calls no lint and reads no text extent
@answer n=17 entity=input-validation#1.92 v=no
why: visible_region and its in_view and crop callers; the scope names no clip region or in-view check
@answer n=18 entity=input-validation#1.93 v=no
why: culling of non-finite geometry before painting; the probe paints nothing and reads no frame
@answer n=19 entity=input-validation#1.94 v=no
why: the select list's box and rows in paint; the probe reads no select list and no frame
@answer n=20 entity=input-validation#1.95 v=no
why: hit testing under a non-invertible transform; the scope names no transform on the probe's served task
@answer n=21 entity=input-validation#1.96 v=no
why: gradient length clamping in paint; the probe reads no gradient
@answer n=22 entity=input-validation#1.97 v=no
why: border-radius scaling in paint; the probe reads no border radius
@answer n=23 entity=input-validation#1.98 v=no
why: hidden inputs left unpainted; the scope names no hidden input and the probe reads no paint
@answer n=24 entity=input-validation#1.99 v=no
why: scroll-target clamping and missing-node scrolls; the scope names no scroll call
@answer n=25 entity=input-validation#1.100 v=no
why: an open select list's own scroll; the scope names no open list and no scroll of one
