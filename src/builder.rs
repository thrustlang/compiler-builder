use crate::logging::LoggingType;
use crate::options::BuildOptions;
use crate::{llvm, logging};

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
    pub fn install(&self) {
        self.reset_llvm_build_path();

        if let Err(err) = self.build_llvm() {
            logging::log(LoggingType::Panic, &err);
        }

        logging::log(LoggingType::Log, "LLVM installed.\n\n");
    }
}

impl CompilerBuilderDependencies<'_> {
    fn build_llvm(&self) -> Result<(), String> {
        logging::log(LoggingType::Log, "Installing LLVM...\n");

        let llvm_build: &llvm::LLVMBuild = self.get_options().get_llvm_build();

        let llvm_downloaded: std::path::PathBuf = llvm::download_llvm(llvm_build)?;
        let llvm_source: std::path::PathBuf = llvm::decompress_llvm(llvm_build, &llvm_downloaded)?;

        llvm::prepare_build_directory(&llvm_source)?;
        llvm::build_and_install(llvm_build, llvm_downloaded, llvm_source)?;

        logging::log(LoggingType::Log, "\nLLVM installed.");

        Ok(())
    }
}

impl CompilerBuilderDependencies<'_> {
    #[inline]
    fn reset_llvm_build_path(&self) {
        let _ = std::fs::remove_dir_all(self.get_options().get_llvm_build_path());
        let _ = std::fs::create_dir_all(self.get_options().get_llvm_build_path());
    }
}

impl CompilerBuilderDependencies<'_> {
    #[inline]
    pub fn get_options(&self) -> &BuildOptions {
        self.options
    }
}
