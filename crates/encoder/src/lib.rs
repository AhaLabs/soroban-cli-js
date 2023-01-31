use soroban_env_host::{
    xdr::{WriteXdr},
};
use std::{sync::Once};

use strval::Spec;

wit_bindgen_guest_rust::generate!("wasm-xdr-tools");

struct WasmToolsJs(Vec<u8>);

export_wasm_xdr_tools_js!(WasmToolsJs);

fn init() {
    static INIT: Once = Once::new();
    INIT.call_once(|| {
        let prev_hook = std::panic::take_hook();
        std::panic::set_hook(Box::new(move |info| {
            console::error(&info.to_string());
            prev_hook(info);
        }));
    });
}

impl exports::Exports for WasmToolsJs {
    fn create_op(
        wasm: Vec<u8>,
        contract_id: String,
        func_name: String,
        json_args: String,
    ) -> Result<Vec<u8>, String> {
        init();
        Spec::from_wasm(&wasm)
            .and_then(|spec| {
                let contract_id_bytes = strval::utils::id_from_str(&contract_id).unwrap();
                Ok(spec
                    .create_op(&contract_id_bytes, &func_name, &json_args)?
                    .to_xdr()?)
            })
            .map_err(|e| e.to_string())
    }

    fn decode_ret(wasm: Vec<u8>, func_name: String, xdr_ret: Vec<u8>) -> Result<String, String> {
        init();
        Spec::from_wasm(&wasm)
            .and_then(|spec| spec.decode_args(&func_name, &xdr_ret))
            .map_err(|e| e.to_string())
    }

    fn run(
        wasm: Vec<u8>,
        contract_id: String,
        func_name: String,
        json_args: String,
    ) -> Result<String, String> {
        init();
        Spec::from_wasm(&wasm)
            .and_then(|spec| {
                spec.run(wasm, &contract_id, &func_name, &json_args)
            })
            .map_err(|e| e.to_string())
    }
}

