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
    use crate::core::context::context::Context;
    use crate::core::export::export::Export;
    use crate::core::lang::execution::round;
    use crate::core::lang::lang::{nbr, rep};
    use crate::core::path::path::path::Path;
    use crate::core::path::slot::slot::Slot::Rep;
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
}