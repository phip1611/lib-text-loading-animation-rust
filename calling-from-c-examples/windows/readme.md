# Calling library *text-loading-animation* from C on Windows.

## Prerequisites
- this special setup (MSVC - Visual Studio Compiler Toolchain) works only on Windows 
  (tested with Windows 10 and Visual Studio Build Tools 2019)
- two ways of execution are provided:
  1) CLI (via batch script) as well as 
  2) CMake (to use in Jetbrains CLion)
- unlike GCC on UNIX, the setup on Windows is quite different; one must install Visual Studio (Build Tools)
  and have a "properly configured developer prompt" (see https://docs.microsoft.com/de-de/cpp/build/building-on-the-command-line?view=vs-2019)
  - see how it is done in `build_and_run.bat` for example
- Rust builds a `*.dll-file` (=dynamically linkable library during runtime) and a `*.dll.lib`-file (for linking during compilation)

## How it works
- `cl` (Windows Compiler) needs the `.lib`-File during linking
- the windows runtime needs the `.dll`-File during execution. It must be in the same directory!
- in `main.c` there is a reference to an external function: see `main.c`
- when `main.exe` is executed the dll-file must be in the same location (directory)

## Hot to start/run
- A: either in Jetbrains CLion (just add the CMake-project and run main)
   - run `build.bat` first; it copies the lib into this directory
- B: on the command line with the help of the script `build_and_run.bat`