use std::io::{BufRead, BufReader};
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};
use std::thread;

use isahc::Body;
use isahc::HttpClient;
use isahc::ReadResponseExt;
use isahc::Response;
use isahc::config::Configurable;
use isahc::config::RedirectPolicy;

use crate::logging;
use crate::logging::LoggingType;

#[derive(Debug)]
pub struct LLVMBuild {
    major: u32,
    minor: u32,
    patch: u32,

    cflags: String,
    cppflags: String,

    c_compiler: String,
    cpp_compiler: String,

    release_type: LLVMReleaseType,

    url: String,
}

impl LLVMBuild {
    #[inline]
    pub fn new() -> Self {
        Self {
            major: 17,
            minor: 0,
            patch: 6,

            c_compiler: "gcc".into(),
            cpp_compiler: "g++".into(),

            cflags: String::default(),
            cppflags: String::default(),

            release_type: LLVMReleaseType::Release,

            url: self::get_default_llvm_url(),
        }
    }
}

impl LLVMBuild {
    #[inline]
    pub fn set_major(&mut self, major: u32) {
        self.major = major;
    }

    #[inline]
    pub fn set_minor(&mut self, minor: u32) {
        self.minor = minor;
    }

    #[inline]
    pub fn set_patch(&mut self, patch: u32) {
        self.patch = patch;
    }

    #[inline]
    pub fn set_c_compiler(&mut self, c_compiler: String) {
        self.c_compiler = c_compiler;
    }

    #[inline]
    pub fn set_cpp_compiler(&mut self, cpp_compiler: String) {
        self.cpp_compiler = cpp_compiler;
    }

    #[inline]
    pub fn set_release_type(&mut self, release_type: LLVMReleaseType) {
        self.release_type = release_type;
    }

    #[inline]
    pub fn set_c_flags(&mut self, cflags: String) {
        self.cflags = cflags;
    }

    #[inline]
    pub fn set_cpp_flags(&mut self, cppflags: String) {
        self.cppflags = cppflags;
    }

    #[inline]
    pub fn set_url(&mut self) {
        self.url = format!(
            "https://github.com/llvm/llvm-project/releases/download/llvmorg-{}.{}.{}/llvm-project-{}.{}.{}.src.tar.xz",
            self.major(),
            self.minor(),
            self.patch(),
            self.major(),
            self.minor(),
            self.patch()
        )
    }
}

impl LLVMBuild {
    #[inline]
    pub fn major(&self) -> u32 {
        self.major
    }

    #[inline]
    pub fn minor(&self) -> u32 {
        self.minor
    }

    #[inline]
    pub fn patch(&self) -> u32 {
        self.patch
    }

    #[inline]
    pub fn c_compiler(&self) -> &str {
        &self.c_compiler
    }

    #[inline]
    pub fn cpp_compiler(&self) -> &str {
        &self.cpp_compiler
    }

    #[inline]
    pub fn release_type(&self) -> &LLVMReleaseType {
        &self.release_type
    }

    #[inline]
    pub fn cpp_flags(&self) -> &str {
        &self.cppflags
    }

    #[inline]
    pub fn c_flags(&self) -> &str {
        &self.cflags
    }

    #[inline]
    pub fn url(&self) -> &str {
        &self.url
    }
}

#[derive(Debug, Default)]
pub enum LLVMReleaseType {
    Debug,

    #[default]
    Release,

    MinSizeRel,
}

//------------------------------------------------------------------------------------------------------------------

impl LLVMReleaseType {
    #[inline]
    pub fn get_repr(&self) -> &str {
        match self {
            LLVMReleaseType::Debug => "Debug",
            LLVMReleaseType::Release => "Release",
            LLVMReleaseType::MinSizeRel => "MinSizeRel",
        }
    }
}

pub fn download_llvm(llvm_build: &LLVMBuild) -> Result<PathBuf, String> {
    let client: HttpClient = HttpClient::builder()
        .redirect_policy(RedirectPolicy::Follow)
        .build()
        .map_err(|e| format!("Failed to create HTTP client: {}", e))?;

    let tmp_path: PathBuf = self::system_temp_dir();

    let name: String = format!(
        "llvm-project-{}.{}.{}.src.tar.xz",
        llvm_build.major, llvm_build.minor, llvm_build.patch
    );

    let full_path: PathBuf = tmp_path.join(name);

    let llvm_url: &str = llvm_build.url();

    let mut response: Response<Body> = client
        .get(llvm_url)
        .map_err(|e| format!("Failed to download {}: {}", llvm_url, e))?;

    if !response.status().is_success() {
        return Err(format!(
            "Failed to download {}: HTTP {}",
            llvm_url,
            response.status()
        ));
    }

    let bytes: Vec<u8> = response
        .bytes()
        .map_err(|e| format!("Failed to read response for {}: {}", llvm_url, e))?;

    let mut file: std::fs::File = std::fs::File::create(&full_path)
        .map_err(|e| format!("Failed to create file {:?}: {}", full_path, e))?;

    std::io::Write::write_all(&mut file, &bytes)
        .map_err(|e| format!("Failed to write to file {:?}: {}", full_path, e))?;

    Ok(full_path)
}

pub fn decompress_llvm(
    llvm_build: &LLVMBuild,
    llvm_archive_path: &Path,
) -> Result<PathBuf, String> {
    let mut tar_command: Command = Command::new("tar");

    tar_command
        .arg("-xf")
        .arg(llvm_archive_path)
        .arg("-C")
        .arg(self::system_temp_dir());

    if tar_command
        .status()
        .map_err(|e| format!("Failed to execute tar: {}", e))?
        .success()
    {
        Ok(self::system_temp_dir().join(self::get_descompressed_folder_directory(llvm_build)))
    } else {
        Err("Failed to decompress LLVM archive".into())
    }
}

pub fn prepare_build_directory(llvm_source: &Path) -> Result<(), String> {
    let build_dir: PathBuf = llvm_source.join("llvm").join("build");

    std::fs::create_dir_all(&build_dir).map_err(|_| "Failed to create llvm build directory!")?;

    Ok(())
}

fn run_command_with_live_output(
    cmd: &mut Command,
    llvm_archive_path: &Path,
    llvm_source: &Path,
) -> Result<(), String> {
    cmd.stdout(Stdio::piped());
    cmd.stderr(Stdio::piped());

    let mut child: std::process::Child = cmd
        .spawn()
        .map_err(|e| format!("Failed to spawn process: {e}"))?;

    let stdout: std::process::ChildStdout = child.stdout.take().unwrap();
    let stderr: std::process::ChildStderr = child.stderr.take().unwrap();

    let stdout_thread: thread::JoinHandle<()> = thread::spawn(move || {
        let reader = BufReader::new(stdout);
        for line in reader.lines().map_while(Result::ok) {
            println!("{}", line);
        }
    });

    let stderr_thread: thread::JoinHandle<()> = thread::spawn(move || {
        let reader: BufReader<std::process::ChildStderr> = BufReader::new(stderr);
        for line in reader.lines().map_while(Result::ok) {
            eprintln!("{}", line);
        }
    });

    let status: std::process::ExitStatus = child
        .wait()
        .map_err(|e| format!("Failed to wait on child: {e}"))?;

    let _ = stdout_thread.join();
    let _ = stderr_thread.join();

    if status.success() {
        Ok(())
    } else {
        self::clear_llvm_build(llvm_archive_path, llvm_source);
        Err(format!("Command failed with status: {}", status))
    }
}

pub fn build_and_install(
    llvm_build: &LLVMBuild,
    llvm_archive_path: PathBuf,
    llvm_source: PathBuf,
) -> Result<(), String> {
    let build_dir: PathBuf = llvm_source.join("llvm").join("build");
    let parent: &Path = build_dir.parent().unwrap_or(&build_dir);
    let install_dir: PathBuf = self::get_thrushlang_install_path();

    run_command_with_live_output(
        Command::new("cmake")
            .arg("-G")
            .arg("Ninja")
            .arg("-S")
            .arg(parent)
            .arg("-B")
            .arg(&build_dir)
            .arg(format!(
                "-DCMAKE_BUILD_TYPE={}",
                llvm_build.release_type().get_repr()
            ))
            .arg(format!("-DCMAKE_C_COMPILER={}", llvm_build.c_compiler()))
            .arg(format!(
                "-DCMAKE_CXX_COMPILER={}",
                llvm_build.cpp_compiler()
            ))
            .arg(format!("-DCMAKE_C_FLAGS={}", llvm_build.c_flags()))
            .arg(format!("-DCMAKE_CXX_FLAGS={}", llvm_build.cpp_flags()))
            .arg("-DCMAKE_DISABLE_FIND_PACKAGE_LibXml2=TRUE")
            .arg("-DLLVM_ENABLE_LIBXML2=0")
            .arg("-DLLVM_TARGETS_TO_BUILD=all")
            .arg("-DLLVM_ENABLE_PROJECTS=llvm")
            .arg("-DLLVM_ENABLE_TERMINFO=OFF")
            .arg("-DLLVM_ENABLE_ZLIB=OFF")
            .arg(format!("-DCMAKE_INSTALL_PREFIX={}", install_dir.display()))
            .args([
                "-DLLVM_INCLUDE_BENCHMARKS=OFF",
                "-DLLVM_BUILD_TESTS=OFF",
                "-DLLVM_BUILD_EXAMPLES=OFF",
                "-DLLVM_INCLUDE_TESTS=OFF",
            ]),
        &llvm_archive_path,
        &llvm_source,
    )?;

    run_command_with_live_output(
        Command::new("ninja").arg("-C").arg(&build_dir),
        &llvm_archive_path,
        &llvm_source,
    )?;

    run_command_with_live_output(
        Command::new("ninja")
            .arg("-C")
            .arg(&build_dir)
            .arg("install"),
        &llvm_archive_path,
        &llvm_source,
    )?;

    Ok(())
}

fn clear_llvm_build(llvm_archive_path: &Path, llvm_source: &Path) {
    let _ = std::fs::remove_file(llvm_archive_path);
    let _ = std::fs::remove_dir_all(llvm_source);
}

fn get_thrushlang_install_path() -> PathBuf {
    match std::env::consts::FAMILY {
        "unix" => PathBuf::from(std::env::var("HOME").unwrap_or_else(|_| {
            logging::log(LoggingType::Panic, "Missing $HOME environment variable.\n");
            std::process::exit(1);
        }))
        .join("thrushlang/backends/llvm/build"),

        "windows" => PathBuf::from(std::env::var("APPDATA").unwrap_or_else(|_| {
            logging::log(
                LoggingType::Panic,
                "Missing $APPDATA environment variable.\n",
            );
            std::process::exit(1);
        }))
        .join("thrushlang/backends/llvm/build"),

        _ => {
            logging::log(
                LoggingType::Panic,
                "Unsopported OS for build Thrush Programming Language backend build.",
            );

            std::process::exit(1);
        }
    }
}

fn get_descompressed_folder_directory(llvm_build: &LLVMBuild) -> String {
    format!(
        "llvm-project-{}.{}.{}.src",
        llvm_build.major, llvm_build.minor, llvm_build.patch
    )
}

fn system_temp_dir() -> PathBuf {
    if let Ok(dir) = std::env::var("TMPDIR") {
        return PathBuf::from(dir);
    }
    if let Ok(dir) = std::env::var("TMP") {
        return PathBuf::from(dir);
    }
    if let Ok(dir) = std::env::var("TEMP") {
        return PathBuf::from(dir);
    }
    if let Ok(dir) = std::env::var("TEMPDIR") {
        return PathBuf::from(dir);
    }

    #[cfg(unix)]
    return PathBuf::from("/tmp");

    #[cfg(windows)]
    {
        if let Ok(userprofile) = std::env::var("USERPROFILE") {
            let mut path = PathBuf::from(userprofile);
            path.push("AppData");
            path.push("Local");
            path.push("Temp");
            return path;
        }
        return PathBuf::from(r"C:\Temp");
    }
}

fn get_default_llvm_url() -> String {
    format!(
        "https://github.com/llvm/llvm-project/releases/download/llvmorg-{}.{}.{}/llvm-project-{}.{}.{}.src.tar.xz",
        17, 0, 6, 17, 0, 6
    )
}
