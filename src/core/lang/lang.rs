use std::fmt::Debug;
use crate::core::path::slot::slot::Slot::{Branch, FoldHood, Nbr, Rep};
use crate::core::vm::round_vm::round_vm::RoundVM;

pub fn nbr<A: Copy + 'static, F>(mut vm: RoundVM, expr: F) -> (RoundVM, A)
where
    F: Fn(RoundVM) -> (RoundVM, A)
{
    vm.nest_in(Nbr(vm.index().clone()));
    let (mut vm_ ,val) = match vm.neighbor() {
        Some(nbr) if nbr.clone() != vm.self_id() => {
            let cloned_val = vm.neighbor_val::<A>().unwrap().clone();
            (vm, cloned_val)
        }
        _ => expr(vm)
    };
    let res = vm_.nest_write(vm_.only_when_folding_on_self(), val);
    vm_.nest_out(true);
    (vm_, res)
}

pub fn rep<A: Copy + 'static, F, G>(mut vm: RoundVM, init: F, fun: G) -> (RoundVM, A)
where
    F: Fn() -> A,
    G: Fn(RoundVM, A) -> (RoundVM, A),
{
    vm.nest_in(Rep(vm.index().clone()));
    let (mut vm_, val) = locally(vm, |vm1| {
        let prev = vm1.previous_round_val().unwrap_or(&init()).clone();
        fun(vm1, prev)
    });
    let res = vm_.nest_write(vm_.unless_folding_on_others(), val);
    vm_.nest_out(true);
    (vm_, res)
}

pub fn foldhood<A: Copy + 'static, F, G, H>(mut vm: RoundVM, init: F, aggr: G, expr: H) -> (RoundVM, A)
where
    F: Fn() -> A,
    G: Fn(A, A) -> A,
    H: Fn(RoundVM) -> (RoundVM, A),
{
    // here we do nest_in after retrieving the neighbours because otherwise it would disalign the device

    vm.nest_in(FoldHood(vm.index().clone()));
    let nbrs = vm.aligned_neighbours::<A>().clone();
    let (vm_, local_init) = locally(vm, |vm_| (vm_, init()));
    // Create a vector of A
    let temp_vec: Vec<A> = Vec::new();
    let (mut vm__, nbrs_vec) = nbrs_computation(vm_, expr, temp_vec, nbrs, local_init);
    let val = nbrs_vec.iter().fold(local_init, |x, y| aggr(x, y.clone()));
    let res = vm__.nest_write(true, val);
    vm__.nest_out(true);
    (vm__, res)
}

fn nbrs_computation<A: Copy + 'static>(vm: RoundVM, expr: impl Fn(RoundVM) -> (RoundVM, A), mut tmp: Vec<A>, mut ids: Vec<i32>, init: A) -> (RoundVM, Vec<A>) {
    if ids.len() == 0 {
        return (vm, tmp);
    } else {
        let current_id = ids.pop();
        let (vm_, res, expr_) = folded_eval(vm, expr, current_id);
        tmp.push(res.unwrap_or(init).clone());
        nbrs_computation(vm_, expr_, tmp, ids, init)
    }
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

pub fn mid(vm: RoundVM) -> (RoundVM, i32) {
    let mid = vm.self_id().clone();
    (vm, mid)
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
    let (mut vm_, result) = expr(vm);
    vm_.status = vm_.status.fold_into(current_neighbour);
    (vm_, result)
}

fn folded_eval<A: Copy + 'static, F>(mut vm: RoundVM, expr: F, id: Option<i32>) -> (RoundVM, Option<A>, F)
    where
        F: Fn(RoundVM) -> (RoundVM, A),
{
    vm.status = vm.status.push();
    vm.status = vm.status.fold_into(id);
    let (mut vm_, res) = expr(vm);
    vm_.status = vm_.status.pop();
    (vm_, Some(res), expr)
}

mod test {
    use std::any::Any;
    use std::collections::HashMap;
    use crate::core::context::context::Context;
    use crate::core::export::export::Export;
    use crate::core::lang::lang::{nbr, rep, branch, foldhood};
    use crate::core::path::path::path::Path;
    use crate::core::path::slot::slot::Slot::FoldHood;

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
        let (_vm1, result) = nbr(vm, |_vm| (_vm, 1));
        assert_eq!(result, 1);
    }

    #[test]
    fn test_rep() {
        let vm = init_vm();

        let (_vm1, result) =
            rep(vm, || 0, |vm1, a| {
                let (avm, res) = nbr(vm1, |_vm| (_vm,a));
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
                    Path::from(vec![FoldHood(0)]),
                    Box::new(1) as Box<dyn Any>
                )])),
            ),
            (
                2,
                Export::from(HashMap::from([(
                    Path::from(vec![FoldHood(0)]),
                    Box::new(2) as Box<dyn Any>
                )])),
            ),
        ]);
        println!("{:?}", vm.context);
        vm.context = Context::new(0, Default::default(), Default::default(), exports);
        println!("{:?}", vm.context);
        let (_vm_, res) =
            foldhood(vm,
                     || 1,
                     |s1, s2| {
                         s1 + s2
                     },
                     |_vm| (_vm,2)
            );
        assert_eq!(res, 7)
    }

    #[test]
    fn test_branch() {
        let vm = init_vm();
        let (_vm1, result) = branch(vm, || false, |vm1| nbr(vm1, |_vm|(_vm,1)), |vm2|  rep(vm2, ||0, |vm2, a| (vm2, a+2)));
        assert_eq!(2, result)
    }
}


