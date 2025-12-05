use crate::logging::LoggingType;
use crate::options::BuildOptions;
use crate::{gcc, llvm, logging, utils};

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

        logging::write(logging::OutputIn::Stdout, "LLVM backend installed.\n\n");

        if self.get_options().get_build_gcc_backend() {
            if let Err(err) = self.build_gcc() {
                logging::log(LoggingType::Panic, &err);
            }

            logging::write(logging::OutputIn::Stdout, "GCC backend installed.\n\n");
        }
    }
}

impl CompilerBuilderDependencies<'_> {
    fn build_llvm(&self) -> Result<(), String> {
        let llvm_build: &llvm::LLVMBuild = self.get_options().get_llvm_build();

        let _ = std::fs::remove_dir(utils::get_compiler_dependencies_build_path());
        let _ = std::fs::create_dir_all(utils::get_compiler_dependencies_build_path());

        logging::write(logging::OutputIn::Stdout, "Downloading LLVM...\n");

        let llvm_downloaded: std::path::PathBuf = llvm::download_llvm(llvm_build)?;
        let llvm_source: std::path::PathBuf = llvm::decompress_llvm(llvm_build, &llvm_downloaded)?;

        logging::write(logging::OutputIn::Stdout, "Building LLVM...\n");

        llvm::prepare_build_directory(&llvm_source)?;
        llvm::build_and_install(llvm_build, llvm_downloaded, llvm_source)?;

        Ok(())
    }

    fn build_gcc(&self) -> Result<(), String> {
        let gcc_build: &gcc::GCCBuild = self.get_options().get_gcc_build();

        logging::write(logging::OutputIn::Stdout, "Downloading GCC...\n");

        let gcc_downloaded: std::path::PathBuf = gcc::download_gcc(gcc_build)?;
        let gcc_source: std::path::PathBuf = gcc::decompress_gcc(gcc_build, &gcc_downloaded)?;

        logging::write(logging::OutputIn::Stdout, "Building GCC...\n");

        gcc::prepare_build_directory(&gcc_source)?;
        gcc::build_and_install(gcc_build, gcc_downloaded, gcc_source)?;

        Ok(())
    }
}

impl CompilerBuilderDependencies<'_> {
    #[inline]
    pub fn get_options(&self) -> &BuildOptions {
        self.options
    }
}
