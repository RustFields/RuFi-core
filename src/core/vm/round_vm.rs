use crate::core::context::context::Context;
use crate::core::export::export::Export;
use crate::core::path::path::path::Path;
use crate::core::path::slot::slot::Slot;
use crate::core::sensor_id::sensor_id::SensorId;
use crate::core::vm::round_vm::round_vm::RoundVM;
use crate::core::vm::vm_status::vm_status::VMStatus;
use std::any::Any;

pub mod round_vm {
    use crate::core::context::context::Context;
    use crate::core::export::export::Export;
    use crate::core::vm::vm_status::vm_status::VMStatus;

    /// A Round correspond to a local computation in a device. Create the context, evaluate the aggregate program and share the exports to the neighborhood.
    ///
    /// * `context` The context of the current round.
    ///
    /// * `status` The status of the current round.
    ///
    /// * `export_stack` The stack of exports of the current round.
    #[derive(Debug)]
    pub struct RoundVM {
        pub(crate) context: Context,
        pub(crate) status: VMStatus,
        pub(crate) export_stack: Vec<Export>,
        pub(crate) isolated: bool,
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
    /// * `export_stack` The stack of exports of the current round.
    pub fn new(context: Context, status: VMStatus, export_stack: Vec<Export>) -> Self {
        Self {
            context,
            status,
            export_stack,
            isolated: false,
        }
    }

    /// The first export of the stack.
    pub fn export_data(&mut self) -> &mut Export {
        self.export_stack.first_mut().unwrap()
    }

    /// The id of the device.
    pub fn self_id(&self) -> i32 {
        self.context.self_id
    }

    pub fn register_root(&mut self, v: Box<dyn Any>) {
        self.export_data().put(Path::new(vec![]), v);
    }

    /// If the computation is folding on a neighbor, get the id of the neighbor
    ///
    /// Returns the id.
    pub fn neighbor(&self) -> &Option<i32> {
        &self.status.neighbour
    }

    ///  The index of the current computation.
    pub fn index(&self) -> &i32 {
        &self.status.index
    }

    pub fn previous_round_val<A: 'static + Clone>(&self) -> Option<&A> {
        self.context
            .read_export_value::<A>(&self.self_id(), &self.status.path)
    }

    pub fn neighbor_val<A: 'static + Clone>(&self) -> Option<&A> {
        self.context.read_export_value::<A>(&self.neighbor().unwrap(), &self.status.path)
    }

    pub fn local_sense<A: 'static>(&self, sensor_id: &SensorId) -> Option<&A> {
        self.context.local_sense::<A>(sensor_id)
    }

    pub fn nbr_sense<A: 'static>(&self, sensor_id: &SensorId) -> Option<&A> {
        self.context.nbr_sense(sensor_id, &self.neighbor().unwrap())
    }

    pub fn folded_eval<A>(&mut self, expr: A, id: i32) -> Option<A> {
        let result = {
            self.status = self.status.push();
            self.status = self.status.fold_into(Some(id));
            Some(expr)
        };
        self.status = self.status.pop();
        match result {
            Some(a) => Some(a),
            None => None //OutOfDomainException::new(self.self_id(), id, self.status.path.clone())
        }
    }

    pub fn nest<A>(&mut self, slot: Slot, write: bool, inc: bool, expr: A) -> A
        where A: FnOnce() -> A + 'static + Copy {
        let cloned_path = self.status.path.clone();
        self.status = self.status.push().nest(slot);
        let result = if write {
            if let Some(&x) = self.export_data().get(&cloned_path) {
                x
            } else {
                self.export_data().put(cloned_path, expr.clone());
                &expr
            }
        } else {
            &expr
        };
        self.status = if inc {
            self.status.pop().inc_index()
        } else {
            self.status.pop()
        };
        result.to_owned()
    }

    pub fn locally<A, F>(&mut self, mut a: F) -> A
    where
        F: FnMut() -> A,
    {
        let current_neighbour = self.neighbor().unwrap();
        self.status = self.status.fold_out();
        self.status = self.status.fold_into(Some(current_neighbour));
        a()
    }

    // This function return a copy of the aligned neighbours, not their reference, this could create problems in some cases
    pub fn aligned_neighbours(&self) -> Vec<i32> {
        let mut tmp: Vec<i32> = Vec::new();
        if !self.isolated {
            tmp = self.context
                .exports
                .iter()
                .filter(|(id, _)| id.clone() != &self.self_id())
                .filter(|(_, export)| self.status.path.is_root() || export.get::<Box<dyn Any>>(&self.status.path).is_some())
                .map(|(id, _)| id.clone())
                .collect();
            tmp.insert(0, self.self_id().clone());
        }
        tmp
    }

    pub fn isolate<A, F>(&mut self, mut a: F) -> A
    where
        F: FnMut() -> A,
    {
        let was_isolated = self.isolated;
        let result = {
            self.isolated = true;
            a()
        };
        self.isolated = was_isolated;
        a()
    }

    /// Whether the device is contained in the neighbor list
    ///
    /// * return true if the device is contained in the neighbor list, false otherwise
    pub fn unless_folding_on_others(&self) -> bool {
        match self.neighbor() {
            Some(neighbor) => neighbor == &self.self_id(),
            None => true,
        }
    }

    /// Whether the device is contained in the neighbor list
    ///
    /// * return true if the device is contained in the neighbor list, false otherwise
    pub fn only_when_folding_on_self(&self) -> bool {
        match self.neighbor() {
            Some(neighbor) => neighbor == &self.self_id(),
            _ => false,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::core::context::context::Context;
    use crate::core::export::export::Export;
    use crate::core::export_factory::export_factory::empty_path;
    use crate::core::path::path::path::Path;
    use crate::core::path::slot::slot::Slot::{Nbr, Rep};
    use crate::core::sensor_id::sensor_id::SensorId;
    use crate::core::vm::round_vm::round_vm::RoundVM;
    use crate::core::vm::vm_status::vm_status::VMStatus;
    use std::any::Any;
    use std::collections::{HashMap, LinkedList};

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
        let export_stack = vec![];
        let factory: Export = Export::new(HashMap::new());
        RoundVM::new(context, status, export_stack, factory)
    }
}
