use std::any::Any;
use std::collections::HashMap;
use crate::core::export::export::Export;
use crate::core::path::path::path::Path;

pub mod export{
    use std::any::Any;
    use std::collections::HashMap;
    use crate::core::path::path::path::Path;

    #[derive(Debug)]
    pub struct Export{
        pub map: HashMap<Path, Box<dyn Any>>,
    }
}

impl Export{
    pub fn new_empty_export() -> Self{
        Export{
            map: HashMap::new(),
        }
    }

    pub fn new<A>(path: Path, value: A) -> Self {
        Export{
            map: HashMap::from([(path.clone(), value.clone())]),
        }
    }

    pub fn root<A>(&self) -> A {
        self.get(Path::new(Vec::new())).unwrap()
    }
    pub fn put<A>(&self, path: &Path, value: &A) -> Self {
        let mut map = self.map.clone();
        map.insert(path, value);
        Export{
            map,
        }
    }

    pub fn get(&self, path: Path) -> Option<A> {
        self.map.get(&path).and_then(|value| Some(value)).cloned()
    }

    pub fn paths(&self) -> &HashMap<Path, A> {
        &self.map
    }

    pub fn eq(&self, other: Export<A>) -> bool {
        self.map == other.map
    }

    pub fn to_str(&self) -> String {
        let mut string = String::new();
        for (path, _value) in self.map.iter() { //unimplemented!("value to string, due to generic type A not supporting to_string()")
            string.push_str(&format!("{}\n", path.to_str()));
        }
        string
    }
}

#[cfg(test)]
mod tests {
    use crate::core::path::slot::slot::Slot::{Nbr, Rep};
    use super::*;
    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn test_new_empty() {
            let export = Export::new_empty_export();
            assert_eq!(export.map.is_empty(), true)
        }

        #[test]
        fn test_new() {
            let export = Export::new(Path::new(vec![Nbr(0), Rep(0)]), 10);
            assert_eq!(export.map.len(), 1);
        }

        #[test]
        fn test_put() {
            let export1 = Export::new(Path::new(vec![Nbr(0)]), 10);
            let path = Path::new(vec![Rep(0), Nbr(0)]);
            let export2 = export1.put(path, 20);
            assert_eq!(export2.paths().len(), 2);
        }

        #[test]
        fn test_get() {
            let path = Path::new(vec![Nbr(0), Rep(0)]);
            let export = Export::new(path, 10);
            assert_eq!(export.get(path.clone()).unwrap(), 10);
        }

        #[test]
        fn test_root() {
            let export = Export::new(Path::new(Vec::new()), 10);
            assert_eq!(export.root(), 10);
        }

        #[test]
        fn test_paths(){
            let path = Path::new(vec![Nbr(0), Rep(0)]);
            let export = Export::new(path.clone(), 10);
            let hm = HashMap::from([(path.clone(), 10)]);
            assert_eq!(export.paths().eq(&hm), true);
        }

        #[test]
        fn test_eq() {
            let path = Path::new(vec![Nbr(0), Rep(0)]);
            let export1 = Export::new(path.clone(), 10);
            let export2 = Export::new(path.clone(), 10);
            assert_eq!(export1.eq(export2), true);
        }

        #[test]
        fn test_to_str() {
            let path = Path::new(vec![Nbr(0), Rep(0)]);
            let export = Export::new(path.clone(), 10);
            assert_eq!(export.to_str(), "P://Nbr(0)/Rep(0) -> 10, "); }
    }

}
