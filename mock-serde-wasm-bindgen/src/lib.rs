use wasm_bindgen::JsValue;

pub fn to_value<T: serde::Serialize + ?Sized>(_value: &T) -> Result<JsValue, &'static str> {
    Ok(JsValue)
}

pub fn from_value<T: serde::de::DeserializeOwned>(_value: JsValue) -> Result<T, &'static str> {
    Err("from_value not implemented in mock")
}
