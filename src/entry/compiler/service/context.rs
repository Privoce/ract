use std::any::Any;
use std::collections::HashSet;
use std::fmt::Debug;
use std::hash::{Hash, Hasher};

#[derive(Debug, Default)]
pub struct Context {
    pub datas: HashSet<Box<dyn ContextData>>,
}

impl Context {
    pub fn insert<T>(&mut self, item: T)
    where
        T: ContextData + 'static,
    {
        self.datas.insert(Box::new(item));
    }

    pub fn clear(&mut self) {
        self.datas.clear();
    }
}

pub trait ContextData: Debug + Any {
    fn as_any(&self) -> &dyn Any;
}

impl<T: Debug + Any> ContextData for T {
    fn as_any(&self) -> &dyn Any {
        self
    }
}

// 自定义 PartialEq 和 Hash
impl PartialEq for dyn ContextData {
    fn eq(&self, other: &Self) -> bool {
        self.as_any().type_id() == other.as_any().type_id()
    }
}

impl Eq for dyn ContextData {}

impl Hash for dyn ContextData {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.as_any().type_id().hash(state);
    }
}
