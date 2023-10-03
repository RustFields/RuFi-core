use crate::core::context::context::Context;
use crate::core::export::export::Export;
use crate::core::lang::execution::round;
use crate::core::vm::round_vm::round_vm::RoundVM;

pub fn init_vm() -> RoundVM {
    let context = Context::new(0, Default::default(), Default::default(), Default::default());
    let mut vm = RoundVM::new(context);
    vm.export_stack.push(Export::new());
    vm
}

fn assert_equivalence<A,F>(program_1: F, program_2: F) -> bool
    where
        F: Fn(RoundVM) -> (RoundVM, A),
        A: Eq + Copy + 'static,
{
    let (_vm_1, res_1) = round(init_vm(), program_1);
    let (_vm_2, res_2) = round(init_vm(), program_2);
    res_1 == res_2
}