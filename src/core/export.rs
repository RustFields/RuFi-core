use std::any::Any;
use std::collections::HashMap;
use crate::core::export::export::Export;
use crate::core::path::path::path::Path;

pub mod export {
    use std::any::Any;
    use std::collections::HashMap;
    use crate::core::path::path::path::Path;

    #[derive(Debug)]
    pub struct Export{
        pub(crate) map: HashMap<Path, Box<dyn Any>>,
    }
}

impl Export {
    pub fn new(map: HashMap<Path, Box<dyn Any>>) -> Self {
        Export{ map }
    }

    pub fn put<A: 'static>(&mut self, path: Path, value: A) {
        self.map.insert(path, Box::new(value));
    }

    pub fn get<A: 'static>(&self, path: &Path) -> Option<&A> {
        self.map.get(path).and_then(|value| value.downcast_ref::<A>())
    }

    pub fn root<A: 'static>(&self) -> &A {
        self.get(&Path::new(Vec::new())).unwrap()
    }

    pub fn paths(&self) -> &HashMap<Path, Box<dyn Any>> {
        &self.map
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
            let export: Export = Export::new(HashMap::new());
            assert_eq!(export.map.is_empty(), true)
        }

        #[test]
        fn test_new() {
            let mut map: HashMap<Path, Box<dyn Any>> = HashMap::new();
            map.insert(Path::new(vec![Rep(0), Nbr(0)]), Box::new(10));
            let export = Export::new(map);
            assert_eq!(export.map.len(), 1);
        }

        #[test]
        fn test_put() {
            let mut map: HashMap<Path, Box<dyn Any>> = HashMap::new();
            map.insert(Path::new(vec![Rep(0)]), Box::new(10));
            let mut export = Export::new(map);
            export.put(Path::new(vec![Rep(0), Nbr(0)]), 20);
            export.put(Path::new(vec![Nbr(0)]), "foo");
            assert_eq!(export.paths().len(), 3);
        }

        #[test]
        fn test_get() {
            let mut map: HashMap<Path, Box<dyn Any>> = HashMap::new();
            map.insert(Path::new(vec![Rep(0), Nbr(0)]), Box::new(10));
            let export = Export::new(map);
            assert_eq!(export.get::<i32>(&Path::new(vec![Rep(0), Nbr(0)])).unwrap(), &10);
        }

        #[test]
        fn test_get_none() {
            let mut map: HashMap<Path, Box<dyn Any>> = HashMap::new();
            map.insert(Path::new(vec![Rep(0), Nbr(0)]), Box::new(10));
            let export = Export::new(map);
            assert_eq!(export.get::<String>(&Path::new(vec![Rep(0), Nbr(0)])), None);
        }

        #[test]
        fn test_root() {
            let mut map: HashMap<Path, Box<dyn Any>> = HashMap::new();
            map.insert(Path::new(vec![]), Box::new(10));
            let export = Export::new(map);
            assert_eq!(export.root::<i32>(), &10);
        }

        #[test]
        #[should_panic]
        fn test_root_panic() {
            let mut map: HashMap<Path, Box<dyn Any>> = HashMap::new();
            map.insert(Path::new(vec![]), Box::new(10));
            let export = Export::new(map);
            assert_eq!(export.root::<String>(), &"foo");
        }

        #[test]
        fn test_paths(){
            let mut map: HashMap<Path, Box<dyn Any>> = HashMap::new();
            let mut map2: HashMap<Path, Box<dyn Any>> = HashMap::new();
            map.insert(Path::new(vec![]), Box::new(10));
            map2.insert(Path::new(vec![]), Box::new(10));
            let export = Export::new(map);
            assert!(export.map.keys().eq(map2.keys()));
        }
    }

}
