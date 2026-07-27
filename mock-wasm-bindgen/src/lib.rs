pub use mock_wasm_bindgen_macro::wasm_bindgen;

pub mod prelude {
    pub use crate::wasm_bindgen;
}

#[derive(Clone, Debug, Default, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct JsValue(pub Option<String>);

impl JsValue {
    pub const NULL: JsValue = JsValue(None);
    pub const UNDEFINED: JsValue = JsValue(None);

    pub fn is_null(&self) -> bool { self.0.is_none() }
    pub fn is_undefined(&self) -> bool { self.0.is_none() }
    pub fn as_string(&self) -> Option<String> { self.0.clone() }
    pub fn as_f64(&self) -> Option<f64> { None }
    pub fn as_bool(&self) -> Option<bool> { None }
}

pub trait JsCast {
    fn dyn_into<T>(self) -> Result<T, Self> where Self: Sized {
        Ok(unsafe { std::mem::transmute_copy(&self) })
    }
    fn dyn_ref<T>(&self) -> Option<&T> {
        None
    }
    fn is_instance_of<T>(&self) -> bool {
        false
    }
    fn unchecked_into<T>(self) -> T where Self: Sized {
        unsafe { std::mem::transmute_copy(&self) }
    }
    fn unchecked_ref<T>(&self) -> &T {
        unsafe { &*(self as *const Self as *const T) }
    }
}

impl<T> JsCast for T {}
