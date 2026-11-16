@fit n=1 row=decision/capture-before-own-recorder sha=99aa4b65f9c1 v=fits type=1
@fit n=2 row=decision/driver-depends-on-telemetry sha=c7f5ef3d4ecb v=not_master type=6
why: A retelling of one manifest dependency line that guarantees nothing beyond what the driver's Cargo.toml holds.
@fit n=3 row=decision/escher-telemetry-sink sha=da411ef4fb40 v=not_master type=4
why: The added clause retells how the subscriber's layers are composed and filtered with no reason stated, which the escher-telemetry code holds.
@fit n=4 row=contract/telemetry/init sha=b302209e4a06 v=not_master type=4
why: The added clause lengthens the init promise with how the subscriber is built from a filtered sink and an unfiltered capture layer, an implementation the code holds.
@fit n=5 row=contract/telemetry/target-filter sha=846b2000e3bf v=not_master type=4
why: The added clause retells how the capture layer sits beside the sink's target drop, a mechanism whose basis states no reason and the code holds.
@fit n=6 row=decision/driver-library-side-effect-free sha=23c7cf39cccb v=fits type=2
