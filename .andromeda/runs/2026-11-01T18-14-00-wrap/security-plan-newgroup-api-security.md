# New groups — security-plan · api-security

## Verdicts
- fanout-security-plan--cascade-g4.md 1 · contradiction · lines 8–8 · «No served network API exists — no TCP or UDP port» · the file says flatly no TCP or UDP port, the proposal has two `blitz-net` tests binding TCP on loopback and narrows the claim to no served port · its own line rewritten
- fanout-security-plan--cascade-g4.md 3 · distinct · no group states what a failed load does to the wait or names the `NetHandler::error` forward in `CountedHandler`; the standing `LOAD_WAIT` bound and the `HttpStatus` row say neither
