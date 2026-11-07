@verdict n=1 ref_file=fanout-architecture--cascade-g3.md ref_n=3 v=contradiction fact=fct-ktfizprz
quote: the MountedData hazard pinned as two `should_panic` tests, one per mode
why: the standing probe line pins the hazard as two `should_panic` tests, while the proposal names the probe's two `…_spawned_acts_*` tests as the fix's witness, passing and failing with `RefCell already borrowed` only under the restored borrowed render
