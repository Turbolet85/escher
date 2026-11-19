@row_set n=1 row=absence/served-network-port cell=capability seen=1 detector=D-arch-resources
value: A served network port in a default build: no TCP or UDP port any binary serves with its default features
basis: fct-xcqgt2ri
because: Under the opt-in devtools feature a binary now serves a TCP port on 127.0.0.1, so the absence holds only for default feature sets — the amended listener claim's stale site.
@row_set n=2 row=absence/network-listener cell=capability seen=1 detector=D-arch-resources
value: A served network (TCP or UDP) listener in any default feature set
basis: fct-xcqgt2ri
because: The CDP server's TCP listener exists behind the opt-in devtools feature, so the absence holds only for default feature sets — the amended listener claim's stale site.
