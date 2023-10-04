title build lazuli, compile test file and run test file

:: Set environment to 64 bit (so the linker works)
@REM call "C:\Program Files (x86)\Microsoft Visual Studio\2022\BuildTools\VC\Auxiliary\Build\vcvars64.bat"

cargo build
@REM lazuli .\src\test_lzl\lazuli_test.lzl -o .\src\test_lzl\test1.obj -debug -k
@REM lazuli .\src\test_lzl\lazuli_test.lzl -o .\src\test_lzl\test2.asm
lazuli .\src\test_lzl\lazuli_test -out .\src\test_lzl\lazulifile.exe -d -k
.\src\test_lzl\lazulifile.exe
:: $LASTEXITCODE
echo Last return code %ERRORLEVEL%