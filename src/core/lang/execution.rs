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