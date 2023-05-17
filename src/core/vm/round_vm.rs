use crate::core::context::context::Context;
use crate::core::export::export::Export;
use crate::core::vm::round_vm::round_vm::RoundVM;
use crate::core::vm::vm_status::vm_status::VMStatus;

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
            None() => true,
        }
    }
}
