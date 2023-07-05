use std::any::Any;
use crate::core::context::context::Context;
use crate::core::export::export::Export;
use crate::core::vm::round_vm::round_vm::RoundVM;

pub fn round<A: Copy + 'static>(vm: RoundVM, program: impl Fn(RoundVM) -> (RoundVM, A)) -> RoundVM {
    let vm = RoundVM::new(vm.context);
    let (mut vm_, res) = program(vm);
    vm_.register_root(res);
    vm_
}

#[cfg(test)]
mod test {
    use crate::core::context::context::Context;
    use crate::core::lang::execution::round;
    use crate::core::lang::lang::nbr;
    use crate::core::vm::round_vm::round_vm::RoundVM;

    fn init_vm() -> RoundVM {
        let context = Context::new(0, Default::default(), Default::default(), Default::default());
        RoundVM::new(context)
    }

    #[test]
    fn test_round() {
        let vm = init_vm();
        let program = |vm1| nbr(vm1, || 1);
        let mut vm = round(vm, program);
        println!("{:?}", vm.export_data().root::<i32>());
    }
}