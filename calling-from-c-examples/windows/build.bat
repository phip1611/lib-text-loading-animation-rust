@echo off

del main.exe main.obj *.dll *.dll.lib

cd ..\..\
cargo build --release
cd "calling-from-c-examples/windows"
xcopy /y ..\..\target\release\text_loading_animation.dll .
xcopy /y ..\..\target\release\text_loading_animation.dll.lib .


rem: sets up environment: it's a complicated process for Windows where a lot of env vars must be set
rem: see: https://docs.microsoft.com/de-de/cpp/build/building-on-the-command-line?view=vs-2019
rem: see: https://docs.microsoft.com/de-de/cpp/build/walkthrough-compile-a-c-program-on-the-command-line?view=vs-2019

rem: doesn't work: header files are not found: call "C:\Program Files (x86)\Microsoft Visual C++ Build Tools\vcbuildtools.bat"
IF NOT DEFINED VCToolsInstallDir (
    rem: we just check if any of the vars that should be defined is defined; in this case we use "VCToolsInstallDir"
    rem: to prevent this to be run multiple time in the same terminal (otherwise PATH gets too long)
    call "C:\Program Files (x86)\Microsoft Visual Studio\2019\Community\VC\Auxiliary\Build\vcvars64.bat"
)
rem: *.dll.lib contains header information during compile; *.dll is the runtime file that must be in the same directory/path

rem: "cl" is windows compiler: produces main.obj and main.exe; main.obj is not needed in the end;
rem:   it is only for the stage between compiling and linking
rem: /W4 is highest warnings flag
cl /W4 main.c /link text_loading_animation.dll.lib
del main.obj
