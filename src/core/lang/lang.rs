use crate::core::path::slot::slot::Slot::{Branch, Nbr, Rep};
use crate::core::vm::round_vm::round_vm::RoundVM;

pub fn nbr<A: Copy + 'static>(vm: RoundVM, expr: impl Fn() -> A) -> (RoundVM, A) {
    let mut vm_ = RoundVM::new(vm.context);
    let val = match vm_.neighbor() {
        Some(nbr) if nbr.clone() != vm_.self_id() => {
            vm_.neighbor_val::<A>().unwrap().clone()
        }
        _ => expr()
    };
    let res = vm_.nest(Nbr(vm_.index().clone()), vm_.only_when_folding_on_self(), true, || val);
    (vm_, res)
}

pub fn rep<A: Copy + 'static>(vm: RoundVM, init: impl Fn() -> A, fun: impl Fn(RoundVM, A) -> (RoundVM, A)) -> (RoundVM, A) {
    let vm_ = RoundVM::new(vm.context);
    let prev = vm_.previous_round_val().unwrap_or(&init()).clone();
    //cannot use vm_.locally
    let (mut vm__, val) = fun(vm_, prev);
    let res = vm__.nest(Rep(vm__.index().clone()), vm__.unless_folding_on_others(), true, || val);
    (vm__, res)
}


pub fn branch<A: Copy + 'static>(vm: RoundVM, cond: impl Fn() -> bool, thn: impl Fn(RoundVM) -> (RoundVM, A), els: impl Fn(RoundVM) -> (RoundVM, A)) -> (RoundVM, A) {
    let mut vm_ = RoundVM::new(vm.context);
    let tag = vm_.locally(cond);
    let (mut vm__, val): (RoundVM, A) = match vm_.neighbor() {
        Some(nbr) if nbr.clone() != vm_.self_id() => {
            let val_clone = vm_.neighbor_val::<A>().unwrap().clone();
            (vm_, val_clone)
        }
        _ => if tag {
            //cannot use vm_.locally
            thn(vm_)
        } else {
            //cannot use vm_.locally
            els(vm_)
        }
    };
    let res = vm__.nest(Branch(vm__.index().clone()), vm__.unless_folding_on_others(), tag, || val);
    (vm__, res)
}

fn locally() {}

mod test {
    use crate::core::context::context::Context;
    use crate::core::export::export::Export;
    use crate::core::lang::lang::{nbr, rep, branch};

    use crate::core::vm::round_vm::round_vm::RoundVM;

    fn init_vm() -> RoundVM {
        let context = Context::new(0, Default::default(), Default::default(), Default::default());
        let mut vm = RoundVM::new(context);
        vm.export_stack.push(Export::new());
        vm
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

    #[test]
    fn test_branch() {
        let vm = init_vm();
        let (_vm1, result) = branch(vm, || false, |vm1| nbr(vm1, ||1), |vm2|  rep(vm2, ||0, |vm2, a| (vm2, a+2)));
        assert_eq!(2, result)
    }
}


