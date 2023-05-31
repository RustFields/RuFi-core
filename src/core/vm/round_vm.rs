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

    pub fn register_root<A: 'static + Copy>(&mut self, v: A) {
        self.export_data().put(Path::new(vec![]), || { v });
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
        self.context
            .read_export_value::<A>(&self.neighbor().unwrap(), &self.status.path)
    }

    pub fn local_sense<A: 'static>(&self, sensor_id: &SensorId) -> Option<&A> {
        self.context.local_sense::<A>(sensor_id)
    }

    pub fn nbr_sense<A: 'static>(&self, sensor_id: &SensorId) -> Option<&A> {
        self.context.nbr_sense(sensor_id, &self.neighbor().unwrap())
    }

    pub fn folded_eval<A, F>(&mut self, expr: F, id: i32) -> Option<A>
        where
            F: Fn() -> A
    {
        self.status = self.status.push();
        self.status = self.status.fold_into(Some(id));
        let result = Some(expr());
        self.status = self.status.pop();
        result
    }

    pub fn nest<A: Clone + 'static, F>(&mut self, slot: Slot, write: bool, inc: bool, expr: F) -> A
        where
            F: Fn() -> A
    {
        let result: A;
        let cloned_path = self.status.path.clone();
        self.status = self.status.push().nest(slot);
        if write {
            if let Some(x) = self.export_data().get::<A>(&cloned_path) {
                result = x.clone();
            } else {
                result = self.export_data().put(cloned_path, || expr());
            }
        } else {
            result = expr();
        };
        if inc {
            self.status = self.status.pop().inc_index()
        } else {
            self.status = self.status.pop()
        };
        result
    }

    pub fn locally<A, F>(&mut self, mut expr: F) -> A
    where
        F: FnMut() -> A,
    {
        let current_neighbour = self.neighbor().unwrap();
        self.status = self.status.fold_out();
        self.status = self.status.fold_into(Some(current_neighbour));
        expr()
    }

    // This function return a copy of the aligned neighbours, not their reference, this could create problems in some cases
    pub fn aligned_neighbours(&self) -> Vec<i32> {
        let mut tmp: Vec<i32> = Vec::new();
        if !self.isolated {
            tmp = self
                .context
                .exports
                .iter()
                .filter(|(id, _)| id.clone() != &self.self_id())
                .filter(|(_, export)| {
                    self.status.path.is_root()
                        || export.get::<Box<dyn Any>>(&self.status.path).is_some()
                })
                .map(|(id, _)| id.clone())
                .collect();
            tmp.insert(0, self.self_id().clone());
        }
        tmp
    }

    pub fn isolate<A, F>(&mut self, mut expr: F) -> A
    where
        F: FnMut() -> A,
    {
        let was_isolated = self.isolated;
        self.isolated = true;
        self.isolated = was_isolated;
        expr()
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
    use crate::core::path::path::path::Path;
    use crate::core::path::slot::slot::Slot::{Nbr, Rep};
    use crate::core::sensor_id::sensor_id::SensorId;
    use crate::core::vm::round_vm::round_vm::RoundVM;
    use crate::core::vm::vm_status::vm_status::VMStatus;
    use std::any::Any;
    use std::collections::{HashMap, LinkedList};

    fn round_vm_builder() -> RoundVM {
        let local_sensor = HashMap::from([(
            SensorId::new("sensor1".to_string()),
            Box::new(10) as Box<dyn Any>,
        )]);
        let nbr_sensor = HashMap::from([(
            SensorId::new("sensor1".to_string()),
            HashMap::from([(0, Box::new(10) as Box<dyn Any>)]),
        )]);
        let export = HashMap::from([
            (7,
            Export::new(HashMap::from([
                (Path::new(vec![Rep(0), Nbr(0)]),
                Box::new(10) as Box<dyn Any>)
            ]))),
            (0,
             Export::new(HashMap::from([
                 (Path::new(vec![Rep(0), Nbr(0)]),
                  Box::new(2) as Box<dyn Any>)
             ]))),
        ]);

        let context = Context::new(7, local_sensor, nbr_sensor, export);
        let status = VMStatus::new(Path::new_empty(), 0, Some(0), LinkedList::new());
        let export_stack = vec![];
        let mut vm = RoundVM::new(context, status, export_stack);
        vm.export_stack.push(Export::new(HashMap::from([(Path::new_empty(), Box::new(0) as Box<dyn Any>)])));
        vm
    }

    fn expr() -> i32 {
        5 * 3
    }

    #[test]
    fn test_export_data() {
        let mut vm = round_vm_builder();
        assert_eq!(vm.export_data().root::<i32>(), &0)
    }

    #[test]
    fn test_register_root() {
        let mut vm = round_vm_builder();
        vm.register_root(Box::new(expr)());
        assert_eq!(vm.export_data().root::<i32>(), &expr())
    }

    #[test]
    fn test_folded_eval() {
        let mut vm = round_vm_builder();
        let result = vm.folded_eval(|| expr, 7);
        assert_eq!(round_vm_builder().status, vm.status);
        assert_eq!(result.unwrap()(), expr())
    }

    #[test]
    fn test_nest() {
        let mut vm = round_vm_builder();
        let result = vm.nest(Rep(vm.index().clone()), false, false, || expr);
        assert_eq!(round_vm_builder().status, vm.status);
        assert_eq!(result(), expr())
    }

    #[test]
    fn test_nest_inc_index() {
        let mut vm = round_vm_builder();
        vm.nest(Rep(vm.index().clone()), false, true, || expr);
        assert_eq!(round_vm_builder().status.index + 1, vm.status.index);
    }

    #[test]
    fn test_nest_write() {
        let mut vm = round_vm_builder();
        vm.nest(Rep(vm.index().clone()), true, false, || expr);
        assert_eq!(vm.export_data().get::<i32>(&Path::new_empty()), None)
    }

    #[test]
    fn test_previous_round_val() {
        let mut vm = round_vm_builder();
        vm.status.path = Path::new(vec![Rep(0), Nbr(0)]);
        assert_eq!(vm.previous_round_val::<i32>().unwrap(), &10)
    }

    #[test]
    fn test_neighbor_val() {
        let mut vm = round_vm_builder();
        vm.status.path = Path::new(vec![Rep(0), Nbr(0)]);
        assert_eq!(vm.neighbor_val::<i32>().unwrap(), &2)
    }

    #[test]
    fn test_local_sense() {
        let mut vm = round_vm_builder();
        assert_eq!(vm.local_sense::<i32>(&SensorId::new("sensor1".to_string())).unwrap(), &10)
    }
}
