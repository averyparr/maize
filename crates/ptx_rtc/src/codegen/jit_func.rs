use std::marker::PhantomData;

pub use inkwell::OptimizationLevel;
pub use inkwell::targets::FileType;
use inkwell::{
    module::Module,
    targets::{CodeModel, InitializationConfig, Target, TargetTriple},
};

use crate::{
    codegen::Codegen,
    cuda::PTXOptions,
    func::{Func, IntoFuncArgs},
};

pub enum TargetMachine {
    PTX(PTXOptions),
}

impl TargetMachine {
    fn triple(&self) -> &'static str {
        match self {
            Self::PTX(_) => "nvptx64-nvidia-cuda",
        }
    }
    fn cpu(&self) -> &'static str {
        match self {
            Self::PTX(ptx) => ptx.cpu(),
        }
    }
    fn features(&self) -> &'static str {
        match self {
            Self::PTX(ptx) => ptx.features(),
        }
    }
}

pub struct PreJitFunc<'ctx, Args> {
    codegen: Codegen<'ctx>,
    module: Module<'ctx>,
    phantom: PhantomData<Args>,
}

impl<'ctx, Args> PreJitFunc<'ctx, Args>
where
    Args: IntoFuncArgs<'ctx>,
{
    pub fn new(func: Func<'ctx, Args>) -> Self {
        Self {
            codegen: *func.get_codegen(),
            module: func.extract_module(),
            phantom: PhantomData,
        }
    }

    pub fn compile(
        &self,
        target_machine: TargetMachine,
        optimization_level: OptimizationLevel,
        file_type: FileType,
    ) -> Box<[u8]> {
        use TargetMachine as TM;
        match target_machine {
            TM::PTX(_) => Target::initialize_nvptx(&InitializationConfig::default()),
        }
        let triple = TargetTriple::create(target_machine.triple());
        let target = Target::from_triple(&triple).expect("Unable to create Target object");

        let compiler = target
            .create_target_machine(
                &triple,
                target_machine.cpu(),
                target_machine.features(),
                optimization_level,
                inkwell::targets::RelocMode::Default,
                CodeModel::Default,
            )
            .expect("Unable to compile!");

        let maybe_ret = compiler
            .write_to_memory_buffer(&self.module, file_type)
            .expect("Should be able to compile!");
        maybe_ret.as_slice().to_vec().into_boxed_slice()
    }
}
