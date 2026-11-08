@answer n=1 entity=standard-contracts#1.1 v=yes mode=stale
why: DioxusDocument in packages/dioxus-native-dom/src/dioxus_document.rs, a reached file gaining the graph method, carries the cited accessibility_tree override range 279-354, which the added lines can move
@answer n=2 entity=standard-contracts#2.1 v=no
why: the document clock sits in blitz-dom config, clock, iframe and lib files that the session does not edit
@answer n=3 entity=standard-contracts#2.2 v=no
why: select_value is a blitz-dom reader in an engine file the session does not edit, and the snapshot adds only the clickable field, not its value read
@answer n=4 entity=standard-contracts#2.3 v=no
why: the select list state and its readers are blitz-dom document.rs members, and the session edits no blitz-dom file
@answer n=5 entity=standard-contracts#2.4 v=no
why: live_select_list and its reads sit in blitz-dom document.rs and pointer.rs, which the session does not edit
@answer n=6 entity=standard-contracts#2.5 v=no
why: the reconcile call in resolve.rs and the field clear in document.rs are blitz-dom code the session does not edit
@answer n=7 entity=standard-contracts#2.6 v=yes mode=stale
why: closed_select_of, its snapshot callers and the is_select and is_multiple_select re-exports sit in reached dioxus-native-dom snapshot.rs and lib.rs, whose cited line ranges 254-264 and 45-48 the new clickable reading and re-exports can move
@answer n=8 entity=standard-contracts#2.7 v=no
why: the multiple-select write record is a blitz-dom mutator path the session does not edit
@answer n=9 entity=standard-contracts#2.8 v=no
why: NetHandler::error is a blitz-dom net.rs trait method that the session does not edit
@answer n=10 entity=standard-contracts#3.1 v=no
why: the mutation, query, CSSOM and accessibility-state surface is blitz-dom behaviour in unedited engine files, and the session adds no AccessKit state or stand accessibility test
@answer n=11 entity=standard-contracts#3.2 v=no
why: the single-select expanded write is a blitz-dom accessibility.rs behaviour the session does not edit
@answer n=12 entity=standard-contracts#3.3 v=no
why: the tree focus write is blitz-dom accessibility.rs behaviour the session does not change
@answer n=13 entity=standard-contracts#3.4 v=no
why: the native control toggled write is blitz-dom accessibility.rs behaviour the session does not change
@answer n=14 entity=standard-contracts#3.5 v=no
why: the resolved width and height reads are in blitz-dom resolved_style.rs, which the session does not edit
@answer n=15 entity=standard-contracts#3.6 v=no
why: restyle on mutation is blitz-dom mutator.rs behaviour, and the session does not edit that file
@answer n=16 entity=standard-contracts#4.1 v=no
why: the engine event driver and its default actions are blitz-dom and blitz-traits behaviour the session does not edit, and the bridge dispatch wrap leaves the converter and the handler order unchanged
@answer n=17 entity=standard-contracts#4.2 v=no
why: the select list default actions are blitz-dom keyboard.rs and pointer.rs behaviour the session does not edit
@answer n=18 entity=standard-contracts#4.3 v=no
why: the label arm focusing a bound select is blitz-dom pointer.rs behaviour the session does not edit
@answer n=19 entity=standard-contracts#4.4 v=no
why: the checkable change dispatch is blitz-dom pointer.rs behaviour the session does not edit
@answer n=20 entity=standard-contracts#5.1 v=no
why: the scroll API, scroll_into_view, visible_region and hit walk are blitz-dom scrolling.rs and node code the session does not edit, and the cited tests are not in the session's reached files
@answer n=21 entity=standard-contracts#6.1 v=no
why: the client rect under transforms is blitz-dom document.rs behaviour the session does not edit, and its named readers keep reading it unchanged
@answer n=22 entity=standard-contracts#6.2 v=no
why: visible_region under transforms is blitz-dom scrolling.rs behaviour the session does not edit, and its driver callers in_view and crop are not moved by the added graph verb
@answer n=23 entity=standard-contracts#6.3 v=no
why: text_extent is a blitz-dom reader in text_extent.rs, which the session does not edit, and its one caller Session::lint is unchanged
@answer n=24 entity=standard-contracts#6.4 v=no
why: the hit walk skip of dropped nodes is blitz-dom node and paint-tree code the session does not edit
@answer n=25 entity=standard-contracts#6.5 v=no
why: the non-invertible transform hit rule is blitz-dom node.rs behaviour the session does not edit
