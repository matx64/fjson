use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn fix(input: &str) -> String {
    fjson::fix(input)
}
