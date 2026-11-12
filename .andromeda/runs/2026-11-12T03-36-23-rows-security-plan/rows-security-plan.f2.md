@row_add n=1 row=absence/request-timeout area=api-security rec=1,2
capability: A request timeout on outbound requests
@row_add n=2 row=decision/session-socket-owner-only area=api-security rec=3,4,5,6
name: Session socket reachable by the owning user only
rule: Access to the driver session socket is the filesystem's alone: the state directory is 0700, the socket file 0600, a directory open to its group or others is refused, and there is no handshake secret, token or peer-credential check.
@row_add n=3 row=decision/dependency-audit-cargo-deny area=dependency-security rec=7,8
name: Dependency audit with cargo-deny
rule: cargo-deny checks advisories and licenses, locked, over the whole workspace's resolved graph with all features for the six platforms CI builds, configured by the root `deny.toml`, ignores per ID only, each with a written reason, re-read on every push.
why: Ratified: audit the whole workspace and check licences.
except: `unsound` stays at its default, which keeps memmap2's unsound advisory out of the gate.
@row_add n=4 row=decision/no-base64-crate area=dependency-security rec=9,10
name: No base64 crate
rule: No base64 crate is taken as a dependency; a shot's base64 is written by hand.
@row_add n=5 row=decision/licence-gate area=dependency-security rec=11,12
name: Licence gate on every push
rule: The audit leg runs upstream's licences table, held byte for byte in the root deny.toml, on every push over the whole workspace's graph, so the fork gates on licences.
why: An open question, decided as the fork's chosen option.
guard: the audit leg
except: upstream's own licenses job stays reverted
@row_add n=6 row=absence/authentication-authorization area=authentication-authorization rec=14,15,16,17,18,19,20,21,22,23,24,13
capability: Authentication or authorization code
@row_add n=7 row=decision/live-runs-use-operator-login area=secret-management rec=25,26
name: Live runs use the operator's own login
rule: The cold-agent live run and the MCP live trial reach the model provider only through the operator's own Claude Code login held outside the repository: no API key, no env read by the pipe, no CI secret, no new secret source and no auth library.
why: Ratified.
@row_add n=8 row=contract/cli-exit-codes/1 area=error-handling rec=27,28,29
boundary: escher-session CLI exit codes
item: 1
promise: The call was refused; the refusal JSON is written to stdout.
@row_add n=9 row=contract/cli-exit-codes/2 area=error-handling rec=28,29
boundary: escher-session CLI exit codes
item: 2
promise: Usage: the argv shape is not one the command takes; one fixed usage line on stderr, stdout empty, nothing booted and no state created.
