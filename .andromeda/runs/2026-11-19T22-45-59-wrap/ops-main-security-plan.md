@row_set n=1 row=absence/served-network-api cell=capability seen=1 detector=D-security-input
value: A served network API in a default build: no default feature set serves a TCP or UDP port
basis: fct-xcqgt2ri
because: The opt-in devtools feature now builds a CDP server bound to a TCP port on 127.0.0.1, so the absence holds only for the default feature sets — the amended listener claim's stale site.
@row_set n=2 row=overview/api-security cell=text seen=1 detector=D-security-input
value: No served network API exists in a default build: no default feature set serves a TCP or UDP port. The one default listener is local, the driver session's Unix-domain socket, and the `escher-session mcp` role serves the driver's verbs as MCP tools over its own stdin and stdout, binding nothing, under bounded requests and results. The opt-in devtools feature builds a CDP server listening on 127.0.0.1 with no authentication. The remaining controls govern outbound requests and in-document navigation.
basis: fct-xcqgt2ri
because: The overview's «no binary serves a TCP or UDP port» is false once the devtools feature is built, as the CDP listener binds 127.0.0.1 — the amended listener claim's stale site.
