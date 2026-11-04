@carried n=1 label=security-plan/threat-model-summary fact=fct-ljkdert4
why: the probe must leave the session socket's boundary exactly as ratified (owner-only access, its closed crossing, masked values, no NodeId) since the scope forbids any new carrier, listener, port, node key or result field and any value it reads from crossing.
quote: the owning user — the directory is created `0700`, a present one open to its group or to others is refused, and
  the socket file is set `0600`; there is no other authentication. The wire carries the lifecycle (`hello`, `stop`) and,
  since 2026-10-10-driver-cli, calls and their answers: a call brings an element id and typed text in, an answer takes
  ids, accessible names, control values (a password's or a file input's as the fixed mask), the snapshot's text and the
  diff out — the text and the diff holding, since 2026-10-10-snapshot-states, three state readings per control
  (selected, pressed, invalid) as closed words and booleans, inside the ratified crossing on the founder's ruling
  (2026-10-10, relayed by the overseer; as measured at escher-0.1.0/chunks/2026-10-10-snapshot-states/report.md) —, and
  no `NodeId` crosses
@carried n=1 run=2 label=security-plan fact=fct-z5ttw3ol
quote: trust boundary the owning user (directory `0700`, a group- or other-open one refused, socket `0600`), no other
  authentication; it carries the lifecycle and calls with their answers — ids, names, control values (a password's or
  file input's masked), snapshot text, diffs, screenshots, a lint's findings — and no `NodeId`.
@carried n=1 run=3 label=security-plan fact=fct-dgkl4a64
quote: `escher-session mcp`, a stdio child an MCP client spawns, speaks JSON-RPC 2.0 on its stdin and stdout, no
  listener, port or socket; trust boundary the spawning client process, no authentication;
@carried n=2 label=security-plan/authentication-authorization fact=fct-c3aavvn3
why: if the probe's served task drives a session, access to it must stay the filesystem's owner-only modes with no handshake, token or peer check added, since the probe adds no listener, carrier or session surface.
quote: No handshake, token, secret or peer-credential check: access is the filesystem's — the state directory is
  created `0700`, a present one open to its group or to others is refused (`StateDirNotPrivate`), and the socket file is
  set `0600`, so only the owning user reaches it; `start` does not check that the answering pid is the child it spawned
@carried n=2 run=2 label=security-plan fact=fct-pgi7ssst
quote: The driver session socket and the MCP stdio server carry no handshake, token, secret or peer check: the socket's
  access is the filesystem's (state directory `0700`, a group- or other-open one refused, socket `0600`), and `start`
  does not check the answering pid; the MCP server answers only the process holding its stdin and stdout and binds
  nothing.
@binding n=1
**threat-model-summary ↔ authentication-authorization**: the session socket's trust boundary is the whole of its access control. Its owner-only `0700` directory and `0600` socket with no other authentication (security-plan/threat-model-summary fct-ljkdert4) are the only thing deciding who reaches it, since there is no handshake, token or peer check (security-plan/authentication-authorization fct-c3aavvn3). So a probe that adds no listener, port or carrier keeps both unchanged, and any served-task session the probe drives runs inside that same owner-only boundary.
@binding n=2
**threat-model-summary ↔ the body (masked values, no NodeId)**: whatever the probe reads through a session answer (an id, a name, a control value) must stay inside the ratified crossing, with a password's or file input's value masked and no `NodeId` crossing (security-plan/threat-model-summary fct-ljkdert4), on the socket and MCP carriers alike (security-plan fct-z5ttw3ol; security-plan fct-dgkl4a64). Nothing the probe reads about the four links (a component name, a handler, a signal's value) may be put in a field of either carrier.
