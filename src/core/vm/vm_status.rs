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
        ) -> Self {
            Self {
                path,
                index,
                neighbour,
                stack,
            }
        }

        pub fn is_folding(&self) -> bool {
            self.neighbour.is_some()
        }

        pub fn fold_into(&self, neighbour: Option<i32>) -> Self {
            Self {
                path: self.path.clone(),
                index: self.index.clone(),
                neighbour,
                stack: self.stack.clone(),
            }
        }

        pub fn fold_out(&self) -> Self {
            Self {
                path: self.path.clone(),
                index: self.index.clone(),
                neighbour: None,
                stack: self.stack.clone(),
            }
        }

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

        pub fn nest(&self, slot: Slot) -> Self {
            Self {
                path: self.path.push(slot),
                index: 0,
                neighbour: self.neighbour.clone(),
                stack: self.stack.clone(),
            }
        }

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
