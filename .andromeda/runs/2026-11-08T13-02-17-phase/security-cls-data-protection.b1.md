@answer n=1 entity=data-protection#1.1 v=yes
why: recording.rs is edited and the scope names the recorded step of the new read and the file modes, which this entity's record content and 0600-in-0700 modes rule over
@answer n=2 entity=data-protection#2.1 v=yes
why: the scope names the committed-checks exception (only the two stand-Counter checks are committed, any other check stays owner-only), and replay.rs reads them through Check::from_bytes
