# Classified — tests · test-data-fixtures.b2

## Answers
test-data-fixtures#3.24 · yes · the stand-in network answer (every fetch of `http://stand.test/load.css` returns `LOADED_CSS` after 200 ms, other stand boots keep `net_provider: None`) binds: `stand_keyboard` and `stand_contrast` read this load page, and the a11y leg re-runs both on the synced engine, so the fixture's behaviour must still hold there
test-data-fixtures#4.1 · no · the absence covers only the 16 files of slice s08 with no match for `fixture`; no reached file or re-read check sits in that slice, and the chunk adds no fixture file or loader, so no line of it is a rule the chunk's code or tests must honour
