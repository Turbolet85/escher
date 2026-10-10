
## 2026-10-10-driver-cli — Driver session contract: wire `v2`, `serve` with an idle expiry, the client's `call`, thirteen errors, 40 re-exports
**Section:** §Standard Contracts → Driver session (the re-exports, `serve`, the lifecycle, `SessionError`, the wire, the non-unix arm, the Not built list)
**Change:**
- Re-exports: was 33 names; now 40 — the session's eleven (`IDLE_EXPIRY`, `call` and `Answer` joined), the schema's 25 (`SessionCommand`, `validate_session`, `Level` joined), `Outcome`, `command_line`, and the harness's two.
- `serve(state_dir, session, idle_expiry)`: a third parameter; a non-blocking accept loop polled every 25 ms; a `call` request is run through `Session::run` and nothing else; every answered request — a refused or malformed one included — counts toward `served` and starts the idle count again, a connection that breaks before its request is whole does neither.
- The lifecycle: `Hello` gains `idle_expiry_s`; `call(state_dir, &Call) -> Result<Answer, SessionError>` with `Answer { accepted, json }` validates first, so a call the schema refuses reaches no session and every call sent fits the request bound; an answer is read under 30 s per read.
- `SessionError`: was twelve variants; now thirteen — `AnswerTooLarge`, read from an `oversize` reply, the call having run — with `kind()`, one fixed kebab-case word per variant, and `to_json()`.
- The wire: was `v1`, `hello` and `stop`, 64-byte requests; now `v2` with `call v2 <verb> [<name>=<kind>:<value>]…` (kinds `t` · `n` · `f`, strict percent-escapes), requests at most 16,384 bytes, lifecycle replies at most 128 (the widest `hello` measures 122, was 87), answers `ok v2 accepted` · `refused` · `oversize` bounded at 1,048,576 bytes on the JSON line.
- Non-unix: `call` joins the functions that return `Unsupported`.
- Not built: now a settle verb or busy-source reply of its own on the socket, and an MCP tool; a verb on the socket, CLI JSON and an idle expiry left the list.
**Why:** the contract is the crate's public surface, and the chunk changed each of these. `client::call` validating first was decided at implement: the plan did not say what a call over the request bound does, and checking first means none is ever sent.
**Kept:** `Session::start`, `act`, `with_time`, `attach`, `stop` and `start` read as they did; `start` still does not check that the answering pid is its child.
**Ref:** .andromeda/runs/2026-10-10T10-03-00-wrap/
