use crate::core::path::slot::slot::Slot::{Branch, FoldHood, Nbr, Rep};
use crate::core::vm::round_vm::round_vm::RoundVM;

pub fn nbr<A: Copy + 'static>(mut vm: RoundVM, expr: impl Fn() -> A) -> (RoundVM, A) {
    vm.nest_in(Nbr(vm.index().clone()));
    let val = match vm.neighbor() {
        Some(nbr) if nbr.clone() != vm.self_id() => {
            vm.neighbor_val::<A>().unwrap().clone()
        }
        _ => expr()
    };
    let res = vm.nest_write(vm.only_when_folding_on_self(), val);
    vm.nest_out(true);
    (vm, res)
}

pub fn rep<A: Copy + 'static>(mut vm: RoundVM, init: impl Fn() -> A, fun: impl Fn(RoundVM, A) -> (RoundVM, A)) -> (RoundVM, A) {
    vm.nest_in(Rep(vm.index().clone()));
    let prev = vm.previous_round_val().unwrap_or(&init()).clone();
    let (mut vm_, val) = locally(vm, |vm1| fun(vm1, prev));
    let res = vm_.nest_write(vm_.unless_folding_on_others(), val);
    vm_.nest_out(true);
    (vm_, res)
}

pub fn foldhood<A: Copy + 'static>(mut vm: RoundVM, init: impl Fn() -> A, aggr: impl Fn(A, A) -> A, expr: impl Fn() -> A) -> (RoundVM, A) {
    vm.nest_in(FoldHood(vm.index().clone()));
    let nbrs = vm.aligned_neighbours().clone();
    let preval = expr();
    let nbrfield =
        nbrs.iter()
            .map(|id| {
                vm.folded_eval(|| preval, id.clone()).unwrap_or(init())
            });
    let val = nbrfield.fold(init(), |x, y| aggr(x, y));
    let res = vm.nest_write(true, val);
    vm.nest_out(true);
    (vm, res)
}

pub fn branch<A: Copy + 'static>(mut vm: RoundVM, cond: impl Fn() -> bool, thn: impl Fn(RoundVM) -> (RoundVM, A), els: impl Fn(RoundVM) -> (RoundVM, A)) -> (RoundVM, A) {
    vm.nest_in(Branch(vm.index().clone()));
    let (vm, tag) = locally(vm, |_vm1| (_vm1, cond()));
    let (mut vm_, val): (RoundVM, A) = match vm.neighbor() {
        Some(nbr) if nbr.clone() != vm.self_id() => {
            let val_clone = vm.neighbor_val::<A>().unwrap().clone();
            (vm, val_clone)
        }
        _ => if tag {
            locally(vm, |vm1| thn(vm1))
        } else {
            locally(vm, |vm1| els(vm1))
        }
    };
    let res = vm_.nest_write(vm_.unless_folding_on_others(), val);
    vm_.nest_out(tag);
    (vm_, res)
}

pub fn mid(vm: RoundVM) -> i32 {
    vm.self_id()
}

/// Evaluates the given expression locally and return the result.
///
/// # Arguments
///
/// * `expr` The expression to evaluate.
///
/// # Generic Parameters
///
/// * `A` - The type of value returned by the expression.
/// * `F` - The type of the closure, which must be a mutable closure that takes no arguments and returns a value of type `A`.
///
/// # Returns
///
/// The result of the closure `expr`.
fn locally<A: Copy + 'static>(mut vm: RoundVM, expr: impl Fn(RoundVM) -> (RoundVM,A)) -> (RoundVM, A) {
    let current_neighbour =
        vm.neighbor().map(|id| id.clone());
    vm.status = vm.status.fold_out();
    vm.status = vm.status.fold_into(current_neighbour);
    expr(vm)
}

mod test {
    use std::any::Any;
    use std::collections::HashMap;
    use crate::core::context::context::Context;
    use crate::core::export::export::Export;
    use crate::core::lang::lang::{nbr, rep, branch, foldhood};
    use crate::core::path::path::path::Path;

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
    fn test_rep() {
        let vm = init_vm();

        let (_vm1, result) =
            rep(vm, || 0, |vm1, a| {
                let (avm, res) = nbr(vm1, || a);
                (avm, res + 1)
            });

        assert_eq!(1, result)
    }

    #[test]
    fn test_foldhood() {
        let mut vm = init_vm();
        let exports = HashMap::from([
            (
                1,
                Export::from(HashMap::from([(
                    Path::new(),
                    Box::new(1) as Box<dyn Any>
                    )])),
            ),
            (
                2,
                Export::from(HashMap::from([(
                    Path::new(),
                    Box::new(2) as Box<dyn Any>
                )])),
            ),
        ]);
        vm.context = Context::new(0, Default::default(), Default::default(), exports);
        let (_vm_, res) =
            foldhood(vm,
                     || 1,
                     |s1, s2| {
                         s1 + s2
                     },
                     ||2
            );
        assert_eq!(res, 7)
    }

    #[test]
    fn test_branch() {
        let vm = init_vm();
        let (_vm1, result) = branch(vm, || false, |vm1| nbr(vm1, ||1), |vm2|  rep(vm2, ||0, |vm2, a| (vm2, a+2)));
        assert_eq!(2, result)
    }
}


