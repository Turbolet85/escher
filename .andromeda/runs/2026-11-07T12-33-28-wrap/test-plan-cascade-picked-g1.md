@account n=1 entity=test-scope-summary#11.1 v=proposed
why: the row states the MountedData hazard pinned by two should_panic tests, which the report retires and inverts into the two spawned_acts pins
@account n=2 entity=test-scope-summary#24.1 v=proposed
why: the row reads 82 unit tests, and the report's gate and CI read 83 passed with upstream's ported test added in dioxus_document.rs
@account n=3 entity=test-scope-summary#24.2 v=not_touched
why: dioxus_document.rs is reached but the report adds only the ported test after mounted_data_of_removed_element_reports_error and the crate reads 83 passed, so the IME-on-mount claim still holds on the changed file
