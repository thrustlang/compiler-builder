use std::io::BufRead;
use std::io::BufReader;
use std::path::Path;
use std::path::PathBuf;
use std::process::Command;
use std::process::Stdio;
use std::thread;

use isahc::Body;
use isahc::HttpClient;
use isahc::ReadResponseExt;
use isahc::Response;
use isahc::config::Configurable;
use isahc::config::RedirectPolicy;

use crate::logging;

const DEFAULT_GCC_SOURCE_URL: &str =
    "https://github.com/gcc-mirror/gcc/archive/refs/tags/releases/gcc-15.2.0.tar.gz";

#[derive(Debug)]
pub struct GCCBuild {
    major: u32,
    minor: u32,
    patch: u32,

    url: String,
    host_shared: bool,

    c_compiler_command: String,
    cpp_compiler_command: String,

    c_compiler_flags: String,
    cpp_compiler_flags: String,

    debug_commands: bool,
}

impl GCCBuild {
    #[inline]
    pub fn new() -> Self {
        GCCBuild {
            major: 15,
            minor: 2,
            patch: 0,

            url: DEFAULT_GCC_SOURCE_URL.to_string(),
            host_shared: true,

            c_compiler_command: String::new(),
            cpp_compiler_command: String::new(),

            c_compiler_flags: String::new(),
            cpp_compiler_flags: String::new(),

            debug_commands: false,
        }
    }
}

impl GCCBuild {
    #[inline]
    pub fn set_major(&mut self, major: u32) {
        self.major = major;
    }

    pub fn set_minor(&mut self, minor: u32) {
        self.minor = minor;
    }

    #[inline]
    pub fn set_patch(&mut self, patch: u32) {
        self.patch = patch;
    }

    #[inline]
    pub fn set_host_shared(&mut self, host_shared: bool) {
        self.host_shared = host_shared;
    }

    #[inline]
    pub fn set_c_compiler_flags(&mut self, flags: String) {
        self.c_compiler_flags = flags;
    }

    #[inline]
    pub fn set_cpp_compiler_flags(&mut self, flags: String) {
        self.cpp_compiler_flags = flags;
    }

    #[inline]
    pub fn set_c_compiler_command(&mut self, command: String) {
        self.c_compiler_command = command;
    }

    #[inline]
    pub fn set_cpp_compiler_command(&mut self, command: String) {
        self.cpp_compiler_command = command;
    }

    #[inline]
    pub fn set_debug_commands(&mut self, debug_commands: bool) {
        self.debug_commands = debug_commands;
    }

    #[inline]
    pub fn setup_all(&mut self) {
        self.url = format!(
            "https://github.com/gcc-mirror/gcc/archive/refs/tags/releases/gcc-{}.{}.{}.tar.gz",
            self.major(),
            self.minor(),
            self.patch()
        );

        if !self.c_compiler_command().is_empty() {
            unsafe { std::env::set_var("CC", self.c_compiler_command()) };
        }

        if !self.cpp_compiler_command().is_empty() {
            unsafe { std::env::set_var("CXX", self.cpp_compiler_command()) };
        }

        if !self.c_compiler_flags().is_empty() {
            unsafe { std::env::set_var("CFLAGS", self.c_compiler_flags()) };
        }

        if !self.cpp_compiler_flags().is_empty() {
            unsafe { std::env::set_var("CXXFLAGS", self.cpp_compiler_flags()) };
        }
    }
}

impl GCCBuild {
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
    pub fn url(&self) -> &str {
        &self.url
    }

    #[inline]
    pub fn host_shared(&self) -> bool {
        self.host_shared
    }

    #[inline]
    pub fn c_compiler_flags(&self) -> &str {
        &self.c_compiler_flags
    }

    #[inline]
    pub fn cpp_compiler_flags(&self) -> &str {
        &self.cpp_compiler_flags
    }

    #[inline]
    pub fn c_compiler_command(&self) -> &str {
        &self.c_compiler_command
    }

    #[inline]
    pub fn cpp_compiler_command(&self) -> &str {
        &self.cpp_compiler_command
    }

    #[inline]
    pub fn debug_commands(&self) -> bool {
        self.debug_commands
    }
}

pub fn download_gcc(gcc_build: &GCCBuild) -> Result<PathBuf, String> {
    let client: HttpClient = HttpClient::builder()
        .redirect_policy(RedirectPolicy::Follow)
        .build()
        .map_err(|e| format!("Failed to create HTTP client: {}", e))?;

    let tmp_path: PathBuf = self::get_system_temp_dir();

    let name: String = format!(
        "gcc-releases-gcc-{}.{}.{}.tar.gz",
        gcc_build.major(),
        gcc_build.minor(),
        gcc_build.patch()
    );

    let full_path: PathBuf = tmp_path.join(name);
    let gcc_url: &str = gcc_build.url();

    let mut response: Response<Body> = client
        .get(gcc_url)
        .map_err(|e| format!("Failed to download {}: {}", gcc_url, e))?;

    if !response.status().is_success() {
        return Err(format!(
            "Failed to download {}: HTTP {}",
            gcc_url,
            response.status()
        ));
    }

    let bytes: Vec<u8> = response
        .bytes()
        .map_err(|e| format!("Failed to read response for {}: {}", gcc_url, e))?;

    let mut file: std::fs::File = std::fs::File::create(&full_path)
        .map_err(|e| format!("Failed to create file {:?}: {}", full_path, e))?;

    std::io::Write::write_all(&mut file, &bytes)
        .map_err(|e| format!("Failed to write to file {:?}: {}", full_path, e))?;

    Ok(full_path)
}

pub fn decompress_gcc(gcc_build: &GCCBuild, gcc_archive_path: &Path) -> Result<PathBuf, String> {
    let mut tar_command: Command = Command::new("tar");

    tar_command
        .arg("-xf")
        .arg(gcc_archive_path)
        .arg("-C")
        .arg(self::get_system_temp_dir());

    if gcc_build.debug_commands() {
        logging::log(
            logging::LoggingType::Debug,
            &format!("Executing tar command: {:?}", tar_command),
        );
    }

    if tar_command
        .status()
        .map_err(|e| format!("Failed to execute tar: {}", e))?
        .success()
    {
        Ok(self::get_system_temp_dir().join(self::get_descompressed_folder_directory(gcc_build)))
    } else {
        Err("Failed to decompress GCC archive".into())
    }
}

fn run_command_with_live_output(
    cmd: &mut Command,
    gcc_archive_path: &Path,
    gcc_source: &Path,
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
        self::clear_gcc_build(gcc_archive_path, gcc_source);
        Err(format!("Command failed with status: {}", status))
    }
}

pub fn build_and_install(
    gcc_build: &GCCBuild,
    gcc_archive_path: PathBuf,
    gcc_source: PathBuf,
) -> Result<(), String> {
    let build_dir: PathBuf = gcc_source.join("build");

    let previous_current_dir: PathBuf =
        std::env::current_dir().map_err(|_| "Failed to get current dir path!")?;

    std::env::set_current_dir(build_dir).map_err(|_| "Failed to set current dir!")?;

    let mut configure_binding: Command = Command::new("../configure");

    let configure_command: &mut Command = configure_binding
        .arg("--enable-languages=jit")
        .arg("--disable-bootstrap");

    if gcc_build.host_shared() {
        configure_command.arg("--enable-host-shared");
    }

    if gcc_build.debug_commands() {
        logging::log(
            logging::LoggingType::Debug,
            &format!("Executing GNU configure command: {:?}", configure_command),
        );
    }

    let mut make_command: Command = Command::new("make");

    if gcc_build.debug_commands() {
        logging::log(
            logging::LoggingType::Debug,
            &format!("Executing GNU make command: {:?}", make_command),
        );
    }

    self::run_command_with_live_output(configure_command, &gcc_archive_path, &gcc_source)?;
    self::run_command_with_live_output(&mut make_command, &gcc_archive_path, &gcc_source)?;

    std::env::set_current_dir(previous_current_dir).map_err(|_| "Failed to set current dir!")?;

    Ok(())
}

pub fn prepare_build_directory(gcc_source: &Path) -> Result<(), String> {
    let build_dir: PathBuf = gcc_source.join("build");

    std::fs::create_dir_all(&build_dir).map_err(|_| "Failed to create gcc build directory!")?;

    Ok(())
}

fn clear_gcc_build(gcc_archive_path: &Path, gcc_source: &Path) {
    let _ = std::fs::remove_file(gcc_archive_path);
    let _ = std::fs::remove_dir_all(gcc_source);
}

fn get_system_temp_dir() -> PathBuf {
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

fn get_descompressed_folder_directory(gcc_build: &GCCBuild) -> String {
    format!(
        "gcc-releases-gcc-{}.{}.{}",
        gcc_build.major(),
        gcc_build.minor(),
        gcc_build.patch()
    )
}
