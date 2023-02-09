use wasm_bindgen::prelude::*;

#[wasm_bindgen(module = "/server.js")]
extern "C" {
    
    type Server;

    #[wasm_bindgen(constructor)]
    fn new(url: String) -> Server;

    #[wasm_bindgen(method)]
    fn friendbot(this: &Server, address: String) -> js_sys::Promise;
    #[wasm_bindgen(method)]
    fn send_transaction(this: &Server) -> js_sys::Promise;
}
