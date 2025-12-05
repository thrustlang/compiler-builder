<img src= "https://github.com/thrushlang/.github/blob/main/assets/logos/thrushlang-logo.png" alt= "logo" style= "width: 80%; height: 80%;"></img>

# Compiler Builder | Commands & Flags

A list of the commands supported by the compiler builder command line.

> [!WARNING]  
> This might be a bit outdated, it could be information that's somewhat distant from the changes.

```console
The Compiler Builder

Usage: compiler-builder [-flag|--flags]

Commands:

• -h, --help, help Show help message.
• -v, --version, version Show the version.

LLVM build flags:

• --llvm-major Set LLVM major version (default: 17).
• --llvm-minor Set LLVM minor version (default: 0).
• --llvm-patch Set LLVM patch version (default: 6).
• --llvm-c-compiler [clang] Set C compiler for LLVM build (default: clang).
• --llvm-cpp-compiler [clang++] Set C++ compiler for LLVM build (default: clang++).
• --llvm-c-flags [-O3] Set C compiler flags for LLVM build.
• --llvm-cpp-flags [-Oz] Set C++ compiler flags for LLVM build.
• --llvm-release-type [Debug|Release|MinSizeRel] Set LLVM release type (Debug, Release, MinSizeRel) (default: Release).
• --llvm-build-share-libs [true|false] Flag indicating if each LLVM component (e.g. Support) is built as a shared library (ON) or as a static library (OFF). Its default value is OFF. On Windows, shared libraries may be used when building with MinGW, including mingw-w64, but not when building with the Microsoft toolchain. s(default: false).
• --llvm-build-x86-libs [true|false] Build 32-bit executables and libraries on 64-bit systems. This option is available only on some 64-bit Unix systems. (default: false).
• --llvm-build-dylib [true|false] If enabled, the target for building the libLLVM shared library is added. This library contains all of LLVM’s components in a single shared library. Defaults to OFF. This cannot be used in conjunction with BUILD_SHARED_LIBS. Tools will only be linked to the libLLVM shared library if LLVM_LINK_LLVM_DYLIB is also ON. The components in the library can be customised by setting LLVM_DYLIB_COMPONENTS to a list of the desired components. This option is not available on Windows. (default: false).
• --llvm-link-statically-libcpp [true|false] Statically link to the C++ standard library if possible. This uses the flag -static-libstdc++, but a Clang host compiler will statically link to libc++ if used in conjunction with the LLVM_ENABLE_LIBCXX flag. Defaults to OFF. (default: false).
• --llvm-use-linker [lld] Override the system’s default linker. For instance, use lld with -DLLVM_USE_LINKER=lld.
• --llvm-use-llvm-libc [true|false] If the LLVM libc overlay is installed in a location where the host linker can access it, all built executables will be linked against the LLVM libc overlay before linking against the system libc. (default: false).
• --llvm-pic [true|false] Add the -fPIC flag to the compiler command-line, if the compiler supports this flag. Some systems, like Windows, do not need this flag (default: true).
• --llvm-libcpp [true|false] If the host compiler and linker support the stdlib flag, -stdlib=libc++ is passed to invocations of both so that the project is built using libc++ instead of stdlibc++ (default: false).
• --llvm-clang-modules [true|false] Compile with Clang Header Modules. (default: false).
• --llvm-pdb [true|false] For Windows builds using MSVC or clang-cl, generate PDB files when CMAKE_BUILD_TYPE is set to Release. (default: false).
• --llvm-temporarily-old-toolchain [true|false] If enabled, the compiler version check will only warn when using a toolchain which is about to be deprecated, instead of emitting an error. (default: false).
• --llvm-optimize-tblgen [true|false] If enabled and building a debug or assert build, the CMake build system will generate a Release build tree to build a fully optimized tablegen for use during the build. Enabling this option can significantly speed up build times, especially when building LLVM in Debug configurations. (default: false).

For more information: https://llvm.org/docs/CMake.html

GCC build flags:

• -gcc Enable to build GCC backend for the compiler.
• --gcc-major Set GCC major version (default: 15).
• --gcc-minor Set GCC minor version (default: 2).
• --gcc-patch Set GCC patch version (default: 0).
• --gcc-host-shared [true|false] Enable host shared for GCC (default: true).
• --gcc-c-compiler-flags [-O2 -g] Set C compiler flags for GCC build.
• --gcc-cpp-compiler-flags [-O2 -g] Set C++ compiler flags for GCC build.
• --gcc-c-compiler-command [gcc] Set C compiler command for GCC build.
• --gcc-cpp-compiler-command [g++] Set C++ compiler command for GCC build.

For more information: https://gcc.gnu.org/onlinedocs/jit/internals/index.html#working-on-the-jit-library

Useful flags:

• --debug-llvm Debug LLVM build commands.
• --debug-gcc Debug GCC build commands.
```