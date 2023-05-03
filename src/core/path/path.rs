pub mod path {
    use crate::core::path::slot::slot::Slot;

    /// A Path is a collection of Slots that behave like an immutable stack
    #[derive(PartialEq, Debug, Clone)]
    pub struct Path {
        pub slots: Vec<Slot>,
    }
}

use crate::core::path::path::path::Path;
use crate::core::path::slot::slot::Slot;

impl Path {
    /// Factory method to create a new Path
    pub fn new(slots: Vec<Slot>) -> Self {
        Path { slots }
    }

    /// Push a Slot into the Path
    pub fn push(&self, slot: Slot) -> Self {
        Self {
            slots: [&self.slots[..], &[slot]].concat(),
        }
    }

    /// Remove the first Slot from the Path
    pub fn pull(&self) -> Self {
        Self {
            slots: self.slots[..self.slots.len() - 1].to_vec(),
        }
    }

    /// Check if the Path is empty
    pub fn is_root(&self) -> bool {
        self.slots.is_empty()
    }

    pub fn to_str(&self) -> String {
        let slots = &self.slots;
        let path = String::from("P://");
        path +
            &slots.into_iter().map(|slot| slot.to_str()).collect::<Vec<String>>().join("/")
    }

    pub fn matches(&self, p: &Path) -> bool {
        self == p
    }

    /// Get the first Slot of the Path
    pub fn head(&self) -> &Slot {
        if let Some(head) = self.slots.first() {
            head
        } else {
            panic!("Path is empty")
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::core::path::path::path::Path;
    use crate::core::path::slot::slot::Slot::{Nbr, Rep, Branch};

    #[test]
    fn test_is_root() {
        let path = Path::new(Vec::new());
        assert!(path.is_root())
    }

    #[test]
    fn test_not_empty_head() {
        let path = Path::new(vec![Rep(0), Nbr(0), Nbr(1), Branch(0)]);
        assert_eq!(path.head(), &Rep(0))
    }

    #[test]
    #[should_panic]
    fn test_empty_head() {
        let path = Path::new(vec![]);
        assert_eq!(path.head(), &Rep(0))
    }

    #[test]
    fn test_push() {
        let path = Path::new(vec![Rep(0), Nbr(0), Nbr(1)]).push(Branch(0));
        assert_eq!(path.slots, vec![Rep(0), Nbr(0), Nbr(1), Branch(0)])
    }

    #[test]
    fn test_not_empty_pull() {
        let path = Path::new(vec![Rep(0), Nbr(0), Nbr(1), Branch(0)]);
        assert_eq!(path.pull(), Path::new(vec![Rep(0), Nbr(0), Nbr(1)]))
    }

    #[test]
    #[should_panic]
    fn test_empty_pull() {
        let path = Path::new(vec![]);
        assert_eq!(path.pull(), Path::new(vec![]))
    }

    #[test]
    fn test_to_str() {
        let path = Path::new(vec![Rep(0), Nbr(0), Nbr(1), Branch(0)]);
        assert_eq!(path.to_str(), "P://Rep(0)/Nbr(0)/Nbr(1)/Branch(0)")
    }
}