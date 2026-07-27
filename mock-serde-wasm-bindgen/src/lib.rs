use wasm_bindgen::JsValue;

pub fn to_value<T: serde::Serialize + ?Sized>(value: &T) -> Result<JsValue, &'static str> {
    let s = serde_json::to_string(value).map_err(|_| "serialization error")?;
    Ok(JsValue(Some(s)))
}

pub fn from_value<T: serde::de::DeserializeOwned>(value: JsValue) -> Result<T, &'static str> {
    if let Some(s) = value.0 {
        serde_json::from_str(&s).map_err(|_| "deserialization error")
    } else {
        Err("JsValue is null or undefined")
    }
}
