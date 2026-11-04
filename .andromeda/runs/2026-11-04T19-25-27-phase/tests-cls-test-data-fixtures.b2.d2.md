@answer n=1 entity=test-data-fixtures#3.24 v=no
why: the `.test` URL row is apps/browser's fixture, and the chunk reaches no browser crate
@answer n=2 entity=test-data-fixtures#3.25 v=no
why: the favicon PNG bytes belong to apps/browser's favicon test, which the chunk does not reach
@answer n=3 entity=test-data-fixtures#3.26 v=no
why: the fetch body belongs to blitz-vibey-script's dom tests, which the probe does not reach
@answer n=4 entity=test-data-fixtures#3.27 v=yes
why: the probe's served reading boots the load page through boot_load, whose LoadNet stand-in answers every fetch with the fixed bytes after a delay, so the probe must meet that answer on the virtual clock without sleeping
@answer n=5 entity=test-data-fixtures#3.28 v=no
why: the render-blocking stylesheet cases are that check's own answers, and the chunk does not reach render_blocking_stylesheet
@answer n=6 entity=test-data-fixtures#3.29 v=no
why: the loopback listeners sit in blitz-net's unit tests, a crate the chunk does not reach
@answer n=7 entity=test-data-fixtures#4.1 v=no
why: the absence search covers slice s08 fixture loaders, a slice the chunk's scope does not name, and it states no rule the probe's code must meet
