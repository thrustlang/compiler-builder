use crate::logging::LoggingType;
use crate::options::BuildOptions;
use crate::{llvm, logging, utils};

#[derive(Debug)]
pub struct CompilerBuilderDependencies<'a> {
    options: &'a BuildOptions,
}

impl<'a> CompilerBuilderDependencies<'a> {
    #[inline]
    pub fn new(options: &'a BuildOptions) -> Self {
        Self { options }
    }
}

impl<'a> CompilerBuilderDependencies<'a> {
    pub fn build(&self) {
        if let Err(err) = self.build_llvm() {
            logging::log(LoggingType::Panic, &err);
        }

        logging::log(LoggingType::Log, "LLVM installed.\n\n");
    }
}

impl CompilerBuilderDependencies<'_> {
    fn build_llvm(&self) -> Result<(), String> {
        let llvm_build: &llvm::LLVMBuild = self.get_options().get_llvm_build();

        let _ = std::fs::remove_dir_all(utils::get_compiler_dependencies_build_path());
        let _ = std::fs::remove_dir_all(utils::get_compiler_dependencies_build_path());

        logging::log(LoggingType::Log, "Downloading LLVM...\n");

        let llvm_downloaded: std::path::PathBuf = llvm::download_llvm(llvm_build)?;
        let llvm_source: std::path::PathBuf = llvm::decompress_llvm(llvm_build, &llvm_downloaded)?;

        logging::log(LoggingType::Log, "Building LLVM...\n");

        llvm::prepare_build_directory(&llvm_source)?;
        llvm::build_and_install(llvm_build, llvm_downloaded, llvm_source)?;

        logging::log(LoggingType::Log, "\nLLVM installed.");

        Ok(())
    }
}

impl CompilerBuilderDependencies<'_> {
    #[inline]
    pub fn get_options(&self) -> &BuildOptions {
        self.options
    }
}
