use wasm_bindgen::JsValue;

#[derive(Copy, Clone, Debug, Default, PartialEq, Eq)]
pub struct Object;

impl Object {
    pub fn new() -> Self {
        Object
    }
}

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct JsString;

impl From<&str> for JsString {
    fn from(_: &str) -> Self {
        JsString
    }
}

impl From<JsString> for JsValue {
    fn from(_: JsString) -> Self {
        JsValue(None)
    }
}

impl std::fmt::Display for JsString {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "")
    }
}

impl From<JsString> for String {
    fn from(_: JsString) -> Self {
        String::new()
    }
}

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct Array;
