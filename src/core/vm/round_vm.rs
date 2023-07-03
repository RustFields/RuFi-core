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
    /// * `context` - The context of the current round.
    ///
    /// * `status` - The status of the current round.
    ///
    /// * `export_stack` - The stack of exports of the current round.
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
    /// * `context` - The context of the current round.
    ///
    /// # Returns
    ///
    /// A `RoundVM` instance.
    pub fn new(context: Context) -> Self {
        Self {
            context,
            status: VMStatus::new(),
            export_stack: vec![],
            isolated: false,
        }
    }

    /// # Returns
    ///
    /// The first export of the stack, of type `&mut Export`.
    pub fn export_data(&mut self) -> &mut Export {
        self.export_stack.first_mut().unwrap()
    }

    /// # Returns
    ///
    /// The id of the device, of type `i32`.
    pub fn self_id(&self) -> i32 {
        self.context.self_id
    }

    /// Register the given value for the root path.
    ///
    /// ### Arguments
    ///
    /// * `v` - The value to register.
    ///
    /// # Generic Parameters
    ///
    /// * `A` - The type of value. It must implement the `Copy` trait
    ///         and have a `'static` lifetime.
    pub fn register_root<A: 'static + Copy>(&mut self, v: A) {
        self.export_data().put(Path::new(), || v);
    }

    /// If the computation is folding on a neighbor, return the id of the neighbor
    ///
    /// # Returns
    ///
    /// An `&Option<i32>` containing the id of the neighbor, if present
    pub fn neighbor(&self) -> &Option<i32> {
        &self.status.neighbour
    }

    /// # Returns
    ///
    ///  The index of the current computation.
    pub fn index(&self) -> &i32 {
        &self.status.index
    }

    /// # Returns
    ///
    ///  The value of the previous round for the current device and the current path.
    ///
    /// # Generic Parameters
    ///
    /// * `A` - The type of value. It must implement the `Clone` trait
    ///         and have a `'static` lifetime.
    pub fn previous_round_val<A: 'static + Clone>(&self) -> Option<&A> {
        self.context
            .read_export_value::<A>(&self.self_id(), &self.status.path)
    }

    /// # Generic Parameters
    ///
    /// * `A` - The type of value. It must implement the `Clone` trait
    ///         and have a `'static` lifetime.
    ///
    /// # Returns
    ///
    ///  The value of the current path for the current neighbor.
    pub fn neighbor_val<A: 'static + Clone>(&self) -> Option<&A> {
        self.context
            .read_export_value::<A>(&self.neighbor().unwrap(), &self.status.path)
    }

    /// ### Arguments
    ///
    /// * - `sensor_id` - The id of the sensor.
    ///
    /// # Generic Parameters
    ///
    /// * `A` - The type of value returned by the sensor. It must have a `'static` lifetime.
    ///
    /// # Returns
    ///
    /// The local value of the given sensor.
    pub fn local_sense<A: 'static>(&self, sensor_id: &SensorId) -> Option<&A> {
        self.context.local_sense::<A>(sensor_id)
    }

    /// ### Arguments
    ///
    /// * `sensor_id` - The id of the sensor.
    ///
    /// # Generic Parameters
    ///
    /// * `A` - The type of value returned by the sensor. It must have a `'static` lifetime.
    ///
    /// # Returns
    ///
    /// The value of the given sensor for the current neighbor.
    pub fn nbr_sense<A: 'static>(&self, sensor_id: &SensorId) -> Option<&A> {
        self.context.nbr_sense(sensor_id, &self.neighbor().unwrap())
    }

    /// Perform a folded evaluation of the given expression in the given neighbor and return the result.
    ///
    /// # Arguments
    ///
    /// * `expr` - The expression to evaluate, which should return a value of type `A`.
    /// * `id` - The id of the neighbor.. It is of type `i32`.
    ///
    /// # Generic Parameters
    ///
    /// * `A` - The type of value returned by the expression.
    /// * `F` - The type of the expression, which must be a closure that takes no arguments and returns a value of type `A`.
    ///
    /// # Returns
    ///
    /// An `Option` containing the result of the expression.
    pub fn folded_eval<A, F>(&mut self, expr: F, id: i32) -> Option<A>
    where
        F: Fn() -> A,
    {
        self.status = self.status.push();
        self.status = self.status.fold_into(Some(id));
        let result = Some(expr());
        self.status = self.status.pop();
        result
    }

    /// Nest the current status, execute the given expression, and return the result.
    ///
    /// This function updates the status by pushing a nested slot, and
    /// evaluates the provided expression. The result of the expression is returned after restoring
    /// the status to its previous state.
    ///
    /// # Arguments
    ///
    /// * `slot` - The slot to nest in the current status.
    /// * `write` - A boolean flag indicating whether to perform a write operation.
    /// * `inc` - A boolean flag indicating whether to increment the index after nesting.
    /// * `expr` - The expression to evaluate, which should return a value of type `A`.
    ///
    /// # Generic Parameters
    ///
    /// * `A` - The type of value returned by the expression. It must implement the `Clone` trait
    ///         and have a `'static` lifetime.
    /// * `F` - The type of the expression, which must be a closure that takes no arguments and
    ///         returns a value of type `A`.
    ///
    /// # Returns
    ///
    /// The result of the expression.
    pub fn nest<A: Clone + 'static, F>(&mut self, slot: Slot, write: bool, inc: bool, expr: F) -> A
    where
        F: Fn() -> A,
    {
        let cloned_path = self.status.path.clone();
        self.status = self.status.push().nest(slot);
        let result: A = if write {
            match self.export_data().get::<A>(&cloned_path) {
                Some(x) => x.clone(),
                _ => self.export_data().put(cloned_path, || expr())
            }
        } else {
            expr()
        };
        self.status = match inc {
            true => self.status.pop().inc_index(),
            false => self.status.pop()
        };
        result
    }

    /// Evaluates the given expression locally and return the result.
    ///
    /// ### Arguments
    ///
    /// * `expr` The expression to evaluate.
    ///
    /// # Generic Parameters
    ///
    /// * `A` - The type of value returned by the expression.
    /// * `F` - The type of the closure, which must be a mutable closure that takes no arguments and returns a value of type `A`.
    ///
    /// # Returns
    ///
    /// The result of the closure `expr`.
    ///
    /// # Panics
    ///
    /// This function panics if the `neighbor` method returns `None`.
    pub fn locally<A, F>(&mut self, mut expr: F) -> A
    where
        F: FnMut() -> A,
    {
        let current_neighbour = self.neighbor().unwrap();
        self.status = self.status.fold_out();
        self.status = self.status.fold_into(Some(current_neighbour));
        expr()
    }

    /// Get a vector of aligned neighbor identifiers.
    ///
    /// # Returns
    ///
    /// A vector of aligned neighbor identifiers.
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

    /// Isolate the current device and evaluate the given expression
    ///
    /// # Arguments
    ///
    /// * `expr` - The closure to execute, which takes no arguments and returns a value of type `A`.
    ///
    /// # Generic Parameters
    ///
    /// * `A` - The type of value returned by the closure.
    /// * `F` - The type of the closure, which must be a mutable closure that takes no arguments and returns a value of type `A`.
    ///
    /// # Returns
    ///
    /// The result of the closure `expr`.
    pub fn isolate<A, F>(&mut self, mut expr: F) -> A
    where
        F: FnMut() -> A,
    {
        let was_isolated = self.isolated;
        self.isolated = true;
        self.isolated = was_isolated;
        expr()
    }

    /// Check if folding is not being performed on neighbor.
    ///
    /// # Returns
    ///
    /// - `true` if folding is being performed on self.
    /// - `false` if folding is being performed on neighbor.
    pub fn unless_folding_on_others(&self) -> bool {
        match self.neighbor() {
            Some(neighbor) => neighbor == &self.self_id(),
            None => true,
        }
    }

    /// Check if folding is being performed on self.
    ///
    /// # Returns
    ///
    /// - `true` if folding is being performed on self.
    /// - `false` otherwise.
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
    use std::any::Any;
    use std::collections::HashMap;
    use crate::core::vm::vm_status::vm_status::VMStatus;

    fn round_vm_builder() -> RoundVM {
        let local_sensor = HashMap::from([(
            SensorId::new("sensor1".to_string()),
            Box::new(10) as Box<dyn Any>,
        )]);
        let nbr_sensor = HashMap::from([(
            SensorId::new("sensor1".to_string()),
            HashMap::from([(0, Box::new(4) as Box<dyn Any>)]),
        )]);
        let exports = HashMap::from([
            (
                7,
                Export::from(HashMap::from([(
                    Path::from(vec![Rep(0), Nbr(0)]),
                    Box::new(10) as Box<dyn Any>,
                )])),
            ),
            (
                0,
                Export::from(HashMap::from([(
                    Path::from(vec![Rep(0), Nbr(0)]),
                    Box::new(2) as Box<dyn Any>,
                )])),
            ),
        ]);

        let context = Context::new(7, local_sensor, nbr_sensor, exports);
        let mut vm = RoundVM::new(context);
        vm.export_stack.push(Export::from(HashMap::from([(
            Path::new(),
            Box::new(0) as Box<dyn Any>,
        )])));
        let mut status = VMStatus::new();
        status.neighbour = Some(0);
        vm.status = status;
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
        assert_eq!(vm.export_data().get::<i32>(&Path::new()), None)
    }

    #[test]
    fn test_previous_round_val() {
        let mut vm = round_vm_builder();
        vm.status.path = Path::from(vec![Rep(0), Nbr(0)]);
        assert_eq!(vm.previous_round_val::<i32>().unwrap(), &10)
    }

    #[test]
    fn test_neighbor_val() {
        let mut vm = round_vm_builder();
        vm.status.path = Path::from(vec![Rep(0), Nbr(0)]);
        assert_eq!(vm.neighbor_val::<i32>().unwrap(), &2)
    }

    #[test]
    fn test_local_sense() {
        let vm = round_vm_builder();
        assert_eq!(
            vm.local_sense::<i32>(&SensorId::new("sensor1".to_string()))
                .unwrap(),
            &10
        )
    }

    #[test]
    fn test_nbr_sense() {
        let vm = round_vm_builder();
        assert_eq!(
            vm.nbr_sense::<i32>(&SensorId::new("sensor1".to_string()))
                .unwrap(),
            &4
        )
    }

    #[test]
    fn test_locally() {
        let mut vm = round_vm_builder();
        assert_eq!(vm.locally(|| expr()), expr())
    }

    #[test]
    fn test_aligned_neighbours() {
        let vm = round_vm_builder();
        assert_eq!(vm.aligned_neighbours(), vec![7, 0])
    }

    #[test]
    fn test_isolate() {
        let mut vm = round_vm_builder();
        let was_isolated = vm.isolated.clone();
        let result = vm.isolate(|| expr());
        assert_eq!(vm.isolated, was_isolated);
        assert_eq!(result, expr())
    }

    #[test]
    fn test_unless_folding_on_others() {
        let mut vm = round_vm_builder();
        assert!(!vm.unless_folding_on_others());
        vm.status.neighbour = None;
        assert!(vm.unless_folding_on_others());
        vm.status.neighbour = Some(7);
        assert!(vm.unless_folding_on_others());
    }

    #[test]
    fn test_only_when_folding_on_self() {
        let mut vm = round_vm_builder();
        assert!(!vm.only_when_folding_on_self());
        vm.status.neighbour = None;
        assert!(!vm.only_when_folding_on_self());
        vm.status.neighbour = Some(7);
        assert!(vm.only_when_folding_on_self());
    }
}
