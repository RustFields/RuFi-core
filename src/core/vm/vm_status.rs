pub mod vm_status {
    use crate::core::path::path::path::Path;
    use crate::core::path::slot::slot::Slot;
    use std::collections::LinkedList;

    #[derive(Debug)]
    pub struct VMStatus {
        pub(crate) path: Path,
        pub(crate) index: i32,
        pub(crate) neighbour: Option<i32>,
        pub(crate) stack: LinkedList<(Path, i32, Option<i32>)>,
    }

    impl VMStatus {
        pub fn new(
            path: Path,
            index: i32,
            neighbour: Option<i32>,
            stack: LinkedList<(Path, i32, Option<i32>)>,
        ) -> VMStatus {
            VMStatus {
                path,
                index,
                neighbour,
                stack,
            }
        }

        pub fn is_folding(&self) -> bool {
            self.neighbour.is_some()
        }

        pub fn fold_into(&self, neighbour: Option<i32>) -> VMStatus {
            VMStatus::new(
                self.path.clone(),
                self.index.clone(),
                neighbour,
                self.stack.clone(),
            )
        }

        pub fn fold_out(&self) -> VMStatus {
            VMStatus::new(
                self.path.clone(),
                self.index.clone(),
                None,
                self.stack.clone(),
            )
        }

        pub fn push(&self) -> VMStatus {
            let mut new_stack = self.stack.clone();
            new_stack.push_front((
                self.path.clone(),
                self.index.clone(),
                self.neighbour.clone(),
            ));
            VMStatus::new(
                self.path.clone(),
                self.index.clone(),
                self.neighbour.clone(),
                new_stack,
            )
        }

        pub fn pop(&self) -> VMStatus {
            let mut new_stack = self.stack.clone();
            new_stack.pop_front();
            match self.stack.clone().front() {
                Some(&(ref p, i, n)) => VMStatus::new(p.clone(), i, n, new_stack),
                _ => panic!(),
            }
        }

        pub fn nest(&self, slot: Slot) -> VMStatus {
            let new_path = self.path.clone();
            new_path.push(slot);
            VMStatus::new(new_path, 0, self.neighbour.clone(), self.stack.clone())
        }

        pub fn inc_index(&self) -> VMStatus {
            VMStatus::new(
                self.path.clone(),
                self.index + 1,
                self.neighbour.clone(),
                self.stack.clone(),
            )
        }
    }
}
