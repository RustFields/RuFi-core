use std::any::Any;
use std::collections::HashMap;
use std::fmt::{Debug, Display, Formatter};
use std::rc::Rc;
use std::str::FromStr;
use serde::{Deserialize, Serialize};
use sede::{serialize_rc_box_any_map, deserialize_rc_box_any_map};
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

    /// Returns the value at the given Path, deserializing it from a String.
    /// This method is needed to retrieve a value from a deserialized Export.
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
    /// The deserialized value at the given Path.
    ///
    /// # Panics
    /// * Panics if there is not a value at the given Path.
    pub fn get_deserialized<A>(&self, path: &Path) -> Result<A, <A as FromStr>::Err>
        where
            A: FromStr,
    {
        let value_str = self.get::<String>(path).unwrap();
        A::from_str(value_str)
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
    ///
    /// # Panics
    /// * Panics if there is not a root value (a value at the empty Path).
    /// * Panics if the type of the root value is not the same as the type of the requested value.
    pub fn root<A: 'static>(&self) -> &A {
        self.get(&Path::new()).unwrap()
    }

    /// Obtain the root value, deserializing it from a String.
    /// This method is needed to retrieve a value from a deserialized Export.
    ///
    /// # Generic Parameters
    ///
    /// * `A` - The type of the value to return. It must have a `'static` lifetime.
    ///
    /// # Returns
    ///
    /// The deserialized root value.
    ///
    /// # Panics
    /// * Panics if there is not a root value (a value at the empty Path).
    /// * Panics if the type of the root value is not the same as the type of the requested value.
    pub fn root_deserialized<A>(&self) -> Result<A, <A as FromStr>::Err>
        where
            A: FromStr,
    {
        self.get_deserialized(&Path::new())
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

/// This private module is needed to serialize and deserialize the HashMap<Path, Rc<Box<dyn Any>>>.
mod sede {
    use std::any::Any;
    use std::collections::HashMap;
    use std::rc::Rc;
    use serde::{Deserializer, Serialize, Serializer};
    use serde::de::Visitor;
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

    struct ExportMapVisitor;

    impl<'de> Visitor<'de> for ExportMapVisitor {
        type Value = HashMap<Path, Rc<Box<dyn Any>>>;
        fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
            formatter.write_str("a map of Paths and Any")
        }

        fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
        {
            let mut result = HashMap::new();
            while let Some((key, value)) = map.next_entry::<String, String>()? {
                let path: Path = serde_json::from_str(&key).unwrap();
                let value: Rc<Box<dyn Any>> = Rc::new(Box::new(value) as Box<dyn Any>);
                result.insert(path, value);
            }
            Ok(result)
        }
    }

    pub fn deserialize_rc_box_any_map<'de, D>(deserializer: D) -> Result<HashMap<Path, Rc<Box<dyn Any>>>, D::Error>
        where
            D: Deserializer<'de>,
    {
        deserializer.deserialize_map(ExportMapVisitor)
    }
}

impl Display for Export {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let string = serde_json::to_string(&self);
        write!(f, "{}", string.unwrap())
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
        let export_ser = serde_json::to_string(&export).unwrap();
        // this deserialized export will have string representations of the values of the original map
        // thus we need to parse the values of the deserialized export to get the actual values
        let export_des: Export = serde_json::from_str(&export_ser).unwrap();
        let value_at_nbr = export.get::<i32>(&path!(Nbr(0))).unwrap();
        let value_at_nbr_des = export_des.get_deserialized::<i32>(&path!(Nbr(0))).unwrap();
        assert_eq!(value_at_nbr, &value_at_nbr_des);
        let root_value = export.root::<i32>();
        let root_value_des = export_des.root_deserialized::<i32>().unwrap();
        assert_eq!(root_value, &root_value_des);
    }
}
