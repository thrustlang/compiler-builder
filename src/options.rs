use crate::llvm::LLVMBuild;

#[derive(Debug)]
pub struct BuildOptions {
    llvm_build: LLVMBuild,
}

impl BuildOptions {
    #[inline]
    pub fn new() -> BuildOptions {
        BuildOptions {
            llvm_build: LLVMBuild::new(),
        }
    }
}

impl BuildOptions {
    #[inline]
    pub fn get_llvm_build(&self) -> &LLVMBuild {
        &self.llvm_build
    }
}

impl BuildOptions {
    #[inline]
    pub fn get_mut_llvm_build(&mut self) -> &mut LLVMBuild {
        &mut self.llvm_build
    }
}
