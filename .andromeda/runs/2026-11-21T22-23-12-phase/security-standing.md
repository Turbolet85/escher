# security · standing conventions

- S1 · each new CI step pins its actions to commit SHAs and runs under a read-only token · dependency-security · authentication-authorization
- S2 · each new build-time fetch is registered as a supply-chain surface in the Dependencies section with its integrity check · dependency-security
- S3 · each advisory ignore kept or added is per ID with a written reason · dependency-security
- S4 · each new CI script or step holds no secret; secrets stay in GitHub Actions secrets and vars · secret-management
  decision/apt-install-no-env-no-secrets · Install script holds no environment or secret — The apt install script reads no environment variable of its own, writes no file and names no secret.
  decision/apt-install-no-secrets · apt-install.sh touches no environment or secret — apt-install.sh reads no environment variable of its own, writes no file and names no secret, token or GitHub context variable.
