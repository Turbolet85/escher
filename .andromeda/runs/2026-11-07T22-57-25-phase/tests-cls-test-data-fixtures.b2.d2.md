@answer n=1 entity=test-data-fixtures#3.24 v=no
why: browser entry helpers in apps/browser; the chunk touches no browser crate
@answer n=2 entity=test-data-fixtures#3.25 v=no
why: the .test TLD test URLs belong to apps/browser tests, which the chunk does not reach
@answer n=3 entity=test-data-fixtures#3.26 v=no
why: the favicon bytes belong to apps/browser favicon tests, which the chunk does not reach
@answer n=4 entity=test-data-fixtures#3.27 v=no
why: the fetch body belongs to blitz-vibey-script's DOM tests, which the chunk does not reach
@answer n=5 entity=test-data-fixtures#3.28 v=yes
why: the stand boot_load is named in the scope and its signals move onto the registry, so its net_provider values (LoadNet's fixed answer on boot_load, None on every other stand boot) must still hold, and the stand checks that read the load page must stay green in both layout modes
@answer n=6 entity=test-data-fixtures#3.29 v=no
why: the render-blocking stylesheet answers are blitz-net load-delivery cases, which the chunk does not reach
@answer n=7 entity=test-data-fixtures#3.30 v=no
why: the loopback listeners are blitz-net unit-test fixtures for load delivery, which the chunk does not reach
@answer n=8 entity=test-data-fixtures#4.1 v=no
why: the absence is a search over slice s08's 16 files, none of which the chunk reaches; the chunk's fixtures/mod.rs rule is carried by its own task item
