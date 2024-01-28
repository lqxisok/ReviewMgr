use wasm_bindgen::prelude::*;
use web_sys::{Element};

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "tauri"])]
    pub async fn invoke(cmd: &str, args: JsValue) -> JsValue;
}

#[wasm_bindgen]
extern "C" {
    // 声明 KaTeX 的 renderMathInElement 函数
    #[wasm_bindgen(js_namespace = katex)]
    pub fn renderMathInElement(elem: &Element, options: &JsValue);
}

#[wasm_bindgen]
extern "C" {
    // 声明 KaTeX 的 renderMathInElement 函数
    #[wasm_bindgen]
    pub fn refresh_render();
}


#[wasm_bindgen]
extern "C" {
    // 声明 KaTeX 的 renderMathInElement 函数
    #[wasm_bindgen]
    pub fn prevent_a_link();
}
