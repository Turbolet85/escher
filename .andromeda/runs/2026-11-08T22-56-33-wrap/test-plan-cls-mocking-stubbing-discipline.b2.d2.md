@answer n=1 entity=mocking-stubbing-discipline#1.26 v=no
why: the WPT diff script and its test sit under CI Python scripts and the WPT runner, which the session does not reach
@answer n=2 entity=mocking-stubbing-discipline#2.1 v=no
why: its document.rs hover tests sit in blitz-dom, a package the session does not reach
@answer n=3 entity=mocking-stubbing-discipline#3.1 v=no
why: the CI workflows, Cross and flake config, Cargo manifests and CI scripts it searches are all unchanged by the session
@answer n=4 entity=mocking-stubbing-discipline#3.2 v=yes mode=stale
why: the reached seven_guis example files sit among the apps it searches, and the report does not show no mock holds on them
@answer n=5 entity=mocking-stubbing-discipline#3.3 v=yes mode=stale
why: the new reached files (stand_handler_coverage, reached.rs, coverage.rs) may join slice s03's searched files and move its count, and the report does not show the claim holds on them
@answer n=6 entity=mocking-stubbing-discipline#3.4 v=yes mode=stale
why: the new reached files may join slice s06's searched files and move its count, and the report does not show the claim holds on them
@answer n=7 entity=mocking-stubbing-discipline#3.5 v=yes mode=stale
why: the new reached files may join slice s07's searched files and move its count, and the report does not show the claim holds on them
@answer n=8 entity=mocking-stubbing-discipline#3.6 v=yes mode=stale
why: the new reached files may join slice s08's searched files and move its count, and the report does not show the no-match reading holds on them
@answer n=9 entity=mocking-stubbing-discipline#3.7 v=yes mode=stale
why: the new reached files may join slice s09's searched files and move its count, and the report does not show the claim holds on them
@answer n=10 entity=mocking-stubbing-discipline#3.8 v=yes mode=stale
why: the new reached files may join slice s11's searched files and move its count, and the report does not show the claim holds on them
