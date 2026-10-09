# The install hang, read from the recorded run — CI#37673662374 on `983d8973`

Read at phase P3 of 2026-10-09-audit-corrections-agent-surfaces, 2026-10-09, from `Turbolet85/escher`:
`gh api "repos/Turbolet85/escher/actions/runs/37673662374/attempts/{1,2}/jobs?per_page=100"` for the jobs, and
`gh api --allow-escape-sequences repos/Turbolet85/escher/actions/jobs/{id}/logs` for each hung job's log. The two
logs were read whole; the lines below are excerpts, colour codes stripped, nothing else changed.

## Attempt 1 — job 112971517774, `Test [default features]`
Job started 2026-10-07T19:19:58Z, concluded `cancelled` 2026-10-07T19:51:30Z. Runner image `ubuntu-24.04`, version
`20261004.327.1`. The step `sudo apt-get update && sudo apt-get install -y libfontconfig1-dev` began at
2026-10-07T19:20:14.95Z; its last line is at 2026-10-07T19:20:35.46Z; the next line is the cancel at
2026-10-07T19:51:29.04Z — 30 min 54 s of silence.

```
2026-10-07T19:20:14.9566143Z ##[group]Run sudo apt-get update && sudo apt-get install -y libfontconfig1-dev
2026-10-07T19:20:15.6705078Z Get:1 file:/etc/apt/apt-mirrors.txt Mirrorlist [144 B]
2026-10-07T19:20:15.7021582Z Get:6 https://packages.microsoft.com/ubuntu/24.04/prod noble InRelease [3600 B]
2026-10-07T19:20:30.9575006Z Ign:2 http://azure.archive.ubuntu.com/ubuntu noble InRelease
2026-10-07T19:20:30.9576784Z Ign:3 http://azure.archive.ubuntu.com/ubuntu noble-updates InRelease
2026-10-07T19:20:31.9584241Z Ign:2 http://azure.archive.ubuntu.com/ubuntu noble InRelease
2026-10-07T19:20:34.8481582Z Hit:2 https://archive.ubuntu.com/ubuntu noble InRelease
2026-10-07T19:20:34.9404555Z Get:3 https://archive.ubuntu.com/ubuntu noble-updates InRelease [126 kB]
2026-10-07T19:20:35.2300858Z Ign:9 http://azure.archive.ubuntu.com/ubuntu noble-updates/main amd64 Packages
2026-10-07T19:20:35.2678949Z Get:4 https://archive.ubuntu.com/ubuntu noble-backports InRelease [126 kB]
2026-10-07T19:20:35.4554460Z Get:5 https://archive.ubuntu.com/ubuntu noble-security InRelease [126 kB]
2026-10-07T19:51:29.0382219Z ##[error]The operation was canceled.
```

## Attempt 2 — job 112987113323, `Build [default features]`
Runner image `ubuntu-24.04`, version `20260927.320.1`. The same step began at 2026-10-07T19:56:34.70Z; its last line
is at 2026-10-07T19:57:13.43Z; the next line is the cancel at 2026-10-07T20:23:49.67Z — 26 min 36 s of silence.

```
2026-10-07T19:56:34.7001612Z ##[group]Run sudo apt-get update && sudo apt-get install -y libfontconfig1-dev
2026-10-07T19:56:36.0390008Z Get:1 file:/etc/apt/apt-mirrors.txt Mirrorlist [144 B]
2026-10-07T19:57:07.3600282Z Ign:2 http://azure.archive.ubuntu.com/ubuntu noble InRelease
2026-10-07T19:57:08.3609013Z Ign:2 http://azure.archive.ubuntu.com/ubuntu noble InRelease
2026-10-07T19:57:12.5010488Z Hit:2 https://archive.ubuntu.com/ubuntu noble InRelease
2026-10-07T19:57:12.6430471Z Get:3 https://archive.ubuntu.com/ubuntu noble-updates InRelease [126 kB]
2026-10-07T19:57:13.0529797Z Ign:10 http://azure.archive.ubuntu.com/ubuntu noble-updates/main amd64 Packages
2026-10-07T19:57:13.1474478Z Get:4 https://archive.ubuntu.com/ubuntu noble-backports InRelease [126 kB]
2026-10-07T19:57:13.4338645Z Get:5 https://archive.ubuntu.com/ubuntu noble-security InRelease [126 kB]
2026-10-07T20:23:49.6730107Z ##[error]The operation was canceled.
```

## What the two logs establish
- Both stalls are inside `apt-get update`. Neither log holds a `Reading package lists`, an `Unpacking` or a
  `Setting up` line: `apt-get install` never started, so no package state was touched when the job was cancelled.
- The signature is the same on two runner image versions: every index of `http://azure.archive.ubuntu.com` reads
  `Ign:`, apt falls back to `https://archive.ubuntu.com`, and after `Get:5 … noble-security InRelease` nothing is
  printed until the cancel.
- No apt timeout or retry line is printed in either log during the silence.

## Not read
- The run's attempt 3 (green) and the second run the route entry names, CI#37687897931 — what failed in its first
  attempt is still not read.
