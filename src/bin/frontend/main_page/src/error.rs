use js_sys::Array;
use std::panic;
use wasm_bindgen::JsValue;

trait ErrorTrait {
    fn log_error(&self) -> !;
}

impl ErrorTrait for String {
    fn log_error(&self) -> ! {
        web_sys::console::error(&Array::from(&JsValue::from(self)));
        panic!("{}", self)
    }
}

impl ErrorTrait for JsValue {
    fn log_error(&self) -> ! {
        web_sys::console::error(&Array::from(self));
        panic!(
            "{}",
            self.as_string()
                .unwrap_or("battleship error: JsValue has not decomposed into String".to_string())
        )
    }
}
