use crate::core::lang::lang::{foldhood, mid, nbr};
use crate::core::vm::round_vm::round_vm::RoundVM;

pub fn mux<A, TH, EL>(vm: RoundVM, cond: bool, th: TH, el: EL) -> (RoundVM, A)
    where
        TH: Fn(RoundVM) -> (RoundVM, A),
        EL: Fn(RoundVM) -> (RoundVM, A),
{
    if cond {
        th(vm)
    } else {
        el(vm)
    }
}

pub fn foldhood_plus<A: Copy + 'static, F, G, H>(vm: RoundVM, init: F, aggr: G, expr: H) -> (RoundVM, A)
    where
        F: Fn(RoundVM) -> (RoundVM, A) + Copy,
        G: Fn(A, A) -> A,
        H: Fn(RoundVM) -> (RoundVM, A) + Copy,
{
    foldhood(
        vm,
        init,
        aggr,
        |vm1| {
            let (vm_, self_id) = mid(vm1);
            let (vm__, nbr_id) = nbr(vm_, |vm2| mid(vm2));
            mux(vm__, self_id == nbr_id, expr, init)
        }
    )
}