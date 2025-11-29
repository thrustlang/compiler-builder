use crate::logging;

pub fn show_help() -> ! {
    logging::write(logging::OutputIn::Stderr, "The Compiler Builder");

    logging::write(
        logging::OutputIn::Stderr,
        &format!(
            "\n\n{} {} {}\n\n",
            "Usage:", "compiler-builder", "[--flags]"
        ),
    );

    logging::write(logging::OutputIn::Stderr, "Commands:\n\n");

    logging::write(
        logging::OutputIn::Stderr,
        &format!(
            "{} {}, {}, {} {}\n",
            "•", "-h", "--help", "help", "Show help message.",
        ),
    );

    logging::write(
        logging::OutputIn::Stderr,
        &format!(
            "{} {}, {}, {} {}\n\n",
            "•", "-v", "--version", "version", "Show the version.",
        ),
    );

    logging::write(logging::OutputIn::Stderr, "LLVM Build Commands:\n\n");

    logging::write(
        logging::OutputIn::Stderr,
        &format!(
            "{} {} {}\n",
            "•", "-llvm-major", "Set LLVM major version (default: 17).",
        ),
    );

    logging::write(
        logging::OutputIn::Stderr,
        &format!(
            "{} {} {}\n",
            "•", "-llvm-minor", "Set LLVM minor version (default: 0).",
        ),
    );

    logging::write(
        logging::OutputIn::Stderr,
        &format!(
            "{} {} {}\n",
            "•", "-llvm-patch", "Set LLVM patch version (default: 6).",
        ),
    );

    logging::write(
        logging::OutputIn::Stderr,
        &format!(
            "{} {} {}\n",
            "•", "-llvm-c-compiler", "Set C compiler for LLVM build (default: clang).",
        ),
    );

    logging::write(
        logging::OutputIn::Stderr,
        &format!(
            "{} {} {}\n",
            "•", "-llvm-cpp-compiler", "Set C++ compiler for LLVM build (default: clang++).",
        ),
    );

    logging::write(
        logging::OutputIn::Stderr,
        &format!(
            "{} {} {}\n",
            "•", "-llvm-c-flags", "Set C flags for LLVM build.",
        ),
    );

    logging::write(
        logging::OutputIn::Stderr,
        &format!(
            "{} {} {}\n",
            "•", "-llvm-cpp-flags", "Set C++ flags for LLVM build.",
        ),
    );

    logging::write(
        logging::OutputIn::Stderr,
        &format!(
            "{} {} {}\n",
            "•",
            "-llvm-release-type",
            "Set LLVM release type (Debug, Release, MinSizeRel) (default: Release).",
        ),
    );

    logging::write(
        logging::OutputIn::Stderr,
        &format!(
            "{} {} {}\n",
            "•", "-llvm-build-share-libs", "Build LLVM shared libraries (default: false).",
        ),
    );

    logging::write(
        logging::OutputIn::Stderr,
        &format!(
            "{} {} {}\n",
            "•", "-llvm-build-x86-libs", "Build x86 (32-bit) libraries for LLVM (default: false).",
        ),
    );

    logging::write(
        logging::OutputIn::Stderr,
        &format!(
            "{} {} {}\n",
            "•", "-llvm-build-dylib", "Build LLVM dynamic library (default: false).",
        ),
    );

    logging::write(
        logging::OutputIn::Stderr,
        &format!(
            "{} {} {}\n",
            "•", "-llvm-link-statically-libcpp", "Link libcpp statically (default: false).",
        ),
    );

    logging::write(
        logging::OutputIn::Stderr,
        &format!(
            "{} {} {}\n\n",
            "•", "-llvm-use-linker", "Set linker to use for LLVM build.",
        ),
    );

    std::process::exit(1);
}
