@row_add n=1 row=decision/ci-test-job-disk-readings area=9-ci-integration fact=F3 detector=none
name: Test job disk readings in the step log only
rule: The fork CI's default-features test job writes a disk reading before its test leg and one after it, the after one always, to its step log only, uploading no artifact for them.
why: The readings measure the disk at the test leg rather than cure it, so a disk red can be read at the next failure.
guard: test_test_job_reads_its_disk
basis: fct-w2ipnyht, fct-v3bawifo
because: The chunk gave the default-features test job two df -h readings kept in the step log with no upload, a CI guarantee the section holds no row for.
@row_none n=1 fact=F1 type=8
what: A record of what this upstream sync moved among the engine's image-fetch log lines and that it added none; the census and the code hold it, and the standing allowlist rows already cover engine-target lines.
@row_none n=2 fact=F2 type=4
relied: SVG image and feImage href loads are engine sub-resource fetches, so the standing rule refusing an engine fetch of a file: URL from a non-file document already covers them.
