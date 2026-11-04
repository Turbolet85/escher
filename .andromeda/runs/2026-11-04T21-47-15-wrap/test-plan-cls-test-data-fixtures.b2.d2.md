@answer n=1 entity=test-data-fixtures#3.24 v=no
why: the browser files it cites (browser_history.rs, persistence lib.rs) are not among the session's reached files
@answer n=2 entity=test-data-fixtures#3.25 v=no
why: favicon.rs is outside the session's reached files and no favicon bytes changed
@answer n=3 entity=test-data-fixtures#3.26 v=no
why: the blitz-vibey-script tests it cites are outside the session's reached files
@answer n=4 entity=test-data-fixtures#3.27 v=no
why: its cited stand and load-page files are unchanged, and run stand stays at 237 over 51 files
@answer n=5 entity=test-data-fixtures#3.28 v=no
why: render_blocking_stylesheet.rs is not among the session's reached files
@answer n=6 entity=test-data-fixtures#3.29 v=no
why: blitz-net lib.rs is not among the session's reached files and the change adds no loopback listener
@answer n=7 entity=test-data-fixtures#4.1 v=yes mode=stale
why: the new blitz-tests target dioxus_runtime_probe.rs is a reached file the report does not place outside slice s08, so the 16-file count and the no-match search can move
