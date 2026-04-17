@echo off
setlocal
echo 🧬 OMNI-KERNEL RELEASE BUILDER 🧬
echo -----------------------------------

:: Check for cargo
where cargo >nul 2>nul
if %errorlevel% neq 0 (
    echo [ERROR] Rust/Cargo not found. Please install from https://rustup.rs
    pause
    exit /b 1
)

echo [1/3] 🔨 Building optimized binary (Release mode)...
cargo build --release

if %errorlevel% neq 0 (
    echo [ERROR] Build failed. Check the errors above.
    pause
    exit /b 1
)

echo [2/3] 📦 Collecting artifacts...
if not exist "dist" mkdir dist
copy /Y "target\release\omni-kernel.exe" "dist\"
xcopy /E /I /Y "dashboard" "dist\dashboard"

echo [3/3] ✨ Cleaning up...
echo -----------------------------------
echo ✅ SUCCESS! 
echo.
echo The production-ready files are in the 'dist' folder:
echo 1. omni-kernel.exe (The Engine)
echo 2. dashboard/ (The UI files)
echo.
echo 🚀 To share: Zip the 'dist' folder and send it.
echo 💡 To run: Just open 'omni-kernel.exe' inside 'dist'.
echo -----------------------------------
pause
