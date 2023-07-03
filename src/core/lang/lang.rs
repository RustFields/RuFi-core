use std::os::unix::raw::time_t;
use crate::core::vm::round_vm::round_vm::RoundVM;

fn nbr<A>(vm: RoundVM, expr: impl Fn() -> A) -> (RoundVM, A) {
    todo!("Implement nbr")
}

fn rep<A>(vm: RoundVM, init: impl Fn() -> A, fun: impl Fn(RoundVM, A) -> (RoundVM, A)) -> (RoundVM, A) {
    todo!("Implement rep")
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
        let (vm, result) = nbr(vm, || 1);
        assert_eq!(result, 1);
    }

    #[test]
    fn test_combine() {
        let vm = init_vm();
        let (vm, result) = rep(vm, || 1, |vm1, x| nbr(vm1, ||x + 1));
    }
}


