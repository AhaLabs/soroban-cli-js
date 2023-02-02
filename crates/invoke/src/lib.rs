use std::{sync::Once};

use strval::Spec;

wit_bindgen_guest_rust::generate!("exports");

struct WasmToolsJs(Vec<u8>);

export_invoker!(WasmToolsJs);

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

impl invoker::Invoker for WasmToolsJs {

    fn invoke(
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

