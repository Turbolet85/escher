# security-plan · picked accounting · group 4 of 4

## Picked
api-security#1.0 · proposed · two `blitz-net` tests now bind TCP on loopback, so «no TCP or UDP port» reads as no served one and F1 registers them as test-only
api-security#2.1 · not touched · `blitz-net/src/lib.rs` is a merge surface, but its added lines are 313, 316 and 464-546, all past this row, so the limit of 6 per host still holds
api-security#2.4 · not touched · `blitz-net/src/lib.rs` is reached, but the report moves only the error arm, the waker and the test module, so the body is still read whole with no cap
api-security#2.6 · proposed · F2, a failed load now ends the wait through the forward in `CountedHandler`, with `settle_waiting` unedited
api-security#2.7 · not touched · the added lines of `blitz-net/src/lib.rs` fall past this row, so header handling still holds
api-security#2.8 · not touched · the added lines of `blitz-net/src/lib.rs` fall past this row, so a non-success status is still typed `ProviderError::HttpStatus`
api-security#2.9 · not touched · `blitz-traits/src/net.rs` adds only `NetHandler::error` at 36-38, so the `AbortSignal` claim holds and only citation digits move
api-security#2.10 · not touched · `blitz-dom/src/net.rs` adds only 144-147, past `stamped_request`, so sub-resource requests are still GETs built by it
dependency-security#1.0 · not touched · the report prints no diff for `deny.toml`, `Cargo.lock` or any manifest and the audit reads `advisories ok`, `licenses ok`, so tool, ignores and reach all hold
dependency-security#2.7 · proposed · F3, this take gets its own lock record beside the last take, whose row stays as history
dependency-security#6.1 · not touched · `deny.toml` is byte-identical and the audit reads `licenses ok`, so the licence row holds and only citation digits move
dependency-security#6.3 · proposed · retired, the `matrix_test` free-disk step is removed, and F4 is placed beside it
secret-management#1.2 · not touched · no reached file bears on it and the report adds no env read, so the login row holds
error-handling#2.1 · not touched · `stylo.rs` is a merge surface, but its added lines are the context scope at 231-238 and the `borrow_data` filter at 771-776, no hook, and the Removed bullet names none, so the row holds
error-handling#4.1 · not touched · `stylo.rs` adds nothing before line 231, so the traversal lock at this row stands where it was and the claim holds
error-handling#5.1 · not touched · no added range of `document.rs` falls in the select-list readers, so they add no panic path and only citation digits move
error-handling#6.1 · proposed · F11 goes beside it, the mounted-event guard; the row itself, on a spawned task, still holds with `events.rs` unreached
error-handling#7.1 · proposed · retired, the key-press panic is fixed, and F10 records the mechanism and witnesses beside it
error-handling#8.1 · proposed · its closing clause points at the retired panic row, and the row-3 skip it rests beside is gone
logging-monitoring#1.1 · proposed · F12, where a failed load's error text stops; the sink rule itself holds with `escher-telemetry` undiffed
logging-monitoring#1.2 · not touched · `blitz-paint/src/lib.rs` adds only line 6 and the report adds no print, so the stdout row holds and only citation digits move
