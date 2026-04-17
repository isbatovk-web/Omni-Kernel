# 🛠️ Installation Guide

Follow these steps to deploy the Omni-Kernel environment on your local machine.

## Supported Platforms
- Windows 10/11 (PowerShell)
- Linux (Ubuntu, Debian, Fedora, Arch)
- macOS (Intel & Apple Silicon)

## Automated Installation
The fastest way to install the Omni-CLI is via our global bootstrap scripts.

### Windows
Open PowerShell as Administrator and run:
```powershell
irm https://omni-kernel.github.io/install.ps1 | iex
```

### Linux & macOS
Run the following in your terminal:
```bash
curl -sSL https://omni-kernel.github.io/install.sh | bash
```

## Manual Verification
Once installed, verify the installation by checking the engine version:
```bash
omni --version
```
Expected output: `Omni-Kernel Engine v1.0.4 (Matrix-Build)`
