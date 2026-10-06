# Windows entry for the cold-agent run pipe: run · status · cleanup · logs.
# scripts/cold-agent.sh owns the whole contract; every argument passes to it unchanged and its exit code is ours.
& bash "$PSScriptRoot/cold-agent.sh" @args
exit $LASTEXITCODE
