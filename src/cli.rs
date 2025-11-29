use crate::constants;
use crate::help;
use crate::llvm;
use crate::logging;
use crate::logging::LoggingType;
use crate::options::BuildOptions;
use crate::utils;

use std::process;

#[derive(Debug)]
pub struct CommandLine {
    options: BuildOptions,
    args: Vec<String>,
    current: usize,
}

#[derive(Debug)]
pub struct ParsedArg {
    key: String,
    value: Option<String>,
}

impl ParsedArg {
    fn new(arg: &str) -> Self {
        if let Some(eq_pos) = arg.find('=') {
            let (key, value) = arg.split_at(eq_pos);

            return Self {
                key: key.to_string(),
                value: Some(value[1..].to_string()),
            };
        }

        if let Some(eq_pos) = arg.find(':') {
            let (key, value) = arg.split_at(eq_pos);

            return Self {
                key: key.to_string(),
                value: Some(value[1..].to_string()),
            };
        }

        Self {
            key: arg.to_string(),
            value: None,
        }
    }
}

impl CommandLine {
    pub fn parse(mut args: Vec<String>) -> CommandLine {
        let processed_args: Vec<String> = Self::preprocess_args(&mut args);

        let mut command_line: CommandLine = Self {
            options: BuildOptions::new(),
            args: processed_args,
            current: 0,
        };

        command_line.build();

        command_line
    }
}

impl CommandLine {
    fn build(&mut self) {
        while !self.is_eof() {
            let argument: String = self.args[self.current].clone();
            self.analyze(argument);
        }

        self.check_requirements();

        self.prepare();
    }

    fn prepare(&mut self) {
        self.get_mut_options().get_mut_llvm_build().set_url();
    }

    fn check_requirements(&self) {
        if !utils::tar_is_available() {
            logging::log(LoggingType::Error, "tar is not installed.\n");
        }

        if !utils::cmake_is_available() {
            logging::log(LoggingType::Error, "cmake is not installed.\n");
        }

        if !utils::ninja_is_available() {
            logging::log(LoggingType::Error, "ninja is not installed.\n");
        }

        let failed: bool =
            utils::tar_is_available() && utils::cmake_is_available() && utils::ninja_is_available();

        if !failed {
            logging::log(LoggingType::Log, "Requirements aren't ok!\n\n");
            process::exit(1);
        }

        logging::log(LoggingType::Log, "Requirements are ok.\n\n");
    }
}

impl CommandLine {
    fn analyze(&mut self, argument: String) {
        let arg: &str = argument.as_str();

        match arg {
            "-h" | "--help" | "help" => {
                self.advance();
                help::show_help();
            }

            "-v" | "--version" | "version" => {
                self.advance();
                logging::write(
                    logging::OutputIn::Stdout,
                    constants::COMPILER_BUILDER_VERSION,
                );
                process::exit(0);
            }

            "-llvm-major" => {
                self.advance();

                let major: u32 = self.peek().to_string().parse().unwrap_or(17);
                self.get_mut_options().get_mut_llvm_build().set_major(major);

                self.advance();
            }

            "-llvm-minor" => {
                self.advance();

                let minor: u32 = self.peek().to_string().parse().unwrap_or(0);
                self.get_mut_options().get_mut_llvm_build().set_minor(minor);

                self.advance();
            }

            "-llvm-patch" => {
                self.advance();

                let patch: u32 = self.peek().to_string().parse().unwrap_or(0);
                self.get_mut_options().get_mut_llvm_build().set_patch(patch);

                self.advance();
            }

            "-llvm-c-compiler" => {
                self.advance();

                let c_compiler: String = self.peek().to_string();

                self.get_mut_options()
                    .get_mut_llvm_build()
                    .set_c_compiler(c_compiler);

                self.advance();
            }

            "-llvm-cpp-compiler" => {
                self.advance();

                let cpp_compiler: String = self.peek().to_string();

                self.get_mut_options()
                    .get_mut_llvm_build()
                    .set_cpp_compiler(cpp_compiler);

                self.advance();
            }

            "-llvm-cpp-flags" => {
                self.advance();

                let flags: String = self.peek().to_string();

                self.get_mut_options()
                    .get_mut_llvm_build()
                    .set_cpp_flags(flags);

                self.advance();
            }

            "-llvm-c-flags" => {
                self.advance();

                let flags: String = self.peek().to_string();

                self.get_mut_options()
                    .get_mut_llvm_build()
                    .set_c_flags(flags);

                self.advance();
            }

            "-llvm-release-type" => {
                self.advance();

                match self.peek() {
                    "Debug" => {
                        self.get_mut_options()
                            .get_mut_llvm_build()
                            .set_release_type(llvm::LLVMReleaseType::Debug);
                    }

                    "Release" => {
                        self.get_mut_options()
                            .get_mut_llvm_build()
                            .set_release_type(llvm::LLVMReleaseType::Release);
                    }

                    "MinSizeRel" => {
                        self.get_mut_options()
                            .get_mut_llvm_build()
                            .set_release_type(llvm::LLVMReleaseType::MinSizeRel);
                    }

                    _ => {
                        self.get_mut_options()
                            .get_mut_llvm_build()
                            .set_release_type(llvm::LLVMReleaseType::Release);
                    }
                }

                self.advance();
            }

            "-llvm-build-share-libs" => {
                self.advance();

                let build_share_libs: bool = self.peek().to_string().parse().unwrap_or(true);

                self.get_mut_options()
                    .get_mut_llvm_build()
                    .set_build_share_libs(build_share_libs);

                self.advance();
            }

            "-llvm-build-x86-libs" => {
                self.advance();

                let build_x86_libs: bool = self.peek().to_string().parse().unwrap_or(true);

                self.get_mut_options()
                    .get_mut_llvm_build()
                    .set_x86_libs(build_x86_libs);

                self.advance();
            }

            "-llvm-build-dylib" => {
                self.advance();

                let build_dylib: bool = self.peek().to_string().parse().unwrap_or(true);

                self.get_mut_options()
                    .get_mut_llvm_build()
                    .set_dylib(build_dylib);

                self.advance();
            }

            "-llvm-link-statically-libcpp" => {
                self.advance();

                let link_statically_libcpp: bool = self.peek().to_string().parse().unwrap_or(true);

                self.get_mut_options()
                    .get_mut_llvm_build()
                    .set_static_link_libcpp(link_statically_libcpp);

                self.advance();
            }

            "-llvm-use-linker" => {
                self.advance();

                let use_linker: String = self.peek().to_string();

                self.get_mut_options()
                    .get_mut_llvm_build()
                    .set_linker(use_linker);

                self.advance();
            }

            _ => {
                help::show_help();
            }
        }
    }
}

impl CommandLine {
    #[inline]
    fn peek(&self) -> &str {
        if self.is_eof() {
            self.report_error("Expected value after flag.");
        }

        &self.args[self.current]
    }

    #[inline]
    fn advance(&mut self) {
        if self.is_eof() {
            self.report_error("Expected value after flag.");
        }

        self.current += 1;
    }

    #[inline]
    fn report_error(&self, msg: &str) -> ! {
        logging::log(LoggingType::Error, msg);
        process::exit(1)
    }

    #[inline]
    fn is_eof(&self) -> bool {
        self.current >= self.args.len()
    }
}

impl CommandLine {
    fn preprocess_args(args: &mut Vec<String>) -> Vec<String> {
        let mut processed: Vec<String> = Vec::with_capacity(args.len() * 2);

        if !args.is_empty() {
            args.remove(0);
        }

        args.iter().for_each(|arg| {
            let parsed: ParsedArg = ParsedArg::new(arg);

            processed.push(parsed.key);

            if let Some(value) = parsed.value {
                processed.push(value);
            }
        });

        processed
    }
}

impl CommandLine {
    #[inline]
    pub fn get_options(&self) -> &BuildOptions {
        &self.options
    }

    #[inline]
    pub fn get_mut_options(&mut self) -> &mut BuildOptions {
        &mut self.options
    }
}
