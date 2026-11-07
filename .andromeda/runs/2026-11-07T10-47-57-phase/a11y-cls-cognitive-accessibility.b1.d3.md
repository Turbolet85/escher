@answer n=1 entity=cognitive-accessibility#1.1 v=no
why: the error and 404 page strings are static browser assets outside the DioxusDocument render path and no re-asserted check witnesses them
@answer n=2 entity=cognitive-accessibility#1.2 v=yes
why: the aria-invalid state and the flight button disabled attribute reach the tree and snapshot through the replayed render, and the Tab focus sequence and snapshot focus checks named in stand_flight_booker, stand_accessibility_ids and stand_snapshot are re-asserted by this chunk
@answer n=3 entity=cognitive-accessibility#2.1 v=no
why: the relative history time labels in browser_history.rs are not touched by the chunk and no re-asserted check names them
@answer n=4 entity=cognitive-accessibility#2.2 v=no
why: the tab hover tooltip is browser CSS and tab strip markup the chunk does not touch and no re-asserted check witnesses it
@answer n=5 entity=cognitive-accessibility#2.3 v=no
why: the status bar hover target and loading text in status_bar.rs are outside the chunk's render path and no re-asserted check names them
@answer n=6 entity=cognitive-accessibility#2.4 v=no
why: the seven_guis card descriptions and tags in app.rs are stand app content the chunk leaves unchanged and no named check witnesses them
@answer n=7 entity=cognitive-accessibility#2.5 v=yes
why: the load-done text and its display toggle after the Load click are a render-path value witnessed by the re-asserted stand_load_page check
