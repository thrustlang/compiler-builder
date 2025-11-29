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

pub fn get_compiler_dependencies_build_path() -> PathBuf {
    match std::env::consts::FAMILY {
        "unix" => PathBuf::from(std::env::var("HOME").unwrap_or_else(|_| {
            logging::log(LoggingType::Panic, "Missing $HOME environment variable.\n");
            std::process::exit(1);
        }))
        .join(".thrushlang/backends/llvm/build"),

        "windows" => PathBuf::from(std::env::var("APPDATA").unwrap_or_else(|_| {
            logging::log(
                LoggingType::Panic,
                "Missing $APPDATA environment variable.\n",
            );
            std::process::exit(1);
        }))
        .join(".thrushlang/backends/llvm/build"),

        _ => {
            logging::log(
                LoggingType::Panic,
                "Unsopported OS for build Thrush Programming Language backend build.",
            );

            std::process::exit(1);
        }
    }
}
