use crate::core::context::context::Context;
use crate::core::export::export::Export;
use crate::core::lang::execution::round;
use crate::core::vm::round_vm::round_vm::RoundVM;

pub fn init_vm() -> RoundVM {
    let context = Context::new(1, Default::default(), Default::default(), Default::default());
    init_with_ctx(context)
}

pub fn init_with_ctx(ctx: Context) -> RoundVM {
    let mut vm = RoundVM::new(ctx);
    vm.export_stack.push(Export::new());
    vm
}

pub fn push_to_ctx<A: Copy + 'static>(mut ctx: Context, path: Path, val: A) -> Context {
    let mut export = Export::new();
    export.put(path, || val);
    ctx.put_export(ctx.self_id, export);
    ctx
}

pub fn vm(self_id: i32, local_sensor: HashMap<SensorId, Box<dyn Any>>, nbr_sensor: HashMap<SensorId, HashMap<i32, Box<dyn Any>>>, exports: HashMap<i32, Export>) -> RoundVM {
    let context = Context::new(self_id, local_sensor, nbr_sensor, exports);
    init_with_ctx(context)
}

pub fn assert_equivalence<A,F, G>(exec_order: Vec<i32>, nbrs: HashMap<i32, Vec<i32>>, program_1: F, program_2: G) -> bool
    where
        F: Fn(RoundVM) -> (RoundVM, A),
        A: Eq + Copy + 'static,
{
    let (_vm_1, res_1) = round(init_vm(), program_1);
    let (_vm_2, res_2) = round(init_vm(), program_2);
    res_1 == res_2
}