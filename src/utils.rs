use std::{path::PathBuf, process::Command};

use crate::logging::{self, LoggingType};

#[inline]
pub fn tar_is_available() -> bool {
    Command::new("tar").arg("--version").output().is_ok()
}

#[inline]
pub fn cmake_is_available() -> bool {
    Command::new("cmake").arg("--version").output().is_ok()
}

#[inline]
pub fn ninja_is_available() -> bool {
    Command::new("ninja").arg("--version").output().is_ok()
}

pub fn get_compiler_llvm_build_path() -> PathBuf {
    match std::env::consts::FAMILY {
        "unix" => PathBuf::from(std::env::var("HOME").unwrap_or_else(|_| {
            logging::log(LoggingType::Panic, "Missing $HOME environment variable.\n");
            std::process::exit(1);
        }))
        .join(".thrustlang/backends/llvm/build"),

        "windows" => PathBuf::from(std::env::var("APPDATA").unwrap_or_else(|_| {
            logging::log(
                LoggingType::Panic,
                "Missing $APPDATA environment variable.\n",
            );
            std::process::exit(1);
        }))
        .join(".thrustlang/backends/llvm/build"),

        _ => {
            logging::log(
                LoggingType::Panic,
                "Unsopported operating system for installing the dependencies required to build the Thrust Compiler LLVM backend.",
            );

            std::process::exit(1);
        }
    }
}

pub fn get_compiler_clang_build_path() -> PathBuf {
    match std::env::consts::FAMILY {
        "unix" => PathBuf::from(std::env::var("HOME").unwrap_or_else(|_| {
            logging::log(LoggingType::Panic, "Missing $HOME environment variable.\n");
            std::process::exit(1);
        }))
        .join(".thrustlang/backends/cbindgen/build"),

        "windows" => PathBuf::from(std::env::var("APPDATA").unwrap_or_else(|_| {
            logging::log(
                LoggingType::Panic,
                "Missing $APPDATA environment variable.\n",
            );
            std::process::exit(1);
        }))
        .join(".thrustlang/backends/cbindgen/build"),

        _ => {
            logging::log(
                LoggingType::Panic,
                "Unsopported operating system for installing the dependencies required to build the Thrust Compiler CBindgen.",
            );

            std::process::exit(1);
        }
    }
}

#[inline]
pub fn reset_compiler_llvm_build_path() {
    let _ = std::fs::remove_dir(self::get_compiler_llvm_build_path());
    let _ = std::fs::create_dir_all(self::get_compiler_llvm_build_path());
}

#[inline]
pub fn reset_compiler_clang_build_path() {
    let _ = std::fs::remove_dir(self::get_compiler_clang_build_path());
    let _ = std::fs::create_dir_all(self::get_compiler_clang_build_path());
}
