@answer n=1 entity=inherited-defaults#1.1 v=no
why: the session adds no crate and moves no manifest, so the publish flags and the versioned published packages still hold.
@answer n=2 entity=inherited-defaults#2.1 v=no
why: no CI workflow, job, leg or gate changed, and the fmt, clippy, rustdoc and coverage gates it names still hold on the new code.
@answer n=3 entity=inherited-defaults#3.1 v=no
why: the command and tool_call spans stay ungated, the actions call only records the verb field, and no feature or manifest moved.
