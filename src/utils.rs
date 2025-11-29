use std::process::Command;

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
