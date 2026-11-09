@answer n=1 entity=input-validation#1.76 v=no
why: its cited files are blitz-dom accessibility code and select_popup tests, none reached by the session
@answer n=2 entity=input-validation#1.77 v=yes mode=stale
why: its citation json.rs:160-166 sits in escher-driver json.rs, a reached file whose lines may have moved
@answer n=3 entity=input-validation#1.78 v=no
why: its rule lives in blitz-dom accessibility.rs, which the session does not reach
@answer n=4 entity=input-validation#1.79 v=no
why: its rule lives in blitz-dom node, mutator and accessibility files the session does not reach
@answer n=5 entity=input-validation#1.80 v=no
why: its rule lives in blitz-dom select and document files the session does not reach
@answer n=6 entity=input-validation#1.81 v=no
why: its rule lives in blitz-dom accessibility and mutator files the session does not reach
@answer n=7 entity=input-validation#1.82 v=no
why: its range rule lives in blitz-dom files the session does not reach, and the coverage line carries no control value
@answer n=8 entity=input-validation#1.83 v=no
why: its keyboard and pointer default actions live in blitz-dom events the session does not reach, and the reached record changes no dispatch
@answer n=9 entity=input-validation#1.84 v=no
why: its change event lives in blitz-dom pointer.rs, which the session does not reach
@answer n=10 entity=input-validation#1.85 v=yes mode=stale
why: its citation dioxus_document.rs:421-426 is in a reached dioxus-native-dom file whose select event lines may have moved
@answer n=11 entity=input-validation#1.86 v=no
why: its select list field lives in blitz-dom document.rs, which the session does not reach
@answer n=12 entity=input-validation#1.87 v=no
why: its open-list key rule lives in blitz-dom keyboard.rs, which the session does not reach
@answer n=13 entity=input-validation#1.88 v=no
why: its pointer rule lives in blitz-dom pointer, document and resolve files the session does not reach
@answer n=14 entity=input-validation#1.89 v=no
why: its hostile-markup rule lives in blitz-dom select list code the session does not reach
@answer n=15 entity=input-validation#1.90 v=no
why: its multiple-select keys and clicks live in blitz-dom files the session does not reach
@answer n=16 entity=input-validation#1.91 v=no
why: its optgroup rule lives in blitz-dom select, form and mutator files the session does not reach
@answer n=17 entity=input-validation#1.92 v=yes mode=stale
why: its citation dioxus_document.rs:34-48 for multiple_select_values is in a reached file whose lines may have moved
@answer n=18 entity=input-validation#1.93 v=no
why: its driver type rule lives in escher-driver execute.rs and schema.rs, which the session does not reach, and no verb is added
@answer n=19 entity=input-validation#1.94 v=yes mode=stale
why: its citation dioxus-native-dom lib.rs:48-51 sits in a reached file whose re-export lines moved as the crate's re-exports grew
@answer n=20 entity=input-validation#1.95 v=no
why: its readers live in blitz-dom document.rs and lib.rs, which the session does not reach
@answer n=21 entity=input-validation#1.96 v=no
why: its mask rule lives in dioxus-native-dom snapshot files and execute.rs, none reached, and the coverage line holds no typed text
@answer n=22 entity=input-validation#1.97 v=no
why: its aspect-ratio rule lives in blitz-dom layout code the session does not reach
@answer n=23 entity=input-validation#1.98 v=no
why: its inline-box rule lives in blitz-dom layout code the session does not reach
@answer n=24 entity=input-validation#1.99 v=no
why: its text-extent reader lives in blitz-dom text_extent.rs, which the session does not reach
@answer n=25 entity=input-validation#1.100 v=no
why: its visible-region reader lives in blitz-dom scrolling and document files the session does not reach
