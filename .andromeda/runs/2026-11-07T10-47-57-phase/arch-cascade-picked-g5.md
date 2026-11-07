@account n=1 entity=standard-contracts#21.1 v=carried
why: its stand readings (68 of 77 clicks, the render-busy span line, the covered readings) and closed sets are what the re-run stand_* checks and the nothing-crosses boundary re-assert
@account n=2 entity=standard-contracts#22.3 v=carried
why: the lint reads the DioxusDocument snapshot and stand_lint, a re-run stand_* witness, pins its readings in both layout modes
@account n=3 entity=standard-contracts#22.4 v=carried
why: the load wait settles through Harness::settle over the render path the chunk changes, an input of the stand_load_page reading the chunk re-runs
@account n=4 entity=standard-contracts#23.1 v=carried
why: stand_lint's at_boot overlap pin reads the snapshot's client rects, a value the re-run stand_* checks re-assert
@account n=5 entity=standard-contracts#23.2 v=carried
why: stand_load_page, a re-run stand_* witness, reads a Dioxus re-render after a load in both layout modes
