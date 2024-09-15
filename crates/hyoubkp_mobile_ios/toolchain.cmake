set(CMAKE_SYSTEM_NAME Darwin)
set(CMAKE_SYSTEM_PROCESSOR arm64)

set(CMAKE_SYSROOT "$ENV{SDKROOT}")
set(prefix arm-apple-darwin11-)
set(toolchain_bin_dir "$ENV{SDKROOT}/../../bin")

SET(CMAKE_CROSSCOMPILING ON)

set(CMAKE_C_COMPILER ${toolchain_bin_dir}/${prefix}clang)
set(CMAKE_CXX_COMPILER ${toolchain_bin_dir}/${prefix}clang++)
set(CMAKE_OBJC_COMPILER ${toolchain_bin_dir}/${prefix}clang)

set(CMAKE_C_COMPILER_WORKS 1)
set(CMAKE_CXX_COMPILER_WORKS 1)
set(CMAKE_OBJC_COMPILER_WORKS 1)

set(CMAKE_FIND_ROOT_PATH_MODE_PROGRAM NEVER)
set(CMAKE_FIND_ROOT_PATH_MODE_LIBRARY ONLY)
set(CMAKE_FIND_ROOT_PATH_MODE_INCLUDE ONLY)
set(CMAKE_FIND_ROOT_PATH_MODE_PACKAGE ONLY)

set(CMAKE_AR ${toolchain_bin_dir}/${prefix}ar)
set(CMAKE_RANLIB ${toolchain_bin_dir}/${prefix}ranlib)
set(CMAKE_INSTALL_NAME_TOOL ${toolchain_bin_dir}/${prefix}install_name_tool)
