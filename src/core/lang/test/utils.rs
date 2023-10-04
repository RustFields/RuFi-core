use std::any::Any;
use std::collections::HashMap;
use std::fmt::Debug;
use crate::core::context::context::Context;
use crate::core::export::export::Export;
use crate::core::lang::execution::round;
use crate::core::path::path::path::Path;
use crate::core::sensor_id::sensor_id::SensorId;
use crate::core::vm::round_vm::round_vm::RoundVM;

pub fn init_vm() -> RoundVM {
    let context = Context::new(1, Default::default(), Default::default(), Default::default());
    init_with_ctx(context)
}

pub fn init_with_ctx(ctx: Context) -> RoundVM {
    let mut vm = RoundVM::new(ctx);
    vm.export_stack.push(Export::new());
    vm
}

pub fn push_to_ctx<A: Copy + 'static>(mut ctx: Context, path: Path, val: A) -> Context {
    let mut export = Export::new();
    export.put(path, || val);
    ctx.put_export(ctx.self_id, export);
    ctx
}

pub fn vm(self_id: i32, local_sensor: HashMap<SensorId, Box<dyn Any>>, nbr_sensor: HashMap<SensorId, HashMap<i32, Box<dyn Any>>>, exports: HashMap<i32, Export>) -> RoundVM {
    let context = Context::new(self_id, local_sensor, nbr_sensor, exports);
    init_with_ctx(context)
}

pub fn assert_equivalence<A,F, G>(exec_order: Vec<i32>, nbrs: HashMap<i32, Vec<i32>>, program_1: F, program_2: G) -> bool
    where
        F: Fn(RoundVM) -> (RoundVM, A) + Copy,
        G: Fn(RoundVM) -> (RoundVM, A) + Copy,
        A: Eq + Copy + 'static + Debug,
{
    let states: HashMap<i32, (RoundVM, RoundVM)> = nbrs.iter().map(|(curr, neighbors)|{
        let ex_1: HashMap<i32, Export> = neighbors.iter().map(|nbr| (nbr.clone(), Export::new())).collect();
        let ex_2: HashMap<i32, Export> = neighbors.iter().map(|nbr| (nbr.clone(), Export::new())).collect();
        (
            curr.clone(),
            (vm(curr.clone(), Default::default(), Default::default(), ex_1),
             vm(curr.clone(), Default::default(), Default::default(), ex_2))
        )
    }).collect();
    assert_equivalence_rec(exec_order, states, program_1, program_2)
}

fn assert_equivalence_rec<A, F, G>(mut exec_order: Vec<i32>, states: HashMap<i32, (RoundVM, RoundVM)>, program_1: F, program_2: G) -> bool
    where
        F: Fn(RoundVM) -> (RoundVM, A) + Copy,
        G: Fn(RoundVM) -> (RoundVM, A) + Copy,
        A: Eq + Copy + 'static + Debug,
{
    if exec_order.is_empty() {
        return true;
    }

    let curr = exec_order.pop().unwrap();

    let new_states: HashMap<i32, (RoundVM, RoundVM)> = states.into_iter().map(|(id, (vm_1, vm_2))| {
        if id == curr {
            let (vm_1_, res_1) = round(vm_1, program_1);
            let (vm_2_, res_2) = round(vm_2, program_2);
            if res_1 != res_2 {
                panic!("Programs are not equivalent: {:?} != {:?}", res_1, res_2);
            }
            (id, (vm_1_, vm_2_))
        } else {
            (id, (vm_1, vm_2))
        }
    }).collect();
    assert_equivalence_rec(exec_order, new_states, program_1, program_2)
}

pub fn fully_connected_topology_map(elems: Vec<i32>) -> HashMap<i32, Vec<i32>> {
    let new_elems = elems.clone();
    elems.into_iter().map(|elem| (elem, new_elems.clone())).collect()
}