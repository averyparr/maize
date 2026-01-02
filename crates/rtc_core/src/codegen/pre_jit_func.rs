use std::marker::PhantomData;

use inkwell::{
    OptimizationLevel,
    module::Module,
    targets::{CodeModel, FileType, InitializationConfig, Target, TargetTriple},
};

use crate::codegen::{func_with_args::Func, target::TargetMachine};

pub struct PreJitFunc<ArgsT, Ret> {
    module: Module<'static>,
    phantom: PhantomData<(ArgsT, Ret)>,
}

impl<ArgsT, Ret> PreJitFunc<ArgsT, Ret> {
    pub fn new(func: Func<ArgsT, Ret>) -> Self {
        let (module, _) = func.extract_module_codegen();
        Self {
            module,
            phantom: PhantomData,
        }
    }

    pub fn as_llvm_ir(&self) -> String {
        self.module.print_to_string().to_string()
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
