use crate::core::vm::round_vm::round_vm::RoundVM;

pub fn round<A: Copy + 'static>(vm: RoundVM, program: impl Fn(RoundVM) -> (RoundVM, A)) -> (RoundVM, A) {
    let (mut vm_, res) = program(vm);
    vm_.register_root(res);
    let res = vm_.export_data().root::<A>().clone();
    (vm_, res)
}

pub fn compose<A, F, G>(expr1: F, expr2: G) -> impl Fn(RoundVM) -> (RoundVM, A)
where
    F: Fn(RoundVM) -> (RoundVM, A),
    G: Fn(RoundVM, A) -> (RoundVM, A),
{
    move |vm| {
        let (vm_, res) = expr1(vm);
        expr2(vm_, res)
    }
}

pub fn combine<A, F, G, H>(vm: RoundVM, expr1: F, expr2: G, comb: H) -> (RoundVM, A)
where
    F: Fn(RoundVM) -> (RoundVM, A),
    G: Fn(RoundVM) -> (RoundVM, A),
    H: Fn(A, A) -> A,
{
    let (vm_, res1) = expr1(vm);
    let (vm__, res2) = expr2(vm_);
    (vm__, comb(res1, res2))
}

#[cfg(test)]
mod test {
    use std::any::Any;
    use std::collections::HashMap;
    use crate::core::context::context::Context;
    use crate::core::export::export::Export;
    use crate::core::lang::execution::round;
    use crate::core::lang::lang::{branch, foldhood, mid, nbr, rep};
    use crate::core::path::path::path::Path;
    use crate::core::path::slot::slot::Slot::{FoldHood, Nbr, Rep};
    use crate::core::vm::round_vm::round_vm::RoundVM;
    use crate::export;
    use crate::path;

    fn init_vm() -> RoundVM {
        let context = Context::new(0, Default::default(), Default::default(), Default::default());
        let mut vm = RoundVM::new(context);
        vm.export_stack.push(Export::new());
        vm
    }

    fn init_with_ctx(ctx: Context) -> RoundVM {
        let mut vm = RoundVM::new(ctx);
        vm.export_stack.push(Export::new());
        vm
    }

    fn push_to_ctx<A: Copy + 'static>(mut ctx: Context, path: Path, val: A) -> Context {
        let mut export = Export::new();
        export.put(path, || val);
        ctx.put_export(ctx.self_id, export);
        ctx
    }

    #[test]
    fn test_multiple_rounds() {
        //create the vm
        let vm = init_vm();
        //write the aggregate program
        let program = |vm1| rep(vm1, || 0, |vm2, a| {
            let (avm, res) = nbr(vm2, |_vm| (_vm,a));
            (avm, res + 1)
        });
        //first round
        let (vm_, res) = round(vm, program);
        assert_eq!(1, res);
        //add to the context the result of the previous round
        let ctx_ = push_to_ctx(vm_.context, Path::from(vec![Rep(0)]), res);
        //second round
        let (_vm__, res_) = round(init_with_ctx(ctx_), program);
        assert_eq!(2, res_);
    }

    #[test]
    fn test_local_value() {
        let context = Context::new(0, Default::default(), Default::default(), Default::default());
        let result = round(init_with_ctx(context), |vm| (vm, 10));
        assert_eq!(10, result.1);
    }

    #[test]
    fn test_alignment() {
        // No neighbor is aligned
        let context = Context::new(0, Default::default(), Default::default(), Default::default());
        // Program: rep(0, foldhood(0)(_ + _)(1))
        let program = |vm1| rep(vm1,
                                || 0,
                                |vm2, _| { foldhood(vm2,
                                                    || 0,
                                                    | a, b | (a + b),
                                                    |vm3| (vm3, 1))});
        let result = round(init_with_ctx(context), program);
        assert_eq!(1, result.1);

        // One neighbor is aligned
        // Export: Map(1 -> Export(Rep(0) -> 1, Rep(0) / FoldHood(0) -> 1))
        let export_dev_1 = export!((path!(Rep(0)), 1), (path!(FoldHood(0), Rep(0)), 1));
        let mut exports: HashMap<i32, Export> = HashMap::new();
        exports.insert(1, export_dev_1);
        let context = Context::new(0, Default::default(), Default::default(), exports);
        let vm = init_with_ctx(context);
        let result = round(vm, program);
        assert_eq!(2, result.1);
    }

    #[test]
    fn test_foldhood_basic() {
        // Export of device 2: Export(/ -> 1, FoldHood(0) -> 1)
        let export_dev_2 = export!((path!(), 1), (path!(FoldHood(0)), 1));
        // Export of device 4: Export(/ -> 3, FoldHood(0) -> 3)
        let export_dev_4 = export!((path!(), 3), (path!(FoldHood(0)), 3));
        // Exports of the context: Map(2 -> Export(/ -> 1, FoldHood(0) -> 1), 4 -> Export(/ -> 3, FoldHood(0) -> 3))
        let mut exports: HashMap<i32, Export> = HashMap::new();
        exports.insert(2, export_dev_2);
        exports.insert(4, export_dev_4);
        let context = Context::new(0, Default::default(), Default::default(), exports);
        // Program: foldhood(1)(_ + _)(2)
        let program = |vm| foldhood(vm,
                                    || 1,
                                    | a, b| (a + b),
                                    |vm1| (vm1, 2));
        let result = round(init_with_ctx(context), program);
        assert_eq!(7, result.1);
    }

    #[test]
    fn test_foldhood_advanced() {
        // Export of device 2: Export(/ -> "1", FoldHood(0) -> "1", FoldHood(0) / Nbr(0) -> 4)
        let export_dev_2 = export!((path!(), 1), (path!(FoldHood(0)), 1), (path!(Nbr(0), FoldHood(0)), 4));
        // Export of device 4: Export(/ -> "3", FoldHood(0) -> "3")
        let export_dev_4 = export!((path!(), 3), (path!(FoldHood(0)), 3), (path!(Nbr(0), FoldHood(0)), 19));
        let mut exports: HashMap<i32, Export> = HashMap::new();
        exports.insert(2, export_dev_2);
        exports.insert(4, export_dev_4);
        let context = Context::new(0, Default::default(), Default::default(), exports);
        // Program: foldhood(-5)(_ + _)(nbr(2))
        let program = |vm| foldhood(vm,
                                    || -5,
                                    | a, b| (a + b),
                                    |vm1| nbr(vm1, |vm2| (vm2, 2)));
        let result = round(init_with_ctx(context), program);
        assert_eq!(20, result.1);
    }

    /*
    #[test]
    fn test_foldhood_failure() {
        // Is the same test as test_foldhood_advanced, but the device 4 is not aligned to FoldHood(0) / Nbr(0) and this cause the error
        // Export of device 2: Export(/ -> "1", FoldHood(0) -> "1", FoldHood(0) / Nbr(0) -> 4)
        let export_dev_2 = export!((path!(), 1), (path!(FoldHood(0)), 1), (path!(Nbr(0), FoldHood(0)), 4));
        // Export of device 4: Export(/ -> "3", FoldHood(0) -> "3")
        let export_dev_4 = export!((path!(), 3), (path!(FoldHood(0)), 3));
        let mut exports: HashMap<i32, Export> = HashMap::new();
        exports.insert(2, export_dev_2);
        exports.insert(4, export_dev_4);
        let context = Context::new(0, Default::default(), Default::default(), exports);
        // Program: foldhood(-5)(_ + _)(nbr(2))
        let program = |vm| foldhood(vm,
                                    || -5,
                                    | a, b| (a + b),
                                    |vm1| nbr(vm1, |vm2| (vm2, 2)));
        let result = round(init_with_ctx(context), program);
        assert_eq!(-4, result.1);
    }
    */

    #[test]
    fn test_nbr() {
        // 1 - NBR needs not to be nested into fold
        let context = Context::new(0, Default::default(), Default::default(), Default::default());
        let result = round(init_with_ctx(context), |vm| nbr(vm, |vm1| (vm1, 7)));
        assert_eq!(7, result.1);

        // 2 - NBR should support interaction between aligned devices
        let context = Context::new(0, Default::default(), Default::default(), create_exports_nbr_test());
        // Program: foldhood(0)(_ + _)(if (nbr(mid()) == mid()) 0 else 1)
        let program = |vm| foldhood(vm,
                                    || 0,
                                    | a, b| (a + b),
                                    |vm1| {
                                        let (vm2, res) = nbr(vm1, |vm3| mid(vm3));
                                        if res == vm2.self_id() { (vm2, 0) } else { (vm2, 1) }
                                    });
        let result = round(init_with_ctx(context), program);
        assert_eq!(2, result.1);
    }

    fn create_exports_nbr_test() -> HashMap<i32, Export> {
        // Create this export: Map(
        //       1 -> Export(/ -> "any", FoldHood(0) -> 1, FoldHood(0) / Nbr(0) -> 1),
        //       2 -> Export(/ -> "any", FoldHood(0) -> 2, FoldHood(0) / Nbr(0) -> 2)
        //     )
        let export_dev_1 = export!((path!(), "any"), (path!(FoldHood(0)), 1), (path!(Nbr(0), FoldHood(0)), 1));
        let export_dev_2 = export!((path!(), "any"), (path!(FoldHood(0)), 2), (path!(Nbr(0), FoldHood(0)), 2));
        let mut exports: HashMap<i32, Export> = HashMap::new();
        exports.insert(1, export_dev_1);
        exports.insert(2, export_dev_2);
        exports
    }

    #[test]
    // Rep should support dynamic evolution of fields
    fn test_rep() {
        let context = Context::new(0, Default::default(), Default::default(), Default::default());
        // Program: rep(9)(_ * 2)
        let program = |vm| rep(vm, || 9, |vm1, a| (vm1, a * 2));
        // Check if rep use the initial value
        let result = round(init_with_ctx(context), program);
        assert_eq!(18, result.1);

        // Export: Map(0 -> Export(Rep(0) -> 7))
        let export_dev_0= export!((path!(Rep(0)), 7));
        let mut exports: HashMap<i32, Export> = HashMap::new();
        exports.insert(0, export_dev_0);
        let context = Context::new(0, Default::default(), Default::default(), exports);
        // Rep should build upon previous state.
        let result = round(init_with_ctx(context), program);
        assert_eq!(14, result.1);
    }

    #[test]
    // Branch should support domain restriction, thus affecting the structure of exports
    fn test_branch() {
        // Program: rep(0) { x => branch(x % 2 == 0)(7)(rep(4)(_ => 4)); x + 1 }
        let program =
            |vm| rep(vm,
                     || 0,
                     |vm1, x| { let res = branch(vm1,
                                                  || x % 2 == 0,
                                                  |vm3| (vm3, 7),
                                                  |vm4| (rep(vm4,
                                                             || 4,
                                                             |vm5, _| (vm5, 4))));
                         return (res.0, x + 1)
        });
        let context = Context::new(0, Default::default(), Default::default(), Default::default());
        let result = round(init_with_ctx(context), program);
        assert_eq!(1, result.1);

        // Export: Map(0 -> Export(Rep(0) -> 1))
        let export_dev_0= export!((path!(Rep(0)), 1));
        let mut exports: HashMap<i32, Export> = HashMap::new();
        exports.insert(0, export_dev_0);
        let context = Context::new(0, Default::default(), Default::default(), exports);
        let result = round(init_with_ctx(context), program);
        assert_eq!(2, result.1);
    }
}