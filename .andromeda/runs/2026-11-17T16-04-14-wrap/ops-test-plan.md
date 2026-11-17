@row_set n=1 row=decision/file-urls-only-for-file-documents cell=rule seen=4 fact=F3 detector=none
value: A `file:` URL loads only for a `file:` document: else a script's `fetch()`, `import` or `<script src>` is refused with a `FetchError`, a sub-resource load, an SVG image or feImage href included, reads no local file, and the browser drops a `file:` favicon or link silently; user loads are not gated.
basis: fct-y4zyhjnd; fct-ubjduri7; fct-ksxva77j; fct-vaq3umam
because: The take adds an SVG image and feImage href fetch as an engine sub-resource request, its scope ratified as measured (no use href), so the row's sub-resource refusal must name that load.
@row_retire n=1 row=decision/only-stand-repeat-frees-disk seen=1 heir=decision/free-disk-on-test-and-stand-repeat detector=none
why: The default-features test job now holds the pinned free-disk step too, so only the stand-repeat job no longer frees disk, and its guard was renamed.
basis: fct-w2ipnyht; fct-wczmmtsv; fct-wewkknlb
because: The free-disk step stands on two jobs where the row says one, and the row's name and rule are made from that one job, so the row is replaced by its heir.
@row_add n=1 row=decision/free-disk-on-test-and-stand-repeat area=ci-integration detector=none
name: Free disk on the test and stand-repeat jobs
rule: The Stand checks repeated job and the default-features test job each free runner disk with a pinned free-disk action after the toolchain and before the cache; the matrix job holds no free-disk step.
guard: test_free_disk_only_on_test_and_stand_repeat
basis: fct-w2ipnyht; fct-wczmmtsv
because: The free-disk step joins the default-features test job under the renamed guard, and the heir states the two jobs the workflow now guarantees.
@row_add n=2 row=decision/test-job-reads-its-disk area=ci-integration fact=F1 detector=none
name: Test job reads its disk
rule: The default-features test job reads its disk before the test leg and after it, even when the leg fails, the readings kept in the step log only and uploaded nowhere.
why: The free-disk step measures rather than cures, so the two readings decide the next disk red.
guard: test_test_job_reads_its_disk
basis: fct-w2ipnyht; fct-v3bawifo
because: The workflow gains two guarded disk readings on the test job as a standing line against disk reds, a configuration guarantee no row states.
@row_add n=3 row=decision/unacquired-ci-job-rerun area=ci-integration fact=F2 detector=none
name: CI job with no runner re-run, not fixed
rule: A CI job that no runner acquired, cancelled with no step run, is re-run rather than fixed; a red for any other cause stops the pass.
basis: fct-vaq3umam; fct-zbabznq5
because: The pass's first CI reading went red on two jobs no runner acquired and the ruling made re-running them the rule, which no CI row states.
@row_none n=1 fact=F4 type=4
relied: A box whose content shrinks reads its scroll offset back in its scroll range after layout, which the stand scroll check relies on when it reads the CRUD list at 0 after a delete; the engine's clamp is the code's and architecture's, the check an inventory.
