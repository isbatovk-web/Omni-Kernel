@echo off
call "C:\Program Files (x86)\Microsoft Visual Studio\2022\BuildTools\VC\Auxiliary\Build\vcvars64.bat"
set PATH=%USERPROFILE%\.cargo\bin;%PATH%
cargo build --target x86_64-pc-windows-msvc > build.log 2>&1
echo Done building. Check build.log for details.
