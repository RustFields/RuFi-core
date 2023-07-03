pub mod path {
    use crate::core::path::slot::slot::Slot;

    /// A Path is a collection of Slots that behave like an immutable stack
    #[derive(PartialEq, Debug, Clone, Eq, Hash)]
    pub struct Path {
        pub(crate) slots: Vec<Slot>,
    }
}

use crate::core::path::path::path::Path;
use crate::core::path::slot::slot::Slot;

impl Path {
    /// Factory method to create a new Path
    pub fn new() -> Self {
        Self {
            slots: vec![],
        }
    }

    /// Push a Slot into the Path
    pub fn push(&self, slot: Slot) -> Self {
        Self {
            slots: [&[slot], &self.slots[..]].concat(),
        }
    }

    /// Remove the first Slot from the Path
    pub fn pull(&self) -> Self {
        let mut new_slots = self.slots.clone();
        new_slots.drain(..1);
        Self { slots: new_slots }
    }

    /// Check if the Path is empty
    pub fn is_root(&self) -> bool {
        self.slots.is_empty()
    }

    /// Return a String representation of the Path
    pub fn to_str(&self) -> String {
        let slots = &self.slots;
        let path = String::from("P://");
        path + &slots
            .into_iter()
            .map(|slot| slot.to_str())
            .collect::<Vec<String>>()
            .join("/")
    }

    /// Check if the Path matches another Path
    pub fn matches(&self, p: &Path) -> bool {
        self == p
    }

    /// Return the first Slot of the Path
    pub fn head(&self) -> &Slot {
        self.slots.first().unwrap()
    }
}

impl From<Vec<Slot>> for Path {
    fn from(slots: Vec<Slot>) -> Self {
        let mut reversed_slots = slots;
        reversed_slots.reverse();
        Self {
            slots: reversed_slots,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::core::path::path::path::Path;
    use crate::core::path::slot::slot::Slot::{Branch, Nbr, Rep};

    #[test]
    fn test_is_root() {
        let empty_path = Path::from(Vec::new());
        let not_empty_path = Path::from(vec![Rep(0), Nbr(0), Nbr(1), Branch(0)]);
        assert!(empty_path.is_root());
        assert!(!not_empty_path.is_root())
    }

    #[test]
    fn test_not_empty_head() {
        let path = Path::from(vec![Rep(0), Nbr(0), Nbr(1), Branch(0)]);
        assert_eq!(path.head(), &Branch(0))
    }

    #[test]
    #[should_panic]
    fn test_empty_head() {
        let path = Path::new();
        assert_eq!(path.head(), &Rep(0))
    }

    #[test]
    fn test_push() {
        let path = Path::from(vec![Rep(0), Nbr(0), Nbr(1)]).push(Branch(0));
        assert_eq!(path.slots, vec![Branch(0), Nbr(1), Nbr(0), Rep(0)])
    }

    #[test]
    fn test_not_empty_pull() {
        let path = Path::from(vec![Rep(0), Nbr(0), Nbr(1), Branch(0)]);
        assert_eq!(path.pull().slots, vec![Nbr(1), Nbr(0), Rep(0)])
    }

    #[test]
    #[should_panic]
    fn test_empty_pull() {
        let path = Path::new();
        assert_eq!(path.pull(), Path::new())
    }

    #[test]
    fn test_to_str() {
        let path = Path::from(vec![Rep(0), Nbr(0), Nbr(1), Branch(0)]);
        assert_eq!(path.to_str(), "P://Branch(0)/Nbr(1)/Nbr(0)/Rep(0)")
    }

    #[test]
    fn test_matches() {
        let path = Path::from(vec![Rep(0), Nbr(0), Nbr(1), Branch(0)]);
        assert!(path.matches(&Path::from(vec![Rep(0), Nbr(0), Nbr(1), Branch(0)])));
        assert!(!path.matches(&Path::from(vec![Nbr(0), Nbr(1), Branch(0)])))
    }
}
