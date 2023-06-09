use crate::core::vm::round_vm::round_vm::RoundVM;

pub fn round<A: Copy + 'static>(vm: RoundVM, program: impl Fn(RoundVM) -> (RoundVM, A)) -> (RoundVM, A) {
    let (mut vm_, res) = program(vm);
    vm_.register_root(res);
    let res = vm_.export_data().root::<A>().clone();
    (vm_, res)
}

#[cfg(test)]
mod test {
    use crate::core::context::context::Context;
    use crate::core::export::export::Export;
    use crate::core::lang::execution::round;
    use crate::core::lang::lang::{nbr, rep};
    use crate::core::path::path::path::Path;
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

    fn push_root_export_to_ctx<A: Copy + 'static>(mut ctx: Context, val: A) -> Context {
        let mut export = Export::new();
        export.put(Path::new(), || val);
        ctx.put_export(ctx.self_id, export);
        ctx
    }

    #[test]
    fn test_multiple_rounds() {
        //create the vm
        let vm = init_vm();
        //write the aggregate program
        let program = |vm1| rep(vm1, || 0, |vm2, a| {
            let (avm, res) = nbr(vm2, || a);
            (avm, res + 1)
        });
        //first round
        let (vm_, res) = round(vm, program);
        assert_eq!(1, res);
        //add to the context the result of the previous round
        let ctx_ = push_root_export_to_ctx(vm_.context, res);
        //second round
        let (_vm__, res_) = round(init_with_ctx(ctx_), program);
        assert_eq!(2, res_);
    }
}