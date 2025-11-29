<img src= "https://github.com/thrushlang/.github/blob/main/assets/logos/thrushlang-logo.png" alt= "logo" style= "width: 80%; height: 80%;"></img>

# Compiler Builder | Commands & Flags

A list of the commands supported by the compiler builder command line.

> [!WARNING]  
> This might be a bit outdated, it could be information that's somewhat distant from the changes.

```console
The Compiler Builder

Usage: compiler-builder [-flags]

Commands:

• -h, --help, help Show help message.
• -v, --version, version Show the version.

LLVM Build flags:

• -llvm-major Set LLVM major version (default: 17).
• -llvm-minor Set LLVM minor version (default: 0).
• -llvm-patch Set LLVM patch version (default: 6).
• -llvm-c-compiler Set C compiler for LLVM build (default: clang).
• -llvm-cpp-compiler Set C++ compiler for LLVM build (default: clang++).
• -llvm-c-flags Set C flags for LLVM build.
• -llvm-cpp-flags Set C++ flags for LLVM build.
• -llvm-release-type Set LLVM release type (Debug, Release, MinSizeRel) (default: Release).
• -llvm-build-share-libs Build LLVM shared libraries (default: false).
• -llvm-build-x86-libs Build x86 (32-bit) libraries for LLVM (default: false).
• -llvm-build-dylib Build LLVM dynamic library (default: false).
• -llvm-link-statically-libcpp Link libcpp statically (default: false).
• -llvm-use-linker Set linker to use for LLVM build.
```