---
description: >-
  Install bubbleshot on Linux via the automatic script, .deb package (Debian/Ubuntu),
  .rpm package (Fedora/RHEL), or build from source. System requirements and
  dependencies included.
tags:
  - installation
  - setup
  - deb
  - rpm
  - build-from-source
  - linux
---

# Installation

The recommended way to install **bubbleshot** is using our automated installation script.

!!! warning "System Requirements"
    Currently, **bubbleshot** is only supported and tested on:
    
    *   **OS:** Linux
    *   **Display Protocol:** Wayland
    *   **Desktop Environment:** GNOME
    *   **Architecture:** x86_64 (amd64)

## Quick Installer

The installer script will automatically handle system dependencies for major distributions.

```bash
curl -sSL https://raw.githubusercontent.com/alvescruz/bubbleshot/main/install.sh | sudo bash
```

### What the installer does:
1. Detects your system architecture.
2. Installs required system dependencies (`libgtk-3`, `libpipewire-0.3`, `libdbus-1`) for **Ubuntu, Fedora, and Arch Linux**.
3. Downloads the latest pre-compiled binary.
4. Moves it to `/usr/local/bin/bubbleshot`.

---

## Manual Dependencies

If you are not using one of the supported distros above, please install these manually before running the script:
- `libgtk-3-0`
- `libpipewire-0.3-0`
- `libdbus-1-3`

## Install via .deb (Debian/Ubuntu)

Download the latest `.deb` from the [releases page](https://github.com/alvescruz/bubbleshot/releases):

```bash
wget https://github.com/alvescruz/bubbleshot/releases/latest/download/bubbleshot_0.2.0_amd64.deb
sudo dpkg -i bubbleshot_0.2.0_amd64.deb
```

## Install via .rpm (Fedora/RHEL)

```bash
wget https://github.com/alvescruz/bubbleshot/releases/latest/download/bubbleshot-0.2.0-1.x86_64.rpm
sudo rpm -i bubbleshot-0.2.0-1.x86_64.rpm
```

## Building from Source

```bash
git clone https://github.com/alvescruz/bubbleshot.git
cd bubbleshot
cargo build --release
sudo cp target/release/bubbleshot /usr/local/bin/
```
