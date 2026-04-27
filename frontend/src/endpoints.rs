use wasm_bindgen::JsCast;

pub fn websocket_url() -> String {
    let scope = js_sys::global().unchecked_into::<web_sys::WorkerGlobalScope>();
    let location = scope.location();
    let scheme = if location.protocol() == "https:" {
        "wss"
    } else {
        "ws"
    };

    format!("{}://{}/ws", scheme, location.host())
}
