@answer n=1 entity=input-validation#1.51 v=no
why: SVG width and height parsing is blitz-dom engine code the replay only passes values to, and no re-run check names it
@answer n=2 entity=input-validation#1.52 v=no
why: border and body margin unsigned parsing is engine-side and unchanged by the chunk, with no named witness
@answer n=3 entity=input-validation#1.53 v=no
why: markup numeric fallbacks sit in layout code the chunk does not change, and no re-run check reads them
@answer n=4 entity=input-validation#1.54 v=no
why: the rowspan clamp is table layout the chunk does not touch
@answer n=5 entity=input-validation#1.55 v=no
why: the multiple select size hint is select_list engine behaviour, not reached and not a re-run witness
@answer n=6 entity=input-validation#1.56 v=no
why: the ol start parse is engine construction code the chunk does not change
@answer n=7 entity=input-validation#1.57 v=no
why: attr_parsed is a tabindex helper the chunk never calls, and the disabled reading it mentions is carried by 1.58
@answer n=8 entity=input-validation#1.58 v=yes
why: the replay must keep removing falsy disabled rather than writing it, and the stand_keyboard and snapshot enabled re-runs witness the presence reading
@answer n=9 entity=input-validation#1.59 v=yes
why: open is read by presence, so a falsy open the replay writes must still take the removal path through clear_attribute, which the set and clear calls carry
@answer n=10 entity=input-validation#1.60 v=no
why: autofocus ignores the value "false" by its own rule, so the writer's falsy handling does not turn it and no re-run check names it
@answer n=11 entity=input-validation#1.61 v=no
why: dir matching is engine-side attribute reading the chunk does not change, with no witness
@answer n=12 entity=input-validation#1.62 v=no
why: the canvas src u64 parse is engine code outside the chunk's reach
@answer n=13 entity=input-validation#1.63 v=no
why: custom-widget attribute replay sits behind a feature the Dioxus bridge does not turn on in this chunk
@answer n=14 entity=input-validation#1.64 v=yes
why: the stable element id is a value task item 7 keeps identical, the buffer holds element ids and stand_element_ids re-runs
@answer n=15 entity=input-validation#1.65 v=yes
why: the label and labelledby names are the snapshot name field that task item 7 keeps identical in the snapshot text and its diff
@answer n=16 entity=input-validation#1.66 v=no
why: aria-describedby and aria-description reach only the platform adapter, which the chunk does not change, and the snapshot reads no description
@answer n=17 entity=input-validation#1.67 v=no
why: aria-expanded, aria-disabled and aria-level feed only the platform tree, and their one snapshot line restates the presence reading of 1.58
@answer n=18 entity=input-validation#1.68 v=no
why: a select's open and aria-expanded steer nothing in select_popup, which is not reached and not a re-run witness
@answer n=19 entity=input-validation#1.69 v=yes
why: the snapshot selected, pressed, checked and invalid words derive from these attributes, and the re-run snapshot checks must keep them identical under task item 7
@answer n=20 entity=input-validation#1.70 v=yes
why: the native checked state the snapshot reports is set by the replayed checked writes and removals, so their order must hold and the snapshot checks witness it
@answer n=21 entity=input-validation#1.71 v=yes
why: inert is read by presence, so a falsy value the replay writes would take effect, and the scope leaves open whether the writer removes it, a fork the planner must cover
@answer n=22 entity=input-validation#1.72 v=no
why: the select list gate is engine select behaviour the chunk does not reach and no re-run check witnesses
@answer n=23 entity=input-validation#1.73 v=no
why: the tree focus value and the inert focus clear sit on the platform tree and blur events the chunk does not reach, and the clear is carried by 1.71
@answer n=24 entity=input-validation#1.74 v=yes
why: range value writes go through the re-seed arm of set_attribute that the replay calls, and NodeState.value is a snapshot value task item 7 keeps identical
@answer n=25 entity=input-validation#1.75 v=yes
why: Enter and Space activation dispatch a Click, a behaviour the re-run stand_keyboard check witnesses, so the chunk re-asserts it as its own work
