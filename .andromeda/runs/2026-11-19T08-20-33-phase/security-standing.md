# security · standing conventions

- S1 · every new or changed cargo invocation in a CI leg or script runs --locked against the committed lockfile · dependency-security
- S2 · every new or changed CI workflow step uses actions pinned to commit SHAs and a read-only token · authentication-authorization · dependency-security
- S3 · a new script validates its arguments before acting and touches no environment value or secret · input-validation
  decision/apt-install-argument-validation · apt-install.sh validates every argument first — apt-install.sh accepts only package names of a fixed form and bounded numeric option values, else prints usage and exits 2 before sudo, timeout, apt-get or dpkg runs. · guard: InstallScriptTest.test_a_bad_call_exits_2_before_anything_runs
  decision/apt-install-no-env-no-secrets · Install script holds no environment or secret — The apt install script reads no environment variable of its own, writes no file and names no secret.
  decision/apt-install-no-secrets · apt-install.sh touches no environment or secret — apt-install.sh reads no environment variable of its own, writes no file and names no secret, token or GitHub context variable.
