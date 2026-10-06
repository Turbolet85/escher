# Windows entry for the agent-driven test contract: boot · run · status · cleanup · logs.
# scripts/agent-run.sh owns the whole contract; every argument passes to it unchanged and its exit code is ours.
& bash "$PSScriptRoot/agent-run.sh" @args
exit $LASTEXITCODE
