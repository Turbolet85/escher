@answer n=1 entity=test-data-fixtures#3.24 v=no
why: browser URL entry helpers in apps/browser; the chunk touches no browser crate
@answer n=2 entity=test-data-fixtures#3.25 v=no
why: the `.test` TLD of browser test URLs; the chunk touches no browser crate
@answer n=3 entity=test-data-fixtures#3.26 v=no
why: the favicon test bytes in apps/browser; the chunk touches no browser crate
@answer n=4 entity=test-data-fixtures#3.27 v=no
why: the blitz-vibey-script fetch body; that package's tests are outside the chunk's scope
@answer n=5 entity=test-data-fixtures#3.28 v=yes
why: stand::boot_load is a stand boot the chunk moves onto the registry, so its LoadNet answer and the rule that every other stand boot keeps net_provider None bind; the load page is read by stand_load_page, stand_keyboard and stand_contrast, a fixture page more than one check reads that joins fixtures/mod.rs
@answer n=6 entity=test-data-fixtures#3.29 v=no
why: render_blocking_stylesheet's cases are blitz-dom and blitz-net stylesheet loading, which the chunk does not change and no task item names
@answer n=7 entity=test-data-fixtures#3.30 v=no
why: blitz-net loopback listeners; blitz-net's delivery tests and stand-ins are not_needed and the chunk touches no blitz-net code
@answer n=8 entity=test-data-fixtures#4.1 v=no
why: the chunk's fixture join lands in fixtures/mod.rs, and the absence search over slice s08 names no file the scope reaches
