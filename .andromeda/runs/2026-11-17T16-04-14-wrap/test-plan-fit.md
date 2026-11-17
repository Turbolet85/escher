@fit n=1 row=decision/file-urls-only-for-file-documents sha=93739303fc96 v=fits type=1
@fit n=2 row=decision/free-disk-on-test-and-stand-repeat sha=7c5a5d1224ac v=fits type=6
@fit n=3 row=decision/test-job-reads-its-disk sha=040f62e012d8 v=not_master type=6
why: Two diagnostic df steps around the test leg retell the workflow's configuration and bind no future work, so the CI workflow file holds them.
