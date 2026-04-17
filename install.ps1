# ═══════════════════════════════════════════════════════════════════
# OMNI-KERNEL WINDOWS ACTIVATION FABRIC
# ═══════════════════════════════════════════════════════════════════

$ErrorActionPreference = "Stop"

# Colors & Logo
Write-Host @"
   ____  __  ____    ____
  / __ \/  |/  / | / /  _/
 / / / / /|_/ /  |/ // /  
/ /_/ / /  / / /|  // /   
\____/_/  /_/_/ |_/___/    CORE 
"@ -ForegroundColor Cyan

Write-Host "`nInitiating Universal Logic Synthesis System..." -ForegroundColor Gray
Start-Sleep -Seconds 1

# Detection & Verification
Write-Host "> Verifying environment..." -ForegroundColor Gray
if (!(Get-Command git -ErrorAction SilentlyContinue)) {
    Write-Host "X Error: Git is required. Install from: https://git-scm.com" -ForegroundColor Red
    return
}

$RELEASE_URL = "https://github.com/isbatovk-web/Omni-Kernel/releases/download/v1.0.4/omni-kernel-win.zip"
$INSTALL_PATH = "$HOME\Omni-Kernel-Fast"

if (Test-Path $INSTALL_PATH) { Remove-Item -Recurse -Force $INSTALL_PATH }
New-Item -ItemType Directory -Path $INSTALL_PATH

Write-Host "> Downloading Pre-compiled Engine (Lightweight)..." -ForegroundColor Cyan
try {
    Invoke-WebRequest -Uri $RELEASE_URL -OutFile "$INSTALL_PATH\omni.zip"
} catch {
    Write-Host "X Error: Could not download the engine. Please ensure the release exists at Github." -ForegroundColor Red
    Write-Host "URL: $RELEASE_URL" -ForegroundColor Gray
    return
}

Write-Host "> Unpacking Universal Logic Fabric..." -ForegroundColor Cyan
Expand-Archive -Path "$INSTALL_PATH\omni.zip" -DestinationPath $INSTALL_PATH
Remove-Item "$INSTALL_PATH\omni.zip"

# Final Recognition
Write-Host "`n[SUCCESS] Omni-Kernel Engine Grounded in 5 Seconds!" -ForegroundColor Green
Write-Host "🚀 Launching Dashboard..." -ForegroundColor Yellow

Set-Location $INSTALL_PATH
Start-Process ".\omni-kernel.exe"
# Wait for the process to start if needed, or just notify
Write-Host "🌐 Link: http://localhost:3000" -ForegroundColor Green
