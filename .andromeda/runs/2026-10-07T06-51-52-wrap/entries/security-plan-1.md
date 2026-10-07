## 2026-10-07-driver-session — the driver session socket: threat vector, access, input rows
**Section:** §Threat Model Summary → Attack surface (new vector: local IPC) · §Authentication & Authorization (new row) · §Input Validation (four new rows: session socket · state directory · label · `escher-session` argv) · §API Security (lead · new control row) · §Secret Management → Environment values read · §Dependency Security → Pinning (citations)
**Change:**
- API Security lead: was "No served API surface exists … listeners are observed absent"; now no served network API and no TCP or UDP port, with one local listener — the driver session's Unix-domain socket `session.sock`, lifecycle messages only.
- Access: no handshake, token or peer-credential check; the state directory is created `0700`, one open to group or others is refused (`StateDirNotPrivate`), the socket is `0600` — the owning user only.
- Validation: requests parsed against a closed grammar (`hello v1`, `stop v1`), bounded at 64 bytes, a 2 s read and write bound, refusals and broken connections changing nothing; replies carry a pid, the label and a count only; the label is 1-32 bytes of `a-z0-9-`, checked before the boot; the binary's argv is closed (two arguments, four tasks) and exits 2 before anything boots; `stop` removes the socket file and the directory, nothing else.
- Environment: two build-time `env!` values in the session checks; the library and the binary read no env var beyond `RUST_LOG`.
- Citations: root `Cargo.toml:104; :114` → `:106; :116`; the blitz-tests dev-dependency range → `:15-40`.
**Why:** a new listener and a new input surface are a security-plan amendment first. A boundary widening, ratified by the founder's own choice (the founder, 2026-10-07, relayed verbatim). Standing: the wire carries nothing of the screen; a command that carries an id, a name, a value, snapshot text or a diff reopens the crossing question.
**Kept:** no idle expiry and no pid check in `start` — recorded as built; a TCP or UDP port was ruled out (it needs an auth surface the version excludes).
**Ref:** .andromeda/runs/2026-10-07T06-51-52-wrap/
