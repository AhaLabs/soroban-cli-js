//! Test suite for the Web and headless browsers.

#![cfg(target_arch = "wasm32")]

extern crate wasm_bindgen_test;
use wasm_bindgen_test::*;

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


wasm_bindgen_test_configure!(run_in_browser);

#[wasm_bindgen_test]
async fn pass() {
    let sever = Server::new("localhost:8001".to_string());
    
}
