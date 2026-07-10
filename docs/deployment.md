---
description: >-
  Release process for bubbleshot: version bumping, tagging, CI/CD with GitHub Actions,
  automated .deb and .rpm package generation, and distribution.
tags:
  - deployment
  - release
  - ci-cd
  - packaging
  - github-actions
---

# Deployment

## Creating a Release

1. Bump the version in `Cargo.toml` and update `CHANGELOG.md`.
2. Commit and push:
   ```bash
   git add Cargo.toml CHANGELOG.md
   git commit -m "Release v0.2.0"
   git tag v0.2.0
   git push origin main --tags
   ```

The CI pipeline (`.github/workflows/release.yml`) will:

1. Build the binary for `x86_64-unknown-linux-gnu`.
2. Generate `.deb` and `.rpm` packages via `cargo-deb` and `cargo-generate-rpm`.
3. Create a **GitHub Release** with the following assets.

## Release Assets

| Asset | Description |
|---|---|
| `bubbleshot-x86_64-unknown-linux-gnu` | Static binary |
| `bubbleshot_0.2.0_amd64.deb` | Debian/Ubuntu package |
| `bubbleshot-0.2.0-1.x86_64.rpm` | Fedora/RHEL package |
| `install.sh` | Distribution-agnostic installer |

## Installation Methods

### Binary (any distro)
```bash
curl -sSL https://github.com/alvescruz/bubbleshot/releases/latest/download/install.sh | sudo bash
```

### Debian/Ubuntu
```bash
wget https://github.com/alvescruz/bubbleshot/releases/download/v0.2.0/bubbleshot_0.2.0_amd64.deb
sudo dpkg -i bubbleshot_0.2.0_amd64.deb
```

### Fedora/RHEL
```bash
wget https://github.com/alvescruz/bubbleshot/releases/download/v0.2.0/bubbleshot-0.2.0-1.x86_64.rpm
sudo rpm -i bubbleshot-0.2.0-1.x86_64.rpm
```

### Build from source
```bash
git clone https://github.com/alvescruz/bubbleshot.git
cd bubbleshot
cargo build --release
sudo cp target/release/bubbleshot /usr/local/bin/
```
