use std::any::Any;
use std::fmt::Error;
use crate::core::context::context::Context;
use crate::core::export::export::Export;
use crate::core::path::path::path::Path;
use crate::core::sensor_id::sensor_id::SensorId;
use crate::core::vm::round_vm::round_vm::RoundVM;
use crate::core::vm::vm_status::vm_status::VMStatus;

pub mod round_vm {
    use std::any::Any;
    use crate::core::context::context::Context;
    use crate::core::export::export::Export;
    use crate::core::vm::vm_status::vm_status::VMStatus;

    /// A Round correspond to a local computation in a device. Create the context, evaluate the aggregate program and share the exports to the neighborhood.
    ///
    /// * `context` The context of the current round.
    ///
    /// * `status` The status of the current round.
    ///
    /// * `exports_stack` The stack of exports of the current round.
    #[derive(Debug)]
    pub struct RoundVM {
        pub(crate) context: Context,
        pub(crate) status: VMStatus,
        pub(crate) exports_stack: Vec<Export>,
    }
}

impl RoundVM {
    /// Create a new RoundVM
    ///
    /// ### Arguments
    ///
    /// * `context` The context of the current round.
    ///
    /// * `status` The status of the current round.
    ///
    /// * `exports_stack` The stack of exports of the current round.
    pub fn new(context: Context, status: VMStatus, exports_stack: Vec<Export>) -> Self {
        Self {
            context,
            status,
            exports_stack,
        }
    }

    /// The first export of the stack.
    pub fn export_data(&self) -> &Export {
        self.exports_stack.first().unwrap()
    }

    /// The id of the device.
    pub fn self_id(&self) -> i32 {
        self.context.self_id
    }

    /// TODO
    pub fn register_root(&self, v: Box<dyn Any>) {
        unimplemented!("todo")
    }

    /// If the computation is folding on a neighbor, get the id of the neighbor
    ///
    /// Returns the id.
    pub fn neighbor(&self) -> Option<i32> {
        self.status.neighbour
    }

    ///  The index of the current computation.
    pub fn index(&self) -> i32 {
        self.status.index
    }

    /// TODO
    pub fn previous_round_val<A:'static + Clone>(&self) -> Option<A>{
        self.context.read_export_value(self.self_id(), self.status.path.to_owned()).cloned()
    }

    /// TODO
    pub fn neighbor_val<A: 'static>(&self) -> A {
        unimplemented!("todo")
        // self.context.read_export_value(self.neighbor().unwrap(), self.status.path).unwrap()//.or_else(self.out_of_domain_exception())
    }

    /// TODO
    pub fn local_sense<A: 'static>(&self, sensor_id: SensorId) -> A {
        unimplemented!()
        //self.context.local_sense(sensor_id).unwrap()//.or_else(self.nbr_sensor_unknown_exception())
    }

    /// TODO
    pub fn neighbor_sense<A: 'static>(&self, sensor_id: SensorId) -> A {
        unimplemented!("todo")
    }

    // TODO pub fn folded_eval(&self, expr: => A)


    /// todo
    fn ensure(b: bool, s: String) {
        unimplemented!("todo")
    }

    /// Whether the device is contained in the neighbor list
    ///
    /// * return true if the device is contained in the neighbor list, false otherwise
    pub fn only_when_folding_on_self(&self) -> bool {
        match self.neighbor() {
            Some(neighbor) => neighbor == self.self_id(),
            _ => false
        }
    }

    /// Whether the device is contained in the neighbor list
    ///
    /// * return true if the device is contained in the neighbor list, false otherwise
    pub fn unless_folding_on_others(&self) -> bool {
        match self.neighbor() {
            Some(neighbor) => neighbor == self.self_id(),
            None => true,
        }
    }
}

#[cfg(test)]
mod tests{
    use std::any::Any;
    use std::collections::{HashMap, LinkedList};
    use crate::core::context::context::Context;
    use crate::core::export::export::Export;
    use crate::core::export_factory::export_factory::empty_path;
    use crate::core::path::path::path::Path;
    use crate::core::path::slot::slot::Slot::{Nbr, Rep};
    use crate::core::sensor_id::sensor_id::SensorId;
    use crate::core::vm::round_vm::round_vm::RoundVM;
    use crate::core::vm::vm_status::vm_status::VMStatus;

    fn round_vm_builder() -> RoundVM {
        let local_sensor = HashMap::from([(
            SensorId::new("test".to_string()),
            Box::new(10) as Box<dyn Any>,
        )]);
        let nbr_sensor = HashMap::from([(
            SensorId::new("test".to_string()),
            HashMap::from([(0, Box::new(10) as Box<dyn Any>)]),
        )]);
        let export = HashMap::from([(
            0,
            Export::new(HashMap::from([(
                Path::new(vec![Rep(0), Nbr(0)]),
                Box::new(10) as Box<dyn Any>,
            )])),
        )]);

        let context = Context::new(7, local_sensor, nbr_sensor, export);
        let status = VMStatus::new(empty_path(), 0, None, LinkedList::new());
        let exports_stack = vec![];
        RoundVM::new(context, status, exports_stack)
    }
}

