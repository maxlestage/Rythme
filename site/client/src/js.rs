//! Small helpers over JavaScript values, for the APIs that a browser may not have (the
//! Web Share API, `launchQueue`, `CompressionStream`, the clipboard of an insecure
//! page…) or that a page may replace: they are looked up when used, and calling them
//! returns an error rather than throwing through Rust.

use js_sys::{Array, Function, Promise, Reflect};
use wasm_bindgen::{JsCast, JsValue};
use wasm_bindgen_futures::JsFuture;

/// `object[key]`, `undefined` when it cannot be read.
pub fn get(object: &JsValue, key: &str) -> JsValue {
    Reflect::get(object, &JsValue::from_str(key)).unwrap_or(JsValue::UNDEFINED)
}

/// `object[key] = value`; false when it cannot be written.
pub fn set(object: &JsValue, key: &str, value: &JsValue) -> bool {
    Reflect::set(object, &JsValue::from_str(key), value).unwrap_or(false)
}

/// `key in object`.
pub fn has(object: &JsValue, key: &str) -> bool {
    object.is_object() && Reflect::has(object, &JsValue::from_str(key)).unwrap_or(false)
}

/// `object[key]` as a string, if it is one.
pub fn string(object: &JsValue, key: &str) -> Option<String> {
    get(object, key).as_string()
}

/// `object[method](...args)`: an error when there is no such method or when it throws.
pub fn call(object: &JsValue, method: &str, args: &[JsValue]) -> Result<JsValue, JsValue> {
    let function: Function = get(object, method).dyn_into().map_err(|_| JsValue::from_str(&format!("no {method}")))?;
    function.apply(object, &args.iter().collect::<Array>())
}

/// The result of a promise (or of any value, as `await` gives it).
pub async fn wait(value: JsValue) -> Result<JsValue, JsValue> {
    let promise = Promise::resolve(&value);
    JsFuture::from(promise).await
}

/// `object[method](...args)`, awaited.
pub async fn call_async(object: &JsValue, method: &str, args: &[JsValue]) -> Result<JsValue, JsValue> {
    wait(call(object, method, args)?).await
}

/// `new Constructor(...args)` of a global constructor, if the browser has it.
pub fn construct(name: &str, args: &[JsValue]) -> Result<JsValue, JsValue> {
    let constructor: Function = get(&js_sys::global(), name).dyn_into().map_err(|_| JsValue::from_str(&format!("no {name}")))?;
    Reflect::construct(&constructor, &args.iter().collect::<Array>())
}

/// A JavaScript object from pairs of keys and values.
pub fn object(fields: &[(&str, JsValue)]) -> JsValue {
    let object = js_sys::Object::new();
    for (key, value) in fields {
        set(&object, key, value);
    }
    object.into()
}

/// The message of an error, as `String(err)` writes it.
pub fn describe(error: &JsValue) -> String {
    call(&js_sys::global(), "String", std::slice::from_ref(error)).ok().and_then(|s| s.as_string()).unwrap_or_else(|| format!("{error:?}"))
}
