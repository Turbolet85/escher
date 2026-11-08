@answer n=1 entity=cognitive-accessibility#1.1 v=no
why: the error and 404 pages live in apps/browser assets, which the chunk neither edits nor reads
@answer n=2 entity=cognitive-accessibility#1.2 v=yes
why: the snapshot model reads the date inputs' aria-invalid state and the replayed refusal reads enabled Some(false), the value the Book button's disabled attribute carries
@answer n=3 entity=cognitive-accessibility#2.1 v=no
why: relative history time labels are browser_history.rs behaviour the chunk does not reach
@answer n=4 entity=cognitive-accessibility#2.2 v=no
why: tab tooltips are browser tab-strip behaviour the chunk does not reach
@answer n=5 entity=cognitive-accessibility#2.3 v=no
why: the status bar's hovered-link and loading text is browser behaviour the chunk does not reach
@answer n=6 entity=cognitive-accessibility#2.4 v=no
why: seven_guis card descriptions and tags are not a handler, component or event name the coverage result carries
@answer n=7 entity=cognitive-accessibility#2.5 v=no
why: the load page's delayed load-done text and display none timing are a stand_load_page check the chunk neither reads nor drives
