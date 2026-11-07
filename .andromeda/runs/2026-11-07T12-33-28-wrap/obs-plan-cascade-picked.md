@account n=1 entity=span-trace-coverage#2.3 v=not_touched
why: the select list's spanless record names no reached file and the report adds no span, field or log line
@account n=2 entity=span-trace-coverage#2.4 v=not_touched
why: its flush_queued_mounted_events guard in dioxus_document.rs stands, the report calling that call unchanged and adding no span
@account n=3 entity=log-coverage#8.9 v=proposed
why: mutation_writer.rs is untouched and its trace! calls re-fire on replay, so F1's silent buffer belongs beside this group
@account n=4 entity=log-coverage#9.4 v=not_touched
why: its two test targets are neither reached nor named by the report
@account n=5 entity=log-coverage#9.8 v=not_touched
why: the a11y leg's ten files are not reached and the probe stays out of that leg
@account n=6 entity=log-coverage#9.13 v=proposed
why: the probe row stays true (10 tests, census 0) and F3's renamed pins and helper go beside it
@account n=7 entity=pii-scrubbing-compliance#1.1 v=proposed
why: F2's in-process hold of the buffer's values sits beside the in-process scrub readings
@account n=8 entity=pii-scrubbing-compliance#1.2 v=not_touched
why: the report leaves the multiple-select form path of dioxus_document.rs alone and adds no log site there, so the claim holds
@account n=9 entity=pii-scrubbing-compliance#1.9 v=not_touched
why: the report lists fct-yvbbn3zt as no-change, the probe still silent with the census reading 0
