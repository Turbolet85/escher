@answer n=1 entity=test-data-fixtures#3.24 v=no
why: the `.test` TLD test URLs are browser history and persistence fixtures in apps/browser, a crate the chunk does not reach
@answer n=2 entity=test-data-fixtures#3.25 v=no
why: the in-test favicon PNG belongs to apps/browser/src/favicon.rs, a crate the chunk does not reach
@answer n=3 entity=test-data-fixtures#3.26 v=no
why: the fetch body belongs to blitz-vibey-script's DOM tests, and the chunk runs no script engine
@answer n=4 entity=test-data-fixtures#3.27 v=yes
why: the served task boots through stand::boot_load with the LoadNet stand-in (task item 2), so its fixed URL, fixed bytes and answer delay bind; the rule that every other stand boot keeps net_provider None binds the Counter and Crud boots the probe makes
@answer n=5 entity=test-data-fixtures#3.28 v=no
why: the render_blocking_stylesheet answer cases are read by that check alone; the chunk reads no render-blocking path and the load page's stylesheet comes from LoadNet
@answer n=6 entity=test-data-fixtures#3.29 v=no
why: the loopback TcpListener fixtures are blitz-net unit tests on 127.0.0.1, a crate the chunk does not reach; the stand's network is the in-process LoadNet
@answer n=7 entity=test-data-fixtures#4.1 v=no
why: an absence record of fixture files or loaders in slice s08 that states no value, boot or rule the probe reads; the probe's pages come from the stand's own boots, not from a fixture file
