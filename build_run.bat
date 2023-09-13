title build lazuli, compile test file and run test file

:: Set environment to 64 bit (so the linker works)
@REM call "C:\Program Files (x86)\Microsoft Visual Studio\2022\BuildTools\VC\Auxiliary\Build\vcvars64.bat"

cargo build
lazuli .\src\test_lzl\lazuli_test.lzl
.\src\test_lzl\lazuli_test.exe
:: $LASTEXITCODE
echo Last return code %ERRORLEVEL%