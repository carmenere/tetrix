# Tetrix
**Tetrix** is a set of libraries and utils designed to simplify **parameterization of arbitrary commands** and **config files**, the **automation of daily routine actions** and the **customization of any environment**: local development environment, CI/CD environment and so on.

<br>

## Project layout
```bash
tetrix
├── ...
├── dtools/
├── ttx-api/
├── ttx-cli/
├── ttx-contracts/
├── ttx-models/
├── ttx-ns/
├── ttx-types/
```

<br>

# Intro
1. Read [**dtools/core/README.md**](https://github.com/carmenere/dtools-core/blob/main/README.md).
2. Install **essential deps**:
```bash
reinit_dtools
install_deps

## Remaining commands are only for linux
install_docker
post_install_docker
# Then: reboot or relogin or sudo -i -u %USERNAME%
```
3. Install **deps for stand** `tetrix`:
```bash
stand tetrix install_cargo_deps
stand tetrix install_services
```
4. Three ways to run **tetrix-api**:
- **Prepare** configs for services, **reinit** services, **rebuild** all and **run** stand
```bash
stand tetrix prepare_reinit_up
```
- **Reinit** services, **rebuild** all and **run** stand
```bash
stand tetrix reinit_up
```
- **Rebuild** all and **run** stand
```bash
stand tetrix up
```

<br>
