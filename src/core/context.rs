use crate::core::context::context::Context;

pub mod context {

    #[derive(PartialEq, Debug, Clone, Eq, Hash)]
    pub struct Context {
        pub(crate) self_id: i32,
    }
}

impl Context {

}