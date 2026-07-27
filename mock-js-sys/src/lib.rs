use wasm_bindgen::JsValue;

#[derive(Copy, Clone, Debug, Default, PartialEq, Eq)]
pub struct Object;

impl Object {
    pub fn new() -> Self {
        Object
    }
}

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct JsString(pub String);

impl From<&str> for JsString {
    fn from(s: &str) -> Self {
        JsString(s.to_string())
    }
}

impl From<String> for JsString {
    fn from(s: String) -> Self {
        JsString(s)
    }
}

impl From<JsString> for JsValue {
    fn from(s: JsString) -> Self {
        JsValue(Some(s.0))
    }
}

impl std::fmt::Display for JsString {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl From<JsString> for String {
    fn from(s: JsString) -> Self {
        s.0
    }
}

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct Array;
