@answer n=1 entity=input-validation#1.51 v=no
why: `<style>` text entity decoding is an engine parse the chunk neither edits nor reads through.
@answer n=2 entity=input-validation#1.52 v=no
why: legacy `bgcolor` parsing is engine value validation the chunk does not reach.
@answer n=3 entity=input-validation#1.53 v=no
why: legacy dimension parsing in Stylo is engine value validation the chunk does not reach.
@answer n=4 entity=input-validation#1.54 v=no
why: SVG width and height parsing is engine value validation the chunk does not reach.
@answer n=5 entity=input-validation#1.55 v=no
why: `border` and body margin integer parsing is an engine value rule outside the read.
@answer n=6 entity=input-validation#1.56 v=no
why: markup numeric attribute fallbacks are layout values the read never reads.
@answer n=7 entity=input-validation#1.57 v=no
why: `rowspan` clamping is a table-layout value rule outside the read.
@answer n=8 entity=input-validation#1.58 v=no
why: a multiple select's `size` hint is a layout value the read never reads, and its engine change is not the chunk's.
@answer n=9 entity=input-validation#1.59 v=no
why: `<ol start>` parsing is a list-numbering value rule outside the read.
@answer n=10 entity=input-validation#1.60 v=no
why: the `attr_parsed` helper is one the chunk does not call, and its disabled clause is carried by entity 61.
@answer n=11 entity=input-validation#1.61 v=yes
why: the read's `disabled` refusal and the snapshot's clickable and enabled readings must follow this presence rule through action_point and is_disabled.
@answer n=12 entity=input-validation#1.62 v=no
why: `open` on details and dialog sets the `:open` state, which the read reports no verb or refusal from.
@answer n=13 entity=input-validation#1.63 v=no
why: autofocus on insert is a focus rule the read neither runs nor reports.
@answer n=14 entity=input-validation#1.64 v=no
why: `dir` case-insensitive matching is a layout direction rule the read does not read.
@answer n=15 entity=input-validation#1.65 v=no
why: canvas `src` u64 parsing belongs to canvas, which stays in 0.4.0.
@answer n=16 entity=input-validation#1.66 v=no
why: custom widget attribute replay sits behind a feature the chunk does not touch and is the widget's own check.
@answer n=17 entity=input-validation#1.67 v=yes
why: the read reports each node's stable id under the author-key and path rule, and a call names that id back, so this grammar binds the read's ids.
@answer n=18 entity=input-validation#1.68 v=yes
why: the read reports stable ids, and this rule says no served id may hold `///`, which the read's ids must keep.
@answer n=19 entity=input-validation#1.69 v=yes
why: snapshot.rs, reached whole, and the readers' obligation to hold no log site or panic path on app input bind the rule that no name is logged and no lookup panics.
@answer n=20 entity=input-validation#1.70 v=yes
why: the new verb and its carrier must hold no description, and this rule names the carriers that may not carry one.
@answer n=21 entity=input-validation#1.71 v=yes
why: snapshot.rs, reached whole, reads `enabled` from the `disabled` attribute by presence alone, which the read's disabled cause and the clickable reading must follow, not `aria-disabled`.
@answer n=22 entity=input-validation#1.72 v=no
why: a select's `open` and `aria-expanded` steering nothing concerns select popup states the read does not report.
@answer n=23 entity=input-validation#1.73 v=yes
why: the snapshot and graph states `selected`, `pressed`, `checked` and `invalid` sit in snapshot.rs and graph.rs, reached whole, and the read's carriers may hold only these closed words and booleans.
@answer n=24 entity=input-validation#1.74 v=no
why: the native checkbox checked-state rule lives in accessibility.rs, which the chunk does not reach, and its snapshot effect is carried by entity 73.
@answer n=25 entity=input-validation#1.75 v=yes
why: an inert node is not focusable and yields no hit, so the read's covered and accepted-now answers for an inert target, and unkeyed_actionable's skip, must follow it.
