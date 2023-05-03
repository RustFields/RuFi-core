use std::collections::HashMap;
use crate::core::export::export::Export;
use crate::core::path::path::path::Path;

pub mod export{
    use std::collections::HashMap;
    use crate::core::path::path::path::Path;

    #[derive(Debug, Clone, PartialEq)]
    pub struct Export<A>{
        pub map: HashMap<Path, A>,
    }
}

impl<A: Clone + PartialEq> Export<A>{
    pub fn new() -> Self{
        Export{
            map: HashMap::new(),
        }
    }

    pub fn root(&self) -> A {
        self.get(Path::new(Vec::new())).unwrap()
    }

    pub fn put(&mut self, path: Path, value: A) -> A {
        self.map.insert(path, value.clone());
        value
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
        fn test_put() {
            let mut export = Export::new();
            let path = Path::new(vec![Nbr(0), Rep(0)]);
            export.put(path, 10);
            assert_eq!(export.paths().len(), 1);
        }

        #[test]
        fn test_get() {
            let mut export = Export::new();
            let path = Path::new(vec![Nbr(0), Rep(0)]);
            export.put(path.clone(), 10);
            assert_eq!(export.get(path.clone()).unwrap(), 10);
        }

        #[test]
        fn test_root() {
            let mut export:Export<Option<i32>> = Export::new();
            let path = Path::new(Vec::new());
            export.put(path.clone(), Some(10));
            assert_eq!(export.root(), Some(10));
        }

        #[test]
        fn test_eq() {
            let mut export1 = Export::new();
            let mut export2 = Export::new();
            let path = Path::new(vec![Nbr(0), Rep(0)]);
            export1.put(path.clone(), 10);
            export2.put(path.clone(), 10);
            assert_eq!(export1.eq(export2), true);
        }

        #[test]
        fn test_to_str() {
            let mut export = Export::new();
            let path = Path::new(vec![Nbr(0), Rep(0)]);
            export.put(path.clone(), 10);
            assert_eq!(export.to_str(), "P://Nbr(0)/Rep(0)\n");
        }
    }

}
