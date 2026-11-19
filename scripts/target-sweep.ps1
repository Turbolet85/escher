# Windows entry for the stale-hash sweep: plan · run.
# scripts/target-sweep.py owns the whole contract and refuses off unix; every argument passes to it unchanged and its exit code is ours.
& python3 "$PSScriptRoot/target-sweep.py" @args
exit $LASTEXITCODE
