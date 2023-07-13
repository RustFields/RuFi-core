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
    use crate::core::lang::lang::{foldhood, nbr, rep};
    use crate::core::path::path::path::Path;
    use crate::core::path::slot::slot::Slot::{FoldHood, Nbr, Rep};
    use crate::core::vm::round_vm::round_vm::RoundVM;

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
        let context = Context::new(0,
                                   Default::default(),
                                   Default::default(),
                                   Default::default());
        let result = round(init_with_ctx(context), |vm| (vm, 10));
        assert_eq!(10, result.1);
    }

    #[test]
    fn test_alignment() {
        // No neighbor is aligned
        let context = Context::new(0, Default::default(),
                                   Default::default(), Default::default());
        let result = round(init_with_ctx(context),
                           |vm1| rep(vm1, || 0, |vm2, _| {
                               foldhood(vm2, || 0,
                                        | a, b | (a + b), || 1 )}));
        assert_eq!(1, result.1);

        // One neighbor is aligned
        let mut exports: HashMap<i32, Export> = HashMap::new();
        exports.insert(1, create_export());

        let context = Context::new(0, Default::default(),
                                   Default::default(), exports);

        let result = round(init_with_ctx(context),
                           |vm1| rep(vm1, || 0, |vm2, _| {
                               foldhood(vm2, || 0,
                                        | a, b| (a + b), || 1 )}));
        assert_eq!(2, result.1);
    }

    fn create_export() -> Export {
        let mut map: HashMap<Path, Box<dyn Any>> = HashMap::new();
        map.insert(Path::from(vec![Rep(0)]), Box::new(1));
        map.insert(Path::from(vec![Rep(0),FoldHood(0)]), Box::new(1));
        Export::from(map)
    }

    fn create_context_foldhood_test() -> Context {
        // Export of device 2: Export(/ -> "1", FoldHood(0) -> "1")
        let mut exports_device_2: HashMap<Path, Box<dyn Any>> = HashMap::new();
        exports_device_2.insert(Path::from(vec![FoldHood(0)]), Box::new(1));
        exports_device_2.insert(Path::from(vec![]), Box::new(1));
        // Export of device 4: Export(/ -> "3", FoldHood(0) -> "3")
        let mut exports_device_4: HashMap<Path, Box<dyn Any>> = HashMap::new();
        exports_device_4.insert(Path::from(vec![FoldHood(0)]), Box::new(3));
        exports_device_4.insert(Path::from(vec![]), Box::new(3));
        // Exports of the context: Map(2 -> Export(/ -> "a", FoldHood(0) -> "a"), 4 -> Export(/ -> "b", FoldHood(0) -> "b"))
        let mut exports: HashMap<i32, Export> = HashMap::new();
        exports.insert(2, Export::from(exports_device_2));
        exports.insert(4, Export::from(exports_device_4));
        // Context of the device
        let context = Context::new(0, Default::default(), Default::default(), exports);
        return context;
    }

    #[test]
    fn test_foldhood() {
        // Context of the device
        let context = create_context_foldhood_test();
        // Aggregate program in Scala: foldhood("1")(_ + _)("2")
        let result = round(init_with_ctx(context),
                           |vm| foldhood(vm, || 1,
                                        | a, b| (a + b), || 2 ));
        assert_eq!(7, result.1);

        // TODO
        //let context = create_context_foldhood_test();
        // Aggregate program in Scala: foldhood("-5")(_ + _)(if (nbr(false)) {0} else {1})
        //let result = round(init_with_ctx(context),
                           //|vm| foldhood(vm, || -5,
                                       // | a, b| (a + b),
                                         //|vm1| if nbr(vm1, || false).1 {0} else {1} ));
        //assert_eq!(-14, result.1);
    }

    #[test]
    fn test_nbr() {
        // 1 - NBR needs not to be nested into fold
        let context = Context::new(0, Default::default(), Default::default(), Default::default());
        let result = round(init_with_ctx(context), |vm| nbr(vm, || 7));
        assert_eq!(7, result.1);

        // 2 - NBR should support interaction between aligned devices
        let context = create_context_nbr_test2();
        // The following program is run: foldhood(0)(_ + _)(if (nbr(mid()) == mid()) 0 else 1)
        // TODO
        //let program = |vm| foldhood(vm,
                               //     || 0,
                                //    | a, b| (a + b),
                                //    |vm1| if nbr(vm1, |vm1| vm1.self_id(vm1)) == mid() {0} else {1} );

    }

    fn create_context_nbr_test2() -> Context {
        // Create this export: Map(
        //       1 -> Export(/ -> "any", FoldHood(0) -> 1, FoldHood(0) / Nbr(0) -> 1),
        //       2 -> Export(/ -> "any", FoldHood(0) -> 2, FoldHood(0) / Nbr(0) -> 2)
        //     )
        let mut export_dev_1: HashMap<Path, Box<dyn Any>> = HashMap::new();
        export_dev_1.insert(Path::from(vec![]), Box::new("any"));
        export_dev_1.insert(Path::from(vec![FoldHood(0)]), Box::new(1));
        export_dev_1.insert(Path::from(vec![FoldHood(0), Nbr(0)]), Box::new(1));
        let mut export_dev_2: HashMap<Path, Box<dyn Any>> = HashMap::new();
        export_dev_2.insert(Path::from(vec![]), Box::new("any"));
        export_dev_2.insert(Path::from(vec![FoldHood(0)]), Box::new(2));
        export_dev_2.insert(Path::from(vec![FoldHood(0), Nbr(0)]), Box::new(2));
        let mut exports: HashMap<i32, Export> = HashMap::new();
        exports.insert(1, Export::from(export_dev_1));
        exports.insert(2, Export::from(export_dev_2));
        // Context of the device
        let context = Context::new(0, Default::default(), Default::default(), exports);
        context
    }
}