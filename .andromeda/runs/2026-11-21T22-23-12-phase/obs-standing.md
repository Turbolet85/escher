# obs · standing conventions

- S1 · each new or changed CI job or step runs on GitHub workflows, its leg log uploaded on failure, uploading nothing local beyond the fork CI's two artifacts · ci-integration · 9-ci-integration
- S2 · each new check or CI diagnostic line prints nothing on success, and on failure names a kind and a count or a phase and exit status in fixed text, never echoing captured content · 3-observability-harness-contract
  contract/apt-install-diagnostic/attempt · The package-install diagnostic line · attempt — One stderr line per failed attempt names the attempt number, and nothing of a user's.
  contract/apt-install-diagnostic/exit-status · The package-install diagnostic line · exit status — The line names the failed attempt's exit status.
  contract/apt-install-diagnostic/phase · The package-install diagnostic line · phase — The line names the phase as one of update, install or repair.
  decision/fixed-diagnostics · Fixed diagnostics — Every escher-session diagnostic is a fixed string holding no path, label, id or value, and no ending holds anything the caller typed.
  decision/log-check-failures-name-kind-count · Log check failures name a kind and a count — A failing log check's message names a kind and a count, never a needle or anything the host wrote.
