@account n=1 entity=threat-model-summary#3.1 v=carried
why: the scope's «nothing crosses» rule (no carrier, listener, port, node key or result field) rules over the session socket's boundary, which the probe's served task may run through
@account n=2 entity=authentication-authorization#2.6 v=carried
why: the socket's only access control is the filesystem's owner-only modes, and the probe must add no listener, handshake or carrier around the session its served task may drive
