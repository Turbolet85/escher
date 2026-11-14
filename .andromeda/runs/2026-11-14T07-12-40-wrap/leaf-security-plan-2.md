@leaf_group group=security-plan-2
@leaf_read leaf=.claude/rules/security.md lines=32
last: - 2026-10-15: A listener or env read that only a test makes 
@leaf_edit n=1 leaf=.claude/rules/security.md line=9 class=omission
leaf-text: Never commit `.env*`.
source: dec-jihszp6m «The seeded source of randomness takes its seed only from in-process code, a root context the virtual DOM's builder gives else the default seed 0, and outside any run seeds from the host's per-process»
replace-with: The seeded source of randomness takes its seed only from in-process code (a root context the virtual DOM's builder gives, else the default seed 0; outside any run, the host's per-process random keys), reading no environment variable, file or carrier — keep it so. Never commit `.env*`.
