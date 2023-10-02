
@REM nasm -f win64 -o .\src\test_lzl\asm_run.obj .\src\test_lzl\lazulifile2.asm
nasm -f win64 -o .\src\test_lzl\asm_run.obj .\testing_stuff\src\hello_world.asm

call "C:\\Program Files (x86)\\Microsoft Visual Studio\\2022\\BuildTools\\VC\\Auxiliary\\Build\\vcvars64.bat"
                
link .\src\test_lzl\asm_run.obj /subsystem:console /out:.\src\test_lzl\asm_run.exe kernel32.lib legacy_stdio_definitions.lib msvcrt.lib
        
.\src\test_lzl\asm_run.exe

echo Last return code %ERRORLEVEL%