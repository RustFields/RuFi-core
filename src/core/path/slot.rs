use crate::core::path::slot::slot::Slot;

pub mod slot {
    #[derive(PartialEq, Debug)]
    pub enum Slot{
        Nbr(i32),
        Rep(i32),
        Branch(i32),
    }
}

impl Slot{
    fn to_str(&self) -> String {
        match self {
            Slot::Nbr(index) => "Nbr(".to_owned()+&index.to_string()+")",
            Slot::Rep(index) => "Rep(".to_owned()+&index.to_string()+")",
            Slot::Branch(index) => "Branch(".to_owned()+&index.to_string()+")",
        }
    }
}

#[cfg(test)]
mod test{
    use super::*;

    #[test]
    fn test_slot_creation(){
        let nbr = Slot::Nbr(0);
        let rep = Slot::Rep(0);
        let branch = Slot::Branch(0);
        assert_eq!(nbr, Slot::Nbr(0));
        assert_eq!(rep, Slot::Rep(0));
        assert_eq!(branch, Slot::Branch(0));
    }

    #[test]
    fn test_slot_to_string() {
        let nbr = Slot::Nbr(0);
        let rep = Slot::Rep(0);
        let branch = Slot::Branch(0);
        assert_eq!(nbr.to_str(), "Nbr(0)");
        assert_eq!(rep.to_str(), "Rep(0)");
        assert_eq!(branch.to_str(), "Branch(0)");
    }
}