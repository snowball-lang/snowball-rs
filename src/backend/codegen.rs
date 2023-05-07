use std::path::Path;

use inkwell::{
    builder::Builder,
    context::Context,
    module::Module,
    targets::{CodeModel, FileType, RelocMode, Target, TargetMachine},
    OptimizationLevel,
};

pub struct Codegen<'ctx> {
    builder: Builder<'ctx>,
    context: &'ctx Context,
    module: Module<'ctx>,
}

impl<'ctx> Codegen<'ctx> {
    pub fn compile(ast: ()) {
        let context = Context::create();
        let builder = context.create_builder();
        let module = context.create_module("main");
        let codegen = Codegen {
            builder,
            context: &context,
            module,
        };
        // calls to generate the shit go here
        codegen.write_object(&Path::new("out.o"));
    }
    pub fn write_object(&self, path: &Path) {
        let triple = TargetMachine::get_default_triple();
        let target = Target::from_triple(&triple).unwrap();
        let cpu = TargetMachine::get_host_cpu_name();
        let features = TargetMachine::get_host_cpu_features();
        let reloc = RelocMode::Default;
        let model = CodeModel::Default;
        let opt = OptimizationLevel::Default;
        let target_machine = target
            .create_target_machine(
                &triple,
                cpu.to_str().unwrap(),
                features.to_str().unwrap(),
                opt,
                reloc,
                model,
            )
            .unwrap();

        target_machine
            .write_to_file(&self.module, FileType::Object, path)
            .unwrap();
    }
    pub fn compile_ast(&mut self) {

    }
}
