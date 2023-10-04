#[cfg(test)]
mod by_equivalence {
    use std::collections::HashMap;
    use rand::Rng;
    use crate::core::lang::lang::{foldhood, mid, nbr, rep};
    use crate::core::lang::test::utils::{assert_equivalence, fully_connected_topology_map};
    use crate::core::vm::round_vm::round_vm::RoundVM;

    struct Fixture {
        exec_order: Vec<i32>,
        nbrs: HashMap<i32, Vec<i32>>,
    }

    impl Fixture {
        fn new() -> Self {
            let mut rng = rand::thread_rng();

            Self {
                exec_order: std::iter::repeat_with(|| rng.gen_range(0..3)).take(100).collect(),
                nbrs: fully_connected_topology_map(vec![0, 1, 2]),
            }
        }
    }

    #[test]
    fn foldhood_multiple_nbrs() {
        let fixture = Fixture::new();
        let program_1 = |vm: RoundVM|{
            foldhood(vm,
                     || 0,
                     |a, b| a + b,
                     |vm| {
                         let (vm_, nbr_1) = nbr(vm, |vm| (vm, 1));
                         let (vm__, nbr_2) = nbr(vm_, |vm| (vm, 2));
                         let (vm___, nbr_3) = nbr(vm__, |vm| mid(vm));
                         (vm___, nbr_1 + nbr_2 + nbr_3)
                     })
        };

        let program_2 = |vm: RoundVM|{
            foldhood(vm,
                     || 0,
                     |a, b| a + b,
                     |vm| {
                         nbr(vm, |vm| {
                             let (vm_, i) = mid(vm);
                             (vm_, 1 + 2 + i)
                         })
                     })
        };

        assert_equivalence(fixture.exec_order, fixture.nbrs, program_1, program_2);
    }


    #[test]
    fn nbr_nbr_ignored() {
        let fixture = Fixture::new();
        let program_1 = |vm: RoundVM|{
            foldhood(vm,
                     || 0,
                     |a, b| a + b,
                     |vm| {
                         nbr(vm, |vm| {
                             let (vm_, mid_1) = mid(vm);
                             let (vm__, nbr_1) = nbr(vm_, |vm| mid(vm));
                             (vm__, mid_1 + nbr_1)
                         })
                     })
        };

        let program_2 = |vm: RoundVM|{
            let (vm_, res) =
                foldhood(vm,
                     || 0,
                     |a, b| a + b,
                     |vm| {
                         nbr(vm, |vm| mid(vm))
                     });
            (vm_, 2 * res)
        };

        assert_equivalence(fixture.exec_order, fixture.nbrs, program_1, program_2);
    }

}