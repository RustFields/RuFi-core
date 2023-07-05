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
    use crate::core::lang::lang::{nbr, rep};
    use crate::core::vm::round_vm::round_vm::RoundVM;

    fn init_vm() -> RoundVM {
        let context = Context::new(0, Default::default(), Default::default(), Default::default());
        RoundVM::new(context)
    }

    #[test]
    fn test_round() {
        let vm = init_vm();
        let program = |vm1| rep(vm1, || 0, |vm2, a| {
            let (avm, res) = nbr(vm2, || a);
            (avm, res + 1)
        });
        let mut vm_ = round(vm, program);
        assert_eq!(&1, vm_.export_data().root::<i32>());
    }
}