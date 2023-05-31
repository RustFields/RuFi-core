pub mod vm_status {
    use crate::core::path::path::path::Path;
    use crate::core::path::slot::slot::Slot;
    use std::collections::LinkedList;

    /// Models the status of the virtual machine.
    ///
    /// * `path` - The path of the computation.
    /// * `index` - The index of the current slot.
    /// * `neighbour` - The id of the current neighbour. If the current slot is not a folding slot, this value is None.
    /// * `stack` - Stack that contains the list of the statuses
    #[derive(Debug, PartialEq, Clone)]
    pub struct VMStatus {
        pub(crate) path: Path,
        pub(crate) index: i32,
        pub(crate) neighbour: Option<i32>,
        pub(crate) stack: LinkedList<(Path, i32, Option<i32>)>,
    }

    impl VMStatus {
        /// ## Create new VMStatus from the given parameters.
        ///
        /// ### Arguments
        ///
        /// * `path` The path of the computation.
        /// * `index` The index of the current slot.
        /// * `neighbour` The id of the current neighbour. If the current slot is not a folding slot, this value is None.
        /// * `stack` Stack that contains the list of the statuses
        pub fn new(
            path: Path,
            index: i32,
            neighbour: Option<i32>,
            stack: LinkedList<(Path, i32, Option<i32>)>,
        ) -> Self {
            Self {
                path,
                index,
                neighbour,
                stack,
            }
        }

        /// Whether the VM is folding or not.
        pub fn is_folding(&self) -> bool {
            self.neighbour.is_some()
        }

        /// Fold the current slot into the given neighbour.
        ///
        /// * `neighbour` he id of the neighbour.
        pub fn fold_into(&self, neighbour: Option<i32>) -> Self {
            Self {
                path: self.path.clone(),
                index: self.index.clone(),
                neighbour,
                stack: self.stack.clone(),
            }
        }

        /// Fold out of the current slot.
        pub fn fold_out(&self) -> Self {
            Self {
                path: self.path.clone(),
                index: self.index.clone(),
                neighbour: None,
                stack: self.stack.clone(),
            }
        }

        /// Push the current status on the stack.
        pub fn push(&self) -> Self {
            let mut new_stack = self.stack.clone();
            new_stack.push_front((
                self.path.clone(),
                self.index.clone(),
                self.neighbour.clone(),
            ));
            Self {
                path: self.path.clone(),
                index: self.index.clone(),
                neighbour: self.neighbour.clone(),
                stack: new_stack,
            }
        }

        /// Pop the current status from the stack.
        pub fn pop(&self) -> Self {
            let mut new_stack = self.stack.clone();
            let front = new_stack.pop_front();
            match front {
                Some((p, i, n)) => Self {
                    path: p.clone(),
                    index: i.clone(),
                    neighbour: n.clone(),
                    stack: new_stack,
                },
                _ => panic!(),
            }
        }

        /// Nest the given slot.
        ///
        /// * `slot` the slot to nest.
        pub fn nest(&self, slot: Slot) -> Self {
            Self {
                path: self.path.push(slot),
                index: 0,
                neighbour: self.neighbour.clone(),
                stack: self.stack.clone(),
            }
        }

        /// Increment the index of the current slot.
        pub fn inc_index(&self) -> Self {
            Self {
                path: self.path.clone(),
                index: self.index + 1,
                neighbour: self.neighbour.clone(),
                stack: self.stack.clone(),
            }
        }
    }
}

#[cfg(test)]
mod test {
    use crate::core::path::slot::slot::Slot::{Nbr, Rep};
    use crate::core::vm::vm_status::vm_status::VMStatus;
    use std::collections::LinkedList;
    use crate::core::path::path::path::Path;

    #[test]
    fn test_empty() {
        let status = VMStatus::new(Path::new(vec![]), 0, None, LinkedList::new());
        assert_eq!(status.path, Path::new(vec![]));
        assert_eq!(status.index, 0);
        assert_eq!(status.neighbour, None)
    }

    #[test]
    fn test_fold_unfold() {
        let status = VMStatus::new(Path::new(vec![]), 0, None, LinkedList::new());
        assert_eq!(status.neighbour, None);
        let s1 = status.fold_into(Some(7));
        let s2 = status.fold_into(Some(8));
        assert_eq!(status.neighbour, None);
        assert!(!status.is_folding());
        assert_eq!(s1.neighbour, Some(7));
        assert!(s1.is_folding());
        assert_eq!(s2.neighbour, Some(8));
        assert!(s2.is_folding())
    }

    #[test]
    #[should_panic]
    fn test_as_stack_panic() {
        let status = VMStatus::new(Path::new(vec![]), 0, None, LinkedList::new());
        status
            .push()
            .fold_into(Some(7))
            .nest(Nbr(2))
            .push()
            .fold_into(Some(8))
            .nest(Rep(4))
            .inc_index()
            .push()
            .pop()
            .pop()
            .pop()
            .pop();
    }

    #[test]
    fn test_as_stack() {
        let status = VMStatus::new(Path::new(vec![]), 0, None, LinkedList::new());
        let s1 = status.push();
        let s2 = s1.fold_into(Some(7)).nest(Nbr(2)).push();
        let s3 = s2.fold_into(Some(8)).nest(Rep(4)).inc_index().push();
        let s4 = s3.pop();
        let s5 = s4.pop();
        let s6 = s5.pop();
        assert_eq!(s4.index, 1);
        assert_eq!(s4.neighbour, Some(8));
        assert_eq!(s4.path.slots, vec![Rep(4), Nbr(2)]);
        assert_eq!(s5.index, 0);
        assert_eq!(s5.neighbour, Some(7));
        assert_eq!(s5.path, Path::new(vec![Nbr(2)]));
        assert_eq!(s6.index, 0);
        assert_eq!(s6.neighbour, None);
        assert_eq!(s6.path, Path::new(vec![]));
    }

    #[test]
    fn test_index() {
        let status = VMStatus::new(Path::new(vec![]), 0, None, LinkedList::new());
        assert_eq!(status.index, 0);
        assert_eq!(status.inc_index().index, 1);
        assert_eq!(status.inc_index().inc_index().inc_index().index, 3);
        assert_eq!(
            status
                .inc_index()
                .inc_index()
                .nest(Nbr(0))
                .inc_index()
                .index,
            1
        )
    }
}
