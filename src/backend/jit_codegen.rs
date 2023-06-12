use std::process::exit;

use inkwell::{context::Context, execution_engine::JitFunction, OptimizationLevel};

use super::codegen::Codegen;

type Main = unsafe extern "C" fn() -> i32;

impl<'ctx> Codegen<'ctx> {
    pub fn jit_run() {
        let context = Context::create();
        let builder = context.create_builder();
        let module = context.create_module("main");
        let exec_engine = module
            .create_jit_execution_engine(OptimizationLevel::Aggressive)
            .unwrap();
        let codegen = Codegen {
            builder,
            context: &context,
            module,
        };
        let main: JitFunction<'_, Main> = unsafe {
            exec_engine.get_function("main").unwrap_or_else(|_| {
                println!("Main function not defined");
                exit(-1)
            })
        };
        let exit_code = unsafe { main.call() };
        println!("Process exited with code {}", exit_code);
    }
}
