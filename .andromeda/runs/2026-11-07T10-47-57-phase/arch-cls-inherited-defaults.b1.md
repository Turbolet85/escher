@answer n=1 entity=inherited-defaults#1.1 v=no
why: the rule fixes which crates publish and how versions pin, and the chunk moves no version, publish flag or in-repo pin
@answer n=2 entity=inherited-defaults#2.1 v=no standing=S4
why: the entity states the fmt, clippy -D warnings and rustdoc -D warnings gates run as ci-leg.sh legs, which is standing convention S4 and is carried by it
@answer n=3 entity=inherited-defaults#3.1 v=yes
why: the buffer module may keep trace calls, and task item 11 names that each trace is gated per call site with a cfg(not(feature = tracing)) fallback, so the new code must honour it
