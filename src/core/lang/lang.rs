use crate::core::path::slot::slot::Slot::{Nbr, Rep};
use crate::core::vm::round_vm::round_vm::RoundVM;

pub fn nbr<A: Copy + 'static>(vm: RoundVM, expr: impl Fn() -> A) -> (RoundVM, A) {
    let mut vm_ = RoundVM::new(vm.context);
    let val = match vm_.neighbor() {
        Some(nbr) if nbr.clone() != vm_.self_id() => {
            vm_.neighbor_val().unwrap_or(&expr()).clone()
        }
        _ => expr()
    };
    let res = vm_.nest(Nbr(vm_.index().clone()), vm_.only_when_folding_on_self(), true, || val);
    (vm_, res)
}

pub fn rep<A: Copy + 'static>(vm: RoundVM, init: impl Fn() -> A, fun: impl Fn(RoundVM, A) -> (RoundVM, A)) -> (RoundVM, A) {
    let vm_ = RoundVM::new(vm.context);
    let prev = vm_.previous_round_val().unwrap_or(&init()).clone();
    let (mut vm__, val) = fun(vm_, prev);
    let res = vm__.nest(Rep(vm__.index().clone()), vm__.unless_folding_on_others(), true, || val);
    (vm__, res)
}

mod test {
    use crate::core::context::context::Context;
    use crate::core::lang::lang::{nbr, rep};
    use crate::core::vm::round_vm::round_vm::RoundVM;

    fn init_vm() -> RoundVM {
        let context = Context::new(0, Default::default(), Default::default(), Default::default());
        RoundVM::new(context)
    }

    #[test]
    fn test_nbr() {
        let vm = init_vm();
        let (_vm1, result) = nbr(vm, || 1);
        assert_eq!(result, 1);
    }

    #[test]
    fn test_combine() {
        let vm = init_vm();

        let (_vm1, result) =
            rep(vm, || 0, |vm1, a| {
                let (avm, res) = nbr(vm1, || a);
                (avm, res + 1)
            });

        assert_eq!(1, result)
    }
}


