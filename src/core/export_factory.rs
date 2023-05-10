pub mod export_factory {
    use std::any::Any;
    use std::collections::HashMap;
    use crate::core::export::export::Export;
    use crate::core::path::path::path::Path;
    use crate::core::path::slot::slot::Slot;

    pub fn empty_path() -> Path {
        Path::new(Vec::new())
    }

    pub fn empty_export() -> Export {
        Export::new(HashMap::new())
    }

    pub fn path(slots: Vec<Slot>) -> Path {
        Path::new(slots)
    }

    pub fn export_from(exps: Vec<(Path, Box<dyn Any>)>) -> Export {
        Export{
            map: HashMap::from_iter(exps.into_iter())
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::core::export::export::Export;
    use crate::core::export_factory::export_factory::*;
    use crate::core::path::slot::slot::Slot::{Nbr, Rep};

    #[test]
    fn test_empty_state() {
        let export: Export = empty_export();
        let path = path(vec![Nbr(0), Rep(0)]);
        assert_eq!(export.get::<i32>(&empty_path()), None);
        assert_eq!(export.get::<i32>(&path), None);
    }

    #[test]
    fn test_root_path() {
        let mut export: Export = empty_export();
        export.put(empty_path(), String::from("foo"));
        assert_eq!(export.get::<String>(&empty_path()).unwrap(), export.root::<String>());
        assert_eq!(export.get::<String>(&empty_path()), Some(&String::from("foo")));
    }

    #[test]
    fn test_non_root_path() {
        let mut export: Export = empty_export();
        let path = path(vec![Nbr(0), Rep(0)]);
        export.put(path.clone(), String::from("bar"));
        assert_eq!(export.get::<String>(&path).unwrap(), &String::from("bar"));
    }

    #[test]
    fn test_overwriting_with_different_type() {
        let mut export: Export = empty_export();
        export.put(empty_path(), String::from("foo"));
        assert_eq!(export.get::<String>(&empty_path()), Some(&String::from("foo")));
        export.put(empty_path(), 77);
        assert_eq!(export.get::<i32>(&empty_path()).unwrap(), &77);
    }
}
