cmake_minimum_required(VERSION 3.16)
project("CLion/CMake example to call Rust lib on Windows" C)

set(CMAKE_C_STANDARD 11)

# THIS PROBABLY NOT WORK IN CLION AND I DON'T KNOW WHY
# CLION USES THE MSVC TOOLCHAIN FROM
# C:\Program Files (x86)\Microsoft Visual Studio\2019\BuildTools\VC\Tools\MSVC\14.28.29333\bin\Hostx86
# INSTEAD OF ..bin\Hostx64
# BUT BECAUSE CARGO BUILDS AN X64-LIB THE LINKING FAILS... but this works on command line :)

add_executable(main main.c)

# trigger cargo build so that *.dll is generated
add_custom_target(
        "CARGO_BUILD"
        COMMAND cargo build
        COMMAND cargo build --release
        WORKING_DIRECTORY "${PROJECT_SOURCE_DIR}/../..")
add_dependencies(main CARGO_BUILD)

# this is needed for compilation
target_link_libraries(main ${CMAKE_CURRENT_SOURCE_DIR}/../../target/release/text_loading_animation.dll.lib)

# copy DLL file into target dir because it needs to be in the same directory during runtime
# (Windows specific thing)
add_custom_command(TARGET main POST_BUILD                  # Adds a post-build event to "main"
        COMMAND ${CMAKE_COMMAND} -E copy_if_different      # which executes "cmake - E copy_if_different..."
        "${PROJECT_SOURCE_DIR}/text_loading_animation.dll" # <--this is in-file
        $<TARGET_FILE_DIR:main>)                           # <--this is out-file path

