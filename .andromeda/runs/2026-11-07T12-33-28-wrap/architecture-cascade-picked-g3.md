@account n=1 entity=existing-scopes#1.23 v=proposed
why: the crate's module list gains the private `mutation_buffer` (lib.rs 21, Crates / modules), which it does not name
@account n=2 entity=existing-scopes#1.31 v=not_touched
why: the blitz-tests unit names neither `dioxus_runtime_probe` nor its pins (that record is existing-scopes#4.2), and its counts and file lists hold on the changed probe file, which keeps 10 tests and adds no target
@account n=3 entity=existing-scopes#4.2 v=proposed
why: it pins the MountedData hazard as two `should_panic` tests, which the report retires for the two `…_spawned_acts_*` tests asserting the spawned task's focus acts
