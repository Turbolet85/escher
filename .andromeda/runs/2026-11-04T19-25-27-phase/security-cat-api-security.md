@carried n=1 label=security-plan/api-security fact=fct-svo6bgmr
why: The probe's test or example code must keep the API surface as it is: no served TCP or UDP port, and any listener a test binds served by no binary and registered as test-only.
quote: No served network API exists — no served TCP or UDP port: two `blitz-net` tests each call `std::net::TcpListener::bind("127.0.0.1:0")` in the crate's `#[cfg(test)] mod tests`, served by no binary, reached by no caller, so no crossing, registered as test-only
@carried n=1 run=2 label=security-plan/api-security fact=fct-svo6bgmr
quote: over its own stdin and stdout — no listener, port or socket, so the session socket stays the one listener
@carried n=2 label=security-plan/api-security fact=fct-brzrm7cu
why: The probe's stand runs (a lean task such as the Counter or CRUD, and a served task) must start no new load: the load page's one fixed in-process fetch is the only one, and every other stand boot must still hold no net provider.
quote: the wait has one served witness on the stand, the load page (`stand::boot_load`, `escher-session start load`): its one fetch of the fixed `http://stand.test/load.css` is answered in process by `LoadNet` after `LOAD_ANSWER_DELAY` (200 ms) from a thread of its own with fixed CSS bytes — no `blitz_net`, socket, listener, print or env read — and, `LoadNet` overriding no `is_noop`, the harness counts the load and the page's click waits it out within `LOAD_WAIT` (`stand_load_page`); every other stand boot holds `net_provider: None` (examples/seven_guis/src/stand.rs:141-175)
@carried n=3 label=security-plan/api-security fact=fct-brzrm7cu
why: The probe must add no verb, argument or result field to the driver's command schema, so the schema's no-wait property must still hold after the chunk.
quote: the driver's command schema keeps this: none of its eight instance-level verbs has an argument or a result field of a wait
@carried n=4 label=security-plan/api-security fact=fct-t6pddbhw
why: A served task the probe drives over the session must leave the local socket's owner-only access and fixed bounds unchanged; the probe must not widen, relax or work around them.
quote: Reachable by the owning user only (state directory `0700`, socket file `0600`, a directory open to its group or to others refused); no handshake secret, token or peer-credential check; one connection at a time, a 2 s read and write bound and 30 s on a client's read of a call's answer, a request line of at most 16,384 bytes, a lifecycle reply of at most 128 and an answer of at most 1,048,576 on its JSON line
@binding n=1
**No served API, one local listener ↔ the body's Threat Model local IPC vector and the chunk's «Nothing crosses» boundary**: the probe adds no listener, port or carrier, so the session socket stays the one listener and the Threat Model's local-IPC vector does not change (security-plan/api-security fct-svo6bgmr) ↔ (security-plan fct-z5ttw3ol); the body's broad line must stay true (security-plan fct-s4jqcqqv).
@binding n=2
**Stand boots without a net provider ↔ the chunk's stand measurements in `examples/seven_guis/`**: the probe's runs on the stand must add no fetch or net provider to any boot but the load page's fixed in-process one (security-plan/api-security fct-brzrm7cu); the body's settle line must stay true (security-plan fct-dlhusvs4).
@binding n=3
**Command schema with no wait field ↔ the body's Input Validation command schema and widening rule**: the probe adds no verb, argument or result field; any new field inside a ratified carrier would fall under the widening rule (security-plan/api-security fct-brzrm7cu) ↔ (security-plan fct-cvyxuuhn) (security-plan fct-n4q5kc73).
@binding n=4
**Local session socket bounds ↔ the body's Authentication & Authorization**: access stays the filesystem's only, owner-only, with no handshake or peer check added or removed (security-plan/api-security fct-t6pddbhw) ↔ (security-plan fct-pgi7ssst); the body's socket line must stay true (security-plan fct-sn4avmoz).
