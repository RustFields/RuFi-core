use std::any::Any;
use std::collections::HashMap;
use std::fmt::{Debug, Display, Formatter};
use std::rc::Rc;
use ser::*;
use serde::{Deserialize, Serialize};
use crate::core::path::path::Path;

/// Abstraction for the result of local computation.
/// It is an AST decorated with the computation value.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Export {
    #[serde(serialize_with = "serialize_rc_box_any_map", deserialize_with = "deserialize_rc_box_any_map")]
    pub(crate) map: HashMap<Path, Rc<Box<dyn Any>>>,
}

#[macro_export]
macro_rules! export {
        ($($x:expr),*) => {{
            let mut temp_map = HashMap::new();
            $(
                temp_map.insert($x.0, std::rc::Rc::new(Box::new($x.1) as Box<dyn Any>));
            )*
            Export::from(temp_map)
        }};
    }

impl Export {
    /// Create new Export.
    ///
    /// # Returns
    ///
    /// The new Export.
    pub fn new() -> Self {
        Self {
            map: HashMap::new()
        }
    }

    /// Inserts a value in the Export at the given Path.
    ///
    /// # Arguments
    ///
    /// * `path` - The Path where to insert the value.
    /// * `value` - The value to insert.
    ///
    /// # Generic Parameters
    ///
    /// * `A` - The type of the value to insert. It must have a `'static` lifetime.
    /// * `F` - The type of the function to insert.
    ///
    /// # Returns
    ///
    /// The inserted value.
    pub fn put<A: 'static, F>(&mut self, path: Path, value: F) -> A
    where F: Fn() -> A {
        self.map.insert(path, Rc::new(Box::new(value())));
        value()
    }

    /// Returns the value at the given Path.
    ///
    /// # Arguments
    ///
    /// * `path` - The Path where to get the value.
    ///
    /// # Generic Parameters
    ///
    /// * `A` - The type of the value to get  to return. It must have a `'static` lifetime.
    ///
    /// # Returns
    ///
    /// The value at the given Path.
    pub fn get<A: 'static>(&self, path: &Path) -> Option<&A> {
        self.map
            .get(path)
            .and_then(|value| value.downcast_ref::<A>())
    }

    /// Obtain the root value.
    ///
    /// # Generic Parameters
    ///
    /// * `A` - The type of the value to return. It must have a `'static` lifetime.
    ///
    /// # Returns
    ///
    /// The root value.
    pub fn root<A: 'static>(&self) -> &A {
        self.get(&Path::new()).unwrap()
    }

    /// Returns the HashMap of the Export.
    ///
    /// # Returns
    ///
    /// The HashMap of the Export.
    pub fn paths(&self) -> &HashMap<Path, Rc<Box<dyn Any>>> {
        &self.map
    }
}

impl From<HashMap<Path, Rc<Box<dyn Any>>>> for Export {
    fn from(map: HashMap<Path, Rc<Box<dyn Any>>>) -> Self {
        Self {
            map
        }
    }
}

/// Since the types inside an Export map can be various, we compare the serialized string versions
/// of the Exports. This will introduce a performance overhead in the presence of very large maps.
/// If a more memory efficient implementation is needed, we would need to downcast the types and
/// check the equality of the correctly downcasted data.
impl PartialEq for Export {
    fn eq(&self, other: &Self) -> bool {
        // Serialize self and other to JSON strings and compare them
        let self_json = serde_json::to_string(self).unwrap();
        let other_json = serde_json::to_string(other).unwrap();
        self_json == other_json
    }
}

impl Display for Export {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        serde_json::to_string(&self).fmt(f)
    }
}

impl Eq for Export {}

/// Private module that defines custom serializer and deserializer
mod ser {
    use std::any::Any;
    use std::collections::HashMap;
    use std::rc::Rc;
    use serde::{Deserialize, Deserializer, Serialize, Serializer};
    use serde::ser::SerializeMap;
    use crate::core::path::path::Path;

    pub fn serialize_rc_box_any_map<S>(data: &HashMap<Path, Rc<Box<dyn Any>>>, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
    {
        // Serialize the data and wrap it in a HashMap<Path, [u8]>
        let serializable_data: HashMap<String, String> =
            data
                .iter()
                .map(|(key, value)| {
                    let key_str = serde_json::to_string(key).unwrap();
                    // if value is an i32, cast as String, otherwise panic
                    if let Some(value) = value.downcast_ref::<i32>() {
                        (key_str, value.clone().to_string())
                    } else if let Some(value) = value.downcast_ref::<bool>() {
                        (key_str, value.clone().to_string())
                    } else if let Some(value) = value.downcast_ref::<String>() {
                        (key_str, value.clone())
                    } else{
                        panic!("Cannot serialize type {:?}", value)
                    }
                })
                .collect();
        serializable_data.serialize(serializer)
    }

    pub fn deserialize_rc_box_any_map<'de, D>(deserializer: D) -> Result<HashMap<Path, Rc<Box<dyn Any>>>, D::Error>
        where
            D: Deserializer<'de>,
    {
        // Deserialize the data and wrap it in a HashMap<i32, Rc<Box<dyn Any>>
        let data: HashMap<String, String> = Deserialize::deserialize(deserializer)?;

        // Convert the deserialized data back into HashMap<i32, Rc<Box<dyn Any>>
        let deserialized_data: HashMap<Path, Rc<Box<dyn Any>>> =
            data
                .into_iter()
                .map(|(key, value)| {
                    let path: Path = serde_json::from_str(&key).unwrap();
                    (path, Rc::new(Box::new(value) as Box<dyn Any>))
                })
                .collect();

        Ok(deserialized_data)
    }
}

#[cfg(test)]
mod tests {
    use crate::core::path::slot::Slot::{Nbr, Rep};
    use crate::path;
    use super::*;

    #[test]
    fn test_new_empty() {
        let export: Export = Export::new();
        assert!(export.map.is_empty())
    }

    #[test]
    fn test_new() {
        /* showing how the macros saves us from writing this:
        let mut map: HashMap<Path, Rc<Box<dyn Any>>> = HashMap::new();
        map.insert(Path::from(vec![Rep(0), Nbr(0)]), Rc::new(Box::new(10)));
        let export = Export::from(map);*/
        let export = export!((path!(Rep(0), Nbr(0)), 10));
        assert_eq!(export.map.len(), 1);
    }

    #[test]
    fn test_put() {
        let mut export = export!((path!(Rep(0)), 10));
        export.put(path!(Rep(0), Nbr(0)), || 20);
        export.put(Path::from(vec![Nbr(0)]), || "foo");
        assert_eq!(export.paths().len(), 3);
    }

    #[test]
    fn test_get() {
        let export = export!((path!(Nbr(0), Rep(0)), 10));
        assert_eq!(
            //path is written in reverse order in the macro
            export.get::<i32>(&Path::from(vec![Rep(0), Nbr(0)])).unwrap(),
            &10
        );
    }

    #[test]
    fn test_get_none() {
        let export = export!((path!(Rep(0), Nbr(0)), 10));
        assert_eq!(export.get::<String>(&Path::from(vec![Rep(0), Nbr(0)])), None);
    }

    #[test]
    fn test_root() {
        let export = export!((Path::new(), 10));
        assert_eq!(export.root::<i32>(), &10);
    }

    #[test]
    #[should_panic]
    fn test_root_panic() {
        let export = export!((Path::new(), 10));
        assert_eq!(export.root::<String>(), &"foo");
    }

    #[test]
    fn test_paths() {
        let export = export!((Path::new(), 10));
        let mut map2: HashMap<Path, Rc<Box<dyn Any>>> = HashMap::new();
        map2.insert(Path::new(), Rc::new(Box::new(10)));
        assert!(export.map.keys().eq(map2.keys()));
    }

    #[test]
    fn test_empty_state() {
        let export: Export = Export::new();
        let path = path!(Nbr(0), Rep(0));
        assert_eq!(export.get::<i32>(&Path::new()), None);
        assert_eq!(export.get::<i32>(&path), None);
    }

    #[test]
    fn test_root_path() {
        let mut export: Export = Export::new();
        export.put(Path::new(), ||String::from("foo"));
        assert_eq!(
            export.get::<String>(&Path::new()).unwrap(),
            export.root::<String>()
        );
        assert_eq!(
            export.get::<String>(&Path::new()),
            Some(&String::from("foo"))
        );
    }

    #[test]
    fn test_non_root_path() {
        let mut export: Export = Export::new();
        let path = path!(Nbr(0), Rep(0));
        export.put(path.clone(), || String::from("bar"));
        assert_eq!(export.get::<String>(&path).unwrap(), &String::from("bar"));
    }

    #[test]
    fn test_overwriting_with_different_type() {
        let mut export: Export = Export::new();
        export.put(Path::new(), || String::from("foo"));
        assert_eq!(
            export.get::<String>(&Path::new()),
            Some(&String::from("foo"))
        );
        export.put(Path::new(), || 77);
        assert_eq!(export.get::<i32>(&Path::new()).unwrap(), &77);
    }

    #[test]
    fn test_serialize_and_deserialize() {
        let export = export![
            (path!(Rep(0), Nbr(0)), 10),
            (path!(Nbr(0)), 10),
            (path!(Rep(0)), 10),
            (Path::new(), 10)
        ];

        export.map.iter().for_each(|(k, v)| {
            println!("{}", k);
        });
        let export_ser = serde_json::to_string(&export).unwrap();
        println!("{}", export_ser);
        let export_des: Export = serde_json::from_str(&export_ser).unwrap();
        println!("{}", export_des);
    }

}
